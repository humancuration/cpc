//! Integration tests for the social service with consent service

use social_graph::{
    application::SocialService,
    domain::{
        model::{ContentType, FeedFilter, Visibility},
        service::consent_service::ConsentService,
    },
    infrastructure::{
        content_providers::{SocialPostProvider, VideoProvider},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
    },
    tests::test_utils::mocks::MockConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_social_service_with_mock_consent() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    
    // Create a mock consent service that allows all content
    let consent_service = Arc::new(MockConsentService::new(vec![]));
    
    let content_providers = vec![
        Arc::new(SocialPostProvider) as Arc<dyn ContentProvider>,
        Arc::new(VideoProvider) as Arc<dyn ContentProvider>,
    ];
    
    let social_service = SocialService::new(repository, consent_service, content_providers);
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // The feed should contain items from both providers
    assert!(!feed.is_empty());
}

#[tokio::test]
async fn test_social_service_with_real_consent() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    let content_providers = vec![
        Arc::new(SocialPostProvider) as Arc<dyn ContentProvider>,
        Arc::new(VideoProvider) as Arc<dyn ContentProvider>,
    ];
    
    let social_service = SocialService::new(repository, consent_service, content_providers);
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // The feed should contain items from both providers
    assert!(!feed.is_empty());
}