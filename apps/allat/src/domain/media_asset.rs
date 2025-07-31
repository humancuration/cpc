use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MediaType {
    Image,
    Video,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: Uuid,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub media_type: MediaType,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl MediaAsset {
    pub fn new(url: String, media_type: MediaType, alt_text: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            url,
            thumbnail_url: None,
            media_type,
            alt_text,
            created_at: Utc::now(),
        }
    }
}