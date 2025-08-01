//! Version history management for collaborative documents

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use crate::core::{Operation, Document, CollaborationError};
use event_bus::{EventBus, DomainEvent};
use serde_json::json;

/// Represents a version of a document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub id: Uuid,
    pub document_id: Uuid,
    pub version_number: u64,
    pub content: String,
    pub operations: Vec<Operation>,
    pub author_id: Uuid,
    pub author_name: String,
    pub created_at: DateTime<Utc>,
    pub commit_message: Option<String>,
    pub conflict_metadata: Option<serde_json::Value>,
}

/// Manages version history for documents
///
/// The VersionManager is responsible for:
/// - Creating and managing document versions
/// - Handling branching and tagging of versions
/// - Supporting conflict-aware versioning
/// - Comparing versions and generating diffs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionManager {
    pub document_id: Uuid,
    pub versions: HashMap<u64, DocumentVersion>,
    pub current_version: u64,
    pub branches: HashMap<String, u64>,
    pub tags: HashMap<String, u64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[serde(skip)]
    pub event_bus: Option<EventBus>,
}

impl VersionManager {
    /// Create a new version manager for a document
    pub fn new(document_id: Uuid) -> Self {
        let mut branches = HashMap::new();
        branches.insert("main".to_string(), 0);
        
        Self {
            document_id,
            versions: HashMap::new(),
            current_version: 0,
            branches,
            tags: HashMap::new(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            event_bus: None,
        }
    }
    
    /// Set the event bus for this version manager
    pub fn set_event_bus(&mut self, event_bus: EventBus) {
        self.event_bus = Some(event_bus);
    }
/// Create a new version from a document
///
/// Creates a new version of the document with the provided metadata.
///
/// # Arguments
/// * `document` - The document to create a version from
/// * `author_id` - The UUID of the author creating the version
/// * `author_name` - The name of the author creating the version
/// * `commit_message` - An optional commit message
/// * `conflict_metadata` - Optional metadata about conflicts that were resolved
///
/// # Returns
/// The version number of the created version, or an error if creation failed
pub fn create_version(
    &mut self,
    document: &Document,
    author_id: Uuid,
    author_name: String,
    commit_message: Option<String>,
    conflict_metadata: Option<serde_json::Value>,
) -> Result<u64, CollaborationError> {
        let version_number = document.version;
        
        let version = DocumentVersion {
            id: Uuid::new_v4(),
            document_id: self.document_id,
            version_number,
            content: document.content.clone(),
            operations: document.operations.clone(),
            author_id,
            author_name,
            created_at: Utc::now(),
            commit_message,
            conflict_metadata,
        };
        
        self.versions.insert(version_number, version.clone());
        self.current_version = version_number;
        
        // Update all branches to point to the new version
        for branch_version in self.branches.values_mut() {
            *branch_version = version_number;
        }
        
        self.updated_at = Utc::now();
        
        // Publish event if event bus is available
        if let Some(ref event_bus) = self.event_bus {
            let event = DomainEvent::new_local(
                "collaboration".to_string(),
                "VersionCreated".to_string(),
                json!({
                    "document_id": self.document_id,
                    "version": version,
                }),
            );
            
            // We're ignoring the result here as we don't want to fail the operation
            // if event publishing fails
            let _ = event_bus.publish(event);
        }
        
        Ok(version_number)
    }

    /// Get a specific version of the document
    pub fn get_version(&self, version_number: u64) -> Option<DocumentVersion> {
        self.versions.get(&version_number).cloned()
    }

    /// Get the latest version
    pub fn get_latest_version(&self) -> Option<DocumentVersion> {
        self.versions.get(&self.current_version).cloned()
    }

    /// List all versions
    pub fn list_versions(&self) -> Vec<DocumentVersion> {
        let mut versions: Vec<DocumentVersion> = self.versions.values().cloned().collect();
        versions.sort_by(|a, b| a.version_number.cmp(&b.version_number));
        versions
    }

    /// Create a new branch
    ///
    /// Creates a new branch pointing to the specified version.
    ///
    /// # Arguments
    /// * `branch_name` - The name of the branch to create
    /// * `version_number` - The version number the branch should point to
    ///
    /// # Returns
    /// Ok(()) if the branch was created successfully, or an error if creation failed
    pub fn create_branch(&mut self, branch_name: String, version_number: u64) -> Result<(), CollaborationError> {
        if self.versions.contains_key(&version_number) {
            self.branches.insert(branch_name, version_number);
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(CollaborationError::InvalidPosition)
        }
    }

    /// Get the version number for a branch
    pub fn get_branch_version(&self, branch_name: &str) -> Option<u64> {
        self.branches.get(branch_name).copied()
    }

    /// Create a tag for a specific version
    pub fn create_tag(&mut self, tag_name: String, version_number: u64) -> Result<(), CollaborationError> {
        if self.versions.contains_key(&version_number) {
            self.tags.insert(tag_name, version_number);
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(CollaborationError::InvalidPosition)
        }
    }

    /// Get the version number for a tag
    pub fn get_tag_version(&self, tag_name: &str) -> Option<u64> {
        self.tags.get(tag_name).copied()
    }

    /// Get conflict history between two versions
    fn get_conflict_history(&self, version_a: u64, version_b: u64) -> Vec<ConflictHistoryEntry> {
        // In a real implementation, this would retrieve actual conflict history
        // For now, we return an empty vector as a placeholder
        vec![]
    }

    /// Compare two versions and return the differences
    pub fn compare_versions(&self, version_a: u64, version_b: u64) -> Result<VersionDiff, CollaborationError> {
        let version_a = self.get_version(version_a)
            .ok_or(CollaborationError::InvalidPosition)?;
        let version_b = self.get_version(version_b)
            .ok_or(CollaborationError::InvalidPosition)?;
            
        // In a real implementation, this would do a more sophisticated diff
        let changes = if version_a.content != version_b.content {
            vec![Change {
                operation: Operation::Replace {
                    start: crate::core::Position { line: 0, column: 0 },
                    end: crate::core::Position { line: usize::MAX, column: usize::MAX },
                    text: version_b.content.clone(),
                    user_id: version_b.author_id,
                    timestamp: version_b.created_at,
                },
                author_id: version_b.author_id,
                author_name: version_b.author_name.clone(),
            }]
        } else {
            vec![]
        };
        
        Ok(VersionDiff {
            version_a: version_a.version_number,
            version_b: version_b.version_number,
            changes,
            conflict_history: self.get_conflict_history(version_a.version_number, version_b.version_number),
        })
    }
    
    /// Create a conflict-aware branch
    ///
    /// Creates a new branch with conflict metadata for tracking conflict resolution history.
    ///
    /// # Arguments
    /// * `branch_name` - The name of the branch to create
    /// * `version_number` - The version number the branch should point to
    /// * `conflict_metadata` - Optional metadata about conflicts related to this branch
    ///
    /// # Returns
    /// Ok(()) if the branch was created successfully, or an error if creation failed
    pub fn create_conflict_aware_branch(&mut self, branch_name: String, version_number: u64, conflict_metadata: Option<serde_json::Value>) -> Result<(), CollaborationError> {
        if self.versions.contains_key(&version_number) {
            // Create the branch
            self.branches.insert(branch_name.clone(), version_number);
            
            // If there's conflict metadata, store it with the branch
            if let Some(metadata) = conflict_metadata {
                // In a real implementation, we would store this metadata with the branch
                // For now, we'll just log it
                tracing::debug!("Creating conflict-aware branch {} with metadata: {:?}", branch_name, metadata);
            }
            
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err(CollaborationError::InvalidPosition)
        }
    }
    
    /// Merge two branches, handling conflicts
    ///
    /// Merges the source branch into the target branch.
    ///
    /// # Arguments
    /// * `source_branch` - The name of the branch to merge from
    /// * `target_branch` - The name of the branch to merge into
    ///
    /// # Returns
    /// A MergeResult with information about the merge, or an error if the merge failed
    pub fn merge_branches(&mut self, source_branch: &str, target_branch: &str) -> Result<MergeResult, CollaborationError> {
        let source_version = self.get_branch_version(source_branch)
            .ok_or(CollaborationError::InvalidPosition)?;
        let target_version = self.get_branch_version(target_branch)
            .ok_or(CollaborationError::InvalidPosition)?;
            
        // In a real implementation, this would do a more sophisticated merge
        // For now, we'll just update the target branch to point to the source version
        self.branches.insert(target_branch.to_string(), source_version);
        self.updated_at = Utc::now();
        
        Ok(MergeResult {
            source_branch: source_branch.to_string(),
            target_branch: target_branch.to_string(),
            merged_version: source_version,
            conflicts_resolved: vec![],
        })
    }
}

/// Represents differences between versions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionDiff {
    pub version_a: u64,
    pub version_b: u64,
    pub changes: Vec<Change>,
    pub conflict_history: Vec<ConflictHistoryEntry>,
}

