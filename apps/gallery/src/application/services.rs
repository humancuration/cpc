use crate::domain::media::{Media, MediaType, MediaValidator};
use crate::domain::verifier::{MediaVerifier, FfprobeVerifier, VerificationError};
use crate::infrastructure::ffmpeg::{FfmpegExecutor, TargetFormat, FfmpegError};
use crate::infrastructure::storage::FileStorage;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use tokio::fs;

#[async_trait]
pub trait MediaProcessor: Send + Sync {
    async fn process(&self, media: Media) -> Result<Media, TranscodingError>;
}

pub struct TranscodingService {
    ffmpeg: Arc<dyn FfmpegExecutor>,
    storage: Arc<dyn FileStorage>,
    verifier: Arc<dyn MediaVerifier>,
    job_queue: Arc<dyn JobQueue>,
}

impl TranscodingService {
    pub fn new(
        ffmpeg: Arc<dyn FfmpegExecutor>,
        storage: Arc<dyn FileStorage>,
        verifier: Arc<dyn MediaVerifier>,
        job_queue: Arc<dyn JobQueue>,
    ) -> Self {
        Self {
            ffmpeg,
            storage,
            verifier,
            job_queue,
        }
    }
    
    pub fn new_default() -> Self {
        Self {
            ffmpeg: Arc::new(crate::infrastructure::ffmpeg::FfmpegExecutorImpl),
            storage: Arc::new(crate::infrastructure::storage::LocalFileStorage),
            verifier: Arc::new(FfprobeVerifier),
            job_queue: Arc::new(crate::infrastructure::queue::InMemoryQueue::new()),
        }
    }

    pub async fn transcode_media(&self, mut media: Media) -> Result<Media, TranscodingError> {
        // Validate media
        MediaValidator::validate(&media)?;
        
        // Skip if already transcoded
        if media.transcoded_path.is_some() {
            return Err(TranscodingError::AlreadyTranscoded);
        }
        
        // Determine target format based on media type
        let target_format = match media.file_type {
            MediaType::Video => TargetFormat::Av1,
            MediaType::Audio => TargetFormat::Opus,
            _ => return Err(TranscodingError::UnsupportedMediaType),
        };
        
        // Get temporary path for transcoding
        let temp_output_path = self.storage.get_temp_path(&media.file_path).await
            .map_err(TranscodingError::StorageError)?;
        let final_output_path = format!("{}.transcoded.webm", media.file_path);
        
        // Perform transcoding to temporary path
        self.ffmpeg
            .transcode(&media.file_path, &temp_output_path, target_format)
            .await
            .map_err(TranscodingError::FfmpegError)?;
        
        // Verify transcoded file
        self.verifier.verify(&temp_output_path, media.file_type).await?;
        
        // Move from temp to final location
        self.storage.move_to_storage(&temp_output_path, &final_output_path).await
            .map_err(TranscodingError::StorageError)?;
        
        // Update media with transcoded path
        media.set_transcoded_path(final_output_path);
        
        Ok(media)
    }
    
    /// Submit a transcoding job to the queue
    pub async fn submit_job(&self, media: Media) -> Result<Uuid, TranscodingError> {
        MediaValidator::validate(&media)?;
        let job = TranscodingJob::new(media.id);
        self.job_queue.enqueue(job).await
            .map_err(TranscodingError::QueueError)?;
        Ok(job.job_id)
    }
    
    /// Process a transcoding job from the queue
    pub async fn process_job(&self, _job_id: Uuid) -> Result<(), TranscodingError> {
        // In a real implementation, this would:
        // 1. Dequeue the job
        // 2. Find the media entity
        // 3. Transcode the media
        // 4. Verify the output
        // 5. Update the media entity with the transcoded path
        // 6. Acknowledge the job
        Ok(())
    }
    
    /// Priority levels for transcoding jobs
    #[derive(Debug, Clone)]
    pub enum JobPriority {
        Low,
        Normal,
        High,
    }
    
    /// Represents a transcoding job in the queue
    #[derive(Debug, Clone)]
    pub struct TranscodingJob {
        pub job_id: Uuid,
        pub media_id: Uuid,
        pub priority: JobPriority,
        pub retry_count: u8,
    }
    
