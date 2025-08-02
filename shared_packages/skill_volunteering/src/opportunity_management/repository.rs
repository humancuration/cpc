//! Repository for opportunity management.

use async_trait::async_trait;
use thiserror::Error;
use uuid::Uuid;

use super::models::{OpportunityApplication, VolunteerOpportunity};

#[derive(Error, Debug)]
pub enum OpportunityRepositoryError {
    #[error("Opportunity not found")]
    NotFound,
    #[error("Application not found")]
    ApplicationNotFound,
    #[error("A database error occurred: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("An unexpected error occurred")]
    Unexpected,
}

pub struct ListOpportunitiesFilters {
    pub cause_id: Option<Uuid>,
    pub skill_id: Option<Uuid>,
    pub only_open: bool, // If true, only return opportunities where deadline > NOW()
    pub limit: i64,
    pub offset: i64,
}

pub struct ListUserApplicationsFilters {
    pub user_id: Uuid,
    pub status: Option<String>,
    pub limit: i64,
    pub offset: i64,
}

/// A trait for abstracting the storage and retrieval of volunteer opportunities.
#[async_trait]
pub trait OpportunityRepository: Send + Sync {
    /// Creates a new volunteer opportunity.
    async fn create_opportunity(
        &self,
        opportunity: &VolunteerOpportunity,
    ) -> Result<(), OpportunityRepositoryError>;

    /// Finds an opportunity by its ID.
    async fn find_opportunity_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<VolunteerOpportunity>, OpportunityRepositoryError>;

    /// Lists opportunities based on the provided filters.
    async fn list_opportunities(
        &self,
        filters: &ListOpportunitiesFilters,
    ) -> Result<(Vec<VolunteerOpportunity>, i64), OpportunityRepositoryError>;

    /// Updates an existing volunteer opportunity.
    async fn update_opportunity(
        &self,
        opportunity: &VolunteerOpportunity,
    ) -> Result<VolunteerOpportunity, OpportunityRepositoryError>;

    /// Updates an existing opportunity application.
    async fn update_application(
        &self,
        application: &OpportunityApplication,
    ) -> Result<OpportunityApplication, OpportunityRepositoryError>;
    
    /// Creates a new application for an opportunity.
    async fn create_application(
        &self,
        user_id: Uuid,
        opportunity_id: Uuid,
    ) -> Result<OpportunityApplication, OpportunityRepositoryError>;

    /// Finds an application by its ID.
    async fn find_application_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<OpportunityApplication>, OpportunityRepositoryError>;

    /// Deletes an opportunity by its ID.
    async fn delete_opportunity(&self, id: Uuid) -> Result<(), OpportunityRepositoryError>;

    async fn list_user_applications(
        &self,
        filters: &ListUserApplicationsFilters,
    ) -> Result<(Vec<OpportunityApplication>, i64), OpportunityRepositoryError>;
}