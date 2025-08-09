//! Data models for skill exchange listings and related entities

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use cpc_wallet::domain::primitives::{Money, Currency};

/// Represents a skill listing in the marketplace
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillListing {
    /// Unique identifier for the listing
    pub id: Uuid,
    
    /// User offering the skill
    pub provider_id: Uuid,
    
    /// Title of the skill listing
    pub title: String,
    
    /// Description of what's being offered
    pub description: String,
    
    /// Category of the skill
    pub category: String,
    
    /// Estimated time required for the exchange
    pub estimated_time: Option<Decimal>,
    
    /// Whether the listing is active
    pub is_active: bool,
    
    /// Timestamp when the listing was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the listing was last updated
    pub updated_at: DateTime<Utc>,
}

impl SkillListing {
    /// Create a new skill listing
    pub fn new(provider_id: Uuid, title: String, description: String, category: String, estimated_time: Option<Decimal>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            provider_id,
            title,
            description,
            category,
            estimated_time,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Deactivate the listing
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = Utc::now();
    }
    
    /// Update the listing details
    pub fn update(&mut self, title: Option<String>, description: Option<String>, category: Option<String>, estimated_time: Option<Decimal>) {
        if let Some(title) = title {
            self.title = title;
        }
        if let Some(description) = description {
            self.description = description;
        }
        if let Some(category) = category {
            self.category = category;
        }
        if let Some(estimated_time) = estimated_time {
            self.estimated_time = Some(estimated_time);
        }
        self.updated_at = Utc::now();
    }
}

/// Represents a claim on a skill listing
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillClaim {
    /// Unique identifier for the claim
    pub id: Uuid,
    
    /// ID of the skill listing being claimed
    pub listing_id: Uuid,
    
    /// User claiming the skill
    pub claimant_id: Uuid,
    
    /// Status of the claim
    pub status: ClaimStatus,
    
    /// Optional message from the claimant
    pub message: Option<String>,
    
    /// Timestamp when the claim was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the claim was last updated
    pub updated_at: DateTime<Utc>,
}

impl SkillClaim {
    /// Create a new skill claim
    pub fn new(listing_id: Uuid, claimant_id: Uuid, message: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            listing_id,
            claimant_id,
            status: ClaimStatus::Pending,
            message,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Accept the claim
    pub fn accept(&mut self) {
        self.status = ClaimStatus::Accepted;
        self.updated_at = Utc::now();
    }
    
    /// Reject the claim
    pub fn reject(&mut self) {
        self.status = ClaimStatus::Rejected;
        self.updated_at = Utc::now();
    }
    
    /// Mark the claim as completed
    pub fn complete(&mut self) {
        self.status = ClaimStatus::Completed;
        self.updated_at = Utc::now();
    }
}

/// Status of a skill claim
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimStatus {
    /// Claim is pending provider response
    Pending,
    
    /// Claim has been accepted by the provider
    Accepted,
    
    /// Claim has been rejected by the provider
    Rejected,
    
    /// Claim has been completed
    Completed,
}

/// Represents the completion of a skill exchange
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillExchangeCompletion {
    /// Unique identifier for the completion record
    pub id: Uuid,
    
    /// ID of the skill listing that was exchanged
    pub listing_id: Uuid,
    
    /// ID of the claim that was completed
    pub claim_id: Uuid,
    
    /// Provider of the skill
    pub provider_id: Uuid,
    
    /// User who claimed the skill
    pub claimant_id: Uuid,
    
    /// Optional rating provided by the claimant
    pub rating: Option<SkillRating>,
    
    /// Optional payment amount if applicable
    pub payment: Option<Money>,
    
    /// Transaction ID if payment was made
    pub transaction_id: Option<Uuid>,
    
    /// Timestamp when the exchange was completed
    pub completed_at: DateTime<Utc>,
}

impl SkillExchangeCompletion {
    /// Create a new skill exchange completion
    pub fn new(
        listing_id: Uuid,
        claim_id: Uuid,
        provider_id: Uuid,
        claimant_id: Uuid,
        rating: Option<SkillRating>,
        payment: Option<Money>,
        transaction_id: Option<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            listing_id,
            claim_id,
            provider_id,
            claimant_id,
            rating,
            payment,
            transaction_id,
            completed_at: Utc::now(),
        }
    }
}

/// Rating for a skill exchange
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SkillRating {
    /// Rating value (1-5)
    pub value: u32,
    
    /// Optional review comment
    pub comment: Option<String>,
    
    /// Timestamp when the rating was created
    pub created_at: DateTime<Utc>,
}

impl SkillRating {
    /// Create a new skill rating
    pub fn new(value: u32, comment: Option<String>) -> Result<Self, String> {
        if value < 1 || value > 5 {
            return Err("Rating must be between 1 and 5".to_string());
        }
        
        Ok(Self {
            value,
            comment,
            created_at: Utc::now(),
        })
    }
}