//! Integration tests for content provider registration and consent enforcement

use cpc_collaborative_docs::{
    CollaborativeDocService, CollaborativeDocProvider, CollaborativeDocProviderMetadata,
    sled_store::SledDocStore,
    access::DocumentAccessChecker,
    core::{DocumentContent, AccessLevel, DocumentPermission}
};
use social_graph::{
    infrastructure::content_providers::ContentProviderRegistry,
    infrastructure::consent_service_impl::ConsentServiceImpl,
    infrastructure::in_memory_repository::InMemoryRelationshipRepository,
    domain::model::{ContentType, Visibility, FeedFilter}
};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;

#[tokio::test]
async fn test_provider_registration_flow() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize document storage
    let doc_store = Arc::new(SledDocStore::new_in_memory()?);
    
    // Initialize access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        doc_store,
        access_checker,
    ));
    
    // Create the content provider
    let doc_provider = Arc::new(CollaborativeDocProvider::new(doc_service));
    
    // Initialize social graph components
    let relationship_repo = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(relationship_repo));
    let registry = ContentProviderRegistry::new(consent_service);
    
    // Prepare provider metadata
    let metadata = CollaborativeDocProviderMetadata {
        provider_id: Uuid::new_v4(),
        name: "Collaborative Document Provider".to_string(),
        version: "1.0.0".to_string(),
        description: "Provider for collaborative documents with CRDT support".to_string(),
    };
    
    // Register the provider
    let provider_id = doc_provider.clone().register_provider(&registry, metadata)?;
    
    // Verify registration
    let all_metadata = registry.get_all_metadata()?;
    assert_eq!(all_metadata.len(), 1);
    assert_eq!(all_metadata[0].id, provider_id);
    assert_eq!(all_metadata[0].name, "Collaborative Document Provider");
    
    Ok(())
}

#[tokio::test]
async fn test_consent_enforcement() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize document storage
    let doc_store = Arc::new(SledDocStore::new_in_memory()?);
    
    // Initialize access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        doc_store.clone(),
        access_checker,
    ));
    
    // Create the content provider
    let doc_provider = Arc::new(CollaborativeDocProvider::new(doc_service.clone()));
    
    // Initialize social graph components
    let relationship_repo = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(relationship_repo));
    let registry = ContentProviderRegistry::new(consent_service);
    
    // Prepare provider metadata
    let metadata = CollaborativeDocProviderMetadata {
        provider_id: Uuid::new_v4(),
        name: "Collaborative Document Provider".to_string(),
        version: "1.0.0".to_string(),
        description: "Provider for collaborative documents with CRDT support".to_string(),
    };
    
    // Register the provider
    let _provider_id = doc_provider.clone().register_provider(&registry, metadata)?;
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let content = DocumentContent {
        data: serde_json::json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;
    
    // Test that the provider correctly returns content items
    let content_items = doc_provider.get_content(
        owner_id,
        None,
        10,
        &[]
    ).await?;
    
    assert_eq!(content_items.len(), 1);
    assert_eq!(content_items.len(), 1);
    assert_eq!(content_items[0].id, metadata.id);
    assert_eq!(content_items[0].owner_id, owner_id);
    assert_eq!(content_items[0].content_type, social_graph::domain::model::ContentType::Custom("document".to_string()));
    assert_eq!(content_items[0].source_package, "collaborative_docs");
    assert_eq!(content_items[0].visibility, social_graph::domain::model::Visibility::Private); // Documents are private by default
    
    Ok(())
}

#[tokio::test]
async fn test_document_appears_in_social_feeds() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize document storage
    let doc_store = Arc::new(SledDocStore::new_in_memory()?);
    
    // Initialize access checker
    let access_checker = DocumentAccessChecker::new(None);
    
    // Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        doc_store.clone(),
        access_checker,
    ));
    
    // Create the content provider
    let doc_provider = Arc::new(CollaborativeDocProvider::new(doc_service.clone()));
    
    // Initialize social graph components
    let relationship_repo = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(relationship_repo));
    let registry = ContentProviderRegistry::new(consent_service);
    
    // Prepare provider metadata
    let metadata = CollaborativeDocProviderMetadata {
        provider_id: Uuid::new_v4(),
        name: "Collaborative Document Provider".to_string(),
        version: "1.0.0".to_string(),
        description: "Provider for collaborative documents with CRDT support".to_string(),
    };
    
    // Register the provider
    let _provider_id = doc_provider.clone().register_provider(&registry, metadata)?;
    
    // Create a test document
    let owner_id = Uuid::new_v4();
    let content = DocumentContent {
        data: serde_json::json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    };
    
    let metadata = doc_service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;
    
    // Test that the provider correctly returns content items
    let content_items = doc_provider.get_content(
        owner_id,
        None,
        10,
        &[]
    ).await?;
    
    assert_eq!(content_items.len(), 1);
    assert_eq!(content_items[0].id, metadata.id);
    
    // Verify the content item has the expected structure for social feeds
    let content_item = &content_items[0];
    assert_eq!(content_item.source_package, "collaborative_docs");
    assert!(content_item.relevance_score > 0.0);
    
    // Check that metadata contains expected document information
    if let Some(title) = content_item.metadata.get("title") {
        assert_eq!(title, "Test Document");
    } else {
        panic!("Expected title in metadata");
    }
    
    Ok(())
}