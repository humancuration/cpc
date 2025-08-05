//! Example demonstrating the dynamic content provider system
//! including dependency conflict scenarios, circular dependency detection, and conflict resolution workflow

use social_graph::{
    application::SocialService,
    domain::model::{ContentType},
    infrastructure::{
        content_providers::{ContentProviderRegistry, ProviderMetadata, SocialPostProvider, VideoProvider},
        in_memory_repository::InMemoryRelationshipRepository,
        consent_service_impl::ConsentServiceImpl,
    },
    domain::service::consent_service::ConsentService,
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Dynamic Content Provider System Example");
    println!("======================================");
    
    // Create registry
    let repository = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(repository.clone()));
    let registry = Arc::new(ContentProviderRegistry::new(consent_service.clone()));
    
    // Register providers dynamically
    println!("1. Registering content providers...");
    
    let social_post_provider = Arc::new(SocialPostProvider);
    let social_post_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "SocialPostProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let video_provider = Arc::new(VideoProvider);
    let video_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "VideoProvider".to_string(),
        content_type: ContentType::Video,
        version: "1.0.0".to_string(),
        dependencies: vec![],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    // Register the providers
    let social_post_id = registry.register_provider(social_post_provider, social_post_metadata)?;
    let video_id = registry.register_provider(video_provider, video_metadata)?;
    
    println!("   Registered SocialPostProvider with ID: {}", social_post_id);
    println!("   Registered VideoProvider with ID: {}", video_id);
    
    // Retrieve metadata
    let social_post_meta = registry.get_all_metadata()?.into_iter().find(|m| m.id == social_post_id).unwrap();
    let video_meta = registry.get_all_metadata()?.into_iter().find(|m| m.id == video_id).unwrap();
    
    println!("   SocialPostProvider metadata: {} v{}", social_post_meta.name, social_post_meta.version);
    println!("   VideoProvider metadata: {} v{}", video_meta.name, video_meta.version);
    
    // Create social service with registry
    let social_service = SocialService::new(
        repository.clone(),
        consent_service.clone(),
        registry.clone(),
    );
    
    // Get universal feed
    let user_id = Uuid::new_v4();
    let feed = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await?;
    
    println!("\n2. Universal feed contains {} items", feed.len());
    
    for (i, item) in feed.iter().enumerate() {
        println!("   {}. {:?} from {} (score: {})",
                 i + 1,
                 item.content_type,
                 item.source_package,
                 item.relevance_score);
    }
    
    // Demonstrate dependency conflict scenarios
    println!("\n3. Demonstrating Dependency Conflict Scenarios:");
    
    // Try to register a provider with a missing dependency
    let provider_with_missing_dep = Arc::new(SocialPostProvider);
    let metadata_with_missing_dep = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "ProviderWithMissingDep".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["NonExistentProvider".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    match registry.register_provider(provider_with_missing_dep, metadata_with_missing_dep) {
        Ok(_) => println!("   ERROR: Registration should have failed!"),
        Err(e) => {
            println!("   Registration failed as expected: {}", e);
            println!("   Error type: MissingDependency");
        }
    }
    
    // Try to register a provider with a version conflict
    let provider_with_version_conflict = Arc::new(VideoProvider);
    let metadata_with_version_conflict = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "ProviderWithVersionConflict".to_string(),
        content_type: ContentType::Video,
        version: "1.0.0".to_string(),
        dependencies: vec!["SocialPostProvider@2.0.0".to_string()], // Requesting version 2.0.0 but only 1.0.0 exists
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    match registry.register_provider(provider_with_version_conflict, metadata_with_version_conflict) {
        Ok(_) => println!("   ERROR: Registration should have failed!"),
        Err(e) => {
            println!("   Registration failed as expected: {}", e);
            println!("   Error type: VersionConflict");
        }
    }
    
    // Demonstrate circular dependency detection
    println!("\n4. Demonstrating Circular Dependency Detection:");
    
    // Register first provider that depends on second
    let provider_a = Arc::new(SocialPostProvider);
    let metadata_a = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "ProviderA".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["ProviderB".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    let provider_a_id = registry.register_provider(provider_a, metadata_a)?;
    println!("   Registered ProviderA with ID: {}", provider_a_id);
    
    // Try to register second provider that depends on first (creating circular dependency)
    let provider_b = Arc::new(VideoProvider);
    let metadata_b = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "ProviderB".to_string(),
        content_type: ContentType::Video,
        version: "1.0.0".to_string(),
        dependencies: vec!["ProviderA".to_string()],
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    match registry.register_provider(provider_b, metadata_b) {
        Ok(_) => println!("   ERROR: Registration should have failed!"),
        Err(e) => {
            println!("   Registration failed as expected: {}", e);
            println!("   Error type: CircularDependency");
        }
    }
    
    // Demonstrate conflict resolution workflow
    println!("\n5. Demonstrating Conflict Resolution Workflow:");
    
    // Register a provider that depends on an existing provider with correct version
    let dependent_provider = Arc::new(SocialPostProvider);
    let dependent_metadata = ProviderMetadata {
        id: Uuid::new_v4(),
        name: "DependentProvider".to_string(),
        content_type: ContentType::SocialPost,
        version: "1.0.0".to_string(),
        dependencies: vec!["SocialPostProvider@1.0.0".to_string()], // Correct version
        state_schema_version: "1.0.0".to_string(),
        compatible_previous_versions: vec![],
        required_interfaces: vec![],
    };
    
    match registry.register_provider(dependent_provider, dependent_metadata) {
        Ok(provider_id) => {
            println!("   DependentProvider registered successfully with ID: {}", provider_id);
            println!("   Conflict resolution successful - dependencies validated");
        }
        Err(e) => {
            println!("   ERROR: Registration should have succeeded: {}", e);
        }
    }
    
    // Show all registered providers
    let all_providers = registry.get_all_providers()?;
    let all_metadata = registry.get_all_metadata()?;
    println!("\n6. Current Registry State:");
    println!("   Total providers registered: {}", all_providers.len());
    
    for metadata in all_metadata {
        println!("   - {} v{} (ID: {})", metadata.name, metadata.version, metadata.id);
        if !metadata.dependencies.is_empty() {
            println!("     Dependencies: {:?}", metadata.dependencies);
        }
    }
    
    // Unregister a provider
    println!("\n7. Unregistering SocialPostProvider...");
    let result = registry.unregister_provider(&social_post_id)?;
    println!("   Unregistration result: {}", result);
    
    // Get feed again
    let feed_after = social_service.get_universal_feed(
        user_id,
        None,
        10,
        None
    ).await?;
    
    println!("\n8. Feed after unregistration contains {} items", feed_after.len());
    
    Ok(())
}