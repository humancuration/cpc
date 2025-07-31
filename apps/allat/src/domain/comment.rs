use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::vote::Vote;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub votes: Vec<Vote>,
    pub parent_id: Option<Uuid>, // For nested replies
}

impl Comment {
    pub fn new(post_id: Uuid, user_id: Uuid, content: String, parent_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            post_id,
            user_id,
            content,
            created_at: now,
            updated_at: now,
            votes: Vec::new(),
            parent_id,
        }
    }
}