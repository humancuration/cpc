use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VoteType {
    Upvote,
    Downvote,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vote {
    pub id: Uuid,
    pub user_id: Uuid,
    pub post_id: Uuid,
    pub vote_type: VoteType,
    pub created_at: DateTime<Utc>,
}

impl Vote {
    pub fn new(user_id: Uuid, post_id: Uuid, vote_type: VoteType) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            post_id,
            vote_type,
            created_at: Utc::now(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteEvent {
    pub user_id: Uuid,
    pub vote_type: VoteType,
}

impl VoteEvent {
    pub fn new(user_id: Uuid, vote_type: VoteType) -> Self {
        Self { user_id, vote_type }
    }
}