/// Represents a conflict history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictHistoryEntry {
    pub conflict_id: Uuid,
    pub resolved_at: DateTime<Utc>,
    pub resolution_strategy: String,
    pub involved_users: Vec<Uuid>,
}

/// Represents a change in the document
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Change {
    pub operation: Operation,
    pub author_id: Uuid,
    pub author_name: String,
}

/// Result of a branch merge operation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MergeResult {
    pub source_branch: String,
    pub target_branch: String,
    pub merged_version: u64,
    pub conflicts_resolved: Vec<serde_json::Value>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Document, Operation, Position};
    use uuid::Uuid;
    use chrono::Utc;
    
    #[test]
    fn test_version_manager_creation() {
        let document_id = Uuid::new_v4();
        let version_manager = VersionManager::new(document_id);
        
        assert_eq!(version_manager.document_id, document_id);
        assert_eq!(version_manager.versions.len(), 0);
        assert_eq!(version_manager.branches.len(), 1); // "main" branch
        assert!(version_manager.branches.contains_key("main"));
    }
    
    #[test]
    fn test_create_version() {
        let document_id = Uuid::new_v4();
        let mut version_manager = VersionManager::new(document_id);
        let document = Document::new("Hello World".to_string());
        let author_id = Uuid::new_v4();
        
        let result = version_manager.create_version(
            &document,
            author_id,
            "Test Author".to_string(),
            Some("Initial commit".to_string()),
            None,
        );
        
        assert!(result.is_ok());
        assert_eq!(version_manager.versions.len(), 1);
        assert_eq!(version_manager.current_version, 0);
    }
    
    #[test]
    fn test_create_branch() {
        let document_id = Uuid::new_v4();
        let mut version_manager = VersionManager::new(document_id);
        let document = Document::new("Hello World".to_string());
        let author_id = Uuid::new_v4();
        
        // Create initial version
        version_manager.create_version(&document, author_id, "Test Author".to_string(), None, None).unwrap();
        
        // Create a branch
        let result = version_manager.create_branch("feature".to_string(), 0);
        assert!(result.is_ok());
        assert_eq!(version_manager.branches.len(), 2); // "main" + "feature"
        assert_eq!(version_manager.get_branch_version("feature"), Some(0));
    }
    
    #[test]
    fn test_create_conflict_aware_branch() {
        let document_id = Uuid::new_v4();
        let mut version_manager = VersionManager::new(document_id);
        let document = Document::new("Hello World".to_string());
        let author_id = Uuid::new_v4();
        
        // Create initial version
        version_manager.create_version(&document, author_id, "Test Author".to_string(), None, None).unwrap();
        
        // Create a conflict-aware branch
        let conflict_metadata = Some(serde_json::json!({
            "conflict_id": Uuid::new_v4().to_string(),
            "resolution_strategy": "timestamp_order"
        }));
        
        let result = version_manager.create_conflict_aware_branch("conflict-fix".to_string(), 0, conflict_metadata);
        assert!(result.is_ok());
        assert_eq!(version_manager.branches.len(), 2); // "main" + "conflict-fix"
        assert_eq!(version_manager.get_branch_version("conflict-fix"), Some(0));
    }
    
    #[test]
    fn test_create_tag() {
        let document_id = Uuid::new_v4();
        let mut version_manager = VersionManager::new(document_id);
        let document = Document::new("Hello World".to_string());
        let author_id = Uuid::new_v4();
        
        // Create initial version
        version_manager.create_version(&document, author_id, "Test Author".to_string(), None, None).unwrap();
        
        // Create a tag
        let result = version_manager.create_tag("v1.0".to_string(), 0);
        assert!(result.is_ok());
        assert_eq!(version_manager.tags.len(), 1);
        assert_eq!(version_manager.get_tag_version("v1.0"), Some(0));
    }
    
    #[test]
    fn test_merge_branches() {
        let document_id = Uuid::new_v4();
        let mut version_manager = VersionManager::new(document_id);
        let document = Document::new("Hello World".to_string());
        let author_id = Uuid::new_v4();
        
        // Create initial version
        version_manager.create_version(&document, author_id, "Test Author".to_string(), None, None).unwrap();
        
        // Create a feature branch
        version_manager.create_branch("feature".to_string(), 0).unwrap();
        
        // Merge branches
        let result = version_manager.merge_branches("feature", "main");
        assert!(result.is_ok());
        
        let merge_result = result.unwrap();
        assert_eq!(merge_result.source_branch, "feature");
        assert_eq!(merge_result.target_branch, "main");
        assert_eq!(merge_result.merged_version, 0);
    }
    
    #[test]
    fn test_compare_versions() {
        let document_id = Uuid::new_v4();
        let version_manager = VersionManager::new(document_id);
        
        // Compare non-existent versions should fail
        let result = version_manager.compare_versions(0, 1);
        assert!(result.is_err());
    }
}

/// Event for version changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VersionEvent {
    VersionCreated {
        document_id: Uuid,
        version_number: u64,
        author_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    BranchCreated {
        document_id: Uuid,
        branch_name: String,
        version_number: u64,
        author_id: Uuid,
        timestamp: DateTime<Utc>,
    },
    TagCreated {
        document_id: Uuid,
        tag_name: String,
        version_number: u64,
        author_id: Uuid,
        timestamp: DateTime<Utc>,
    },
}