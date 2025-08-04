//! Integration tests for the ContentProvider system

use social_graph::{
    application::SocialService,
    domain::model::{ContentType, FeedFilter},
    infrastructure::{
        content_providers::{register_providers},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_adapter::ConsentAdapter,
    },
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_complete_content_provider_flow() {
    // Create all necessary components
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = consent_manager::ConsentService::new();
    let consent_adapter = Arc::new(ConsentAdapter::new(consent_service));
    
    // Create social service
    let mut social_service = SocialService::new(repository, consent_adapter);
    
    // Register all content providers
    register_providers(&mut social_service);
    
    // Verify providers were registered
    // Note: We can't directly access the content_providers field since it's private
    // but we can test the functionality
    
    // Test getting universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        20,
        None
    ).await.unwrap();
    
    // Should contain content from both providers
    assert!(!feed.is_empty());
    
    // Check that we have both social posts and videos
    let mut has_social_posts = false;
    let mut has_videos = false;
    
    for item in &feed {
        match item.content_type {
            ContentType::SocialPost => has_social_posts = true,
            ContentType::Video => has_videos = true,
            _ => {}
        }
    }
    
    assert!(has_social_posts, "Should have social posts in feed");
    assert!(has_videos, "Should have videos in feed");
    
    // Test filtering by content type
    let social_post_filter = vec![FeedFilter {
        content_type: Some(ContentType::SocialPost),
        package: None,
        visibility: None,
    }];
    
    let social_posts = social_service.get_universal_feed(
        user_id,
        None,
        10,
        Some(social_post_filter)
    ).await.unwrap();
    
    // Should contain only social posts
    assert!(!social_posts.is_empty());
    assert!(social_posts.iter().all(|item| item.content_type == ContentType::SocialPost));
    
    // Test filtering by video content type
    let video_filter = vec![FeedFilter {
        content_type: Some(ContentType::Video),
        package: None,
        visibility: None,
    }];
    
    let videos = social_service.get_universal_feed(
        user_id,
        None,
        10,
        Some(video_filter)
    ).await.unwrap();
    
    // Should contain only videos
    assert!(!videos.is_empty());
    assert!(videos.iter().all(|item| item.content_type == ContentType::Video));
    
    // Test sorting - items should be sorted by relevance score (descending)
    if feed.len() > 1 {
        for i in 1..feed.len() {
            assert!(feed[i-1].relevance_score >= feed[i].relevance_score, 
                    "Feed should be sorted by relevance score descending");
        }
    }
}

#[tokio::test]
async fn test_content_provider_limiting() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = consent_manager::ConsentService::new();
    let consent_adapter = Arc::new(ConsentAdapter::new(consent_service));
    
    let mut social_service = SocialService::new(repository, consent_adapter);
    register_providers(&mut social_service);
    
    let user_id = Uuid::new_v4();
    
    // Test with small limit
    let small_feed = social_service.get_universal_feed(
        user_id,
        None,
        2,
        None
    ).await.unwrap();
    
    assert!(small_feed.len() <= 2);
    
    // Test with larger limit
    let large_feed = social_service.get_universal_feed(
        user_id,
        None,
        50,
        None
    ).await.unwrap();
    
    // Should have more items but limited by provider implementations
    assert!(large_feed.len() > small_feed.len());
}