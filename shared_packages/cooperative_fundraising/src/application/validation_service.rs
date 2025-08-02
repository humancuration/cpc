//! Contribution validation service

use crate::domain::{Campaign, CampaignType};
use crate::application::ApplicationError;
use uuid::Uuid;
use rust_decimal::Decimal;

#[async_trait::async_trait]
pub trait ContributionValidator: Send + Sync {
    /// Validate a monetary contribution
    async fn validate_monetary_contribution(
        &self,
        campaign: &Campaign,
        amount: Decimal,
        currency: &str,
    ) -> Result<(), ApplicationError>;
    
    /// Validate a volunteer contribution
    async fn validate_volunteer_contribution(
        &self,
        campaign: &Campaign,
        opportunity_id: Uuid,
        hours: u32,
    ) -> Result<(), ApplicationError>;
}

pub struct ContributionValidatorImpl;

impl ContributionValidatorImpl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ContributionValidator for ContributionValidatorImpl {
    async fn validate_monetary_contribution(
        &self,
        campaign: &Campaign,
        amount: Decimal,
        currency: &str,
    ) -> Result<(), ApplicationError> {
        // Only allow monetary contributions for donation campaigns
        if !campaign.is_donation_campaign() {
            return Err(ApplicationError::MonetaryNotAllowed);
        }
        
        // Amount must be positive
        if amount <= Decimal::ZERO {
            return Err(ApplicationError::ValidationError(
                "Contribution amount must be positive".to_string()
            ));
        }
        
        // For donation campaigns, verify currency matches campaign currency
        if let Some(details) = &campaign.donation_details {
            if currency != details.currency {
                return Err(ApplicationError::ValidationError(
                    "Currency mismatch".to_string()
                ));
            }
        }
        
        Ok(())
    }
    
    async fn validate_volunteer_contribution(
        &self,
        campaign: &Campaign,
        _opportunity_id: Uuid,
        hours: u32,
    ) -> Result<(), ApplicationError> {
        // Volunteer contributions are allowed for all campaign types
        // Hours must be positive
        if hours == 0 {
            return Err(ApplicationError::ValidationError(
                "Volunteer hours must be positive".to_string()
            ));
        }
        
        Ok(())
    }
}