use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsentRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub consent_type: String,
    pub granted: bool,
    pub granted_at: chrono::DateTime<chrono::Utc>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[async_trait::async_trait]
pub trait ConsentService {
    async fn grant_consent(&self, user_id: Uuid, consent_type: String) -> Result<ConsentRecord, ConsentError>;
    async fn revoke_consent(&self, user_id: Uuid, consent_type: String) -> Result<(), ConsentError>;
    async fn check_consent(&self, user_id: Uuid, consent_type: String) -> Result<bool, ConsentError>;
}

#[derive(Debug, thiserror::Error)]
pub enum ConsentError {
    #[error("User not found")]
    UserNotFound,
    #[error("Invalid consent type")]
    InvalidConsentType,
    #[error("Consent expired")]
    ConsentExpired,
}