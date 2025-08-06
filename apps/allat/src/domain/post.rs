use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::vote::Vote;
use crate::domain::media_asset::MediaAsset;
use std::fmt::Write;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub community_id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub votes: Vec<Vote>,
    pub parent_id: Option<Uuid>, // For threaded comments
    pub media_assets: Vec<MediaAsset>,
}

impl Post {
    pub fn new(community_id: Uuid, user_id: Uuid, title: String, content: String, parent_id: Option<Uuid>, media_assets: Vec<MediaAsset>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            community_id,
            user_id,
            title,
            content,
            created_at: now,
            updated_at: now,
            votes: Vec::new(),
            parent_id,
            media_assets,
        }
    }
    
    pub fn get_searchable_text(&self) -> String {
        let mut text = String::new();
        write!(&mut text, "{} {}", self.title, self.content).unwrap();
        text
    }
}