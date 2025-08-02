//! Domain models for opportunity management.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a volunteer opportunity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VolunteerOpportunity {
    pub id: Uuid,
    pub cause_id: Uuid,
    pub title: String,
    pub description: String,
    pub required_skills: Vec<Uuid>,
    pub estimated_hours: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub deadline: Option<DateTime<Utc>>,
    pub created_by: Uuid,
}

impl VolunteerOpportunity {
    /// Creates a new volunteer opportunity.
    pub fn new(
        cause_id: Uuid,
        title: String,
        description: String,
        required_skills: Vec<Uuid>,
        estimated_hours: Option<i32>,
        deadline: Option<DateTime<Utc>>,
        created_by: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            cause_id,
            title,
            description,
            required_skills,
            estimated_hours,
            created_at: Utc::now(),
            deadline,
            created_by,
        }
    }
}

/// Represents the status of an application.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[sqlx(type_name = "application_status", rename_all = "lowercase")]
pub enum ApplicationStatus {
    Pending,
    Accepted,
    Rejected,
    Completed,
}

impl std::fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApplicationStatus::Pending => write!(f, "pending"),
            ApplicationStatus::Accepted => write!(f, "accepted"),
            ApplicationStatus::Rejected => write!(f, "rejected"),
            ApplicationStatus::Completed => write!(f, "completed"),
        }
    }
}

impl std::str::FromStr for ApplicationStatus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(ApplicationStatus::Pending),
            "accepted" => Ok(ApplicationStatus::Accepted),
            "rejected" => Ok(ApplicationStatus::Rejected),
            "completed" => Ok(ApplicationStatus::Completed),
            _ => Err(()),
        }
    }
}

/// Represents a user's application for an opportunity.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OpportunityApplication {
    pub id: Uuid,
    pub opportunity_id: Uuid,
    pub user_id: Uuid,
    pub applied_at: DateTime<Utc>,
    pub status: ApplicationStatus,
    pub volunteer_hours: Option<Decimal>,
}