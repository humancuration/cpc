//! Social integration helpers for skill volunteering

use crate::endorsement_management::models::SkillEndorsement;
use shared_packages::social_integration::domain::social_event::SocialEvent;
use uuid::Uuid;
use chrono::Utc;

/// Helper for creating social events related to skill volunteering
pub struct SkillVolunteeringSocialIntegration;

impl SkillVolunteeringSocialIntegration {
    /// Create a social event when an opportunity is shared
    pub fn create_opportunity_shared_event(user_id: Uuid, opportunity_id: Uuid) -> SocialEvent {
        SocialEvent::OpportunityShared {
            user_id,
            opportunity_id,
            timestamp: Utc::now(),
        }
    }
    
    /// Create a social event when a user volunteers for an opportunity
    pub fn create_volunteered_event(user_id: Uuid, opportunity_id: Uuid, hours_contributed: f32) -> SocialEvent {
        SocialEvent::Volunteered {
            user_id,
            opportunity_id,
            hours_contributed,
            timestamp: Utc::now(),
        }
    }
    
    /// Create a social event when a skill endorsement is given
    pub fn create_skill_endorsed_event(
        endorser_id: Uuid,
        recipient_id: Uuid,
        skill_id: Uuid,
        rating: u32,
    ) -> SocialEvent {
        // This could be a new event type if we want to track it separately
        // For now, we'll use the Volunteered event as a proxy for skill-related activities
        SocialEvent::Volunteered {
            user_id: endorser_id,
            opportunity_id: skill_id, // Using skill_id as proxy
            hours_contributed: rating as f32,
            timestamp: Utc::now(),
        }
    }
}

/// Trait for services that want to integrate with social systems
#[async_trait::async_trait]
pub trait SocialIntegrationClient: Send + Sync {
    /// Handle a social event
    async fn handle_social_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Mock implementation for testing
pub struct MockSocialIntegrationClient;

#[async_trait::async_trait]
impl SocialIntegrationClient for MockSocialIntegrationClient {
    async fn handle_social_event(&self, _event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
}