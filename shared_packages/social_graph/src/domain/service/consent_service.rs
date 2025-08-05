//! Domain service for consent management in the social graph

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;
use crate::domain::model::Visibility;

/// Trait defining the consent service contract
#[async_trait]
pub trait ConsentService: Send + Sync {
    /// Check if a viewer can view content based on visibility settings and relationships
    async fn can_view_content(
        &self,
        viewer_id: Uuid,
        content_owner_id: Uuid,
        visibility: Visibility
    ) -> Result<bool, ConsentError>;
}

/// Errors that can occur during consent operations
#[derive(Debug, Error)]
pub enum ConsentError {
    /// Failed to check friendship relationship
    #[error("Friendship check failed")]
    FriendshipCheckFailed,
    
    /// Failed to check group membership
    #[error("Group membership check failed")]
    GroupCheckFailed,
    
    /// Generic error for other consent-related issues
    #[error("Consent operation failed: {0}")]
    Other(String),
}