//! Tests for the PostgreSQL unified post repository

#[cfg(test)]
mod tests {
    use super::*;
    use crate::infrastructure::repositories::PostgresUnifiedPostRepository;
    use crate::domain::{
        post::{UnifiedPost, AppSource, PostMetadata, EngagementMetrics, PrivacySettings, MediaAttachment, MediaType},
        social_event::SocialEvent,
    };
    use sqlx::PgPool;
    use uuid::Uuid;
    use chrono::Utc;
    use std::collections::HashMap;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    
    /// Create a test unified post
    fn create_test_post(source: AppSource, author_id: Uuid) -> UnifiedPost {
        let metadata = PostMetadata {
            created_at: Utc::now(),
            updated_at: Utc::now(),
            engagement: EngagementMetrics {
                upvotes: 10,
                comments: 5,
                shares: 3,
                views: 100,
            },
            media_attachments: vec![
                MediaAttachment {
                    id: Uuid::new_v4(),
                    url: "https://example.com/image.jpg".to_string(),
                    media_type: MediaType::Image,
                    alt_text: Some("Test image".to_string()),
                }
            ],
            hashtags: vec!["test".to_string(), "social".to_string()],
            privacy: PrivacySettings {
                is_public: true,
                allowed_viewers: vec![],
                shareable: true,
            },
        };
        
        let mut post = UnifiedPost::new(
            source,
            Uuid::new_v4(),
            author_id,
            "Test post content".to_string(),
            metadata,
        );
        
        post.add_property("test_key".to_string(), "test_value".to_string());
        
        post
    }
    
    #[sqlx::test]
    async fn test_save_and_find_by_id(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        let post = create_test_post(AppSource::Allat, author_id);
        let post_id = post.id;
        
        // Save the post
        repo.save(&post).await.unwrap();
        
        // Find the post by ID
        let found = repo.find_by_id(post_id).await.unwrap();
        assert!(found.is_some());
        
        let found_post = found.unwrap();
        assert_eq!(found_post.id, post.id);
        assert_eq!(found_post.source, post.source);
        assert_eq!(found_post.author_id, post.author_id);
        assert_eq!(found_post.content, post.content);
        assert_eq!(found_post.metadata.engagement.upvotes, post.metadata.engagement.upvotes);
        assert_eq!(found_post.metadata.engagement.comments, post.metadata.engagement.comments);
        assert_eq!(found_post.metadata.engagement.shares, post.metadata.engagement.shares);
        assert_eq!(found_post.get_property("test_key"), post.get_property("test_key"));
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_save_updates_existing_post(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        let mut post = create_test_post(AppSource::Yapper, author_id);
        
        // Save the initial post
        repo.save(&post).await.unwrap();
        
        // Update the post
        post.metadata.engagement.upvotes = 20;
        post.metadata.engagement.comments = 10;
        post.add_property("updated_key".to_string(), "updated_value".to_string());
        
        // Save the updated post
        repo.save(&post).await.unwrap();
        
        // Find the updated post
        let found = repo.find_by_id(post.id).await.unwrap();
        assert!(found.is_some());
        
        let found_post = found.unwrap();
        assert_eq!(found_post.metadata.engagement.upvotes, 20);
        assert_eq!(found_post.metadata.engagement.comments, 10);
        assert_eq!(found_post.get_property("updated_key"), Some(&"updated_value".to_string()));
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_find_by_author(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        let other_author_id = Uuid::new_v4();
        
        // Create posts for the author
        let post1 = create_test_post(AppSource::Allat, author_id);
        let post2 = create_test_post(AppSource::Yapper, author_id);
        
        // Create a post for another author
        let other_post = create_test_post(AppSource::Allat, other_author_id);
        
        // Save all posts
        repo.save(&post1).await.unwrap();
        repo.save(&post2).await.unwrap();
        repo.save(&other_post).await.unwrap();
        
        // Find posts by author
        let author_posts = repo.find_by_author(author_id).await.unwrap();
        assert_eq!(author_posts.len(), 2);
        
        // Verify all posts belong to the author
        for post in author_posts {
            assert_eq!(post.author_id, author_id);
        }
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_find_by_source(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        
        // Create posts from different sources
        let allat_post = create_test_post(AppSource::Allat, author_id);
        let yapper_post = create_test_post(AppSource::Yapper, author_id);
        
        // Save all posts
        repo.save(&allat_post).await.unwrap();
        repo.save(&yapper_post).await.unwrap();
        
        // Find posts by source
        let allat_posts = repo.find_by_source(AppSource::Allat).await.unwrap();
        let yapper_posts = repo.find_by_source(AppSource::Yapper).await.unwrap();
        
        assert_eq!(allat_posts.len(), 1);
        assert_eq!(yapper_posts.len(), 1);
        
        assert_eq!(allat_posts[0].source, AppSource::Allat);
        assert_eq!(yapper_posts[0].source, AppSource::Yapper);
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_find_nonexistent_post_returns_none(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let nonexistent_id = Uuid::new_v4();
        
        // Try to find a post that doesn't exist
        let found = repo.find_by_id(nonexistent_id).await.unwrap();
        assert!(found.is_none());
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_find_by_author_returns_empty_for_no_posts(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        
        // Find posts by author with no posts
        let author_posts = repo.find_by_author(author_id).await.unwrap();
        assert!(author_posts.is_empty());
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_find_by_source_returns_empty_for_no_posts(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        
        // Find posts by source with no posts
        let source_posts = repo.find_by_source(AppSource::Allat).await.unwrap();
        assert!(source_posts.is_empty());
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_json_serialization_deserialization(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        
        // Create a post with complex properties
        let mut post = create_test_post(AppSource::Allat, author_id);
        post.add_property("complex_key".to_string(), r#"{"nested": "value", "number": 42}"#.to_string());
        post.add_property("array_key".to_string(), r#"["item1", "item2", "item3"]"#.to_string());
        
        // Save the post
        repo.save(&post).await.unwrap();
        
        // Retrieve the post
        let found = repo.find_by_id(post.id).await.unwrap();
        assert!(found.is_some());
        
        let found_post = found.unwrap();
        assert_eq!(found_post.get_property("complex_key"), post.get_property("complex_key"));
        assert_eq!(found_post.get_property("array_key"), post.get_property("array_key"));
        
        Ok(())
    }
    
    #[sqlx::test]
    async fn test_timestamps_are_preserved(pool: PgPool) -> sqlx::Result<()> {
        let repo = PostgresUnifiedPostRepository::new(pool);
        let author_id = Uuid::new_v4();
        let post = create_test_post(AppSource::Yapper, author_id);
        let created_at = post.metadata.created_at;
        let updated_at = post.metadata.updated_at;
        
        // Save the post
        repo.save(&post).await.unwrap();
        
        // Retrieve the post
        let found = repo.find_by_id(post.id).await.unwrap();
        assert!(found.is_some());
        
        let found_post = found.unwrap();
        assert_eq!(found_post.metadata.created_at, created_at);
        assert_eq!(found_post.metadata.updated_at, updated_at);
        
        Ok(())
    }
}