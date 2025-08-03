//! Media transcoding service for live streams

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

/// Transcoding service for converting live streams to WebM/AV1 format
pub struct Transcoder {
    /// Active transcoding jobs
    active_jobs: HashMap<Uuid, TranscodingJob>,
}

/// Represents a transcoding job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscodingJob {
    /// Unique identifier for the job
    pub id: Uuid,
    
    /// Stream key being transcoded
    pub stream_key: String,
    
    /// Input format information
    pub input_format: MediaFormat,
    
    /// Output format information
    pub output_format: MediaFormat,
    
    /// Current status of the job
    pub status: TranscodingStatus,
    
    /// Progress percentage (0-100)
    pub progress: u8,
    
    /// When the job started
    pub started_at: chrono::DateTime<chrono::Utc>,
    
    /// When the job completed (if finished)
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Any error that occurred during transcoding
    pub error: Option<String>,
}

/// Media format specification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaFormat {
    /// Codec name (e.g., "h264", "av1")
    pub codec: String,
    
    /// Width in pixels
    pub width: u32,
    
    /// Height in pixels
    pub height: u32,
    
    /// Frame rate
    pub fps: f32,
    
    /// Bitrate in kbps
    pub bitrate_kbps: u32,
    
    /// Audio codec (e.g., "aac", "opus")
    pub audio_codec: String,
    
    /// Audio bitrate in kbps
    pub audio_bitrate_kbps: u32,
    
    /// Audio sample rate in Hz
    pub audio_sample_rate: u32,
}

/// Status of a transcoding job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TranscodingStatus {
    /// Job is queued
    Queued,
    
    /// Job is currently processing
    Processing,
    
    /// Job completed successfully
    Completed,
    
    /// Job failed
    Failed,
    
    /// Job was cancelled
    Cancelled,
}

impl Transcoder {
    /// Create a new transcoder
    pub fn new() -> Self {
        Self {
            active_jobs: HashMap::new(),
        }
    }
    
    /// Start a new transcoding job
    pub fn start_transcoding_job(
        &mut self,
        stream_key: String,
        input_format: MediaFormat,
        output_format: MediaFormat,
    ) -> TranscodingJob {
        let job = TranscodingJob {
            id: Uuid::new_v4(),
            stream_key,
            input_format,
            output_format,
            status: TranscodingStatus::Queued,
            progress: 0,
            started_at: chrono::Utc::now(),
            completed_at: None,
            error: None,
        };
        
        self.active_jobs.insert(job.id, job.clone());
        job
    }
    
    /// Update the status of a transcoding job
    pub fn update_job_status(&mut self, job_id: Uuid, status: TranscodingStatus, progress: u8) -> Option<()> {
        if let Some(job) = self.active_jobs.get_mut(&job_id) {
            job.status = status;
            job.progress = progress.min(100);
            
            if matches!(status, TranscodingStatus::Completed | TranscodingStatus::Failed | TranscodingStatus::Cancelled) {
                job.completed_at = Some(chrono::Utc::now());
            }
            
            Some(())
        } else {
            None
        }
    }
    
    /// Set an error for a transcoding job
    pub fn set_job_error(&mut self, job_id: Uuid, error: String) -> Option<()> {
        if let Some(job) = self.active_jobs.get_mut(&job_id) {
            job.status = TranscodingStatus::Failed;
            job.error = Some(error);
            job.completed_at = Some(chrono::Utc::now());
            Some(())
        } else {
            None
        }
    }
    
    /// Get a transcoding job by ID
    pub fn get_job(&self, job_id: Uuid) -> Option<TranscodingJob> {
        self.active_jobs.get(&job_id).cloned()
    }
    
    /// Get all active transcoding jobs for a stream
    pub fn get_stream_jobs(&self, stream_key: &str) -> Vec<TranscodingJob> {
        self.active_jobs
            .values()
            .filter(|job| job.stream_key == stream_key)
            .cloned()
            .collect()
    }
    
    /// Cancel a transcoding job
    pub fn cancel_job(&mut self, job_id: Uuid) -> Option<TranscodingJob> {
        if let Some(job) = self.active_jobs.get_mut(&job_id) {
            job.status = TranscodingStatus::Cancelled;
            job.completed_at = Some(chrono::Utc::now());
            Some(job.clone())
        } else {
            None
        }
    }
    
    /// Create a standard WebM/AV1 output format
    pub fn create_webm_av1_format(width: u32, height: u32, bitrate_kbps: u32) -> MediaFormat {
        MediaFormat {
            codec: "av1".to_string(),
            width,
            height,
            fps: 30.0,
            bitrate_kbps,
            audio_codec: "opus".to_string(),
            audio_bitrate_kbps: 128,
            audio_sample_rate: 48000,
        }
    }
    
    /// Create an adaptive bitrate ladder
    pub fn create_adaptive_bitrate_ladder(base_width: u32, base_height: u32) -> Vec<MediaFormat> {
        let mut formats = Vec::new();
        
        // Common resolutions for streaming
        let resolutions = [
            (base_width, base_height),           // Source
            (1920, 1080), // 1080p
            (1280, 720),  // 720p
            (854, 480),   // 480p
            (640, 360),   // 360p
            (426, 240),   // 240p
        ];
        
        for (width, height) in resolutions.iter() {
            // Skip resolutions larger than the source
            if *width > base_width || *height > base_height {
                continue;
            }
            
            // Calculate bitrate based on resolution (simplified)
            let bitrate_kbps = match (width, height) {
                (1920, 1080) => 6000,
                (1280, 720) => 3500,
                (854, 480) => 1500,
                (640, 360) => 800,
                (426, 240) => 400,
                _ => 8000, // Source quality
            };
            
            formats.push(MediaFormat {
                codec: "av1".to_string(),
                width: *width,
                height: *height,
                fps: 30.0,
                bitrate_kbps,
                audio_codec: "opus".to_string(),
                audio_bitrate_kbps: 128,
                audio_sample_rate: 48000,
            });
        }
        
        formats
    }
}

impl Default for Transcoder {
    fn default() -> Self {
        Self::new()
    }
}