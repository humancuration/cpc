//! Campaign domain models

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

/// Types of campaigns in the cooperative fundraising system
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CampaignType {
    /// Pure community participation drive (no monetary aspect)
    /// Grants 1 membership share per participant (max 1 per person)
    CooperativeMembership,
    
    /// Pure donation drive (no membership implications)
    /// Like GoFundMe - funds specific external needs
    PureDonation,
    
    /// SEC Regulation Crowdfunding (Reg CF)
    /// [TODO: Regulatory] - External compliance only
    RegCF,
    
    /// SEC Regulation A+ (Reg A)
    /// [TODO: Regulatory] - External compliance only
    RegA,
    
    /// SEC Regulation D (Reg D)
    /// [TODO: Regulatory] - External compliance only
    RegD,
}

/// Status of a campaign
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CampaignStatus {
    Draft,
    Active,
    Completed,
    Failed,
    Cancelled,
}

/// Requirements for cooperative membership campaigns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MembershipRequirements {
    pub max_participants: Option<u32>,
    pub required_actions: Vec<String>, // e.g., "attend_meeting", "complete_onboarding"
}

/// Details for donation campaigns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DonationDetails {
    pub funding_goal: Option<Decimal>,
    pub external_use_case: String, // Required for compliance
    pub currency: String,
}

/// A fundraising campaign
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Campaign {
    pub id: Uuid,
    pub campaign_type: CampaignType,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub owner_user_id: Uuid,
    pub status: CampaignStatus,
    
    // For membership campaigns
    pub membership_requirements: Option<MembershipRequirements>,
    
    // For donation campaigns
    pub donation_details: Option<DonationDetails>,
}

impl Campaign {
    /// Create a new cooperative membership campaign
    pub fn new_membership_campaign(
        title: String,
        description: String,
        owner_user_id: Uuid,
        requirements: MembershipRequirements,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            campaign_type: CampaignType::CooperativeMembership,
            title,
            description,
            created_at: Utc::now(),
            owner_user_id,
            status: CampaignStatus::Draft,
            membership_requirements: Some(requirements),
            donation_details: None,
        }
    }
    
    /// Create a new donation campaign
    pub fn new_donation_campaign(
        campaign_type: CampaignType,
        title: String,
        description: String,
        owner_user_id: Uuid,
        details: DonationDetails,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            campaign_type,
            title,
            description,
            created_at: Utc::now(),
            owner_user_id,
            status: CampaignStatus::Draft,
            membership_requirements: None,
            donation_details: Some(details),
        }
    }
    
    /// Check if this is a membership campaign
    pub fn is_membership_campaign(&self) -> bool {
        matches!(self.campaign_type, CampaignType::CooperativeMembership)
    }
    
    /// Check if this is a donation campaign
    pub fn is_donation_campaign(&self) -> bool {
        matches!(
            self.campaign_type,
            CampaignType::PureDonation | CampaignType::RegCF | CampaignType::RegA | CampaignType::RegD
        )
    }
    
    /// Activate the campaign
    pub fn activate(&mut self) -> Result<(), CampaignError> {
        if self.status != CampaignStatus::Draft {
            return Err(CampaignError::InvalidStatusTransition);
        }
        
        self.status = CampaignStatus::Active;
        Ok(())
    }
    
    /// Complete the campaign
    pub fn complete(&mut self) -> Result<(), CampaignError> {
        if self.status != CampaignStatus::Active {
            return Err(CampaignError::InvalidStatusTransition);
        }
        
        self.status = CampaignStatus::Completed;
        Ok(())
    }
    
    /// Fail the campaign
    pub fn fail(&mut self) -> Result<(), CampaignError> {
        if self.status != CampaignStatus::Active {
            return Err(CampaignError::InvalidStatusTransition);
        }
        
        self.status = CampaignStatus::Failed;
        Ok(())
    }
    
    /// Cancel the campaign
    pub fn cancel(&mut self) -> Result<(), CampaignError> {
        if !matches!(self.status, CampaignStatus::Draft | CampaignStatus::Active) {
            return Err(CampaignError::InvalidStatusTransition);
        }
        
        self.status = CampaignStatus::Cancelled;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum CampaignError {
    #[error("Invalid status transition")]
    InvalidStatusTransition,
    #[error("Campaign validation failed: {0}")]
    ValidationFailed(String),
}