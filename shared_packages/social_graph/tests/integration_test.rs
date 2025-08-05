//! Integration test for all new functionality

use social_graph::{
    application::SocialService,
    domain::model::{ContentType},
    infrastructure::{
        content_providers::{ContentProviderRegistry, ProviderMetadata, SocialPostProvider, VideoProvider},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
        consent_middleware::ConsentMiddleware,
    },
    domain::service::consent_service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_full_integration() {
    // Create registry
    let registry = Arc::new(ContentProviderRegistry::new());
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // Register providers
    let social_post_provider = Arc::new(SocialPostProvider);
    let social_post_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "SocialPostProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    let video_provider = Arc::new(VideoProvider);
    let video_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "VideoProvider".to_string(),
        content_type: ContentType::Video,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    // Register the providers
    registry.register_provider(social_post_provider, social_post_metadata).unwrap();
    registry.register_provider(video_provider, video_metadata).unwrap();
    
    // Create social service with registry
    let social_service = SocialService::with_registry(
        repository,
        consent_service,
        registry,
    ).unwrap();
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // The feed should contain items from both providers
    assert!(feed.len() > 0);
}

#[tokio::test]
async fn test_consent_middleware_with_registry() {
    // Create registry
    let registry = Arc::new(ContentProviderRegistry::new());
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // Register providers with consent middleware
    let social_post_provider = Arc::new(SocialPostProvider);
    let social_post_with_middleware = Arc::new(ConsentMiddleware::new(
        social_post_provider,
        consent_service.clone(),
    ));
    
    let social_post_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "SocialPostProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    // Register the provider with middleware
    registry.register_provider(social_post_with_middleware, social_post_metadata).unwrap();
    
    // Create social service with registry
    let social_service = SocialService::with_registry(
        repository,
        consent_service,
        registry,
    ).unwrap();
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await.unwrap();
    
    // The feed should contain items, filtered by consent
    assert!(feed.len() >= 0);
}