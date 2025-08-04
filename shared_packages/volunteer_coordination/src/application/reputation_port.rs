//! Reputation verification application port (hexagonal boundary)
//! Provides an async interface to verify contributions with an external reputation system.

use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::models::{ContributionId, ContributionKind};

#[derive(Debug, thiserror::Error)]
pub enum ReputationError {
    #[error("Upstream reputation service unavailable")]
    Unavailable,
    #[error("Invalid request: {0}")]
    InvalidRequest(String),
    #[error("Unexpected error: {0}")]
    Other(String),
}

#[async_trait]
pub trait ReputationPort: Send + Sync {
    async fn verify_contribution(
        &self,
        contribution_id: ContributionId,
        kind: ContributionKind,
        amount_hours: Option<f32>,
        contributor_id: Uuid,
    ) -> Result<bool, ReputationError>;
}