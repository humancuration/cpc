//! Data models for volunteer activities and related entities

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use wallet::domain::primitives::{Money, Currency};

/// Represents a volunteer activity
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolunteerActivity {
    /// Unique identifier for the activity
    pub id: Uuid,
    
    /// User who performed the volunteer work
    pub user_id: Uuid,
    
    /// Organization associated with the activity (if any)
    pub organization_id: Option<Uuid>,
    
    /// Description of the volunteer activity
    pub description: String,
    
    /// Number of hours volunteered
    pub hours: Decimal,
    
    /// Whether the hours have been verified by an organization admin
    pub verified: bool,
    
    /// ID of the verifier (if verified)
    pub verified_by: Option<Uuid>,
    
    /// Timestamp when the activity was verified
    pub verified_at: Option<DateTime<Utc>>,
    
    /// Whether the volunteer hours have been converted to Dabloons
    pub converted_to_dabloons: bool,
    
    /// ID of the Dabloon conversion transaction (if converted)
    pub conversion_transaction_id: Option<Uuid>,
    
    /// Timestamp when the activity was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the activity was last updated
    pub updated_at: DateTime<Utc>,
}

impl VolunteerActivity {
    /// Create a new volunteer activity
    pub fn new(user_id: Uuid, organization_id: Option<Uuid>, description: String, hours: Decimal) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            organization_id,
            description,
            hours,
            verified: false,
            verified_by: None,
            verified_at: None,
            converted_to_dabloons: false,
            conversion_transaction_id: None,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Mark the activity as verified
    pub fn verify(&mut self, verifier_id: Uuid) {
        self.verified = true;
        self.verified_by = Some(verifier_id);
        self.verified_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }
    
    /// Mark the activity as converted to Dabloons
    pub fn mark_as_converted(&mut self, transaction_id: Uuid) {
        self.converted_to_dabloons = true;
        self.conversion_transaction_id = Some(transaction_id);
        self.updated_at = Utc::now();
    }
}

/// Represents a verification request for volunteer hours
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolunteerVerification {
    /// Unique identifier for the verification
    pub id: Uuid,
    
    /// ID of the volunteer activity being verified
    pub activity_id: Uuid,
    
    /// ID of the organization admin performing the verification
    pub verifier_id: Uuid,
    
    /// Status of the verification
    pub status: VerificationStatus,
    
    /// Optional notes about the verification
    pub notes: Option<String>,
    
    /// Timestamp when the verification was created
    pub created_at: DateTime<Utc>,
    
    /// Timestamp when the verification was completed
    pub completed_at: Option<DateTime<Utc>>,
}

impl VolunteerVerification {
    /// Create a new verification request
    pub fn new(activity_id: Uuid, verifier_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            activity_id,
            verifier_id,
            status: VerificationStatus::Pending,
            notes: None,
            created_at: Utc::now(),
            completed_at: None,
        }
    }
    
    /// Approve the verification
    pub fn approve(&mut self, notes: Option<String>) {
        self.status = VerificationStatus::Approved;
        self.notes = notes;
        self.completed_at = Some(Utc::now());
    }
    
    /// Reject the verification
    pub fn reject(&mut self, notes: Option<String>) {
        self.status = VerificationStatus::Rejected;
        self.notes = notes;
        self.completed_at = Some(Utc::now());
    }
}

/// Status of a volunteer verification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VerificationStatus {
    /// Verification is pending review
    Pending,
    
    /// Verification has been approved
    Approved,
    
    /// Verification has been rejected
    Rejected,
}

/// Represents a Dabloon conversion from volunteer hours
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DabloonConversion {
    /// Unique identifier for the conversion
    pub id: Uuid,
    
    /// ID of the volunteer activity that was converted
    pub activity_id: Uuid,
    
    /// User who performed the volunteer work
    pub user_id: Uuid,
    
    /// Number of hours that were converted
    pub hours_converted: Decimal,
    
    /// Amount of Dabloons credited
    pub dabloons_credited: Money,
    
    /// Transaction ID in the wallet system
    pub transaction_id: Uuid,
    
    /// Skill rate used for conversion
    pub skill_rate: Decimal,
    
    /// Timestamp when the conversion was created
    pub created_at: DateTime<Utc>,
}

impl DabloonConversion {
    /// Create a new Dabloon conversion
    pub fn new(
        activity_id: Uuid,
        user_id: Uuid,
        hours_converted: Decimal,
        dabloons_credited: Money,
        transaction_id: Uuid,
        skill_rate: Decimal,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            activity_id,
            user_id,
            hours_converted,
            dabloons_credited,
            transaction_id,
            skill_rate,
            created_at: Utc::now(),
        }
    }
}