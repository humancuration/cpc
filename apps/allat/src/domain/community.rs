use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Community {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub rules: Vec<String>,
    pub created_at: DateTime<Utc>,
}

impl Community {
    pub fn new(name: String, description: String, rules: Vec<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            rules,
            created_at: Utc::now(),
        }
    }
}