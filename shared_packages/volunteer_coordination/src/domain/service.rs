//! Service traits for Volunteer Coordination

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::domain::models::{
    ApplicationId, ApplicationStatus, ContributionId, ContributionKind, OpportunityId,
    VolunteerApplication, VolunteerContribution, VolunteerOpportunity,
};

#[derive(Debug, Clone, PartialEq)]
pub enum VolunteerServiceError {
    RepositoryError(String),
    ValidationError(String),
    Unauthorized,
    NotFound,
    Conflict(String),
}

impl std::fmt::Display for VolunteerServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VolunteerServiceError::RepositoryError(e) => write!(f, "Repository error: {}", e),
            VolunteerServiceError::ValidationError(e) => write!(f, "Validation error: {}", e),
            VolunteerServiceError::Unauthorized => write!(f, "Unauthorized"),
            VolunteerServiceError::NotFound => write!(f, "Not found"),
            VolunteerServiceError::Conflict(e) => write!(f, "Conflict: {}", e),
        }
    }
}
impl std::error::Error for VolunteerServiceError {}

#[async_trait]
pub trait VolunteerOpportunityService: Send + Sync {
    async fn create_opportunity(
        &self,
        org_id: Uuid,
        created_by: Uuid,
        title: String,
        description: String,
        tags: Vec<String>,
        location: Option<String>,
        starts_at: Option<DateTime<Utc>>,
        ends_at: Option<DateTime<Utc>>,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError>;

    async fn publish_opportunity(
        &self,
        opportunity_id: OpportunityId,
        user_id: Uuid,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError>;

    async fn close_opportunity(
        &self,
        opportunity_id: OpportunityId,
        user_id: Uuid,
    ) -> Result<VolunteerOpportunity, VolunteerServiceError>;

    async fn get_opportunity(
        &self,
        opportunity_id: OpportunityId,
    ) -> Result<Option<VolunteerOpportunity>, VolunteerServiceError>;
}

#[async_trait]
pub trait VolunteerApplicationService: Send + Sync {
    async fn submit_application(
        &self,
        opportunity_id: OpportunityId,
        applicant_id: Uuid,
        motivation: Option<String>,
    ) -> Result<VolunteerApplication, VolunteerServiceError>;

    async fn review_application(
        &self,
        application_id: ApplicationId,
        reviewer_id: Uuid,
        status: ApplicationStatus,
    ) -> Result<VolunteerApplication, VolunteerServiceError>;

    async fn get_application(
        &self,
        application_id: ApplicationId,
    ) -> Result<Option<VolunteerApplication>, VolunteerServiceError>;
}

#[async_trait]
pub trait VolunteerContributionService: Send + Sync {
    async fn log_contribution(
        &self,
        opportunity_id: OpportunityId,
        contributor_id: Uuid,
        kind: ContributionKind,
        amount: f32,
        notes: Option<String>,
        occurred_at: DateTime<Utc>,
    ) -> Result<VolunteerContribution, VolunteerServiceError>;

    async fn verify_contribution(
        &self,
        contribution_id: ContributionId,
        verifier_id: Uuid,
        verification_ref: Option<Uuid>,
    ) -> Result<VolunteerContribution, VolunteerServiceError>;

    async fn get_contribution(
        &self,
        contribution_id: ContributionId,
    ) -> Result<Option<VolunteerContribution>, VolunteerServiceError>;
}