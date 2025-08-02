//! Campaign management service

use crate::domain::{Campaign, CampaignType, CampaignStatus};
use crate::application::repository::{CampaignRepository, ContributionRepository, MembershipRepository};
use crate::application::ApplicationError;
use uuid::Uuid;

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
    /// Create a new campaign
    pub async fn create_campaign(&self, campaign: Campaign) -> Result<Campaign, ApplicationError> {
        // Validate the campaign
        self.validate_campaign(&campaign)?;
        
        // Save the campaign
        self.campaign_repository.save(&campaign).await?;
        
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
        // Validate the campaign
        self.validate_campaign(&campaign)?;
        
        // Save the campaign
        self.campaign_repository.save(&campaign).await?;
        
        Ok(campaign)
    }
    
    /// Delete a campaign
    pub async fn delete_campaign(&self, id: Uuid) -> Result<(), ApplicationError> {
        let campaign = self.campaign_repository.find_by_id(id).await?
            .ok_or(ApplicationError::NotFound)?;
        
        // Safety checks
        if campaign.status != CampaignStatus::Draft {
            return Err(ApplicationError::ValidationFailed(
                "Only DRAFT campaigns can be deleted".to_string()
            ));
        }
        
        let has_contributions = self.contribution_repository
            .exists_for_campaign(id).await?;
        if has_contributions {
            return Err(ApplicationError::ValidationFailed(
                "Cannot delete campaign with contributions".to_string()
            ));
        }
        
        let has_memberships = self.membership_repository
            .exists_for_campaign(id).await?;
        if has_memberships {
            return Err(ApplicationError::ValidationFailed(
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
            .ok_or(ApplicationError::CampaignError(
                crate::domain::CampaignError::ValidationFailed("Campaign not found".to_string())
            ))?;
        
        campaign.activate()?;
        self.campaign_repository.save(&campaign).await?;
        
        Ok(campaign)
    }
    
    /// Complete a campaign
    pub async fn complete_campaign(&self, id: Uuid) -> Result<Campaign, ApplicationError> {
        let mut campaign = self.campaign_repository
            .find_by_id(id)
            .await?
            .ok_or(ApplicationError::CampaignError(
                crate::domain::CampaignError::ValidationFailed("Campaign not found".to_string())
            ))?;
        
        campaign.complete()?;
        self.campaign_repository.save(&campaign).await?;
        
        Ok(campaign)
    }
    
    /// Validate a campaign
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