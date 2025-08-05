//! Example demonstrating hot-swapping of content providers with state migration

use social_graph::{
    application::SocialService,
    domain::model::{ContentType, FeedFilter, ContentProviderError, ContentProvider, ContentItem, Visibility},
    infrastructure::{
        content_providers::{ContentProviderRegistry, ProviderMetadata},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
    },
};
use std::sync::{Arc, Mutex};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Version 1 of a content provider
#[derive(Debug)]
pub struct ContentProviderV1 {
    state: Arc<Mutex<ProviderV1State>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderV1State {
    pub processed_count: u64,
    pub last_id: Option<Uuid>,
}

impl ContentProviderV1 {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ProviderV1State {
                processed_count: 0,
                last_id: None,
            })),
        }
    }
    
    pub fn update_state(&self, count: u64, last_id: Option<Uuid>) {
        let mut state = self.state.lock().unwrap();
        state.processed_count = count;
        state.last_id = last_id;
    }
}

#[async_trait::async_trait]
impl ContentProvider for ContentProviderV1 {
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
        let mut items = Vec::new();
        for i in 0..limit.min(3) {
            items.push(ContentItem {
                id: Uuid::new_v4(),
                owner_id: Uuid::new_v4(),
                content_type: ContentType::SocialPost,
                source_package: "provider_v1".to_string(),
                metadata: serde_json::json!({
                    "title": format!("V1 Post {}", i + 1),
                    "content": format!("Content from V1 provider {}", i + 1),
                }),
                timestamp: Utc::now(),
                visibility: Visibility::Public,
                relevance_score: 0.7,
            });
        }
        Ok(items)
    }
    
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let state = self.state.lock().unwrap();
        let serialized = serde_json::to_vec(&*state)?;
        Ok(serialized)
    }
    
    fn deserialize_state(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let state: ProviderV1State = serde_json::from_slice(data)?;
        let mut self_state = self.state.lock().unwrap();
        *self_state = state;
        Ok(())
    }
}

/// Version 2 of a content provider with enhanced state
#[derive(Debug)]
pub struct ContentProviderV2 {
    state: Arc<Mutex<ProviderV2State>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderV2State {
    pub processed_count: u64,
    pub last_id: Option<Uuid>,
    pub quality_score: f32, // New field in V2
}

impl ContentProviderV2 {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ProviderV2State {
                processed_count: 0,
                last_id: None,
                quality_score: 0.5, // Default quality score
            })),
        }
    }
    
    pub fn update_state(&self, count: u64, last_id: Option<Uuid>, quality: f32) {
        let mut state = self.state.lock().unwrap();
        state.processed_count = count;
        state.last_id = last_id;
        state.quality_score = quality;
    }
}

#[async_trait::async_trait]
impl ContentProvider for ContentProviderV2 {
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
        let mut items = Vec::new();
        for i in 0..limit.min(5) {
            items.push(ContentItem {
                id: Uuid::new_v4(),
                owner_id: Uuid::new_v4(),
                content_type: ContentType::SocialPost,
                source_package: "provider_v2".to_string(),
                metadata: serde_json::json!({
                    "title": format!("V2 Post {}", i + 1),
                    "content": format!("Enhanced content from V2 provider {}", i + 1),
                    "quality": "high",
                }),
                timestamp: Utc::now(),
                visibility: Visibility::Public,
                relevance_score: 0.9,
            });
        }
        Ok(items)
    }
    
    fn serialize_state(&self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let state = self.state.lock().unwrap();
        let serialized = serde_json::to_vec(&*state)?;
        Ok(serialized)
    }
    
    fn deserialize_state(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let state: ProviderV2State = serde_json::from_slice(data)?;
        let mut self_state = self.state.lock().unwrap();
        *self_state = state;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Hot-Swapping Content Providers with State Migration");
    println!("==================================================");
    
    // Create repository and consent service
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    
    // Create registry
    let registry = Arc::new(ContentProviderRegistry::new(consent_service.clone()));
    
    // Create social service
    let social_service = SocialService::new(repository.clone(), consent_service.clone(), registry.clone());
    
    println!("\n1. Registering initial provider (V1):");
    
    // Create and register V1 provider
    let provider_v1 = Arc::new(ContentProviderV1::new()) as Arc<dyn ContentProvider>;
    provider_v1.update_state(50, Some(Uuid::new_v4()));
    
    let metadata_v1 = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "HotSwapProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let provider_id = registry.register_provider(provider_v1, metadata_v1.clone())?;
    println!("   Registered V1 provider with ID: {}", provider_id);
    
    // Verify V1 provider is working
    let user_id = Uuid::new_v4();
    let feed_v1 = social_service.get_universal_feed(user_id, None, 5, None).await?;
    println!("   V1 provider feed contains {} items", feed_v1.len());
    for (i, item) in feed_v1.iter().enumerate() {
        println!("     {}. {} from {}", i + 1, item.source_package, item.relevance_score);
    }
    
    println!("\n2. Hot-swapping to V2 provider with state migration:");
    
    // Create V2 provider for update
    let provider_v2 = Arc::new(ContentProviderV2::new()) as Arc<dyn ContentProvider>;
    
    let metadata_v2 = ProviderMetadata {
        id: provider_id, // Same ID for update
        name: "HotSwapProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "2.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "2.0.0".to_string(),
        compatible_previous_versions: vec!["1.0.0".to_string()], // Compatible with V1
        required_interfaces: vec![],
    };
    
    // Perform the hot-swap
    let updated_id = registry.update_provider(provider_v2, metadata_v2)?;
    println!("   Hot-swapped to V2 provider with ID: {}", updated_id);
    
    // Verify V2 provider is working
    let feed_v2 = social_service.get_universal_feed(user_id, None, 5, None).await?;
    println!("   V2 provider feed contains {} items", feed_v2.len());
    for (i, item) in feed_v2.iter().enumerate() {
        println!("     {}. {} from {}", i + 1, item.source_package, item.relevance_score);
    }
    
    println!("\n3. Verifying state migration:");
    
    // In a real implementation, we would have a way to verify the state was migrated
    // For this example, we'll just show that the provider was updated
    let all_metadata = registry.get_all_metadata()?;
    for meta in all_metadata {
        if meta.id == provider_id {
            println!("   Provider is now {} v{}", meta.name, meta.version);
        }
    }
    
    println!("\n4. Testing backward compatibility:");
    
    // Register another V1 provider to show multiple providers can coexist
    let another_v1 = Arc::new(ContentProviderV1::new()) as Arc<dyn ContentProvider>;
    let another_v1_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "AnotherV1Provider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let another_provider_id = registry.register_provider(another_v1, another_v1_metadata)?;
    println!("   Registered another V1 provider with ID: {}", another_provider_id);
    
    // Get combined feed
    let combined_feed = social_service.get_universal_feed(user_id, None, 10, None).await?;
    println!("   Combined feed contains {} items from both providers", combined_feed.len());
    
    let mut v1_count = 0;
    let mut v2_count = 0;
    
    for item in &combined_feed {
        if item.source_package == "provider_v1" {
            v1_count += 1;
        } else if item.source_package == "provider_v2" {
            v2_count += 1;
        }
    }
    
    println!("   Feed breakdown: {} items from V1, {} items from V2", v1_count, v2_count);
    
    println!("\nHot-swapping example completed successfully!");
    
    Ok(())
}