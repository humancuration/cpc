//! Achievement system for social enhancements

use async_trait::async_trait;
use uuid::Uuid;
use crate::models::{Achievement, AchievementType, VolunteerMilestone};
use volunteer_core::models::VolunteerActivity;
use skill_exchange_core::models::SkillExchangeCompletion;
use notification_core::application::service::NotificationService;
use notification_core::domain::{types::{Notification, NotificationCategory, NotificationPriority, ChannelType}, preferences::UserPreferences};
use common_utils::error::CommonError;
use std::sync::Arc;

// Conditionally import common_utils logging or fallback to tracing
#[cfg(feature = "common-utils-integration")]
use common_utils::logging::{info, warn, error, debug};
#[cfg(not(feature = "common-utils-integration"))]
use tracing::{info, warn, error, debug};

/// Service trait for achievement operations
#[async_trait]
pub trait AchievementService {
    /// Check and award volunteer hour achievements
    async fn check_volunteer_achievements(&self, user_id: Uuid, total_volunteer_hours: f64) -> Result<Vec<Achievement>, CommonError>;
    
    /// Check and award skill mastery achievements
    async fn check_skill_achievements(&self, user_id: Uuid, completed_exchanges: usize) -> Result<Vec<Achievement>, CommonError>;
    
    /// Award a specific achievement to a user
    async fn award_achievement(&self, user_id: Uuid, achievement_type: AchievementType) -> Result<Achievement, CommonError>;
}

/// Implementation of the AchievementService
pub struct AchievementServiceImpl {
    notification_service: Arc<dyn NotificationService>,
}

impl AchievementServiceImpl {
    /// Create a new achievement service
    pub fn new(notification_service: Arc<dyn NotificationService>) -> Self {
        Self {
            notification_service,
        }
    }
    
    /// Send a notification about a new achievement
    async fn send_achievement_notification(&self, user_id: Uuid, achievement: &Achievement) -> Result<(), CommonError> {
        let notification = Notification::new_immediate(
            user_id.to_string(),
            NotificationCategory::Social,
            NotificationPriority::High,
            "New Achievement Earned!".to_string(),
            format!("Congratulations! You've earned the '{}' achievement.", achievement.title),
            serde_json::to_value(achievement).unwrap_or(serde_json::Value::Null),
            vec![ChannelType::InApp, ChannelType::Email],
        );
        
        self.notification_service.send_notification(notification).await
            .map_err(|e| CommonError::ServiceError(format!("Failed to send achievement notification: {}", e)))
    }
    
    /// Get achievement details based on type
    fn get_achievement_details(&self, achievement_type: &AchievementType) -> (String, String, Option<String>) {
        match achievement_type {
            AchievementType::VolunteerHours(milestone) => {
                match milestone {
                    VolunteerMilestone::TenHours => (
                        "10-Hour Volunteer".to_string(),
                        "You've contributed 10 hours of volunteer work!".to_string(),
                        Some("https://example.com/badges/volunteer-10h.png".to_string()),
                    ),
                    VolunteerMilestone::FiftyHours => (
                        "50-Hour Volunteer".to_string(),
                        "You've contributed 50 hours of volunteer work!".to_string(),
                        Some("https://example.com/badges/volunteer-50h.png".to_string()),
                    ),
                    VolunteerMilestone::HundredHours => (
                        "100-Hour Volunteer".to_string(),
                        "You've contributed 100 hours of volunteer work!".to_string(),
                        Some("https://example.com/badges/volunteer-100h.png".to_string()),
                    ),
                    VolunteerMilestone::FiveHundredHours => (
                        "500-Hour Volunteer".to_string(),
                        "You've contributed 500 hours of volunteer work!".to_string(),
                        Some("https://example.com/badges/volunteer-500h.png".to_string()),
                    ),
                }
            },
            AchievementType::SkillMaster => (
                "Skill Master".to_string(),
                "You've completed 10 skill exchanges!".to_string(),
                Some("https://example.com/badges/skill-master.png".to_string()),
            ),
            AchievementType::ChallengeCompleted => (
                "Challenge Champion".to_string(),
                "You've completed a community challenge!".to_string(),
                Some("https://example.com/badges/challenge-champion.png".to_string()),
            ),
            AchievementType::SocialEngagement => (
                "Social Butterfly".to_string(),
                "You're actively engaging with the community!".to_string(),
                Some("https://example.com/badges/social-butterfly.png".to_string()),
            ),
        }
    }
}

#[async_trait]
impl AchievementService for AchievementServiceImpl {
    async fn check_volunteer_achievements(&self, user_id: Uuid, total_volunteer_hours: f64) -> Result<Vec<Achievement>, CommonError> {
        let mut achievements = Vec::new();
        
        // Check for different volunteer hour milestones
        if total_volunteer_hours >= 10.0 && total_volunteer_hours < 50.0 {
            let achievement = self.award_achievement(
                user_id,
                AchievementType::VolunteerHours(VolunteerMilestone::TenHours)
            ).await?;
            achievements.push(achievement);
        }
        
        if total_volunteer_hours >= 50.0 && total_volunteer_hours < 100.0 {
            let achievement = self.award_achievement(
                user_id,
                AchievementType::VolunteerHours(VolunteerMilestone::FiftyHours)
            ).await?;
            achievements.push(achievement);
        }
        
        if total_volunteer_hours >= 100.0 && total_volunteer_hours < 500.0 {
            let achievement = self.award_achievement(
                user_id,
                AchievementType::VolunteerHours(VolunteerMilestone::HundredHours)
            ).await?;
            achievements.push(achievement);
        }
        
        if total_volunteer_hours >= 500.0 {
            let achievement = self.award_achievement(
                user_id,
                AchievementType::VolunteerHours(VolunteerMilestone::FiveHundredHours)
            ).await?;
            achievements.push(achievement);
        }
        
        Ok(achievements)
    }
    
    async fn check_skill_achievements(&self, user_id: Uuid, completed_exchanges: usize) -> Result<Vec<Achievement>, CommonError> {
        let mut achievements = Vec::new();
        
        // Check for skill mastery (10 completed exchanges)
        if completed_exchanges >= 10 {
            let achievement = self.award_achievement(
                user_id,
                AchievementType::SkillMaster
            ).await?;
            achievements.push(achievement);
        }
        
        Ok(achievements)
    }
    
    async fn award_achievement(&self, user_id: Uuid, achievement_type: AchievementType) -> Result<Achievement, CommonError> {
        // Get achievement details
        let (title, description, image_url) = self.get_achievement_details(&achievement_type);
        
        // Create the achievement
        let achievement = Achievement::new(user_id, achievement_type, title, description, image_url);
        
        // Send notification
        self.send_achievement_notification(user_id, &achievement).await?;
        
        // Log the achievement
        info!("Awarded achievement '{}' to user {}", achievement.title, user_id);
        
        Ok(achievement)
    }
}