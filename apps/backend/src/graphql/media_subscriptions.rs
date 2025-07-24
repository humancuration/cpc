use async_graphql::*;
use futures::Stream;
use std::pin::Pin;
use uuid::Uuid;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::services::media_service_enhanced::{MediaService, MediaProcessingEvent};
use crate::models::media::{ProcessingUpdate, ProcessingStatus};

#[derive(Default)]
pub struct MediaSubscriptions;

#[derive(SimpleObject, Clone)]
pub struct MediaStatusUpdate {
    pub media_id: Uuid,
    pub status: String,
    pub progress: Option<u32>,
    pub message: Option<String>,
    pub processed_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub error: Option<String>,
}

impl From<MediaProcessingEvent> for MediaStatusUpdate {
    fn from(event: MediaProcessingEvent) -> Self {
        match event {
            MediaProcessingEvent::StatusChanged { media_id, status, progress } => {
                MediaStatusUpdate {
                    media_id,
                    status: status.to_string(),
                    progress,
                    message: None,
                    processed_url: None,
                    thumbnail_url: None,
                    error: None,
                }
            }
            MediaProcessingEvent::ProcessingCompleted { media_id, result_url, thumbnail_url } => {
                MediaStatusUpdate {
                    media_id,
                    status: "completed".to_string(),
                    progress: Some(100),
                    message: Some("Processing completed successfully".to_string()),
                    processed_url: Some(result_url),
                    thumbnail_url,
                    error: None,
                }
            }
            MediaProcessingEvent::ProcessingFailed { media_id, error } => {
                MediaStatusUpdate {
                    media_id,
                    status: "failed".to_string(),
                    progress: None,
                    message: Some(error.clone()),
                    processed_url: None,
                    thumbnail_url: None,
                    error: Some(error),
                }
            }
            MediaProcessingEvent::RetryScheduled { media_id, retry_count } => {
                MediaStatusUpdate {
                    media_id,
                    status: "retrying".to_string(),
                    progress: None,
                    message: Some(format!("Retry scheduled (attempt {})", retry_count)),
                    processed_url: None,
                    thumbnail_url: None,
                    error: None,
                }
            }
            MediaProcessingEvent::ProcessingStarted { media_id, job_id } => {
                MediaStatusUpdate {
                    media_id,
                    status: "processing".to_string(),
                    progress: Some(0),
                    message: Some(format!("Processing started (job: {})", job_id)),
                    processed_url: None,
                    thumbnail_url: None,
                    error: None,
                }
            }
        }
    }
}

#[Subscription]
impl MediaSubscriptions {
    /// Subscribe to media processing status updates
    async fn media_status_updated(
        &self,
        ctx: &Context<'_>,
        media_id: Uuid,
    ) -> Result<impl Stream<Item = MediaStatusUpdate>> {
        let service = ctx.data_unchecked::<Arc<MediaService>>();
        
        let rx = service
            .subscribe_to_media_updates(media_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        let stream = UnboundedReceiverStream::new(rx)
            .map(MediaStatusUpdate::from);
        
        Ok(stream)
    }

    /// Subscribe to all media processing events for a post
    async fn post_media_status_updated(
        &self,
        ctx: &Context<'_>,
        post_id: Uuid,
    ) -> Result<impl Stream<Item = MediaStatusUpdate>> {
        let service = ctx.data_unchecked::<Arc<MediaService>>();
        
        // Get all media items for this post
        let media_items = service
            .get_media_for_post(post_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        // Create a combined stream for all media items
        let (tx, rx) = mpsc::unbounded_channel();
        
        for media_item in media_items {
            let service = service.clone();
            let tx = tx.clone();
            
            tokio::spawn(async move {
                if let Ok(rx) = service.subscribe_to_media_updates(media_item.id).await {
                    let mut stream = UnboundedReceiverStream::new(rx);
                    while let Some(event) = stream.next().await {
                        let _ = tx.send(MediaStatusUpdate::from(event));
                    }
                }
            });
        }
        
        let stream = UnboundedReceiverStream::new(rx);
        Ok(stream)
    }

    /// Subscribe to global media processing events (for admin dashboard)
    async fn all_media_status_updated(
        &self,
        ctx: &Context<'_>,
    ) -> Result<impl Stream<Item = MediaStatusUpdate>> {
        // This would typically require admin permissions
        let _ = ctx.data_unchecked::<Arc<MediaService>>();
        
        // For now, return empty stream - implement based on requirements
        let (_tx, rx) = mpsc::unbounded_channel();
        let stream = UnboundedReceiverStream::new(rx);
        Ok(stream)
    }
}

/// Helper function to create a subscription filter
pub fn create_media_subscription_filter(
    media_id: Uuid,
) -> impl Fn(&MediaStatusUpdate) -> bool + Clone {
    move |update| update.media_id == media_id
}