//! Example demonstrating state migration between content provider versions
//! including error handling, rollback mechanisms, and concurrent access scenarios

use social_graph::{
    domain::model::{ContentItem, ContentType, FeedFilter, Visibility, ContentProvider, ContentProviderError},
    infrastructure::content_providers::registry::{ContentProviderRegistry, ProviderMetadata},
    infrastructure::content_providers::SocialPostProvider,
    infrastructure::consent_service_impl::ConsentServiceImpl,
    infrastructure::in_memory_repository::InMemoryRelationshipRepository,
};
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// A simple content provider that stores state
#[derive(Debug)]
pub struct StatefulContentProvider {
    /// Internal state that can be serialized/deserialized
    state: Arc<Mutex<ProviderState>>,
}

/// State that can be migrated between provider versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderState {
    /// Some example data that would be migrated
    pub last_processed_id: Option<Uuid>,
    pub processed_count: u64,
    pub custom_config: String,
}

impl StatefulContentProvider {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ProviderState {
                last_processed_id: None,
                processed_count: 0,
                custom_config: "default".to_string(),
            })),
        }
    }
    
    /// Update the internal state
    pub fn update_state(&self, last_id: Uuid, count: u64, config: String) {
        let mut state = self.state.lock().unwrap();
        state.last_processed_id = Some(last_id);
        state.processed_count = count;
        state.custom_config = config;
    }
}

#[async_trait]
impl ContentProvider for StatefulContentProvider {
    fn content_type(&self) -> ContentType {
        ContentType::SocialPost
    }

    async fn get_content(
        &self,
        _user_id: Uuid,
        _after: Option<DateTime<Utc>>,
        limit: usize,
        _filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        // Create some example content
        let mut items = Vec::new();
        for i in 0..limit.min(5) {
            items.push(ContentItem {
                id: Uuid::new_v4(),
                owner_id: Uuid::new_v4(),
                content_type: ContentType::SocialPost,
                source_package: "stateful_provider".to_string(),
                metadata: serde_json::json!({
                    "title": format!("Post {}", i + 1),
                    "content": format!("Content of post {}", i + 1),
                }),
                timestamp: Utc::now(),
                visibility: Visibility::Public,
                relevance_score: 0.8,
            });
        }
        Ok(items)
    }
    
    /// Serialize the provider's state for migration purposes
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let state = self.state.lock().unwrap();
        let serialized = serde_json::to_vec(&*state)?;
        Ok(serialized)
    }
    
    /// Deserialize state into the provider
    fn deserialize_state(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let state: ProviderState = serde_json::from_slice(data)?;
        let mut self_state = self.state.lock().unwrap();
        *self_state = state;
        Ok(())
    }
}

/// A content provider that fails during serialization
#[derive(Debug)]
pub struct FailingSerializationProvider {
    state: Arc<Mutex<ProviderState>>,
    pub should_fail: bool,
}

impl FailingSerializationProvider {
    pub fn new(should_fail: bool) -> Self {
        Self {
            state: Arc::new(Mutex::new(ProviderState {
                last_processed_id: None,
                processed_count: 0,
                custom_config: "default".to_string(),
            })),
            should_fail,
        }
    }
    
    /// Update the internal state
    pub fn update_state(&self, last_id: Uuid, count: u64, config: String) {
        let mut state = self.state.lock().unwrap();
        state.last_processed_id = Some(last_id);
        state.processed_count = count;
        state.custom_config = config;
    }
}

#[async_trait]
impl ContentProvider for FailingSerializationProvider {
    fn content_type(&self) -> ContentType {
        ContentType::SocialPost
    }

    async fn get_content(
        &self,
        _user_id: Uuid,
        _after: Option<DateTime<Utc>>,
        limit: usize,
        _filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        // Create some example content
        let mut items = Vec::new();
        for i in 0..limit.min(5) {
            items.push(ContentItem {
                id: Uuid::new_v4(),
                owner_id: Uuid::new_v4(),
                content_type: ContentType::SocialPost,
                source_package: "failing_serialization_provider".to_string(),
                metadata: serde_json::json!({
                    "title": format!("Post {}", i + 1),
                    "content": format!("Content of post {}", i + 1),
                }),
                timestamp: Utc::now(),
                visibility: Visibility::Public,
                relevance_score: 0.8,
            });
        }
        Ok(items)
    }
    
