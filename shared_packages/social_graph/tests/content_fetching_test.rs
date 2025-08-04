//! Tests for the actual content fetching functionality

use social_graph::{
    application::SocialService,
    domain::model::{ContentType, FeedFilter},
    infrastructure::{
        content_providers::{SocialPostProvider, VideoProvider},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_adapter::ConsentAdapter,
    },
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_social_post_provider_content() {
    let provider = SocialPostProvider;
    
    let user_id = Uuid::new_v4();
    let content = provider.get_content(
        user_id,
        None,
        5,
        &[]
    ).await.unwrap();
    
    // Should return some placeholder content
    assert!(!content.is_empty());
    assert!(content.len() <= 3); // Limited to 3 in implementation
    
    // All items should be social posts
    for item in &content {
        assert_eq!(item.content_type, ContentType::SocialPost);
        assert_eq!(item.source_package, "social_graph");
    }
}

#[tokio::test]
async fn test_video_provider_content() {
    let provider = VideoProvider;
    
    let user_id = Uuid::new_v4();
    let content = provider.get_content(
        user_id,
        None,
        5,
        &[]
    ).await.unwrap();
    
    // Should return some placeholder content
    assert!(!content.is_empty());
    assert!(content.len() <= 2); // Limited to 2 in implementation
    
    // All items should be videos
    for item in &content {
        assert_eq!(item.content_type, ContentType::Video);
        assert_eq!(item.source_package, "video_package");
    }
}

#[tokio::test]
async fn test_provider_filtering() {
    let provider = SocialPostProvider;
    
    let user_id = Uuid::new_v4();
    
    // Test with matching filter
    let matching_filter = vec![FeedFilter {
        content_type: Some(ContentType::SocialPost),
        package: None,
        visibility: None,
    }];
    
    let content = provider.get_content(
        user_id,
        None,
        5,
        &matching_filter
    ).await.unwrap();
    
    // Should return content
    assert!(!content.is_empty());
    
    // Test with non-matching filter
    let non_matching_filter = vec![FeedFilter {
        content_type: Some(ContentType::Video),
        package: None,
        visibility: None,
    }];
    
    let content = provider.get_content(
        user_id,
        None,
        5,
        &non_matching_filter
    ).await.unwrap();
    
    // Should return no content
    assert!(content.is_empty());
}

#[tokio::test]
async fn test_universal_feed_with_content() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = consent_manager::ConsentService::new();
    let consent_adapter = Arc::new(ConsentAdapter::new(consent_service));
    
    let mut social_service = SocialService::new(repository, consent_adapter);
    register_providers(&mut social_service);
    
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // Should contain content from both providers
    assert!(!feed.is_empty());
    
    // Check that we have both social posts and videos
    let mut has_social_posts = false;
    let mut has_videos = false;
    
    for item in &feed {
        if item.content_type == ContentType::SocialPost {
            has_social_posts = true;
        } else if item.content_type == ContentType::Video {
            has_videos = true;
        }
    }
    
    assert!(has_social_posts);
    assert!(has_videos);
    
    // Check that items are sorted by relevance score (descending)
    for i in 1..feed.len() {
        assert!(feed[i-1].relevance_score >= feed[i].relevance_score);
    }
}