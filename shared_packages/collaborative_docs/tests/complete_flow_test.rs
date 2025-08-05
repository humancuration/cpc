//! Complete flow test showing integration between collaborative docs and social graph

use cpc_collaborative_docs::{
    CollaborativeDocService, CollaborativeDocProvider, CollaborativeDocProviderMetadata,
    sled_store::SledDocStore,
    access::DocumentAccessChecker,
    core::DocumentContent
};
use social_graph::{
    infrastructure::content_providers::ContentProviderRegistry,
    infrastructure::consent_service_impl::ConsentServiceImpl,
    infrastructure::in_memory_repository::InMemoryRelationshipRepository,
    domain::model::ContentType
};
use std::sync::Arc;
use uuid::Uuid;
use serde_json::json;

#[tokio::test]
async fn test_complete_flow() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Initialize document storage
    let doc_store = Arc::new(SledDocStore::new_in_memory()?);
    
    // Step 2: Initialize access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Step 3: Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        doc_store,
        access_checker,
    ));
    
    // Step 4: Create the content provider
    let doc_provider = Arc::new(CollaborativeDocProvider::new(doc_service.clone()));
    
    // Step 5: Initialize social graph components
    let relationship_repo = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(relationship_repo));
    let registry = ContentProviderRegistry::new(consent_service);
    
    // Step 6: Prepare provider metadata
    let metadata = CollaborativeDocProviderMetadata {
        provider_id: Uuid::new_v4(),
        name: "Collaborative Document Provider".to_string(),
        version: "1.0.0".to_string(),
        description: "Provider for collaborative documents with CRDT support".to_string(),
    };
    
    // Step 7: Register the provider using the convenience method
    let provider_id = doc_provider.clone().register_provider(&registry, metadata)?;
    
    // Step 8: Verify registration
    let all_metadata = registry.get_all_metadata()?;
    assert_eq!(all_metadata.len(), 1);
    assert_eq!(all_metadata[0].id, provider_id);
    assert_eq!(all_metadata[0].content_type, ContentType::Custom("document".to_string()));
    
    // Step 9: Create a test document
    let owner_id = Uuid::new_v4();
    let content = DocumentContent {
        data: json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;
    
    // Step 10: Verify the document appears in the content provider
    let content_items = doc_provider.get_content(
        owner_id,
        None,
        10,
        &[]
    ).await?;
    
    assert_eq!(content_items.len(), 1);
    assert_eq!(content_items[0].id, metadata.id);
    assert_eq!(content_items[0].owner_id, owner_id);
    assert_eq!(content_items[0].source_package, "collaborative_docs");
    
    // Step 11: Verify document metadata in content item
    let title = content_items[0].metadata.get("title").and_then(|t| t.as_str());
    assert_eq!(title, Some("Test Document"));
    
    println!("Complete flow test passed!");
    println!("  - Provider registered with ID: {}", provider_id);
    println!("  - Document created with ID: {}", metadata.id);
    println!("  - Document appears in content feed");
    
    Ok(())
}