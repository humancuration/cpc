// DEPRECATED - Refactored to use collaboration_engine
// This file has been deprecated as part of the refactor to use the collaboration_engine package.
// The new implementation can be found in the application/collaboration_service.rs file.
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use uuid::Uuid;
    use crate::domain::value_objects::{DocumentTitle, DocumentContent};
    use crate::domain::models::{Document, PermissionLevel};
    use crate::infrastructure::repository::DocumentRepository;
    
    // Mock repository for testing
    struct MockRepository;
    
    #[async_trait::async_trait]
    impl DocumentRepository for MockRepository {
        async fn create_document(&self, _document: &Document) -> Result<(), DocumentError> {
            Ok(())
        }
        
        async fn get_document(&self, _id: Uuid) -> Result<Document, DocumentError> {
            let title = DocumentTitle::new("Test Document".to_string()).unwrap();
            let content = DocumentContent::new(serde_json::json!({"text": "Hello, world!"}));
            Ok(Document {
                id: Uuid::new_v4(),
                owner_id: Uuid::new_v4(),
                title,
                content,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
                is_deleted: false,
            })
        }
        
        async fn update_document(&self, _document: &Document) -> Result<(), DocumentError> {
            Ok(())
        }
        
        async fn delete_document(&self, _id: Uuid) -> Result<(), DocumentError> {
            Ok(())
        }
        
        async fn get_documents_by_owner(&self, _owner_id: Uuid) -> Result<Vec<Document>, DocumentError> {
            Ok(vec![])
        }
        
        async fn create_document_share(&self, _share: &crate::domain::models::DocumentShare) -> Result<(), DocumentError> {
            Ok(())
        }
        
        async fn get_document_share(&self, _document_id: Uuid, _user_id: Uuid) -> Result<crate::domain::models::DocumentShare, DocumentError> {
            Ok(crate::domain::models::DocumentShare {
                id: Uuid::new_v4(),
                document_id: Uuid::new_v4(),
                shared_with: Uuid::new_v4(),
                permission_level: PermissionLevel::Edit,
                created_at: chrono::Utc::now(),
                expires_at: None,
            })
        }
        
        async fn create_document_version(&self, _version: &crate::domain::models::DocumentVersion) -> Result<(), DocumentError> {
            Ok(())
        }
        
        async fn get_document_versions(&self, _document_id: Uuid) -> Result<Vec<crate::domain::models::DocumentVersion>, DocumentError> {
            Ok(vec![])
        }
        
        async fn get_latest_version_number(&self, _document_id: Uuid) -> Result<i32, DocumentError> {
            Ok(1)
        }
        
        async fn save_crdt_document(&self, _document_id: Uuid, _crdt_document: &crate::crdt::document::CRDTDocument) -> Result<(), DocumentError> {
            Ok(())
        }
        
        async fn load_crdt_document(&self, _document_id: Uuid) -> Result<Option<crate::crdt::document::CRDTDocument>, DocumentError> {
            Ok(None)
        }
    }
    
    #[tokio::test]
    async fn test_realtime_collaboration_service_creation() {
        let repository = Arc::new(MockRepository);
        let node_id = Uuid::new_v4();
        let p2p_network = Arc::new(Mutex::new(PandaNetwork::new(node_id)));
        let p2p_sync_service = Arc::new(PandaSyncService::new(p2p_network.clone()));
        
        let service = RealtimeCollaborationService::new(
            repository,
            p2p_network,
            p2p_sync_service,
        );
        
        // The service should be created successfully
        assert!(true);
    }
    
    #[tokio::test]
    async fn test_initialize_document() {
        let repository = Arc::new(MockRepository);
        let node_id = Uuid::new_v4();
        let p2p_network = Arc::new(Mutex::new(PandaNetwork::new(node_id)));
        let p2p_sync_service = Arc::new(PandaSyncService::new(p2p_network.clone()));
        
        let service = RealtimeCollaborationService::new(
            repository,
            p2p_network,
            p2p_sync_service,
        );
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Initialize a document
        let result = service.initialize_document(document_id, user_id);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_apply_operation() {
        let repository = Arc::new(MockRepository);
        let node_id = Uuid::new_v4();
        let p2p_network = Arc::new(Mutex::new(PandaNetwork::new(node_id)));
        let p2p_sync_service = Arc::new(PandaSyncService::new(p2p_network.clone()));
        
        let service = RealtimeCollaborationService::new(
            repository,
            p2p_network,
            p2p_sync_service,
        );
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Initialize a document
        let result = service.initialize_document(document_id, user_id);
        assert!(result.is_ok());
        
        // Create an operation
        let id = crate::crdt::id::CRDTId::new(user_id, 1, 1);
        let value = serde_json::json!({"text": "Hello, CRDT!"});
        let operation = crate::crdt::operations::DocumentOperation::Insert {
            position: 0,
            value,
            id,
            parent_id: None,
        };
        
        // Apply the operation
        let result = service.apply_operation(document_id, operation);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_subscribe_to_operations() {
        let repository = Arc::new(MockRepository);
        let node_id = Uuid::new_v4();
        let p2p_network = Arc::new(Mutex::new(PandaNetwork::new(node_id)));
        let p2p_sync_service = Arc::new(PandaSyncService::new(p2p_network.clone()));
        
        let service = RealtimeCollaborationService::new(
            repository,
            p2p_network,
            p2p_sync_service,
        );
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Initialize a document
        let result = service.initialize_document(document_id, user_id);
        assert!(result.is_ok());
        
        // Subscribe to operations
        let result = service.subscribe_to_operations(document_id);
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_document_content() {
        let repository = Arc::new(MockRepository);
        let node_id = Uuid::new_v4();
        let p2p_network = Arc::new(Mutex::new(PandaNetwork::new(node_id)));
        let p2p_sync_service = Arc::new(PandaSyncService::new(p2p_network.clone()));
        
        let service = RealtimeCollaborationService::new(
            repository,
            p2p_network,
            p2p_sync_service,
        );
        
        let document_id = Uuid::new_v4();
        let user_id = Uuid::new_v4();
        
        // Initialize a document
        let result = service.initialize_document(document_id, user_id);
        assert!(result.is_ok());
        
        // Get document content
        let result = service.get_document_content(document_id);
        assert!(result.is_ok());
    }
}