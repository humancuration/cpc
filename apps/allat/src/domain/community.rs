use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rules: String,
    pub created_at: DateTime<Utc>,
    pub moderator_ids: Vec<Uuid>,
}

impl Community {
    pub fn new(name: String, description: String, creator_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            rules: String::new(),
            created_at: Utc::now(),
            moderator_ids: vec![creator_id],
        }
    }
}