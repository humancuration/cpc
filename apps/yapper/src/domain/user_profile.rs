use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: Uuid,
    pub user_id: Uuid,
    pub display_name: String,
    pub bio: Option<String>,
    pub followers_count: u32,
    pub following_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    pub fn new(user_id: Uuid, display_name: String, bio: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            display_name,
            bio,
            followers_count: 0,
            following_count: 0,
            created_at: now,
            updated_at: now,
        }
    }
}