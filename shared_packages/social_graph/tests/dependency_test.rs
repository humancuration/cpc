//! Tests for dependency resolution and conflict handling

use social_graph::{
    domain::model::ContentType,
    infrastructure::content_providers::{
        registry::{ContentProviderRegistry, ProviderMetadata, DependencyError},
    },
};
use std::sync::Arc;
use uuid::Uuid;
use social_graph::infrastructure::content_providers::SocialPostProvider;
use social_graph::infrastructure::consent_service_impl::ConsentServiceImpl;
use social_graph::infrastructure::in_memory_repository::InMemoryRelationshipRepository;

fn setup_registry() -> ContentProviderRegistry {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository));
    ContentProviderRegistry::new(consent_service)
}

fn create_test_metadata(name: &str, dependencies: Vec<String>) -> ProviderMetadata {
    ProviderMetadata {
        id: Uuid::new_v4(),
        name: name.to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies,
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    }
}

#[tokio::test]
async fn test_missing_dependency() {
    let registry = setup_registry();
    
    // Create provider with missing dependency
    let provider = Arc::new(SocialPostProvider);
    let metadata = create_test_metadata("TestProvider", vec!["MissingDependency".to_string()]);
    
    // Registration should fail due to missing dependency
    let result = registry.register_provider(provider, metadata);
    assert!(result.is_err());
    
    // Check that the error is a dependency error
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Missing dependency"));
}

#[tokio::test]
async fn test_version_mismatch() {
    let registry = setup_registry();
    
    // Register first provider
    let provider1 = Arc::new(SocialPostProvider);
    let metadata1 = create_test_metadata("Provider1", vec![]);
    // Modify metadata1 to have version "2.0.0"
    let metadata1 = ProviderMetadata {
        version: "2.0.0".to_string(),
        ..metadata1
    };
    
    assert!(registry.register_provider(provider1, metadata1.clone()).is_ok());
    
    // Create provider that depends on Provider1 with version 1.0.0
    let provider2 = Arc::new(SocialPostProvider);
    let metadata2 = create_test_metadata("Provider2", vec!["Provider1@1.0.0".to_string()]);
    
    // Registration should fail due to version mismatch
    let result = registry.register_provider(provider2, metadata2);
    assert!(result.is_err());
    
    // Check that the error is a dependency error
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Version conflict"));
}

#[tokio::test]
async fn test_circular_dependency_detection() {
    let registry = setup_registry();
    
    // Register first provider that depends on second
    let provider1 = Arc::new(SocialPostProvider);
    let metadata1 = create_test_metadata("Provider1", vec!["Provider2".to_string()]);
    
    // Register second provider that depends on first (creating circular dependency)
    let provider2 = Arc::new(SocialPostProvider);
    let metadata2 = create_test_metadata("Provider2", vec!["Provider1".to_string()]);
    
    // Register first provider
    assert!(registry.register_provider(provider1, metadata1).is_ok());
    
    // Registration of second provider should fail due to circular dependency
    let result = registry.register_provider(provider2, metadata2);
    assert!(result.is_err());
    
    // Check that the error is a dependency error
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Circular dependency"));
}

#[tokio::test]
async fn test_dependency_resolution_with_multiple_levels() {
    let registry = setup_registry();
    
    // Register first provider (no dependencies)
    let provider1 = Arc::new(SocialPostProvider);
    let metadata1 = create_test_metadata("Provider1", vec![]);
    let id1 = registry.register_provider(provider1, metadata1).unwrap();
    
    // Register second provider (depends on first)
    let provider2 = Arc::new(SocialPostProvider);
    let metadata2 = create_test_metadata("Provider2", vec!["Provider1".to_string()]);
    let id2 = registry.register_provider(provider2, metadata2).unwrap();
    
    // Register third provider (depends on second, which depends on first)
    let provider3 = Arc::new(SocialPostProvider);
    let metadata3 = create_test_metadata("Provider3", vec!["Provider2".to_string()]);
    let id3 = registry.register_provider(provider3, metadata3).unwrap();
    
    // All providers should be registered successfully
    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_ne!(id1, id3);
    
    // Verify all providers are in the registry
    let all_providers = registry.get_all_providers().unwrap();
    assert_eq!(all_providers.len(), 3);
}

#[tokio::test]
async fn test_version_range_compatibility() {
    let registry = setup_registry();
    
    // Register first provider with version 1.2.3
    let provider1 = Arc::new(SocialPostProvider);
    let metadata1 = create_test_metadata("Provider1", vec![]);
    let metadata1 = ProviderMetadata {
        version: "1.2.3".to_string(),
        ..metadata1
    };
    assert!(registry.register_provider(provider1, metadata1).is_ok());
    
    // Create provider that depends on Provider1 with compatible version
    let provider2 = Arc::new(SocialPostProvider);
    let metadata2 = create_test_metadata("Provider2", vec!["Provider1@1.2.3".to_string()]);
    
    // Registration should succeed with exact version match
    let result = registry.register_provider(provider2, metadata2);
    assert!(result.is_ok());
}