use std::sync::Arc;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use cpc_core::models::social::post::{MediaItem, MediaType, ProcessingStatus};
use tokio::fs;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use tokio::sync::{mpsc, RwLock};
use futures::StreamExt;
use std::collections::HashMap;

use crate::repositories::media_repository::MediaRepository;
use crate::models::media::{CreateMediaItem, UpdateMediaItem, MediaProcessingJob, ProcessingResult};
use crate::grpc::media_processor_adapter::{GrpcMediaProcessor, RetryConfig, TimeoutConfig};

#[derive(Debug, thiserror::Error)]
pub enum MediaServiceError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Invalid file type")]
    InvalidFileType,
    #[error("File too large")]
    FileTooLarge,
    #[error("Storage error: {0}")]
    Storage(String),
    #[error("Processing error: {0}")]
    Processing(String),
    #[error("gRPC error: {0}")]
    Grpc(String),
    #[error("Media not found")]
    MediaNotFound,
}

#[derive(Debug, Clone)]
pub enum MediaProcessingEvent {
    StatusChanged {
        media_id: Uuid,
        status: ProcessingStatus,
        progress: Option<u32>,
    },
    ProcessingStarted {
        media_id: Uuid,
        job_id: String,
    },
    ProcessingCompleted {
        media_id: Uuid,
        result_url: String,
        thumbnail_url: Option<String>,
    },
    ProcessingFailed {
        media_id: Uuid,
        error: String,
    },
    RetryScheduled {
        media_id: Uuid,
        retry_count: u32,
    },
}

#[derive(Clone)]
pub struct MediaService {
    repo: Arc<dyn MediaRepository>,
    storage_path: PathBuf,
    max_file_size: u64,
    grpc_processor: Arc<GrpcMediaProcessor>,
    event_subscribers: Arc<RwLock<HashMap<Uuid, Vec<mpsc::UnboundedSender<MediaProcessingEvent>>>>>,
}

