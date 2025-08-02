//! Contribution domain models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Types of contributions
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ContributionType {
    Monetary,
    VolunteerAction,
}

/// Status of volunteer contribution verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VerificationStatus {
    Pending,
    Verified,
    Disputed,
    Rejected,
}

/// A contribution to a campaign
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Contribution {
    pub id: Uuid,
    pub campaign_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
    
    // For monetary contributions
    pub cpay_transaction_id: Option<Uuid>,
    pub amount: Option<Decimal>,
    pub currency: Option<String>,
    
    // For volunteer actions
    pub opportunity_id: Option<Uuid>, // Links to skill_volunteering opportunities
    pub hours: Option<u32>,
    pub verification_status: Option<VerificationStatus>,
}

impl Contribution {
    /// Create a new monetary contribution
    pub fn new_monetary(
        campaign_id: Uuid,
        user_id: Uuid,
        cpay_transaction_id: Uuid,
        amount: Decimal,
        currency: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            campaign_id,
            user_id,
            created_at: Utc::now(),
            cpay_transaction_id: Some(cpay_transaction_id),
            amount: Some(amount),
            currency: Some(currency),
            opportunity_id: None,
            hours: None,
            verification_status: None,
        }
    }
    
    /// Create a new volunteer contribution
    pub fn new_volunteer(
        campaign_id: Uuid,
        user_id: Uuid,
        opportunity_id: Uuid,
        hours: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            campaign_id,
            user_id,
            created_at: Utc::now(),
            cpay_transaction_id: None,
            amount: None,
            currency: None,
            opportunity_id: Some(opportunity_id),
            hours: Some(hours),
            verification_status: Some(VerificationStatus::Pending),
        }
    }
    
    /// Get the type of contribution
    pub fn contribution_type(&self) -> ContributionType {
        if self.cpay_transaction_id.is_some() {
            ContributionType::Monetary
        } else {
            ContributionType::VolunteerAction
        }
    }
    
    /// Check if this is a monetary contribution
    pub fn is_monetary(&self) -> bool {
        matches!(self.contribution_type(), ContributionType::Monetary)
    }
    
    /// Check if this is a volunteer contribution
    pub fn is_volunteer(&self) -> bool {
        matches!(self.contribution_type(), ContributionType::VolunteerAction)
    }
    
    /// Verify a volunteer contribution
    pub fn verify(&mut self) -> Result<(), ContributionError> {
        if !self.is_volunteer() {
            return Err(ContributionError::NotVolunteerContribution);
        }
        
        self.verification_status = Some(VerificationStatus::Verified);
        Ok(())
    }
    
    /// Dispute a volunteer contribution
    pub fn dispute(&mut self) -> Result<(), ContributionError> {
        if !self.is_volunteer() {
            return Err(ContributionError::NotVolunteerContribution);
        }
        
        self.verification_status = Some(VerificationStatus::Disputed);
        Ok(())
    }
    
    /// Reject a volunteer contribution
    pub fn reject(&mut self) -> Result<(), ContributionError> {
        if !self.is_volunteer() {
            return Err(ContributionError::NotVolunteerContribution);
        }
        
        self.verification_status = Some(VerificationStatus::Rejected);
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ContributionError {
    #[error("Operation not valid for this contribution type")]
    NotVolunteerContribution,
    #[error("Contribution validation failed: {0}")]
    ValidationFailed(String),
}