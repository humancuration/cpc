use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// Supported media types for processing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Video,
    Audio,
    Image,
}

/// Supported video codecs (only royalty-free)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VideoCodec {
    AV1,
}

/// Supported audio codecs (only royalty-free)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AudioCodec {
    Opus,
}

/// Supported container formats
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerFormat {
    WebM,
    PNG,
    JPEG,
}

/// Media processing configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProcessingConfig {
    pub video_codec: VideoCodec,
    pub audio_codec: AudioCodec,
    pub container_format: ContainerFormat,
    pub video_bitrate: Option<u32>,
    pub audio_bitrate: Option<u32>,
    pub resolution: Option<(u32, u32)>,
    pub frame_rate: Option<f32>,
    pub quality: Option<u8>, // 0-100
}

impl Default for MediaProcessingConfig {
    fn default() -> Self {
        Self {
            video_codec: VideoCodec::AV1,
            audio_codec: AudioCodec::Opus,
            container_format: ContainerFormat::WebM,
            video_bitrate: Some(1000000), // 1 Mbps
            audio_bitrate: Some(128000),  // 128 kbps
            resolution: None,
            frame_rate: Some(30.0),
            quality: Some(80),
        }
    }
}

/// Media metadata extracted from files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaMetadata {
    pub id: Uuid,
    pub file_name: String,
    pub file_size: u64,
    pub media_type: MediaType,
    pub duration: Option<f64>, // in seconds
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub frame_rate: Option<f32>,
    pub bitrate: Option<u32>,
    pub codec: Option<String>,
    pub created_at: DateTime<Utc>,
    pub checksum: String, // SHA-256 hash
}

/// Media processing job status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed(String),
}

/// Media processing job
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProcessingJob {
    pub id: Uuid,
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub config: MediaProcessingConfig,
    pub status: ProcessingStatus,
    pub progress: f32, // 0.0 to 1.0
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

/// Thumbnail generation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThumbnailConfig {
    pub width: u32,
    pub height: u32,
    pub quality: u8, // 0-100
    pub timestamp: Option<f64>, // For video thumbnails, timestamp in seconds
}

impl Default for ThumbnailConfig {
    fn default() -> Self {
        Self {
            width: 320,
            height: 240,
            quality: 80,
            timestamp: Some(1.0), // 1 second into video
        }
    }
}

/// Media upload information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaUpload {
    pub id: Uuid,
    pub original_filename: String,
    pub content_type: String,
    pub file_size: u64,
    pub upload_path: PathBuf,
    pub processed_path: Option<PathBuf>,
    pub thumbnail_path: Option<PathBuf>,
    pub metadata: Option<MediaMetadata>,
    pub uploaded_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

/// Media processing result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub job_id: Uuid,
    pub success: bool,
    pub output_path: Option<PathBuf>,
    pub thumbnail_path: Option<PathBuf>,
    pub metadata: Option<MediaMetadata>,
    pub processing_time: f64, // in seconds
    pub error_message: Option<String>,
}