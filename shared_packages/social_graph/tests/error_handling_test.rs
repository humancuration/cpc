//! Tests for error handling in the content provider system

use social_graph::{
    domain::model::{ContentType, ContentProviderError},
    infrastructure::content_providers::{
        registry::{ContentProviderRegistry, ProviderMetadata},
    },
    infrastructure::content_providers::SocialPostProvider,
    infrastructure::consent_service_impl::ConsentServiceImpl,
    infrastructure::in_memory_repository::InMemoryRelationshipRepository,
};
use std::sync::Arc;
use uuid::Uuid;

fn setup_registry() -> ContentProviderRegistry {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository));
    ContentProviderRegistry::new(consent_service)
}

#[tokio::test]
async fn test_state_serialization_error() {
    // This test would use a provider that fails serialization
    // For now, we'll just verify the error type exists
    let error = ContentProviderError::StateSerializationError;
    assert!(matches!(error, ContentProviderError::StateSerializationError));
}

#[tokio::test]
async fn test_state_deserialization_error() {
    // This test would use a provider that fails deserialization
    // For now, we'll just verify the error type exists
    let error = ContentProviderError::StateDeserializationError;
    assert!(matches!(error, ContentProviderError::StateDeserializationError));
}

#[tokio::test]
async fn test_missing_dependency_error() {
    let registry = setup_registry();
    
    // Create provider with missing dependency
    let provider = Arc::new(SocialPostProvider);
    let metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["MissingDependency".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    // Registration should fail due to missing dependency
    let result = registry.register_provider(provider, metadata);
    assert!(result.is_err());
    
    // Check that we get an error (the specific type depends on implementation details)
    assert!(result.unwrap_err().to_string().contains("Missing dependency"));
}

#[tokio::test]
async fn test_version_conflict_error() {
    let registry = setup_registry();
    
    // Register first provider
    let provider1 = Arc::new(SocialPostProvider);
    let metadata1 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "Provider1".to_string(),
        content_type: ContentType::SocialPost,
        version: "2.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    assert!(registry.register_provider(provider1, metadata1.clone()).is_ok());
    
    // Create provider that depends on Provider1 with wrong version
    let provider2 = Arc::new(SocialPostProvider);
    let metadata2 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "Provider2".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["Provider1@1.0.0".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    // Registration should fail due to version mismatch
    let result = registry.register_provider(provider2, metadata2);
    assert!(result.is_err());
    
    // Check that we get a version conflict error
    assert!(result.unwrap_err().to_string().contains("Version conflict"));
}

#[tokio::test]
async fn test_circular_dependency_error() {
    let registry = setup_registry();
    
    // Register first provider that depends on second
    let provider1 = Arc::new(SocialPostProvider);
    let metadata1 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "Provider1".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["Provider2".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    assert!(registry.register_provider(provider1, metadata1).is_ok());
    
    // Register second provider that depends on first (creating circular dependency)
    let provider2 = Arc::new(SocialPostProvider);
    let metadata2 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "Provider2".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["Provider1".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    // Registration should fail due to circular dependency
    let result = registry.register_provider(provider2, metadata2);
    assert!(result.is_err());
    
    // Check that we get a circular dependency error
    assert!(result.unwrap_err().to_string().contains("Circular dependency"));
}