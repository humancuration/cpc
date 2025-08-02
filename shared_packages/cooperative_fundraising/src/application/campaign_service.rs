//! Campaign management service

use crate::domain::{Campaign, CampaignType, CampaignStatus};
use crate::application::repository::{CampaignRepository, ContributionRepository, MembershipRepository};
use crate::application::ApplicationError;
use uuid::Uuid;
use rust_decimal::Decimal;

/// Structured creation errors for campaign creation flow
#[derive(Debug, thiserror::Error)]
pub enum CreationError {
    #[error("Invalid title: {0}")]
    InvalidTitle(String),
    #[error("Invalid description: {0}")]
    InvalidDescription(String),
    #[error("Invalid campaign type: {0}")]
    InvalidCampaignType(String),
    #[error("Invalid membership requirements: {0}")]
    InvalidMembershipRequirements(String),
    #[error("Invalid donation details: {0}")]
    InvalidDonationDetails(String),
    #[error("Database error: {0}")]
    DatabaseError(String),
}

impl From<ApplicationError> for CreationError {
    fn from(err: ApplicationError) -> Self {
        match err {
            ApplicationError::RepositoryError(e) => CreationError::DatabaseError(format!("{e}")),
            ApplicationError::ValidationError(msg) => CreationError::InvalidCampaignType(msg),
            other => CreationError::DatabaseError(format!("{other}")),
        }
    }
}

pub struct CampaignService {
    campaign_repository: Box<dyn CampaignRepository>,
    contribution_repository: Box<dyn ContributionRepository>,
    membership_repository: Box<dyn MembershipRepository>,
}

impl CampaignService {
    pub fn new(
        campaign_repository: Box<dyn CampaignRepository>,
        contribution_repository: Box<dyn ContributionRepository>,
        membership_repository: Box<dyn MembershipRepository>,
    ) -> Self {
        Self {
            campaign_repository,
            contribution_repository,
            membership_repository,
        }
    }

    /// Create a new campaign with validation and transactional safety.
    /// Validation occurs before any DB ops. Repository errors are mapped to CreationError::DatabaseError.
    pub async fn create_campaign(&self, campaign: Campaign) -> Result<Campaign, CreationError> {
        // Validate all rules BEFORE DB
        self.validate_creation_rules(&campaign)?;

        // Save via repository (repository handles its own tx internally currently)
        self.campaign_repository
            .save(&campaign)
            .await
            .map_err(|e| CreationError::DatabaseError(format!("{e}")))?;

        Ok(campaign)
    }
    
    /// Get a campaign by ID
    pub async fn get_campaign(&self, id: Uuid) -> Result<Option<Campaign>, ApplicationError> {
        self.campaign_repository.find_by_id(id).await
    }
    
