//! Feed service for unified social feeds with enhanced features

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use sled::{Db, Tree};
use std::collections::HashMap;
use std::time::Duration;

use crate::domain::{
    post::{UnifiedPost, AppSource},
    social_event::SocialEvent,
    feed_preferences::{FeedPreferences, FeedPreferencesRepository, FeedAlgorithmType},
};
use crate::application::feed_algorithms::{FeedAlgorithm, ChronologicalFeedAlgorithm, EngagementFeedAlgorithm};
use crate::infrastructure::repositories::UserFollowingRepository;

/// Service for generating unified social feeds with enhanced features
#[derive(Debug)]
pub struct FeedService {
    social_integration_service: Box<dyn SocialIntegrationServiceTrait + Send + Sync>,
    user_following_repository: Box<dyn UserFollowingRepository + Send + Sync>,
    feed_preferences_repository: Box<dyn FeedPreferencesRepository + Send + Sync>,
    algorithms: HashMap<FeedAlgorithmType, Box<dyn FeedAlgorithm + Send + Sync>>,
    feed_cache: Tree,
    pre_fetch_hooks: Vec<Box<dyn Fn(Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>>,
    post_process_hooks: Vec<Box<dyn Fn(&mut Vec<UnifiedPost>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>>,
}

impl FeedService {
    /// Create a new feed service with enhanced features
    pub fn new(
        social_integration_service: Box<dyn SocialIntegrationServiceTrait + Send + Sync>,
        user_following_repository: Box<dyn UserFollowingRepository + Send + Sync>,
        feed_preferences_repository: Box<dyn FeedPreferencesRepository + Send + Sync>,
        sled_db: Db,
    ) -> Self {
        let mut algorithms = HashMap::new();
        algorithms.insert(FeedAlgorithmType::Chronological, Box::new(ChronologicalFeedAlgorithm) as Box<dyn FeedAlgorithm + Send + Sync>);
        algorithms.insert(FeedAlgorithmType::Engagement, Box::new(EngagementFeedAlgorithm) as Box<dyn FeedAlgorithm + Send + Sync>);
        
        Self {
            social_integration_service,
            user_following_repository,
            feed_preferences_repository,
            algorithms,
            feed_cache: sled_db.open_tree("feed_cache").expect("Failed to open feed cache tree"),
            pre_fetch_hooks: Vec::new(),
            post_process_hooks: Vec::new(),
        }
    }
    
    /// Add a custom algorithm to the service
    pub fn add_algorithm(
        &mut self,
        algorithm_type: FeedAlgorithmType,
        algorithm: Box<dyn FeedAlgorithm + Send + Sync>
    ) {
        self.algorithms.insert(algorithm_type, algorithm);
    }
    
    /// Add a pre-fetch hook that runs before feed generation
    pub fn add_pre_fetch_hook(
        &mut self, 
        hook: Box<dyn Fn(Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>
    ) {
        self.pre_fetch_hooks.push(hook);
    }
    
    /// Add a post-process hook that runs after feed generation
    pub fn add_post_process_hook(
        &mut self, 
        hook: Box<dyn Fn(&mut Vec<UnifiedPost>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> + Send + Sync>
    ) {
        self.post_process_hooks.push(hook);
    }
    
    /// Clear the cache for a specific user
    pub fn clear_user_cache(&self, user_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let cache_key = format!("user_feed_{}", user_id);
        self.feed_cache.remove(cache_key)?;
        Ok(())
    }
    
    /// Get a user's unified feed with all enhancements
    pub async fn get_user_feed(&self, user_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // Run pre-fetch hooks
        for hook in &self.pre_fetch_hooks {
            hook(user_id)?;
        }
        
        // Check cache
        let cache_key = format!("user_feed_{}", user_id);
        if let Ok(Some(cached)) = self.feed_cache.get(&cache_key) {
            if let Ok(posts) = bincode::deserialize(&cached) {
                tracing::debug!("Feed cache hit for user {}", user_id);
                return Ok(posts);
            }
        }
        
        tracing::debug!("Feed cache miss for user {}, generating new feed", user_id);
        
        // Get user preferences
        let preferences = self.feed_preferences_repository.get_preferences(user_id).await?;
        
        // Generate feed
        let mut posts = Vec::new();
        let following = self.user_following_repository.get_following(user_id).await?;
        
        for followed_user_id in following {
            let user_posts = self.social_integration_service.get_posts_by_author(followed_user_id).await?;
            posts.extend(user_posts);
        }
        
        // Apply algorithm based on preferences
        if let Some(algorithm) = self.algorithms.get(&preferences.algorithm) {
            posts = algorithm.generate_feed(posts, user_id);
        }
        
        // Apply max_items preference
        if posts.len() > preferences.max_items as usize {
            posts.truncate(preferences.max_items as usize);
        }
        
        // Filter based on preferences
        if !preferences.include_media {
            posts.retain(|post| post.metadata.media_attachments.is_empty());
        }
        
        // Run post-process hooks
        for hook in &self.post_process_hooks {
            hook(&mut posts)?;
        }
        
        // Cache results with 5-minute TTL
        if let Ok(serialized) = bincode::serialize(&posts) {
            let _ = self.feed_cache.insert(cache_key, serialized);
            // Note: set_ttl is not directly available on Tree, this would need custom TTL handling
        }
        
        Ok(posts)
    }
    
    /// Get a chronological feed of all posts
    pub async fn get_chronological_feed(&self) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would fetch all posts from the repository
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }
    
    /// Get an algorithmic feed for a user with preferences
    pub async fn get_algorithmic_feed(&self, user_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // Use the same logic as get_user_feed but with algorithmic preferences
        let preferences = self.feed_preferences_repository.get_preferences(user_id).await?;
        
        // Temporarily change algorithm to engagement if it's chronological
        let mut temp_preferences = preferences.clone();
        if matches!(temp_preferences.algorithm, FeedAlgorithmType::Chronological) {
            temp_preferences.algorithm = FeedAlgorithmType::Engagement;
        }
        
        // Save temporary preferences
        self.feed_preferences_repository.save_preferences(user_id, temp_preferences).await?;
        
        let result = self.get_user_feed(user_id).await?;
        
        // Restore original preferences
        self.feed_preferences_repository.save_preferences(user_id, preferences).await?;
        
        Ok(result)
    }
    
    /// Add a following relationship
    pub async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = self.user_following_repository.follow(follower_id, followed_id).await;
        
        // Clear cache when following changes
        let _ = self.clear_user_cache(follower_id);
        
        result
    }
    
    /// Remove a following relationship
    pub async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let result = self.user_following_repository.unfollow(follower_id, followed_id).await;
        
        // Clear cache when following changes
        let _ = self.clear_user_cache(follower_id);
        
        result
    }
    
    /// Update user preferences
    pub async fn update_preferences(
        &self, 
        user_id: Uuid, 
        preferences: FeedPreferences
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.feed_preferences_repository.save_preferences(user_id, preferences).await?;
        
        // Clear cache when preferences change
        self.clear_user_cache(user_id)?;
        
        Ok(())
    }
    
    /// Get user preferences
    pub async fn get_preferences(&self, user_id: Uuid) -> Result<FeedPreferences, Box<dyn std::error::Error + Send + Sync>> {
        self.feed_preferences_repository.get_preferences(user_id).await
    }
}

/// Trait for social integration service (for dependency injection)
#[async_trait]
pub trait SocialIntegrationServiceTrait {
    /// Get unified posts by author
    async fn get_posts_by_author(&self, author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Handle a social event
    async fn handle_social_event(&self, event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::{
        InMemoryUserFollowingRepository, 
        InMemoryFeedPreferencesRepository
    };
    use crate::domain::post::{PostMetadata, EngagementMetrics};
    
    struct MockSocialIntegrationService;
    
    #[async_trait]
    impl SocialIntegrationServiceTrait for MockSocialIntegrationService {
        async fn get_posts_by_author(&self, author_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
            Ok(vec![UnifiedPost {
                id: Uuid::new_v4(),
                source: crate::domain::AppSource::Allat,
                original_id: Uuid::new_v4(),
                author_id,
                content: "Test post".to_string(),
                metadata: PostMetadata {
                    created_at: chrono::Utc::now(),
                    updated_at: chrono::Utc::now(),
                    engagement: EngagementMetrics::new(),
                    media_attachments: vec![],
                    hashtags: vec![],
                    privacy: crate::domain::post::PrivacySettings {
                        is_public: true,
                        allowed_viewers: vec![],
                        shareable: true,
                    },
                },
                properties: std::collections::HashMap::new(),
            }])
        }
        
        async fn handle_social_event(&self, _event: SocialEvent) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_feed_service_creation() {
        let sled_db = sled::Config::new().temporary(true).open().unwrap();
        let social_service = Box::new(MockSocialIntegrationService);
        let user_repo = Box::new(InMemoryUserFollowingRepository::new());
        let preferences_repo = Box::new(InMemoryFeedPreferencesRepository::new());
        
        let service = FeedService::new(social_service, user_repo, preferences_repo, sled_db);
        assert!(!service.algorithms.is_empty());
    }
    
    #[tokio::test]
    async fn test_get_user_feed() {
        let sled_db = sled::Config::new().temporary(true).open().unwrap();
        let social_service = Box::new(MockSocialIntegrationService);
        let user_repo = Box::new(InMemoryUserFollowingRepository::new());
        let preferences_repo = Box::new(InMemoryFeedPreferencesRepository::new());
        
        let service = FeedService::new(social_service, user_repo, preferences_repo, sled_db);
        
        let user_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        // Follow the author
        service.follow_user(user_id, author_id).await.unwrap();
        
        // Get feed
        let feed = service.get_user_feed(user_id).await.unwrap();
        assert!(!feed.is_empty());
    }
}