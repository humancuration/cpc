use std::sync::Arc;
use std::path::{Path, PathBuf};
use uuid::Uuid;
use cpc_core::models::social::post::{MediaItem, MediaType, ProcessingStatus};
use tokio::fs;
use sha2::{Sha256, Digest};
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;
use futures::StreamExt;

use crate::repositories::media_repository::MediaRepository;
use crate::models::media::{CreateMediaItem, UpdateMediaItem, MediaProcessingJob, ProcessingResult};
use crate::grpc::media_processor_adapter::GrpcMediaProcessor;

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
}

#[derive(Clone)]
pub struct MediaService {
    repo: Arc<dyn MediaRepository>,
    storage_path: PathBuf,
    max_file_size: u64,
    grpc_client: Arc<crate::grpc::media_processor_client::MediaProcessorClient>,
    event_tx: mpsc::UnboundedSender<MediaProcessingEvent>,
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
    },
    ProcessingFailed {
        media_id: Uuid,
        error: String,
    },
}

impl MediaService {
    pub fn new(
        repo: Arc<dyn MediaRepository>,
        storage_path: PathBuf,
        max_file_size: u64,
        grpc_processor: Arc<GrpcMediaProcessor>,
        event_tx: mpsc::UnboundedSender<MediaProcessingEvent>,
    ) -> Self {
        Self {
            repo,
            storage_path,
            max_file_size,
            grpc_client,
            event_tx,
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

        // Generate unique filename
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let file_extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("bin");
        
        let filename = format!("{}_{}.{}", post_id, timestamp, file_extension);
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
        // Update status to processing
        self.repo
            .update_processing_status(media_id, ProcessingStatus::Processing)
            .await
            .map_err(MediaServiceError::Database)?;

        self.event_tx.send(MediaProcessingEvent::StatusChanged {
            media_id,
            status: ProcessingStatus::Processing,
            progress: Some(0),
        }).ok();

        // Get media item
        let media_item = self.repo
            .find_by_id(media_id)
            .await
            .map_err(MediaServiceError::Database)?
            .ok_or_else(|| MediaServiceError::Processing("Media item not found".to_string()))?;

        // Create processing job
        let job = MediaProcessingJob {
            media_id: media_item.id,
            media_type: media_item.media_type,
            source_url: media_item.url.clone(),
            target_formats: self.get_target_formats(media_item.media_type),
            quality: "high".to_string(),
        };

        // Send to worker via gRPC
        match self.grpc_client.process_media(job).await {
            Ok(result) => {
                self.repo
                    .update_processing_status(media_id, ProcessingStatus::Completed)
                    .await
                    .map_err(MediaServiceError::Database)?;

                self.event_tx.send(MediaProcessingEvent::ProcessingCompleted {
                    media_id,
                    result_url: result.processed_url,
                }).ok();
            }
            Err(e) => {
                self.repo
                    .update_processing_status(media_id, ProcessingStatus::Failed)
                    .await
                    .map_err(MediaServiceError::Database)?;

                self.event_tx.send(MediaProcessingEvent::ProcessingFailed {
                    media_id,
                    error: e.to_string(),
                }).ok();
            }
        }

        Ok(())
    }

    pub async fn process_media(&self, media_id: Uuid) -> Result<(), MediaServiceError> {
        // This is a synchronous wrapper for async processing
        self.process_media_async(media_id).await
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

    pub async fn update_media_status(
        &self,
        media_id: Uuid,
        status: ProcessingStatus,
    ) -> Result<(), MediaServiceError> {
        self.repo
            .update_processing_status(media_id, status)
            .await
            .map_err(MediaServiceError::Database)?;

        self.event_tx.send(MediaProcessingEvent::StatusChanged {
            media_id,
            status,
            progress: None,
        }).ok();

        Ok(())
    }

    fn get_target_formats(&self, media_type: MediaType) -> Vec<String> {
        match media_type {
            MediaType::Image => vec!["thumbnail".to_string(), "medium".to_string(), "large".to_string()],
            MediaType::Video => vec!["720p".to_string(), "1080p".to_string(), "thumbnail".to_string()],
            MediaType::Audio => vec!["mp3".to_string(), "aac".to_string()],
            _ => vec!["original".to_string()],
        }
    }

    pub async fn validate_file_type(&self, file_path: &Path) -> Result<MediaType, MediaServiceError> {
        let extension = file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_lowercase();

        match extension.as_str() {
            "jpg" | "jpeg" | "png" | "gif" | "webp" => Ok(MediaType::Image),
            "mp4" | "avi" | "mov" | "mkv" | "webm" => Ok(MediaType::Video),
            "mp3" | "wav" | "flac" | "aac" | "ogg" => Ok(MediaType::Audio),
            "pdf" | "doc" | "docx" | "txt" => Ok(MediaType::Document),
            _ => Ok(MediaType::Unknown),
        }
    }

    pub async fn generate_thumbnail(&self, media_id: Uuid) -> Result<String, MediaServiceError> {
        // Placeholder for thumbnail generation
        let media_item = self.repo
            .find_by_id(media_id)
            .await
            .map_err(MediaServiceError::Database)?
            .ok_or_else(|| MediaServiceError::Processing("Media not found".to_string()))?;

        Ok(format!("{}_thumbnail", media_item.url))
    }
}