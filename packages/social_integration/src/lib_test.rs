//! Tests for the social integration crate

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{
        post::{UnifiedPost, AppSource, PostMetadata, EngagementMetrics, PrivacySettings},
        social_event::{SocialEvent, VoteType},
    };
    use crate::application::{
        social_integration_service::SocialIntegrationService,
        feed_service::FeedService,
    };
    use crate::infrastructure::{
        repositories::InMemoryUnifiedPostRepository,
        clients::{AllatClient, YapperClient},
    };
    use cpc_wallet::domain::{
        wallet::Wallet,
        primitives::{Money, Currency},
    };
    use uuid::Uuid;
    use chrono::Utc;

    #[test]
    fn test_unified_post_creation() {
        let post_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        let content = "Hello, world!".to_string();
        
        let metadata = PostMetadata {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            engagement: EngagementMetrics::new(),
            media_attachments: Vec::new(),
            hashtags: Vec::new(),
            privacy: PrivacySettings {
                is_public: true,
                allowed_viewers: Vec::new(),
                shareable: true,
            },
        };
        
        let post = UnifiedPost::new(
            AppSource::Allat,
            post_id,
            author_id,
            content.clone(),
            metadata,
        );
        
        assert_eq!(post.source, AppSource::Allat);
        assert_eq!(post.original_id, post_id);
        assert_eq!(post.author_id, author_id);
        assert_eq!(post.content, content);
    }
    
    #[test]
    fn test_social_event_user_id() {
        let user_id = Uuid::new_v4();
        let post_id = Uuid::new_v4();
        
        let event = SocialEvent::PostCreated {
            user_id,
            post_id,
            timestamp: Utc::now(),
        };
        
        assert_eq!(event.user_id(), &user_id);
    }
    
    #[test]
    #[tokio::test]
    async fn test_in_memory_repository() {
        let repo = InMemoryUnifiedPostRepository::new();
        let post_id = Uuid::new_v4();
        let author_id = Uuid::new_v4();
        
        let metadata = PostMetadata {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            engagement: EngagementMetrics::new(),
            media_attachments: Vec::new(),
            hashtags: Vec::new(),
            privacy: PrivacySettings {
                is_public: true,
                allowed_viewers: Vec::new(),
                shareable: true,
            },
        };
        
        let post = UnifiedPost::new(
            AppSource::Allat,
            post_id,
            author_id,
            "Test post".to_string(),
            metadata,
        );
        
        // Save the post
        assert!(repo.save(&post).await.is_ok());
        
        // Find the post by ID
        let found = repo.find_by_id(post.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, post.id);
        
        // Find posts by author
        let author_posts = repo.find_by_author(author_id).await.unwrap();
        assert_eq!(author_posts.len(), 1);
        assert_eq!(author_posts[0].id, post.id);
        
        // Find posts by source
        let source_posts = repo.find_by_source(AppSource::Allat).await.unwrap();
        assert_eq!(source_posts.len(), 1);
        assert_eq!(source_posts[0].id, post.id);
    }
}