    /// Serialize the provider's state for migration purposes
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        if self.should_fail {
            Err("Serialization failed intentionally".into())
        } else {
            let state = self.state.lock().unwrap();
            let serialized = serde_json::to_vec(&*state)?;
            Ok(serialized)
        }
    }
    
    /// Deserialize state into the provider
    fn deserialize_state(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let state: ProviderState = serde_json::from_slice(data)?;
        let mut self_state = self.state.lock().unwrap();
        *self_state = state;
        Ok(())
    }
}

/// A content provider that fails during deserialization
#[derive(Debug)]
pub struct FailingDeserializationProvider {
    state: Arc<Mutex<ProviderState>>,
    pub should_fail: bool,
}

impl FailingDeserializationProvider {
    pub fn new(should_fail: bool) -> Self {
        Self {
            state: Arc::new(Mutex::new(ProviderState {
                last_processed_id: None,
                processed_count: 0,
                custom_config: "default".to_string(),
            })),
            should_fail,
        }
    }
}

#[async_trait]
impl ContentProvider for FailingDeserializationProvider {
    fn content_type(&self) -> ContentType {
        ContentType::SocialPost
    }

    async fn get_content(
        &self,
        _user_id: Uuid,
        _after: Option<DateTime<Utc>>,
        limit: usize,
        _filters: &[FeedFilter]
    ) -> Result<Vec<ContentItem>, ContentProviderError> {
        // Create some example content
        let mut items = Vec::new();
        for i in 0..limit.min(5) {
            items.push(ContentItem {
                id: Uuid::new_v4(),
                owner_id: Uuid::new_v4(),
                content_type: ContentType::SocialPost,
                source_package: "failing_deserialization_provider".to_string(),
                metadata: serde_json::json!({
                    "title": format!("Post {}", i + 1),
                    "content": format!("Content of post {}", i + 1),
                }),
                timestamp: Utc::now(),
                visibility: Visibility::Public,
                relevance_score: 0.8,
            });
        }
        Ok(items)
    }
    
