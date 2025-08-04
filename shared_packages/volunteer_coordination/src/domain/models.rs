//! Domain models for Volunteer Coordination
//!
//! Core entities: VolunteerOpportunity, VolunteerApplication, VolunteerContribution,
//! plus value types and errors. Mirrors collaborative_workspace style.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Identifier newtypes for clarity
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct OpportunityId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ApplicationId(pub Uuid);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ContributionId(pub Uuid);

/// Status for an opportunity lifecycle
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OpportunityStatus {
    Draft,
    Published,
    Closed,
    Archived,
}

/// Status for an application
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApplicationStatus {
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
    Withdrawn,
}

/// Contribution type/category
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContributionKind {
    Hours,
    Deliverable,
    Donation,
    Other,
}

/// Volunteer Opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerOpportunity {
    pub id: OpportunityId,
    pub org_id: Uuid,
    pub created_by: Uuid,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub status: OpportunityStatus,
    pub location: Option<String>,
    pub starts_at: Option<DateTime<Utc>>,
    pub ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Application to an opportunity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerApplication {
    pub id: ApplicationId,
    pub opportunity_id: OpportunityId,
    pub applicant_id: Uuid,
    pub motivation: Option<String>,
    pub status: ApplicationStatus,
    pub submitted_at: DateTime<Utc>,
    pub decided_at: Option<DateTime<Utc>>,
    pub reviewer_id: Option<Uuid>,
}

/// Logged contribution for an opportunity by a contributor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerContribution {
    pub id: ContributionId,
    pub opportunity_id: OpportunityId,
    pub contributor_id: Uuid,
    pub kind: ContributionKind,
    /// For hours, this is the amount of hours; for donation/deliverable it can be quantity or points.
    pub amount: f32,
    pub notes: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    /// Whether verified by reputation service or organizer
    pub verified: bool,
    /// Optional verification reference id from reputation system
    pub verification_ref: Option<Uuid>,
}

/// Validation helpers (simple)
impl VolunteerOpportunity {
    pub fn validate(&self) -> Result<(), String> {
        if self.title.trim().is_empty() {
            return Err("Title cannot be empty".into());
        }
        if let (Some(s), Some(e)) = (self.starts_at, self.ends_at) {
            if e < s {
                return Err("ends_at cannot be earlier than starts_at".into());
            }
        }
        Ok(())
    }
}

impl VolunteerApplication {
    pub fn validate(&self) -> Result<(), String> {
        if self.applicant_id == Uuid::nil() {
            return Err("Applicant id is required".into());
        }
        Ok(())
    }
}

impl VolunteerContribution {
    pub fn validate(&self) -> Result<(), String> {
        if self.amount < 0.0 {
            return Err("Amount cannot be negative".into());
        }
        Ok(())
    }
}