use async_trait::async_trait;
use uuid::Uuid;
use chrono::DateTime;
use chrono::Utc;
use crate::application::privacy_service::{ConsentType, ConsentStatus};
use crate::domain::errors::{Result, MusicPlayerError};

/// Repository interface for user consent management
#[async_trait]
pub trait ConsentRepository: Send + Sync {
    /// Store user consent for a specific type
    async fn store_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType,
        granted: bool,
        expires_at: Option<DateTime<Utc>>
    ) -> Result<()>;
    
    /// Get user consent status
    async fn get_consent(
        &self,
        user_id: Uuid,
        consent_type: ConsentType
    ) -> Result<Option<ConsentStatus>>;
    
    /// Get all consent records for a user
    async fn get_all_consents(
        &self,
        user_id: Uuid
    ) -> Result<Vec<(ConsentType, ConsentStatus)>>;
    
    /// Revoke all consents for a user
    async fn revoke_all_consents(
        &self,
        user_id: Uuid
    ) -> Result<()>;
}