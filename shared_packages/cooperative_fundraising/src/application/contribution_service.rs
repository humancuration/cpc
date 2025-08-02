//! Contribution management service

use crate::domain::{Contribution, Campaign};
use crate::application::repository::{ContributionRepository, CampaignRepository};
use crate::application::validation_service::ContributionValidator;
use crate::application::cpay_integration::CpayIntegration;
use crate::application::wallet_integration::WalletIntegration;
use crate::application::skill_volunteering_adapter::SkillVolunteeringAdapter;
use crate::application::ApplicationError;
use uuid::Uuid;
use rust_decimal::Decimal;

pub struct ContributionService {
    contribution_repository: Box<dyn ContributionRepository>,
    campaign_repository: Box<dyn CampaignRepository>,
    contribution_validator: Box<dyn ContributionValidator>,
    cpay_integration: Box<dyn CpayIntegration>,
    wallet_integration: Box<dyn WalletIntegration>,
    skill_volunteering_adapter: Box<dyn SkillVolunteeringAdapter>,
}

impl ContributionService {
    pub fn new(
        contribution_repository: Box<dyn ContributionRepository>,
        campaign_repository: Box<dyn CampaignRepository>,
        contribution_validator: Box<dyn ContributionValidator>,
        cpay_integration: Box<dyn CpayIntegration>,
        wallet_integration: Box<dyn WalletIntegration>,
        skill_volunteering_adapter: Box<dyn SkillVolunteeringAdapter>,
    ) -> Self {
        Self {
            contribution_repository,
            campaign_repository,
            contribution_validator,
            cpay_integration,
            wallet_integration,
            skill_volunteering_adapter,
        }
    }
    
    /// Make a monetary contribution to a campaign
    pub async fn make_monetary_contribution(
        &self,
        campaign_id: Uuid,
        user_id: Uuid,
        amount: Decimal,
        currency: String,
        cpay_transaction_id: Uuid,
    ) -> Result<Contribution, ApplicationError> {
        // Get the campaign
        let campaign = self.campaign_repository
            .find_by_id(campaign_id)
            .await?
            .ok_or(ApplicationError::ValidationError(
                "Campaign not found".to_string()
            ))?;
        
        // Validate the contribution
        self.contribution_validator
            .validate_monetary_contribution(&campaign, amount, &currency)
            .await?;
        
        // Verify the cpay transaction
        self.cpay_integration
            .verify_transaction(cpay_transaction_id, amount, &currency)
            .await
            .map_err(|e| ApplicationError::PaymentError(e.to_string()))?;
        
        // Create the contribution
        let contribution = Contribution::new_monetary(
            campaign_id,
            user_id,
            cpay_transaction_id,
            amount,
            currency,
        );
        
        // Save the contribution
        self.contribution_repository.save(&contribution).await?;
        
        // Record in wallet
        self.wallet_integration
            .record_donation(user_id, campaign_id, amount, &contribution.id)
            .await
            .map_err(|e| ApplicationError::IntegrationError(e.to_string()))?;
        
        Ok(contribution)
    }
    
    /// Record a volunteer contribution to a campaign
    pub async fn record_volunteer_contribution(
        &self,
        campaign_id: Uuid,
        user_id: Uuid,
        opportunity_id: Uuid,
        hours: u32,
    ) -> Result<Contribution, ApplicationError> {
        // Get the campaign
        let campaign = self.campaign_repository
            .find_by_id(campaign_id)
            .await?
            .ok_or(ApplicationError::ValidationError(
                "Campaign not found".to_string()
            ))?;
        
        // Validate the contribution
        self.contribution_validator
            .validate_volunteer_contribution(&campaign, opportunity_id, hours)
            .await?;
        
        // Verify the opportunity exists
        self.skill_volunteering_adapter
            .verify_opportunity(opportunity_id)
            .await
            .map_err(|e| ApplicationError::IntegrationError(e.to_string()))?;
        
        // Create the contribution
        let contribution = Contribution::new_volunteer(
            campaign_id,
            user_id,
            opportunity_id,
            hours,
        );
        
        // Save the contribution
        self.contribution_repository.save(&contribution).await?;
        
        Ok(contribution)
    }
    
    /// List contributions for a campaign
    pub async fn list_contributions(
        &self,
        campaign_id: Uuid,
        user_id: Option<Uuid>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<Contribution>, ApplicationError> {
        let result = self.contribution_repository
            .list_by_campaign(campaign_id, user_id, limit, offset)
            .await?;
        
        Ok(result.contributions)
    }
    
    /// Get total monetary contributions for a campaign
    pub async fn get_total_monetary_contributions(
        &self,
        campaign_id: Uuid,
    ) -> Result<Decimal, ApplicationError> {
        self.contribution_repository
            .get_total_monetary_contributions(campaign_id)
            .await
    }
}