    /// List campaigns with optional filtering
    pub async fn list_campaigns(
        &self,
        campaign_type: Option<CampaignType>,
        status: Option<CampaignStatus>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Campaign>, ApplicationError> {
        let result = self.campaign_repository
            .list(campaign_type, status, limit, offset)
            .await?;
        
        Ok(result.campaigns)
    }
    
    /// Update a campaign
    pub async fn update_campaign(&self, campaign: Campaign) -> Result<Campaign, ApplicationError> {
        // Keep legacy validation for update path
        self.validate_campaign(&campaign)?;
        self.campaign_repository.save(&campaign).await?;
        Ok(campaign)
    }
    
    /// Delete a campaign
    pub async fn delete_campaign(&self, id: Uuid) -> Result<(), ApplicationError> {
        let campaign = self.campaign_repository.find_by_id(id).await?
            .ok_or(ApplicationError::ValidationError("Campaign not found".to_string()))?;
        
        // Safety checks
        if campaign.status != CampaignStatus::Draft {
            return Err(ApplicationError::ValidationError(
                "Only DRAFT campaigns can be deleted".to_string()
            ));
        }
        
        let has_contributions = self.contribution_repository
            .exists_for_campaign(id).await?;
        if has_contributions {
            return Err(ApplicationError::ValidationError(
                "Cannot delete campaign with contributions".to_string()
            ));
        }
        
        let has_memberships = self.membership_repository
            .exists_for_campaign(id).await?;
        if has_memberships {
            return Err(ApplicationError::ValidationError(
                "Cannot delete campaign with membership shares".to_string()
            ));
        }
        
        // Soft delete
        self.campaign_repository.soft_delete(id).await?;
        Ok(())
    }
    
    /// Activate a campaign
    pub async fn activate_campaign(&self, id: Uuid) -> Result<Campaign, ApplicationError> {
        let mut campaign = self.campaign_repository
            .find_by_id(id)
            .await?
            .ok_or(ApplicationError::ValidationError("Campaign not found".to_string()))?;
        
        campaign.activate()?;
        self.campaign_repository.save(&campaign).await?;
        
        Ok(campaign)
    }
    
    /// Complete a campaign
    pub async fn complete_campaign(&self, id: Uuid) -> Result<Campaign, ApplicationError> {
        let mut campaign = self.campaign_repository
            .find_by_id(id)
            .await?
            .ok_or(ApplicationError::ValidationError("Campaign not found".to_string()))?;
        
        campaign.complete()?;
        self.campaign_repository.save(&campaign).await?;
        
        Ok(campaign)
    }

    /// New creation-time validation rules per feature spec
    fn validate_creation_rules(&self, campaign: &Campaign) -> Result<(), CreationError> {
        // Title 5-100
        let title_len = campaign.title.trim().chars().count();
        if title_len < 5 || title_len > 100 {
            return Err(CreationError::InvalidTitle(
                "Title must be between 5 and 100 characters".to_string(),
            ));
        }
        // Description 20-1000
        let desc_len = campaign.description.trim().chars().count();
        if desc_len < 20 || desc_len > 1000 {
            return Err(CreationError::InvalidDescription(
                "Description must be between 20 and 1000 characters".to_string(),
            ));
        }
        // Only DRAFT status allowed
        if campaign.status != CampaignStatus::Draft {
            return Err(CreationError::InvalidCampaignType(
                "Only DRAFT status allowed when creating a new campaign".to_string(),
            ));
        }
        // Membership campaigns require max_participants > 0
        if campaign.is_membership_campaign() {
            let req = campaign
                .membership_requirements
                .as_ref()
                .ok_or_else(|| CreationError::InvalidMembershipRequirements(
                    "Membership campaigns must have requirements".to_string(),
                ))?;
            if let Some(max) = req.max_participants {
                if max == 0 {
                    return Err(CreationError::InvalidMembershipRequirements(
                        "Membership campaigns require max_participants > 0".to_string(),
                    ));
                }
            } else {
                return Err(CreationError::InvalidMembershipRequirements(
                    "Membership campaigns require max_participants > 0".to_string(),
                ));
            }
        }
        // Donation campaigns require positive funding_goal
        if campaign.is_donation_campaign() {
            let details = campaign
                .donation_details
                .as_ref()
                .ok_or_else(|| CreationError::InvalidDonationDetails(
                    "Donation campaigns must have details".to_string(),
                ))?;
            if details.external_use_case.trim().is_empty() {
                return Err(CreationError::InvalidDonationDetails(
                    "Donation campaigns must have an external use case".to_string(),
                ));
            }
            if let Some(goal) = details.funding_goal {
                if goal <= Decimal::ZERO {
                    return Err(CreationError::InvalidDonationDetails(
                        "Donation funding_goal must be positive".to_string(),
                    ));
                }
            } else {
                return Err(CreationError::InvalidDonationDetails(
                    "Donation campaigns require a funding_goal".to_string(),
                ));
            }
        }
        Ok(())
    }
    
    /// Existing validation retained for update paths and other operations
    fn validate_campaign(&self, campaign: &Campaign) -> Result<(), ApplicationError> {
        // For membership campaigns, requirements are required
        if campaign.is_membership_campaign() && campaign.membership_requirements.is_none() {
            return Err(ApplicationError::ValidationError(
                "Membership campaigns must have requirements".to_string()
            ));
        }
        
        // For donation campaigns, details are required
        if campaign.is_donation_campaign() && campaign.donation_details.is_none() {
            return Err(ApplicationError::ValidationError(
                "Donation campaigns must have details".to_string()
            ));
        }
        
        // For donation campaigns, external use case is required
        if let Some(details) = &campaign.donation_details {
            if details.external_use_case.is_empty() {
                return Err(ApplicationError::ValidationError(
                    "Donation campaigns must have an external use case".to_string()
                ));
            }
        }
        
        Ok(())
    }
}