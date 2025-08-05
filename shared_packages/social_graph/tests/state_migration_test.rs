//! Tests for state migration functionality

use social_graph::{
    domain::model::{ContentType, ContentProvider},
    infrastructure::{
        content_providers::{
            registry::{ContentProviderRegistry, ProviderMetadata},
        },
        consent_service_impl::ConsentServiceImpl,
        in_memory_repository::InMemoryRelationshipRepository,
    },
    domain::service::consent_service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

// Import the failing mock provider
mod test_utils;
use test_utils::mock_failing_provider::FailingMockProvider;

// Simple mock content provider for testing
#[derive(Debug)]
struct MockContentProvider;

#[async_trait::async_trait]
impl ContentProvider for MockContentProvider {
    fn content_type(&self) -> ContentType {
        ContentType::SocialPost
    }

    async fn get_content(
        &self,
        _user_id: uuid::Uuid,
        _after: Option<chrono::DateTime<chrono::Utc>>,
        _limit: usize,
        _filters: &[crate::domain::model::FeedFilter],
    ) -> Result<Vec<crate::domain::model::ContentItem>, crate::domain::model::ContentProviderError> {
        Ok(vec![])
    }
    
    // Use default implementations for serialize_state and deserialize_state
}

#[tokio::test]
async fn test_provider_registration_with_new_metadata() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository));
    let registry = ContentProviderRegistry::new(consent_service);
    
    let provider = Arc::new(MockContentProvider);
    let metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let result = registry.register_provider(provider, metadata);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_dependency_resolution() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository));
    let registry = ContentProviderRegistry::new(consent_service);
    
    // Register first provider
    let provider1 = Arc::new(MockContentProvider);
    let metadata1 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "Provider1".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    assert!(registry.register_provider(provider1, metadata1.clone()).is_ok());
    
    // Register second provider that depends on first
    let provider2 = Arc::new(MockContentProvider);
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
    
    let result = registry.register_provider(provider2, metadata2);
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_provider_update() {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository));
    let registry = ContentProviderRegistry::new(consent_service);
    
    // Register initial provider
    let provider_id = Uuid::new_v4();
    let provider1 = Arc::new(MockContentProvider);
    let metadata1 = ProviderMetadata {
        id: provider_id,
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    assert!(registry.register_provider(provider1, metadata1).is_ok());
    
    // Update provider
    let provider2 = Arc::new(MockContentProvider);
    let metadata2 = ProviderMetadata {
        id: provider_id, // Same ID for update
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "2.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "2.0.0".to_string(),
        compatible_previous_versions: vec!["1.0.0".to_string()],
        required_interfaces: vec![],
    };
    
    let result = registry.update_provider(provider2, metadata2);
    assert!(result.is_ok());
}

fn setup_registry() -> (ContentProviderRegistry, Arc<ConsentServiceImpl>) {
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository));
    let registry = ContentProviderRegistry::new(consent_service.clone());
    (registry, consent_service)
}

fn create_test_metadata() -> ProviderMetadata {
    ProviderMetadata {
        id: Uuid::new_v4(),
        name: "TestProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    }
}

#[tokio::test]
async fn test_serialization_failure() {
    let (mut registry, consent_service) = setup_registry();
    
    // Create provider that fails serialization
    let failing_provider = FailingMockProvider {
        fail_serialize: true,
        fail_deserialize: false,
        state: std::sync::Mutex::new("test_state".to_string()),
    };
    
    // Attempt update that should fail
    let result = registry.update_provider(
        Arc::new(failing_provider),
        create_test_metadata()
    );
    
    assert!(result.is_err());
    // Verify error is StateSerializationError
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Failed to serialize provider state"));
}

#[tokio::test]
async fn test_deserialization_failure() {
    let (mut registry, consent_service) = setup_registry();
    
    // First register a working provider to have some state
    let working_provider = FailingMockProvider {
        fail_serialize: false,
        fail_deserialize: false,
        state: std::sync::Mutex::new("initial_state".to_string()),
    };
    
    let metadata = create_test_metadata();
    let provider_id = metadata.id;
    
    // Register the working provider first
    assert!(registry.register_provider(Arc::new(working_provider), metadata.clone()).is_ok());
    
    // Create provider that fails deserialization
    let failing_provider = FailingMockProvider {
        fail_serialize: false,
        fail_deserialize: true,
        state: std::sync::Mutex::new("new_state".to_string()),
    };
    
    // Attempt update that should fail
    let result = registry.update_provider(
        Arc::new(failing_provider),
        metadata
    );
    
    assert!(result.is_err());
    // Verify error is StateDeserializationError
    let error = result.unwrap_err().to_string();
    assert!(error.contains("Failed to deserialize provider state"));
}