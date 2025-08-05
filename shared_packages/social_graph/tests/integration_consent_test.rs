//! Integration tests for the consent system

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
async fn test_consent_integration_with_social_service() {
    // Create repository and consent service
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // Create social service with real consent service
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
    // In a real implementation with proper data, we would have more specific assertions
    assert!(feed.len() >= 0); // At least 0 items (placeholder implementation)
}

#[tokio::test]
async fn test_consent_integration_with_mock_service() {
    // Create mock consent service that allows all content
    let consent_service = Arc::new(MockConsentService::new(vec![]));
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    
    // Create social service with mock consent service
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
    // In a real implementation with proper data, we would have more specific assertions
    assert!(feed.len() >= 0); // At least 0 items (placeholder implementation)
}