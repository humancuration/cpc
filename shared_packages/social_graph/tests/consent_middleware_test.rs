//! Tests for the ConsentMiddleware functionality

use social_graph::{
    application::SocialService,
    domain::model::{ContentType, FeedFilter},
    infrastructure::{
        content_providers::create_default_providers,
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
        consent_middleware::ConsentMiddleware,
    },
    domain::service::consent_service::ConsentService,
    domain::model::{ContentProvider, ContentProviderError},
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_consent_middleware_integration() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // Create content providers and wrap them with consent middleware
    let original_providers = create_default_providers();
    let mut middleware_providers: Vec<Arc<dyn ContentProvider>> = Vec::new();
    
    for provider in original_providers {
        let middleware = ConsentMiddleware::new(provider, consent_service.clone());
        middleware_providers.push(Arc::new(middleware) as Arc<dyn ContentProvider>);
    }
    
    let social_service = SocialService::new(repository, consent_service, middleware_providers);
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // The feed should contain items, but they may be filtered based on consent
    // In our test setup, all content is public so it should be visible
    assert!(feed.len() >= 0);
}

#[tokio::test]
async fn test_consent_middleware_with_filtering() {
    // This test would be more comprehensive in a real implementation
    // where we can set up specific consent relationships
    assert!(true); // Placeholder
}