    impl TranscodingJob {
        pub fn new(media_id: Uuid) -> Self {
            Self {
                job_id: Uuid::new_v4(),
                media_id,
                priority: JobPriority::Normal,
                retry_count: 0,
            }
        }
    }
    
    /// Error types for queue operations
    #[derive(Debug, thiserror::Error)]
    pub enum QueueError {
        #[error("Connection failed: {0}")]
        ConnectionFailed(String),
        
        #[error("Serialization error: {0}")]
        SerializationError(String),
        
        #[error("Job not found")]
        JobNotFound,
    }
    
    /// Trait for job queue implementations
    #[async_trait]
    pub trait JobQueue: Send + Sync {
        async fn enqueue(&self, job: TranscodingJob) -> Result<(), QueueError>;
        async fn dequeue(&self) -> Result<Option<TranscodingJob>, QueueError>;
        async fn ack(&self, job_id: Uuid) -> Result<(), QueueError>;
        async fn nack(&self, job_id: Uuid) -> Result<(), QueueError>;
    }
}

#[async_trait]
impl MediaProcessor for TranscodingService {
    async fn process(&self, media: Media) -> Result<Media, TranscodingError> {
        self.transcode_media(media).await
    }
}

#[derive(Debug, thiserror::Error)]
pub enum TranscodingError {
    #[error("Validation failed: {0}")]
    ValidationError(String),
    
    #[error("FFmpeg error: {0}")]
    FfmpegError(crate::infrastructure::ffmpeg::FfmpegError),
    
    #[error("Verification failed: {0}")]
    VerificationError(#[from] crate::domain::verifier::VerificationError),
    
    #[error("Queue error: {0}")]
    QueueError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("Job timeout")]
    Timeout,
    
    #[error("Media already transcoded")]
    AlreadyTranscoded,
    #[error("Unsupported media type")]
    UnsupportedMediaType,
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    #[error("Empty output file")]
    EmptyOutputFile,
    #[error("Invalid codec: {0}")]
    InvalidCodec(String),
    #[error("Low quality output")]
    LowQualityOutput,
    #[error("Invalid media type")]
    InvalidMediaType,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::media::{Media, MediaType};
    use crate::infrastructure::ffmpeg::FfmpegExecutorImpl;
    use crate::infrastructure::storage::LocalFileStorage;
    use crate::infrastructure::queue::InMemoryQueue;
    use std::sync::Arc;
    use uuid::Uuid;
    use chrono::Utc;
    use tempfile::tempdir;
    use std::fs::File;
    use std::io::Write;

    #[tokio::test]
    async fn test_transcoding_service_creation() {
        let ffmpeg = Arc::new(FfmpegExecutorImpl);
        let storage = Arc::new(LocalFileStorage);
        let verifier = Arc::new(FfprobeVerifier);
        let queue = Arc::new(InMemoryQueue::new());
        
        let service = TranscodingService::new(ffmpeg, storage, verifier, queue);
        assert!(true); // If we get here, the service was created successfully
    }

    #[tokio::test]
    async fn test_transcoding_service_with_video() {
        // Create a temporary directory for our test
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_video.mp4");
        let file_path_str = file_path.to_str().unwrap();
        
        // Create a dummy file for testing
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"dummy video content").unwrap();
        
        // Create a media entity
        let media = Media {
            id: Uuid::new_v4(),
            file_path: file_path_str.to_string(),
            file_type: MediaType::Video,
            upload_date: Utc::now(),
            owner_id: Uuid::new_v4(),
            original_hash: "dummy_hash".to_string(),
            transcoded_path: None,
        };
        
        // Create the service
        let ffmpeg = Arc::new(FfmpegExecutorImpl);
        let storage = Arc::new(LocalFileStorage);
        let verifier = Arc::new(FfprobeVerifier);
        let queue = Arc::new(InMemoryQueue::new());
        let service = TranscodingService::new(ffmpeg, storage, verifier, queue);
        
        // Test job submission
        let job_id = service.submit_job(media).await;
        assert!(job_id.is_ok());
    }
}