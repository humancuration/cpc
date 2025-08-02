#[cfg(test)]
mod tests {
    use super::super::feed_service::*;
    use crate::domain::{
        post::{UnifiedPost, PostMetadata, EngagementMetrics, AppSource, PrivacySettings},
        feed_preferences::{FeedPreferences, FeedAlgorithmType},
    };
    use crate::infrastructure::repositories::{
        InMemoryUserFollowingRepository, 
        InMemoryFeedPreferencesRepository
    };
    use sled::Config;
    use uuid::Uuid;

    // Simple mock service
    struct MockSocialService;

    #[async_trait::async_trait]
    impl SocialIntegrationServiceTrait for MockSocialService {
        async fn get_posts_by_author(&self, _author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![])
        }
        
        async fn handle_social_event(&self, _event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_service_creation() {
        let sled_db = Config::new().temporary(true).open().unwrap();
        let social_service = Box::new(MockSocialService);
        let user_repo = Box::new(InMemoryUserFollowingRepository::new());
        let preferences_repo = Box::new(InMemoryFeedPreferencesRepository::new());
        
        let service = FeedService::new(social_service, user_repo, preferences_repo, sled_db);
        assert!(!service.algorithms.is_empty());
    }

    #[tokio::test]
    async fn test_preferences_management() {
        let sled_db = Config::new().temporary(true).open().unwrap();
        let social_service = Box::new(MockSocialService);
        let user_repo = Box::new(InMemoryUserFollowingRepository::new());
        let preferences_repo = Box::new(InMemoryFeedPreferencesRepository::new());
        
        let service = FeedService::new(social_service, user_repo, preferences_repo, sled_db);
        
        let user_id = Uuid::new_v4();
        let preferences = FeedPreferences {
            algorithm: FeedAlgorithmType::Engagement,
            max_items: 50,
            ..Default::default()
        };
        
        service.update_preferences(user_id, preferences.clone()).await.unwrap();
        
        let retrieved = service.get_preferences(user_id).await.unwrap();
        assert_eq!(retrieved.algorithm, preferences.algorithm);
        assert_eq!(retrieved.max_items, 50);
    }
}