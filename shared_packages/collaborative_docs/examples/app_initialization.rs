//! Example showing how an app would initialize the collaborative document provider during startup

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
use realtime_signaling::SignalingService;
use feed_utils::PreviewService;
use std::sync::Arc;
use uuid::Uuid;
/// App state that holds all the services
pub struct AppState {
    pub doc_service: Arc<CollaborativeDocService>,
    pub doc_provider: Arc<CollaborativeDocProvider>,
    pub provider_registry: ContentProviderRegistry,
    pub signaling_service: Arc<SignalingService>,
    pub preview_service: Arc<PreviewService>,
}
}

/// Initialize the app with all required services
pub async fn initialize_app() -> Result<AppState, Box<dyn std::error::Error>> {
    println!("Initializing collaborative document service...");
    
    // Step 1: Initialize document storage
    // In a real app, this would be a persistent store like PostgreSQL
    // For this example, we use an in-memory store
    let doc_store = Arc::new(SledDocStore::new_in_memory()?);
    println!("  ✓ Document storage initialized");
    
    // Step 2: Initialize access checker for consent management
    let access_checker = DocumentAccessChecker::new(None);
    println!("  ✓ Access checker initialized");
    
    // Step 3: Create document service
    let doc_service = Arc::new(CollaborativeDocService::new(
        doc_store,
        access_checker,
    ));
    println!("  ✓ Document service created");
    
    // Step 4: Create the content provider for social graph integration
    let doc_provider = Arc::new(CollaborativeDocProvider::new(doc_service.clone()));
    println!("  ✓ Content provider created");
    
    // Step 5: Initialize social graph components for content provider registry
    let relationship_repo = Arc::new(InMemoryRelationshipRepository::new());
    let consent_service = Arc::new(ConsentServiceImpl::new(relationship_repo));
    let provider_registry = ContentProviderRegistry::new(consent_service);
    println!("  ✓ Social graph components initialized");
    
    // Step 6: Prepare provider metadata
    let metadata = CollaborativeDocProviderMetadata {
        provider_id: Uuid::new_v4(),
        name: "Collaborative Document Provider".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        description: "Provider for collaborative documents with CRDT support".to_string(),
    };
    println!("  ✓ Provider metadata prepared");
    
    // Step 7: Register the provider with the social graph
    let provider_id = doc_provider.clone().register_provider(&provider_registry, metadata)?;
    println!("  ✓ Provider registered with ID: {}", provider_id);
    
    // Step 8: Initialize real-time signaling service
    let signaling_service = Arc::new(SignalingService::new());
    println!("  ✓ Real-time signaling service initialized");
    
    // Step 9: Initialize preview service
    let preview_service = Arc::new(PreviewService::new(doc_service.clone()));
    println!("  ✓ Preview service initialized");
    
    println!("App initialization complete!");
    
    Ok(AppState {
        doc_service,
        doc_provider,
        provider_registry,
        signaling_service,
        preview_service,
    })
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the app
    let app_state = initialize_app().await?;
    
    // Now the app is ready to use!
    // The collaborative document provider is registered with the social graph
    // and documents will automatically appear in user feeds based on consent rules
    
    println!("\nApp is ready!");
    println!("Document service: {:?}", app_state.doc_service.as_ref() as *const _);
    println!("Content provider: {:?}", app_state.doc_provider.as_ref() as *const _);
    println!("Signaling service: {:?}", app_state.signaling_service.as_ref() as *const _);
    println!("Preview service: {:?}", app_state.preview_service.as_ref() as *const _);
    
    // In a real app, you would now start your web server or UI here
    // and the services would be available to handle requests
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_app_initialization() -> Result<(), Box<dyn std::error::Error>> {
        let app_state = initialize_app().await?;
        
        // Verify all components are initialized
        assert!(!Arc::ptr_eq(&app_state.doc_service, &Arc::new(CollaborativeDocService::new(
            Arc::new(SledDocStore::new_in_memory()?),
            DocumentAccessChecker::new(None),
        ))));
        
        // Verify provider is registered
        let metadata = app_state.provider_registry.get_all_metadata()?;
        assert_eq!(metadata.len(), 1);
        assert_eq!(metadata[0].name, "Collaborative Document Provider");
        
        // Verify signaling service is initialized
        assert!(!Arc::ptr_eq(&app_state.signaling_service, &Arc::new(SignalingService::new())));
        
        // Verify preview service is initialized
        assert!(!Arc::ptr_eq(&app_state.preview_service, &Arc::new(PreviewService::new(app_state.doc_service.clone()))));
        
        Ok(())
    }
}