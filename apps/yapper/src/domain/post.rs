use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum YapError {
    #[error("Content exceeds 280 characters")]
    ContentTooLong,
    #[error("Content is empty")]
    ContentEmpty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Yap {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub like_count: u32,
    pub share_count: u32,
    pub parent_id: Option<Uuid>, // For threads
}

impl Yap {
    pub fn new(user_id: Uuid, content: String) -> Result<Self, YapError> {
        if content.is_empty() {
            return Err(YapError::ContentEmpty);
        }
        
        if content.chars().count() > 280 {
            return Err(YapError::ContentTooLong);
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            content,
            created_at: Utc::now(),
            like_count: 0,
            share_count: 0,
            parent_id: None,
        })
    }
}