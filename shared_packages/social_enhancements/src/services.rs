//! Service layer for social enhancements

use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{GroupChallenge, ChallengeProgress};
use social_integration::domain::social_event::SocialEvent;
use social_integration::application::social_integration_service::SocialIntegrationService;
use volunteer_core::models::VolunteerActivity;
use skill_exchange_core::models::SkillExchangeCompletion;
use common_utils::error::CommonError;
use std::sync::Arc;

// Conditionally import common_utils logging or fallback to tracing
#[cfg(feature = "common-utils-integration")]
use common_utils::logging::{info, warn, error, debug};
#[cfg(not(feature = "common-utils-integration"))]
use tracing::{info, warn, error, debug};

/// Service trait for social enhancement operations
#[async_trait]
pub trait SocialEnhancementService {
    /// Automatically post a volunteer activity to the social feed
    async fn post_volunteer_activity(&self, activity: &VolunteerActivity) -> Result<(), CommonError>;
    
    /// Automatically post a skill exchange completion to the social feed
    async fn post_skill_exchange_completion(&self, completion: &SkillExchangeCompletion) -> Result<(), CommonError>;
    
    /// Update challenge progress for a user
    async fn update_challenge_progress(&self, challenge_id: Uuid, user_id: Uuid, progress: f64) -> Result<ChallengeProgress, CommonError>;
    
    /// Get active group challenges
    async fn get_active_challenges(&self) -> Result<Vec<GroupChallenge>, CommonError>;
}

/// Implementation of the SocialEnhancementService
pub struct SocialEnhancementServiceImpl {
    social_service: Arc<dyn SocialIntegrationService>,
}

impl SocialEnhancementServiceImpl {
    /// Create a new social enhancement service
    pub fn new(social_service: Arc<dyn SocialIntegrationService>) -> Self {
        Self {
            social_service,
        }
    }
    
    /// Create a social event
    async fn create_social_event(&self, event: SocialEvent) -> Result<(), CommonError> {
        self.social_service.handle_social_event(event).await
            .map_err(|e| CommonError::ServiceError(format!("Failed to create social event: {}", e)))
    }
}

#[async_trait]
impl SocialEnhancementService for SocialEnhancementServiceImpl {
    async fn post_volunteer_activity(&self, activity: &VolunteerActivity) -> Result<(), CommonError> {
        // Only post verified activities
        if !activity.verified {
            return Ok(());
        }
        
        // Create a social event for the volunteer activity
        let event = SocialEvent::Volunteered {
            user_id: activity.user_id,
            opportunity_id: activity.id,
            hours_contributed: activity.hours.to_f32().unwrap_or(0.0),
            timestamp: activity.created_at,
        };
        
        self.create_social_event(event).await
    }
    
    async fn post_skill_exchange_completion(&self, completion: &SkillExchangeCompletion) -> Result<(), CommonError> {
        // Create a social event for the skill exchange completion
        // We'll use the Volunteered event type as a proxy for skill-related activities
        let event = SocialEvent::Volunteered {
            user_id: completion.claimant_id,
            opportunity_id: completion.listing_id,
            hours_contributed: 1.0, // Default value for skill exchanges
            timestamp: completion.completed_at,
        };
        
        self.create_social_event(event).await
    }
    
    async fn update_challenge_progress(&self, challenge_id: Uuid, user_id: Uuid, progress: f64) -> Result<ChallengeProgress, CommonError> {
        // In a real implementation, this would update the database
        // For now, we'll just create and return a progress record
        let progress_record = ChallengeProgress::new(
            challenge_id,
            user_id,
            rust_decimal::Decimal::from_f64(progress).unwrap_or_default(),
        );
        
        // Log the progress update
        info!("Updated challenge progress for user {}: {}%", user_id, progress);
        
        Ok(progress_record)
    }
    
    async fn get_active_challenges(&self) -> Result<Vec<GroupChallenge>, CommonError> {
        // In a real implementation, this would query the database for active challenges
        // For now, we'll return an empty vector
        Ok(vec![])
    }
}