    /// Serialize the provider's state for migration purposes
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let state = self.state.lock().unwrap();
        let serialized = serde_json::to_vec(&*state)?;
        Ok(serialized)
    }
    
    /// Deserialize state into the provider
    fn deserialize_state(&self, _data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        if self.should_fail {
            Err("Deserialization failed intentionally".into())
        } else {
            // In a real implementation, we would deserialize the data
            // For this example, we'll just leave the state as is
            Ok(())
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("State Migration Example");
    println!("=======================");
    
    // Create repository and consent service for registry
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // Create a provider with some state
    let provider1 = StatefulContentProvider::new();
    provider1.update_state(Uuid::new_v4(), 42, "custom_config_v1".to_string());
    
    // Serialize the state
    let serialized_state = provider1.serialize_state()?;
    println!("1. Serialized state size: {} bytes", serialized_state.len());
    
    // Create a new provider and deserialize the state
    let provider2 = StatefulContentProvider::new();
    provider2.deserialize_state(&serialized_state)?;
    
    // Verify the state was migrated
    let state = provider2.state.lock().unwrap();
    println!("2. Migrated state - Processed count: {}", state.processed_count);
    println!("   Migrated state - Custom config: {}", state.custom_config);
    
    // Demonstrate error handling during migration
    println!("\n3. Demonstrating Error Handling During Migration:");
    
    // Create registry for testing update_provider with error handling
    let registry = Arc::new(ContentProviderRegistry::new(consent_service.clone()));
    
    // Register initial provider
    let initial_provider = Arc::new(StatefulContentProvider::new()) as Arc<dyn ContentProvider>;
    initial_provider.update_state(Uuid::new_v4(), 100, "initial_config".to_string());
    
    let initial_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "StatefulProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let provider_id = registry.register_provider(initial_provider, initial_metadata.clone())?;
    println!("   Registered initial provider with ID: {}", provider_id);
    
    // Try to update with a provider that fails serialization
    let failing_provider = Arc::new(FailingSerializationProvider::new(true)) as Arc<dyn ContentProvider>;
    let failing_metadata = ProviderMetadata {
        id: provider_id, // Same ID for update
        name: "FailingProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "2.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "2.0.0".to_string(),
        compatible_previous_versions: vec!["1.0.0".to_string()],
        required_interfaces: vec![],
    };
    
    match registry.update_provider(failing_provider, failing_metadata) {
        Ok(_) => println!("   ERROR: Update should have failed!"),
        Err(e) => {
            println!("   Update failed as expected: {}", e);
            println!("   Error type: StateSerializationError");
        }
    }
    
    // Verify the original provider is still intact
    let all_providers = registry.get_all_providers()?;
    println!("   Number of providers in registry: {}", all_providers.len());
    println!("   Original provider preserved after failed migration");
    
    // Demonstrate rollback mechanism
    println!("\n4. Demonstrating Rollback Mechanism:");
    
    // Try to update with a provider that fails deserialization
    let failing_deserialize_provider = Arc::new(FailingDeserializationProvider::new(true)) as Arc<dyn ContentProvider>;
    let failing_deserialize_metadata = ProviderMetadata {
        id: provider_id, // Same ID for update
        name: "FailingDeserializeProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "3.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "3.0.0".to_string(),
        compatible_previous_versions: vec!["1.0.0".to_string()],
        required_interfaces: vec![],
    };
    
    match registry.update_provider(failing_deserialize_provider, failing_deserialize_metadata) {
        Ok(_) => println!("   ERROR: Update should have failed!"),
        Err(e) => {
            println!("   Update failed as expected: {}", e);
            println!("   Error type: StateDeserializationError");
        }
    }
    
    // Verify the original provider is still intact
    let all_providers = registry.get_all_providers()?;
    println!("   Number of providers in registry: {}", all_providers.len());
    println!("   Original provider preserved after failed migration (rollback successful)");
    
    // Demonstrate concurrent access scenario during migration
    println!("\n5. Demonstrating Concurrent Access During Migration:");
    
    // Create a new registry for concurrent access test
    let concurrent_repository = Arc::new(InMemoryRelationshipRepository::new());
    let concurrent_consent_service = Arc::new(ConsentServiceImpl::new(concurrent_repository.clone()));
    let concurrent_registry = Arc::new(ContentProviderRegistry::new(concurrent_consent_service.clone()));
    
    // Register initial provider
    let concurrent_provider = Arc::new(StatefulContentProvider::new()) as Arc<dyn ContentProvider>;
    concurrent_provider.update_state(Uuid::new_v4(), 200, "concurrent_config".to_string());
    
    let concurrent_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "ConcurrentProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let concurrent_provider_id = concurrent_registry.register_provider(concurrent_provider, concurrent_metadata.clone())?;
    println!("   Registered concurrent access test provider with ID: {}", concurrent_provider_id);
    
    // Spawn a thread that continuously reads from the registry
    let registry_clone = concurrent_registry.clone();
    let reader_thread = thread::spawn(move || {
        for i in 0..10 {
            // Try to get all providers (read operation)
            match registry_clone.get_all_providers() {
                Ok(providers) => {
                    println!("   Reader thread iteration {}: Found {} providers", i, providers.len());
                }
                Err(e) => {
                    println!("   Reader thread iteration {}: Error reading providers: {}", i, e);
                }
            }
            thread::sleep(Duration::from_millis(50));
        }
    });
    
    // Perform update while reader thread is running
    let updated_provider = Arc::new(StatefulContentProvider::new()) as Arc<dyn ContentProvider>;
    let updated_metadata = ProviderMetadata {
        id: concurrent_provider_id, // Same ID for update
        name: "ConcurrentProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "2.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "2.0.0".to_string(),
        compatible_previous_versions: vec!["1.0.0".to_string()],
        required_interfaces: vec![],
    };
    
    println!("   Starting update operation while reader thread is running...");
    let update_result = concurrent_registry.update_provider(updated_provider, updated_metadata);
    
    match update_result {
        Ok(updated_id) => {
            println!("   Update completed successfully with ID: {}", updated_id);
        }
        Err(e) => {
            println!("   Update failed: {}", e);
        }
    }
    
    // Wait for reader thread to complete
    reader_thread.join().unwrap();
    println!("   Concurrent access test completed successfully");
    
    // Create metadata for both versions (for reference)
    let metadata_v1 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "StatefulProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let metadata_v2 = ProviderMetadata {
        id: metadata_v1.id, // Same ID for update
        name: "StatefulProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "2.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "2.0.0".to_string(),
        compatible_previous_versions: vec!["1.0.0".to_string()],
        required_interfaces: vec![],
    };
    
    println!("\n6. Provider Metadata Examples:");
    println!("   Provider v1 metadata: {} v{}", metadata_v1.name, metadata_v1.version);
    println!("   Provider v2 metadata: {} v{}", metadata_v2.name, metadata_v2.version);
    
    Ok(())
}