//! Versioning and history management for collaborative documents

use crate::core::{DocumentContent, DocumentError, DocumentMetadata};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Error types for versioning operations
#[derive(Error, Debug)]
pub enum VersioningError {
    #[error("Version not found: {0}")]
    VersionNotFound(u64),
    #[error("Invalid diff: {0}")]
    InvalidDiff(String),
    #[error("History error: {0}")]
    HistoryError(String),
}

/// Document version information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentVersion {
    pub version_id: Uuid,
    pub document_id: Uuid,
    pub version_number: u64,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
    pub commit_message: Option<String>,
    pub content_hash: String,
}

/// Document diff representation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentDiff {
    pub from_version: u64,
    pub to_version: u64,
    pub changes: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

/// Document history entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentHistoryEntry {
    pub version: DocumentVersion,
    pub metadata: DocumentMetadata,
}

/// Version history manager
pub struct VersionHistory {
    versions: HashMap<u64, DocumentVersion>,
    diffs: Vec<DocumentDiff>,
}

impl VersionHistory {
    /// Create a new version history manager
    pub fn new() -> Self {
        Self {
            versions: HashMap::new(),
            diffs: Vec::new(),
        }
    }

    /// Add a new version to the history
    pub fn add_version(
        &mut self,
        version: DocumentVersion,
    ) -> Result<(), VersioningError> {
        self.versions.insert(version.version_number, version);
        Ok(())
    }

    /// Get a specific version
    pub fn get_version(
        &self,
        version_number: u64,
    ) -> Result<&DocumentVersion, VersioningError> {
        self.versions
            .get(&version_number)
            .ok_or(VersioningError::VersionNotFound(version_number))
    }

    /// Get all versions
    pub fn get_all_versions(&self) -> Vec<&DocumentVersion> {
        self.versions.values().collect()
    }

    /// Get the latest version
    pub fn get_latest_version(&self) -> Option<&DocumentVersion> {
        self.versions.values().max_by_key(|v| v.version_number)
    }

    /// Add a diff between versions
    pub fn add_diff(&mut self, diff: DocumentDiff) -> Result<(), VersioningError> {
        self.diffs.push(diff);
        Ok(())
    }

    /// Get diffs between versions
    pub fn get_diffs(
        &self,
        from_version: u64,
        to_version: u64,
    ) -> Vec<&DocumentDiff> {
        self.diffs
            .iter()
            .filter(|diff| {
                diff.from_version >= from_version && diff.to_version <= to_version
            })
            .collect()
    }

    /// Reconstruct document content at a specific version
    pub fn reconstruct_content(
        &self,
        base_content: &DocumentContent,
        target_version: u64,
    ) -> Result<DocumentContent, VersioningError> {
        // This is a simplified implementation
        // In a real system, this would apply diffs to reconstruct content
        Ok(base_content.clone())
    }
}

/// Diff calculator for document changes
pub struct DiffCalculator;

impl DiffCalculator {
    /// Calculate diff between two document contents
    pub fn calculate_diff(
        from: &DocumentContent,
        to: &DocumentContent,
        created_by: Uuid,
    ) -> Result<DocumentDiff, VersioningError> {
        // This is a simplified implementation
        // In a real system, this would calculate actual differences
        let changes = serde_json::json!({
            "operation": "update",
            "from_hash": calculate_content_hash(from),
            "to_hash": calculate_content_hash(to),
        });

        Ok(DocumentDiff {
            from_version: 0, // This would be determined by the versioning system
            to_version: 0,   // This would be determined by the versioning system
            changes,
            created_at: Utc::now(),
            created_by,
        })
    }

    /// Apply a diff to document content
    pub fn apply_diff(
        content: &DocumentContent,
        diff: &DocumentDiff,
    ) -> Result<DocumentContent, VersioningError> {
        // This is a simplified implementation
        // In a real system, this would apply the actual diff changes
        Ok(content.clone())
    }
}

/// Calculate a hash for document content
fn calculate_content_hash(content: &DocumentContent) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let mut hasher = DefaultHasher::new();
    content.data.to_string().hash(&mut hasher);
    content.format.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// Document history manager
pub struct DocumentHistoryManager {
    histories: HashMap<Uuid, VersionHistory>,
}

