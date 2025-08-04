//! Repository traits for Volunteer Coordination

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::{
    ApplicationId, ApplicationStatus, ContributionId, VolunteerApplication, VolunteerContribution,
    VolunteerOpportunity, OpportunityId, OpportunityStatus,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VolunteerRepositoryError {
    NotFound,
    DatabaseError(String),
    ValidationError(String),
    Conflict(String),
}

impl std::fmt::Display for VolunteerRepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolunteerRepositoryError::NotFound => write!(f, "Not found"),
            VolunteerRepositoryError::DatabaseError(e) => write!(f, "Database error: {}", e),
            VolunteerRepositoryError::ValidationError(e) => write!(f, "Validation error: {}", e),
            VolunteerRepositoryError::Conflict(e) => write!(f, "Conflict: {}", e),
        }
    }
}
impl std::error::Error for VolunteerRepositoryError {}

#[async_trait]
pub trait OpportunityRepository: Send + Sync {
    async fn insert(&self, opportunity: &VolunteerOpportunity) -> Result<(), VolunteerRepositoryError>;
    async fn update_status(&self, id: OpportunityId, status: OpportunityStatus) -> Result<(), VolunteerRepositoryError>;
    async fn get(&self, id: OpportunityId) -> Result<Option<VolunteerOpportunity>, VolunteerRepositoryError>;
}

#[async_trait]
pub trait ApplicationRepository: Send + Sync {
    async fn insert(&self, app: &VolunteerApplication) -> Result<(), VolunteerRepositoryError>;
    async fn update_status(
        &self,
        id: ApplicationId,
        status: ApplicationStatus,
        reviewer_id: Option<Uuid>,
        decided_at: Option<DateTime<Utc>>,
    ) -> Result<(), VolunteerRepositoryError>;
    async fn get(&self, id: ApplicationId) -> Result<Option<VolunteerApplication>, VolunteerRepositoryError>;
}

#[async_trait]
pub trait ContributionRepository: Send + Sync {
    async fn insert(&self, c: &VolunteerContribution) -> Result<(), VolunteerRepositoryError>;
    async fn verify(
        &self,
        id: ContributionId,
        verified: bool,
        verification_ref: Option<Uuid>,
    ) -> Result<(), VolunteerRepositoryError>;
    async fn get(&self, id: ContributionId) -> Result<Option<VolunteerContribution>, VolunteerRepositoryError>;
}