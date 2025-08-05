//! Example showing how to register the collaborative document provider with the social graph

use cpc_collaborative_docs::{
    CollaborativeDocService, CollaborativeDocProvider, CollaborativeDocProviderMetadata,
    sled_store::SledDocStore,
    access::DocumentAccessChecker
};
use social_graph::{
    infrastructure::content_providers::ContentProviderRegistry,
    infrastructure::consent_service_impl::ConsentServiceImpl,
    infrastructure::in_memory_repository::InMemoryRelationshipRepository
};
use std::sync::Arc;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize document storage (using in-memory for this example)
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
    
    println!("Successfully registered collaborative document provider with ID: {}", provider_id);
    
    // Verify registration
    let all_metadata = registry.get_all_metadata()?;
    println!("Total registered providers: {}", all_metadata.len());
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_provider_registration() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize document storage (using in-memory for this example)
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
}