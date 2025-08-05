//! Tests for the dynamic content provider system

use social_graph::{
    application::SocialService,
    domain::model::{ContentType, FeedFilter},
    infrastructure::{
        content_providers::{ContentProviderRegistry, ProviderMetadata, SocialPostProvider, VideoProvider},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
    },
    domain::service::consent_service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::test]
async fn test_dynamic_provider_registration() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    let registry = Arc::new(ContentProviderRegistry::new(consent_service.clone()));
    
    // Register providers dynamically
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
    registry.register_provider(social_post_provider.clone(), social_post_metadata).unwrap();
    registry.register_provider(video_provider.clone(), video_metadata).unwrap();
    
    // Verify providers were registered
    let all_providers = registry.get_all_providers().unwrap();
    assert_eq!(all_providers.len(), 2);
    
    // Create social service with registry
    let social_service = SocialService::new(
        repository,
        consent_service,
        registry,
    );
    
    // Test that the service works with the registered providers
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
async fn test_provider_metadata() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    let registry = Arc::new(ContentProviderRegistry::new(consent_service.clone()));
    
    let provider = Arc::new(SocialPostProvider);
    let metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    let provider_id = registry.register_provider(provider, metadata.clone()).unwrap();
    
    // Retrieve metadata
    let retrieved_metadata = registry.get_metadata(&provider_id).unwrap().unwrap();
    assert_eq!(retrieved_metadata.name, "TestProvider");
    assert_eq!(retrieved_metadata.content_type, ContentType::SocialPost);
    assert_eq!(retrieved_metadata.version, "1.0.0");
}

#[tokio::test]
fn test_provider_unregistration() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    let registry = Arc::new(ContentProviderRegistry::new(consent_service.clone()));
    
    let provider = Arc::new(SocialPostProvider);
    let metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
    };
    
    let provider_id = registry.register_provider(provider, metadata).unwrap();
    
    // Verify provider was registered
    let all_providers = registry.get_all_providers().unwrap();
    assert_eq!(all_providers.len(), 1);
    
    // Unregister provider
    let result = registry.unregister_provider(&provider_id).unwrap();
    assert!(result);
    
    // Verify provider was unregistered
    let all_providers = registry.get_all_providers().unwrap();
    assert_eq!(all_providers.len(), 0);
}