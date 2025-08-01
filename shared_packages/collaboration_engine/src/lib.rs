//! # Collaboration Engine
//!
//! A shared package providing real-time document collaboration capabilities including:
//! - Operational Transformation/CRDT algorithms for conflict-free editing
//! - Shared cursor presence tracking
//! - Conflict resolution system
//! - Version history management
//! - Event bus integration for real-time messaging

pub mod core;
pub mod presence;
pub mod conflict_resolution;
pub mod versioning;
pub mod schema_registry;

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn test_document_creation() {
        let content = "Hello, World!".to_string();
        let document = core::Document::new(content.clone());
        
        assert_eq!(document.content, content);
        assert_eq!(document.version, 0);
        assert_eq!(document.operations.len(), 0);
    }

    #[test]
    fn test_crdt_document_creation() {
        let crdt_doc = core::CRDTDocument::new();
        
        assert_eq!(crdt_doc.content.len(), 0);
        assert_eq!(crdt_doc.version_vector.len(), 0);
    }

    #[test]
    fn test_presence_manager_creation() {
        let document_id = Uuid::new_v4();
        let presence_manager = presence::PresenceManager::new(document_id);
        
        assert_eq!(presence_manager.document_id, document_id);
        assert_eq!(presence_manager.users.len(), 0);
    }

    #[test]
    fn test_conflict_resolver_creation() {
        let document_id = Uuid::new_v4();
        let conflict_resolver = conflict_resolution::ConflictResolver::new(document_id);
        
        assert_eq!(conflict_resolver.document_id, document_id);
        assert_eq!(conflict_resolver.conflicts.len(), 0);
    }

    #[test]
    fn test_version_manager_creation() {
        let document_id = Uuid::new_v4();
        let version_manager = versioning::VersionManager::new(document_id);
        
        assert_eq!(version_manager.document_id, document_id);
        assert_eq!(version_manager.versions.len(), 0);
        assert_eq!(version_manager.branches.len(), 1); // "main" branch
    }
    
    #[test]
    fn test_schema_registry_creation() {
        let registry = schema_registry::SchemaRegistry::new();
        
        assert_eq!(registry.list_versions("test").len(), 0);
    }
}