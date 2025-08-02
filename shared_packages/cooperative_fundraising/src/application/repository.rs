//! Repository traits for the cooperative fundraising system

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::{Campaign, Contribution, Membership, CampaignType, CampaignStatus};
use crate::application::ApplicationError;
use rust_decimal::Decimal;

/// Represents the paginated result of listing campaigns
#[derive(Debug, PartialEq, Eq)]
pub struct PaginatedCampaigns {
    pub campaigns: Vec<Campaign>,
    pub total_count: i64,
}

/// Represents the paginated result of listing contributions
#[derive(Debug, PartialEq, Eq)]
pub struct PaginatedContributions {
    pub contributions: Vec<Contribution>,
    pub total_count: i64,
}

#[async_trait]
pub trait CampaignRepository: Send + Sync {
    /// Save a campaign
    async fn save(&self, campaign: &Campaign) -> Result<(), ApplicationError>;
    
    /// Find a campaign by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Campaign>, ApplicationError>;
    
    /// List campaigns with optional filtering
    async fn list(
        &self,
        campaign_type: Option<CampaignType>,
        status: Option<CampaignStatus>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PaginatedCampaigns, ApplicationError>;
    
    /// Delete a campaign by ID
    async fn delete(&self, id: Uuid) -> Result<(), ApplicationError>;
    
    /// Check if a campaign has any contributions
    async fn exists_for_campaign(&self, campaign_id: Uuid) -> Result<bool, ApplicationError>;
    
    /// Soft delete a campaign by ID
    async fn soft_delete(&self, id: Uuid) -> Result<(), ApplicationError>;
}

#[async_trait]
pub trait ContributionRepository: Send + Sync {
    /// Save a contribution
    async fn save(&self, contribution: &Contribution) -> Result<(), ApplicationError>;
    
    /// Find a contribution by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Contribution>, ApplicationError>;
    
    /// List contributions for a campaign
    async fn list_by_campaign(
        &self,
        campaign_id: Uuid,
        user_id: Option<Uuid>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PaginatedContributions, ApplicationError>;
    
    /// Get total monetary contributions for a campaign
    async fn get_total_monetary_contributions(
        &self,
        campaign_id: Uuid,
    ) -> Result<Decimal, ApplicationError>;
    
    /// Delete a contribution by ID
    async fn delete(&self, id: Uuid) -> Result<(), ApplicationError>;
    
    /// Check if any contributions exist for a campaign
    async fn exists_for_campaign(&self, campaign_id: Uuid) -> Result<bool, ApplicationError>;
}

#[async_trait]
pub trait MembershipRepository: Send + Sync {
    /// Save a membership
    async fn save(&self, membership: &Membership) -> Result<(), ApplicationError>;
    
    /// Check if a user has a membership
    async fn user_has_membership(&self, user_id: Uuid) -> Result<bool, ApplicationError>;
    
    /// Get user's membership
    async fn get_user_membership(&self, user_id: Uuid) -> Result<Option<Membership>, ApplicationError>;
    
    /// Delete a membership by ID
    async fn delete(&self, user_id: Uuid, campaign_id: Uuid) -> Result<(), ApplicationError>;
    
    /// Check if any memberships exist for a campaign
    async fn exists_for_campaign(&self, campaign_id: Uuid) -> Result<bool, ApplicationError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Campaign not found")]
    CampaignNotFound,
    #[error("Contribution not found")]
    ContributionNotFound,
    #[error("Membership not found")]
    MembershipNotFound,
    #[error("Unexpected error")]
    Unexpected,
}

impl From<RepositoryError> for ApplicationError {
    fn from(error: RepositoryError) -> Self {
        ApplicationError::RepositoryError(error)
    }
}