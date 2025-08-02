//! Membership management service

use crate::domain::{Membership, Campaign};
use crate::application::repository::{MembershipRepository, CampaignRepository, ContributionRepository};
use crate::application::ApplicationError;
use uuid::Uuid;

pub struct MembershipService {
    membership_repository: Box<dyn MembershipRepository>,
    campaign_repository: Box<dyn CampaignRepository>,
    contribution_repository: Box<dyn ContributionRepository>,
}

impl MembershipService {
    pub fn new(
        membership_repository: Box<dyn MembershipRepository>,
        campaign_repository: Box<dyn CampaignRepository>,
        contribution_repository: Box<dyn ContributionRepository>,
    ) -> Self {
        Self {
            membership_repository,
            campaign_repository,
            contribution_repository,
        }
    }
    
    /// Join the cooperative by participating in a membership campaign
    pub async fn join_cooperative(
        &self,
        user_id: Uuid,
        campaign_id: Uuid,
    ) -> Result<Membership, ApplicationError> {
        // Check if user already has membership
        if self.membership_repository.user_has_membership(user_id).await? {
            return Err(ApplicationError::UserAlreadyHasMembership);
        }
        
        // Get the campaign
        let campaign = self.campaign_repository
            .find_by_id(campaign_id)
            .await?
            .ok_or(ApplicationError::ValidationError(
                "Campaign not found".to_string()
            ))?;
        
        // Verify it's a membership campaign
        if !campaign.is_membership_campaign() {
            return Err(ApplicationError::ValidationError(
                "Can only join cooperative through membership campaign".to_string()
            ));
        }
        
        // Verify campaign is active
        if campaign.status != crate::domain::CampaignStatus::Active {
            return Err(ApplicationError::ValidationError(
                "Campaign is not active".to_string()
            ));
        }
        
        // Verify user has completed required actions
        if let Some(requirements) = &campaign.membership_requirements {
            self.verify_requirements_met(user_id, requirements, campaign_id).await?;
        }
        
        // Create membership
        let membership = Membership::new(user_id, campaign_id);
        
        // Save membership
        self.membership_repository.save(&membership).await?;
        
        Ok(membership)
    }
    
    /// Check if a user has membership
    pub async fn user_has_membership(&self, user_id: Uuid) -> Result<bool, ApplicationError> {
        self.membership_repository.user_has_membership(user_id).await
    }
    
    /// Get user's membership
    pub async fn get_user_membership(&self, user_id: Uuid) -> Result<Option<Membership>, ApplicationError> {
        self.membership_repository.get_user_membership(user_id).await
    }
    
    /// Verify that user has met campaign requirements
    async fn verify_requirements_met(
        &self,
        user_id: Uuid,
        requirements: &crate::domain::MembershipRequirements,
        campaign_id: Uuid,
    ) -> Result<(), ApplicationError> {
        // For now, we'll check if the user has made any contributions to this campaign
        // In a real implementation, this would check specific required actions
        let contributions = self.contribution_repository
            .list_by_campaign(campaign_id, Some(user_id), None, None)
            .await?;
        
        if contributions.contributions.is_empty() {
            return Err(ApplicationError::RequirementsNotMet);
        }
        
        Ok(())
    }
}