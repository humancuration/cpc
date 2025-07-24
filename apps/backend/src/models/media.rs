use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
    Unknown,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaType::Image => write!(f, "image"),
            MediaType::Video => write!(f, "video"),
            MediaType::Audio => write!(f, "audio"),
            MediaType::Document => write!(f, "document"),
            MediaType::Unknown => write!(f, "unknown"),
        }
    }
}

impl From<&str> for MediaType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "image" => MediaType::Image,
            "video" => MediaType::Video,
            "audio" => MediaType::Audio,
            "document" => MediaType::Document,
            _ => MediaType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProcessingStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl fmt::Display for ProcessingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ProcessingStatus::Pending => write!(f, "pending"),
            ProcessingStatus::Processing => write!(f, "processing"),
            ProcessingStatus::Completed => write!(f, "completed"),
            ProcessingStatus::Failed => write!(f, "failed"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItem {
    pub id: Uuid,
    pub post_id: Option<Uuid>,
    pub url: String,
    pub media_type: MediaType,
    pub processing_status: ProcessingStatus,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub original_filename: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMediaItem {
    pub post_id: Option<Uuid>,
    pub url: String,
    pub media_type: MediaType,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub original_filename: Option<String>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMediaItem {
    pub processing_status: Option<ProcessingStatus>,
    pub url: Option<String>,
    pub file_size: Option<i64>,
    pub metadata: Option<serde_json::Value>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaProcessingJob {
    pub media_id: Uuid,
    pub media_type: MediaType,
    pub source_url: String,
    pub target_formats: Vec<String>,
    pub quality: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingResult {
    pub media_id: Uuid,
    pub processed_url: String,
    pub thumbnail_url: Option<String>,
    pub formats: Vec<ProcessedFormat>,
    pub metadata: Option<serde_json::Value>,
    pub processing_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedFormat {
    pub format: String,
    pub url: String,
    pub file_size: i64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_ms: Option<u64>,
    pub mime_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingUpdate {
    pub media_id: Uuid,
    pub status: ProcessingStatus,
    pub progress: Option<u32>,
    pub message: Option<String>,
    pub processed_url: Option<String>,
    pub thumbnail_url: Option<String>,
    pub error: Option<String>,
}

impl MediaItem {
    pub fn is_ready(&self) -> bool {
        self.processing_status == ProcessingStatus::Completed
    }

    pub fn is_processing(&self) -> bool {
        self.processing_status == ProcessingStatus::Processing
    }

    pub fn has_failed(&self) -> bool {
        self.processing_status == ProcessingStatus::Failed
    }

    pub fn can_be_processed(&self) -> bool {
        matches!(
            self.processing_status,
            ProcessingStatus::Pending | ProcessingStatus::Failed
        )
    }
}