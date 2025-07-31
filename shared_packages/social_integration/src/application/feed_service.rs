//! Feed service for unified social feeds

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::{
    post::{UnifiedPost, AppSource},
    social_event::SocialEvent,
};
use crate::infrastructure::repositories::UserFollowingRepository;

/// Service for generating unified social feeds
#[derive(Debug)]
pub struct FeedService {
    social_integration_service: Box<dyn SocialIntegrationServiceTrait + Send + Sync>,
    user_following_repository: Box<dyn UserFollowingRepository + Send + Sync>,
}

impl FeedService {
    /// Create a new feed service
    pub fn new(
        social_integration_service: Box<dyn SocialIntegrationServiceTrait + Send + Sync>,
        user_following_repository: Box<dyn UserFollowingRepository + Send + Sync>,
    ) -> Self {
        Self {
            social_integration_service,
            user_following_repository,
        }
    }
    /// Get a user's unified feed
    pub async fn get_user_feed(&self, user_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // Get posts from users that the user is following
        let mut posts = Vec::new();
        
        let following = self.user_following_repository.get_following(user_id).await?;
        for followed_user_id in following {
            let user_posts = self.social_integration_service.get_posts_by_author(followed_user_id).await?;
            posts.extend(user_posts);
        }
        
        // Sort posts by creation time (newest first)
        posts.sort_by(|a, b| b.metadata.created_at.cmp(&a.metadata.created_at));
        
        Ok(posts)
    }
    }
    
    /// Get a chronological feed of all posts
    pub async fn get_chronological_feed(&self) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // In a real implementation, this would fetch all posts from the repository
        // For now, we'll return an empty vector
        Ok(Vec::new())
    }
    
    /// Get an algorithmic feed for a user
    pub async fn get_algorithmic_feed(&self, user_id: Uuid) -> Result<Vec<UnifiedPost>, Box<dyn std::error::Error + Send + Sync>> {
        // Simple algorithmic feed based on engagement
        let mut posts = self.get_chronological_feed().await?;
        
        // Sort by engagement score (simplified)
        posts.sort_by(|a, b| {
            let a_score = a.metadata.engagement.upvotes + a.metadata.engagement.comments + a.metadata.engagement.shares;
            let b_score = b.metadata.engagement.upvotes + b.metadata.engagement.comments + b.metadata.engagement.shares;
            b_score.cmp(&a_score)
        });
        
        // Take top 50 posts
        posts.truncate(50);
        
        Ok(posts)
    }
    
    /// Add a following relationship
    pub async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.user_following_repository.follow(follower_id, followed_id).await
    }
    
    /// Remove a following relationship
    pub async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.user_following_repository.unfollow(follower_id, followed_id).await
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