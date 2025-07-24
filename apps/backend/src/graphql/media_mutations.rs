use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;
use chrono;

use cpc_core::models::social::post::{MediaType, ProcessingStatus};

use crate::services::media_service::{MediaService, MediaServiceError};

#[derive(Default)]
pub struct MediaMutation;

#[derive(SimpleObject)]
pub struct MediaUpload {
    pub upload_url: String,
    pub media_id: Uuid,
    pub expires_at: i64,
}

#[derive(SimpleObject)]
pub struct MediaStatus {
    pub media_id: Uuid,
    pub status: String,
    pub progress: Option<u32>,
    pub url: String,
}

#[derive(InputObject)]
pub struct CreateMediaUploadInput {
    pub filename: String,
    pub content_type: String,
    pub file_size: i64,
}

#[derive(InputObject)]
pub struct CompleteMediaUploadInput {
    pub media_id: Uuid,
    pub processing_status: String,
}

#[derive(InputObject)]
pub struct ProcessMediaInput {
    pub media_id: Uuid,
    pub target_formats: Vec<String>,
    pub quality: String,
}

#[Object]
impl MediaMutation {
    /// Create a presigned URL for media upload
    async fn create_media_upload(
        &self,
        ctx: &Context<'_>,
        input: CreateMediaUploadInput,
    ) -> Result<MediaUpload> {
        let media_service = ctx.data_unchecked::<Arc<MediaService>>();
        
        // Determine media type based on content type
        let media_type = match input.content_type.as_str() {
            "image/jpeg" | "image/png" | "image/gif" | "image/webp" => MediaType::Image,
            "video/mp4" | "video/webm" | "video/quicktime" => MediaType::Video,
            "audio/mpeg" | "audio/wav" | "audio/ogg" => MediaType::Audio,
            _ => MediaType::Unknown,
        };

        // Create a temporary media item to get an ID
        let create_item = crate::models::media::CreateMediaItem {
            post_id: None,
            url: format!("pending://{}", input.filename),
            media_type,
            file_size: Some(input.file_size),
            mime_type: Some(input.content_type.clone()),
            original_filename: Some(input.filename.clone()),
            metadata: None,
        };

        let media_item = media_service
            .repo
            .create_media_item(create_item)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        // Generate a presigned URL
        let upload_url = format!("/api/media/upload?id={}&type={}", media_item.id, media_type);

        Ok(MediaUpload {
            upload_url,
            media_id: media_item.id,
            expires_at: chrono::Utc::now().timestamp() + 3600, // 1 hour expiry
        })
    }

    /// Complete a media upload and trigger processing
    async fn complete_media_upload(
        &self,
        ctx: &Context<'_>,
        input: CompleteMediaUploadInput,
    ) -> Result<bool> {
        let media_service = ctx.data_unchecked::<Arc<MediaService>>();
        
        // Parse processing status
        let status = match input.processing_status.as_str() {
            "COMPLETED" => ProcessingStatus::Completed,
            "FAILED" => ProcessingStatus::Failed,
            "PROCESSING" => ProcessingStatus::Processing,
            _ => ProcessingStatus::Pending,
        };

        // Update media processing status
        media_service
            .repo
            .update_processing_status(input.media_id, status)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        // If completed, start processing
        if status == ProcessingStatus::Completed {
            tokio::spawn({
                let media_service = media_service.clone();
                async move {
                    let _ = media_service.process_media_async(input.media_id).await;
                }
            });
        }

        Ok(true)
    }

    /// Create a new media item and start processing
    async fn create_media(
        &self,
        ctx: &Context<'_>,
        post_id: Uuid,
        url: String,
        media_type: MediaType,
        file_size: Option<i64>,
        mime_type: Option<String>,
        original_filename: Option<String>,
    ) -> Result<MediaItem> {
        let media_service = ctx.data_unchecked::<Arc<MediaService>>();
        
        let create_item = crate::models::media::CreateMediaItem {
            post_id,
            url,
            media_type,
            file_size,
            mime_type,
            original_filename,
            metadata: None,
        };

        let media_item = media_service
            .repo
            .create_media_item(create_item)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        // Start processing asynchronously
        tokio::spawn({
            let media_service = media_service.clone();
            async move {
                let _ = media_service.process_media_async(media_item.id).await;
            }
        });

        Ok(media_item)
    }

    /// Get media items for a post
    async fn get_media_for_post(
        &self,
        ctx: &Context<'_>,
        post_id: Uuid,
    ) -> Result<Vec<MediaItem>> {
        let media_service = ctx.data_unchecked::<Arc<MediaService>>();
        
        let media_items = media_service
            .repo
            .find_by_post_id(post_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(media_items)
    }

    /// Get media processing status
    async fn get_media_status(
        &self,
        ctx: &Context<'_>,
        media_id: Uuid,
    ) -> Result<MediaStatus> {
        let media_service = ctx.data_unchecked::<Arc<MediaService>>();
        
        let media_item = media_service
            .repo
            .find_by_id(media_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?
            .ok_or_else(|| Error::new("Media not found".to_string()))?;

        Ok(MediaStatus {
            media_id: media_item.id,
            status: media_item.processing_status.to_string(),
            progress: None,
            url: media_item.url,
        })
    }

    /// Delete a media item
    async fn delete_media(
        &self,
        ctx: &Context<'_>,
        media_id: Uuid,
    ) -> Result<bool> {
        let media_service = ctx.data_unchecked::<Arc<MediaService>>();
        
        media_service
            .repo
            .delete_media_item(media_id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        Ok(true)
    }
}