impl DocumentHistoryManager {
    /// Create a new document history manager
    pub fn new() -> Self {
        Self {
            histories: HashMap::new(),
        }
    }

    /// Get or create history for a document
    fn get_history_mut(&mut self, document_id: Uuid) -> &mut VersionHistory {
        self.histories
            .entry(document_id)
            .or_insert_with(VersionHistory::new)
    }

    /// Add a version to document history
    pub fn add_version(
        &mut self,
        document_id: Uuid,
        version: DocumentVersion,
    ) -> Result<(), VersioningError> {
        self.get_history_mut(document_id).add_version(version)
    }

    /// Get document version
    pub fn get_version(
        &self,
        document_id: Uuid,
        version_number: u64,
    ) -> Result<DocumentVersion, VersioningError> {
        self.histories
            .get(&document_id)
            .ok_or(VersioningError::HistoryError(
                "Document history not found".to_string(),
            ))?
            .get_version(version_number)
            .cloned()
    }

    /// Get document history
    pub fn get_history(
        &self,
        document_id: Uuid,
    ) -> Result<Vec<DocumentHistoryEntry>, DocumentError> {
        let history = self.histories.get(&document_id).ok_or(
            DocumentError::DocumentNotFound(document_id),
        )?;

        let mut entries = Vec::new();
        for version in history.get_all_versions() {
            // In a real implementation, we would fetch actual metadata
            let metadata = DocumentMetadata {
                id: version.document_id,
                title: format!("Version {}", version.version_number),
                owner_id: version.created_by,
                created_at: version.created_at,
                updated_at: version.created_at,
                content_type: "text/plain".to_string(),
                tags: vec![],
                version: version.version_number,
            };

            entries.push(DocumentHistoryEntry {
                version: version.clone(),
                metadata,
            });
        }

        Ok(entries)
    }

    /// Calculate diff between document versions
    pub fn calculate_diff(
        &self,
        document_id: Uuid,
        from_version: u64,
        to_version: u64,
    ) -> Result<DocumentDiff, VersioningError> {
        let history = self.histories.get(&document_id).ok_or(
            VersioningError::HistoryError("Document history not found".to_string()),
        )?;

        // This is a simplified implementation
        // In a real system, this would calculate actual differences
        let changes = serde_json::json!({
            "operation": "version_diff",
            "from_version": from_version,
            "to_version": to_version,
        });

        Ok(DocumentDiff {
            from_version,
            to_version,
            changes,
            created_at: Utc::now(),
            created_by: Uuid::nil(), // This would be the user who requested the diff
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_version_history() {
        let mut history = VersionHistory::new();
        
        let version = DocumentVersion {
            version_id: Uuid::new_v4(),
            document_id: Uuid::new_v4(),
            version_number: 1,
            created_at: Utc::now(),
            created_by: Uuid::new_v4(),
            commit_message: Some("Initial version".to_string()),
            content_hash: "hash1".to_string(),
        };
        
        assert!(history.add_version(version.clone()).is_ok());
        assert_eq!(history.get_all_versions().len(), 1);
        assert_eq!(history.get_latest_version().unwrap().version_number, 1);
    }

    #[test]
    fn test_diff_calculator() {
        let from_content = DocumentContent {
            data: json!({"text": "Hello"}),
            format: "json".to_string(),
        };
        
        let to_content = DocumentContent {
            data: json!({"text": "Hello World"}),
            format: "json".to_string(),
        };
        
        let diff = DiffCalculator::calculate_diff(
            &from_content,
            &to_content,
            Uuid::new_v4(),
        ).unwrap();
        
        assert_eq!(diff.changes["operation"], "update");
    }

    #[test]
    fn test_document_history_manager() {
        let mut manager = DocumentHistoryManager::new();
        let document_id = Uuid::new_v4();
        
        let version = DocumentVersion {
            version_id: Uuid::new_v4(),
            document_id,
            version_number: 1,
            created_at: Utc::now(),
            created_by: Uuid::new_v4(),
            commit_message: Some("Initial version".to_string()),
            content_hash: "hash1".to_string(),
        };
        
        assert!(manager.add_version(document_id, version).is_ok());
        
        let retrieved_version = manager.get_version(document_id, 1).unwrap();
        assert_eq!(retrieved_version.version_number, 1);
    }
}