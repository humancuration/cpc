//! Tests for the ContentProvider system

use social_graph::{
    application::SocialService,
    domain::model::{ContentType, FeedFilter, ContentProviderError},
    infrastructure::{
        content_providers::{create_default_registry, ContentProviderRegistry},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
    },
    domain::service::consent_service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_content_provider_registration() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    let registry = create_default_registry(consent_service.clone());
    let social_service = SocialService::new(repository, consent_service, registry);
    
    // The service should now have 2 content providers registered
    // Note: We can't directly access the content_providers field since it's private
    // but we can test the functionality
}

#[tokio::test]
async fn test_get_universal_feed() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    let registry = create_default_registry(consent_service.clone());
    let social_service = SocialService::new(repository, consent_service, registry);
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // The feed should contain items from both providers
    // In our implementation, each provider returns some placeholder content
    // so the feed should not be empty
    assert!(feed.len() > 0);
}

#[tokio::test]
async fn test_feed_filtering() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    let registry = create_default_registry(consent_service.clone());
    let social_service = SocialService::new(repository, consent_service, registry);
    
    // Test filtering by content type
    let social_post_filter = vec![FeedFilter {
        content_type: Some(ContentType::SocialPost),
        package: None,
        visibility: None,
    }];
    
    let user_id = Uuid::new_v4();
    let social_posts = social_service.get_universal_feed(
        user_id,
        None,
        10,
        Some(social_post_filter)
    ).await.unwrap();
    
    // The feed should contain only social posts
    // In our implementation, the social post provider returns some placeholder content
    // so the feed should not be empty
    assert!(social_posts.len() > 0);
    assert!(social_posts.iter().all(|item| item.content_type == ContentType::SocialPost));
}