use serde::{Deserialize, Serialize};
use uuid::Uuid;
use rust_decimal::Decimal;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmaScore {
    pub user_id: Uuid,
    pub total_score: Decimal,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KarmaEvent {
    pub id: Uuid,
    pub user_id: Uuid,
    pub event_type: String,
    pub points: Decimal,
    pub description: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait::async_trait]
pub trait KarmaService {
    async fn get_karma_score(&self, user_id: Uuid) -> Result<KarmaScore, KarmaError>;
    async fn add_karma_event(&self, event: KarmaEvent) -> Result<(), KarmaError>;
    async fn calculate_karma(&self, user_id: Uuid) -> Result<Decimal, KarmaError>;
}

#[derive(Debug, thiserror::Error)]
pub enum KarmaError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid karma event")]
    InvalidEvent,
    #[error("Database error")]
    DatabaseError,
}