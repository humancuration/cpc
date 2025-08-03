//! Service layer tests for achievement functionality

#[cfg(test)]
mod tests {
    use super::*;
    use crate::achievements::AchievementServiceImpl;
    use crate::models::{AchievementType, VolunteerMilestone};
    use uuid::Uuid;
    use common_utils::error::CommonError;
    use std::sync::Arc;
    
    // Mock notification service implementation
    struct MockNotificationService {
        should_fail: bool,
    }
    
    #[async_trait::async_trait]
    impl notification_core::application::service::NotificationService for MockNotificationService {
        async fn send_notification(&self, _notification: notification_core::domain::types::Notification) -> Result<(), notification_core::domain::error::NotificationError> {
            if self.should_fail {
                Err(notification_core::domain::error::NotificationError::ServiceError("Notification service error".to_string()))
            } else {
                Ok(())
            }
        }
        
        async fn get_user_preferences(&self, _user_id: &str) -> Result<notification_core::domain::preferences::UserPreferences, notification_core::domain::error::NotificationError> {
            if self.should_fail {
                Err(notification_core::domain::error::NotificationError::ServiceError("Notification service error".to_string()))
            } else {
                Ok(notification_core::domain::preferences::UserPreferences::default())
            }
        }
        
        async fn update_user_preferences(&self, _user_id: &str, _preferences: notification_core::domain::preferences::UserPreferences) -> Result<(), notification_core::domain::error::NotificationError> {
            if self.should_fail {
                Err(notification_core::domain::error::NotificationError::ServiceError("Notification service error".to_string()))
            } else {
                Ok(())
            }
        }
    }
    
    #[tokio::test]
    async fn test_check_volunteer_achievements_ten_hours() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute
        let result = service.check_volunteer_achievements(
            Uuid::new_v4(),
            10.0, // Exactly 10 hours
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 1);
        
        // Check that it's the correct achievement type
        match &achievements[0].achievement_type {
            AchievementType::VolunteerHours(VolunteerMilestone::TenHours) => {},
            _ => panic!("Expected TenHours achievement"),
        }
    }
    
    #[tokio::test]
    async fn test_check_volunteer_achievements_fifty_hours() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute
        let result = service.check_volunteer_achievements(
            Uuid::new_v4(),
            50.0, // Exactly 50 hours
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 1);
        
        // Check that it's the correct achievement type
        match &achievements[0].achievement_type {
            AchievementType::VolunteerHours(VolunteerMilestone::FiftyHours) => {},
            _ => panic!("Expected FiftyHours achievement"),
        }
    }
    
    #[tokio::test]
    async fn test_check_volunteer_achievements_hundred_hours() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute
        let result = service.check_volunteer_achievements(
            Uuid::new_v4(),
            100.0, // Exactly 100 hours
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 1);
        
        // Check that it's the correct achievement type
        match &achievements[0].achievement_type {
            AchievementType::VolunteerHours(VolunteerMilestone::HundredHours) => {},
            _ => panic!("Expected HundredHours achievement"),
        }
    }
    
    #[tokio::test]
    async fn test_check_volunteer_achievements_five_hundred_hours() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute
        let result = service.check_volunteer_achievements(
            Uuid::new_v4(),
            500.0, // Exactly 500 hours
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 1);
        
        // Check that it's the correct achievement type
        match &achievements[0].achievement_type {
            AchievementType::VolunteerHours(VolunteerMilestone::FiveHundredHours) => {},
            _ => panic!("Expected FiveHundredHours achievement"),
        }
    }
    
    #[tokio::test]
    async fn test_check_volunteer_achievements_no_milestone() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute with less than 10 hours
        let result = service.check_volunteer_achievements(
            Uuid::new_v4(),
            5.0, // Less than 10 hours
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 0);
    }
    
    #[tokio::test]
    async fn test_check_skill_achievements_master() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute with 10 completed exchanges
        let result = service.check_skill_achievements(
            Uuid::new_v4(),
            10, // Exactly 10 exchanges
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 1);
        
        // Check that it's the correct achievement type
        match &achievements[0].achievement_type {
            AchievementType::SkillMaster => {},
            _ => panic!("Expected SkillMaster achievement"),
        }
    }
    
    #[tokio::test]
    async fn test_check_skill_achievements_not_enough() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        
        // Execute with less than 10 completed exchanges
        let result = service.check_skill_achievements(
            Uuid::new_v4(),
            5, // Less than 10 exchanges
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievements = result.unwrap();
        assert_eq!(achievements.len(), 0);
    }
    
    #[tokio::test]
    async fn test_award_achievement_success() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.award_achievement(
            user_id,
            AchievementType::VolunteerHours(VolunteerMilestone::TenHours),
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievement = result.unwrap();
        assert_eq!(achievement.user_id, user_id);
        assert_eq!(achievement.title, "10-Hour Volunteer");
        assert_eq!(achievement.description, "You've contributed 10 hours of volunteer work!");
    }
    
    #[tokio::test]
    async fn test_award_achievement_notification_failure() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: true });
        let service = AchievementServiceImpl::new(notification_service);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.award_achievement(
            user_id,
            AchievementType::VolunteerHours(VolunteerMilestone::TenHours),
        ).await;
        
        // Assert - Even if notification fails, the achievement should still be awarded
        assert!(result.is_ok());
        let achievement = result.unwrap();
        assert_eq!(achievement.user_id, user_id);
    }
    
    #[tokio::test]
    async fn test_award_skill_master_achievement() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.award_achievement(
            user_id,
            AchievementType::SkillMaster,
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievement = result.unwrap();
        assert_eq!(achievement.user_id, user_id);
        assert_eq!(achievement.title, "Skill Master");
        assert_eq!(achievement.description, "You've completed 10 skill exchanges!");
    }
    
    #[tokio::test]
    async fn test_award_challenge_completed_achievement() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.award_achievement(
            user_id,
            AchievementType::ChallengeCompleted,
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievement = result.unwrap();
        assert_eq!(achievement.user_id, user_id);
        assert_eq!(achievement.title, "Challenge Champion");
        assert_eq!(achievement.description, "You've completed a community challenge!");
    }
    
    #[tokio::test]
    async fn test_award_social_engagement_achievement() {
        // Setup
        let notification_service = Arc::new(MockNotificationService { should_fail: false });
        let service = AchievementServiceImpl::new(notification_service);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.award_achievement(
            user_id,
            AchievementType::SocialEngagement,
        ).await;
        
        // Assert
        assert!(result.is_ok());
        let achievement = result.unwrap();
        assert_eq!(achievement.user_id, user_id);
        assert_eq!(achievement.title, "Social Butterfly");
        assert_eq!(achievement.description, "You're actively engaging with the community!");
    }
}