impl MediaService {
    pub fn new(
        repo: Arc<dyn MediaRepository>,
        storage_path: PathBuf,
        max_file_size: u64,
        grpc_processor: Arc<GrpcMediaProcessor>,
    ) -> Self {
        Self {
            repo,
            storage_path,
            max_file_size,
            grpc_processor,
            event_subscribers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn upload_media(
        &self,
        file_path: PathBuf,
        media_type: MediaType,
        post_id: Uuid,
    ) -> Result<MediaItem, MediaServiceError> {
        // Validate file
        let metadata = fs::metadata(&file_path).await?;
        if metadata.len() > self.max_file_size {
            return Err(MediaServiceError::FileTooLarge);
        }

        // Validate file type
        let actual_media_type = self.validate_file_type(&file_path).await?;
        if actual_media_type != media_type {
            return Err(MediaServiceError::InvalidFileType);
        }

        // Generate unique filename
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let file_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        
        let filename = format!("{}_{}_{}.{}", post_id, timestamp, media_type, file_extension);
        let dest_path = self.storage_path.join(&filename);

        // Create storage directory if it doesn't exist
        fs::create_dir_all(&self.storage_path).await?;

        // Copy file to storage
        fs::copy(&file_path, &dest_path).await?;

        // Generate URL
        let url = format!("media://{}", filename);

        // Create media item in database
        let create_item = CreateMediaItem {
            post_id,
            url: url.clone(),
            media_type,
            file_size: Some(metadata.len() as i64),
            mime_type: None,
            original_filename: None,
            metadata: None,
        };

        let media_item = self.repo
            .create_media_item(create_item)
            .await
            .map_err(MediaServiceError::Database)?;

        // Start processing asynchronously
        let service = self.clone();
        tokio::spawn(async move {
            let _ = service.process_media_async(media_item.id).await;
        });

        Ok(media_item)
    }

    pub async fn process_media_async(&self, media_id: Uuid) -> Result<(), MediaServiceError> {
        // Get media item
        let media_item = self.repo
            .find_by_id(media_id)
            .await
            .map_err(MediaServiceError::Database)?
            .ok_or_else(|| MediaServiceError::MediaNotFound)?;

        // Only process if in pending or failed state
        if !media_item.can_be_processed() {
            return Ok(());
        }

        // Update status to processing
        self.repo
            .update_processing_status(media_id, ProcessingStatus::Processing)
            .await
            .map_err(MediaServiceError::Database)?;

        self.broadcast_event(MediaProcessingEvent::StatusChanged {
            media_id,
            status: ProcessingStatus::Processing,
            progress: Some(0),
        });

        // Create processing job
        let job = MediaProcessingJob {
            media_id: media_item.id,
            media_type: media_item.media_type,
            source_url: media_item.url.clone(),
            target_formats: self.get_target_formats(media_item.media_type),
            quality: "high".to_string(),
        };

        // Process media using gRPC
        match self.grpc_processor.process_media(job).await {
            Ok(result) => {
                // Update media item with processed results
                let update_item = UpdateMediaItem {
                    processing_status: Some(ProcessingStatus::Completed),
                    url: Some(result.processed_url.clone()),
                    metadata: result.metadata,
                    processed_at: Some(Utc::now()),
                    ..Default::default()
                };

                self.repo
                    .update_media_item(media_id, update_item)
                    .await
                    .map_err(MediaServiceError::Database)?;

                self.broadcast_event(MediaProcessingEvent::ProcessingCompleted {
                    media_id,
                    result_url: result.processed_url,
                    thumbnail_url: result.thumbnail_url,
                });
            }
            Err(e) => {
                // Update status to failed
                self.repo
                    .update_processing_status(media_id, ProcessingStatus::Failed)
                    .await
                    .map_err(MediaServiceError::Database)?;

                self.broadcast_event(MediaProcessingEvent::ProcessingFailed {
                    media_id,
                    error: e.to_string(),
                });
            }
        }

        Ok(())
    }

    pub async fn retry_media_processing(&self, media_id: Uuid) -> Result<(), MediaServiceError> {
        let media_item = self.repo
            .find_by_id(media_id)
            .await
            .map_err(MediaServiceError::Database)?
            .ok_or_else(|| MediaServiceError::MediaNotFound)?;

        if media_item.processing_status != ProcessingStatus::Failed {
            return Ok(());
        }

        // Reset status to pending
        self.repo
            .update_processing_status(media_id, ProcessingStatus::Pending)
            .await
            .map_err(MediaServiceError::Database)?;

        self.broadcast_event(MediaProcessingEvent::RetryScheduled {
            media_id,
            retry_count: 1,
        });

        // Start processing
        let service = self.clone();
        tokio::spawn(async move {
            let _ = service.process_media_async(media_id).await;
        });

        Ok(())
    }

    pub async fn get_media_for_post(&self, post_id: Uuid) -> Result<Vec<MediaItem>, MediaServiceError> {
        let media_items = self.repo
            .find_by_post_id(post_id)
            .await
            .map_err(MediaServiceError::Database)?;
        
        Ok(media_items)
    }

    pub async fn get_pending_media(&self) -> Result<Vec<MediaItem>, MediaServiceError> {
        let media_items = self.repo
            .find_pending_media()
            .await
            .map_err(MediaServiceError::Database)?;
        
        Ok(media_items)
    }

    pub async fn get_processing_status(&self, media_id: Uuid) -> Result<ProcessingStatus, MediaServiceError> {
        let media_item = self.repo
            .find_by_id(media_id)
            .await
            .map_err(MediaServiceError::Database)?
            .ok_or_else(|| MediaServiceError::MediaNotFound)?;
        
        Ok(media_item.processing_status)
    }

    pub async fn subscribe_to_media_updates(
        &self,
        media_id: Uuid,
    ) -> Result<mpsc::UnboundedReceiver<MediaProcessingEvent>, MediaServiceError> {
        let (tx, rx) = mpsc::unbounded_channel();
        
        let mut subscribers = self.event_subscribers.write().await;
        subscribers
            .entry(media_id)
            .or_insert_with(Vec::new)
            .push(tx);
        
        Ok(rx)
    }

    fn broadcast_event(&self, event: MediaProcessingEvent) {
        let media_id = match &event {
            MediaProcessingEvent::StatusChanged { media_id, .. } => *media_id,
            MediaProcessingEvent::ProcessingStarted { media_id, .. } => *media_id,
            MediaProcessingEvent::ProcessingCompleted { media_id, .. } => *media_id,
            MediaProcessingEvent::ProcessingFailed { media_id, .. } => *media_id,
            MediaProcessingEvent::RetryScheduled { media_id, .. } => *media_id,
        };

        tokio::spawn({
            let subscribers = self.event_subscribers.clone();
            async move {
                let mut subscribers = subscribers.write().await;
                if let Some(subs) = subscribers.get_mut(&media_id) {
                    subs.retain(|tx| tx.send(event.clone()).is_ok());
                }
            }
        });
    }

    async fn validate_file_type(&self, file_path: &Path) -> Result<MediaType, MediaServiceError> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" | "bmp" => Ok(MediaType::Image),
            "mp4" | "avi" | "mov" | "mkv" | "webm" | "flv" => Ok(MediaType::Video),
            "mp3" | "wav" | "flac" | "aac" | "ogg" | "m4a" => Ok(MediaType::Audio),
            "pdf" | "doc" | "docx" | "txt" | "ppt" | "pptx" => Ok(MediaType::Document),
            _ => Ok(MediaType::Unknown),
        }
    }

    fn get_target_formats(&self, media_type: MediaType) -> Vec<String> {
        match media_type {
            MediaType::Image => vec![
                "thumbnail".to_string(),
                "small".to_string(),
                "medium".to_string(),
                "large".to_string(),
                "original".to_string(),
            ],
            MediaType::Video => vec![
                "thumbnail".to_string(),
                "240p".to_string(),
                "480p".to_string(),
                "720p".to_string(),
                "1080p".to_string(),
                "original".to_string(),
            ],
            MediaType::Audio => vec![
                "mp3".to_string(),
                "aac".to_string(),
                "wav".to_string(),
                "original".to_string(),
            ],
            _ => vec!["original".to_string()],
        }
    }
}

impl Default for UpdateMediaItem {
    fn default() -> Self {
        Self {
            processing_status: None,
            url: None,
            file_size: None,
            metadata: None,
            processed_at: None,
        }
    }
}