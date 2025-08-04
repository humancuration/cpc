//! Event bus integration helpers for Volunteer Coordination
//! Reuses the SocialEventBus and adds convenience builders for volunteer events.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub use social_interactions::infrastructure::event_bus::{SocialEvent, SocialEventBus};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolunteerEvents;

impl VolunteerEvents {
    pub fn opportunity_created(opportunity_id: Uuid, org_id: Uuid, created_by: Uuid) -> SocialEvent {
        SocialEvent::OpportunityCreated { opportunity_id, org_id, created_by }
    }

    pub fn application_submitted(application_id: Uuid, opportunity_id: Uuid, applicant_id: Uuid) -> SocialEvent {
        SocialEvent::ApplicationSubmitted { application_id, opportunity_id, applicant_id }
    }

    pub fn contribution_logged(contribution_id: Uuid, opportunity_id: Uuid, contributor_id: Uuid) -> SocialEvent {
        SocialEvent::ContributionLogged { contribution_id, opportunity_id, contributor_id }
    }
}