use std::sync::{Arc, Mutex};
use crate::{
    conflict_resolver::{ConflictResolver, FileConflict, ConflictResolution},
    file_utils::{verify_merkle_tree, compute_file_diff, apply_file_diff},
    metadata::{FileMetadata, MetadataStore},
    network::{FileChangeNotification, FilePatchRequest, FilePatch},
    storage::NetworkError,
};

pub struct NetworkHandler {
    metadata_store: Arc<dyn MetadataStore + Send + Sync>,
    conflict_resolver: Arc<Mutex<ConflictResolver>>,
}

impl NetworkHandler {
    pub fn new(
        metadata_store: Arc<dyn MetadataStore + Send + Sync>,
        conflict_resolver: Arc<Mutex<ConflictResolver>>,
    ) -> Self {
        Self {
            metadata_store,
            conflict_resolver,
        }
    }

    /// Process file change notification from remote peer
    pub async fn handle_file_change_notification(
        &self,
        notification: FileChangeNotification,
    ) -> Result<Option<FilePatchRequest>, NetworkError> {
        let local_metadata = match self.metadata_store.get_metadata(&notification.file_path).await {
            Ok(meta) => meta,
            Err(_) => {
                // File doesn't exist locally, request full sync
                return Ok(Some(FilePatchRequest {
                    file_path: notification.file_path,
                    base_version: 0,
                }));
            }
        };

        // Check if we need an update
        if local_metadata.version < notification.version {
            // Request patch from current version
            Ok(Some(FilePatchRequest {
                file_path: notification.file_path,
                base_version: local_metadata.version,
            }))
        } else {
            // We're up-to-date
            Ok(None)
        }
    }

    /// Generate patch for requested file version
    pub async fn handle_file_patch_request(
        &self,
        request: FilePatchRequest,
    ) -> Result<Option<FilePatch>, NetworkError> {
        let current_metadata = self.metadata_store.get_metadata(&request.file_path).await?;
        
        // Verify we have a newer version
        if current_metadata.version <= request.base_version {
            return Ok(None);
        }

        // Get base metadata (could be stored or reconstructed)
        let base_metadata = self.metadata_store
            .get_versioned_metadata(&request.file_path, request.base_version)
            .await?;

        // Compute diff between versions
        let patches = compute_file_diff(&base_metadata, &current_metadata);

        Ok(Some(FilePatch {
            file_path: request.file_path,
            patches: bincode::serialize(&patches).map_err(|_| NetworkError::SerializationError)?,
            new_version: current_metadata.version,
        }))
    }

    /// Apply received patch to local file
    pub async fn handle_file_patch(
        &self,
        patch: FilePatch,
    ) -> Result<(), NetworkError> {
        let mut local_metadata = self.metadata_store.get_metadata(&patch.file_path).await?;
        
        // Verify version compatibility
        if local_metadata.version != patch.new_version - 1 {
            // Check for conflict
            let remote_metadata = FileMetadata {
                version: patch.new_version,
                ..local_metadata.clone()
            };
            
            let conflict = self.conflict_resolver
                .lock()
                .map_err(|_| NetworkError::StorageError("Mutex poisoned".into()))?
                .detect_conflict(&local_metadata, &remote_metadata);
            
            if let Some(conflict) = conflict {
                // Handle conflict resolution (simplified)
                let resolution = ConflictResolution::Merge; // In real impl, would prompt user
                self.resolve_conflict(conflict, resolution).await?;
            }
            return Err(NetworkError::InvalidData);
        }

        // Deserialize patches
        let patches: Vec<ChunkDiff> = bincode::deserialize(&patch.patches)
            .map_err(|_| NetworkError::SerializationError)?;

        // Apply patches to metadata
        apply_file_diff(&mut local_metadata, patches)
            .map_err(|e| NetworkError::StorageError(e))?;

        // Verify Merkle tree integrity
        verify_merkle_tree(&local_metadata)
            .map_err(|e| NetworkError::VerificationFailed(e))?;

        // Update metadata store
        self.metadata_store.store_metadata(&patch.file_path, local_metadata).await?;

        Ok(())
    }

    /// Resolve file conflict with chosen strategy
    async fn resolve_conflict(
        &self,
        conflict: FileConflict,
        resolution: ConflictResolution,
    ) -> Result<(), NetworkError> {
        let mut resolver = self.conflict_resolver
            .lock()
            .map_err(|_| NetworkError::StorageError("Mutex poisoned".into()))?;
        
        resolver.resolve_conflict(&conflict.file_id, resolution)
            .map_err(|e| NetworkError::StorageError(e))?;
        
        // In real implementation, we would apply the resolution to the actual file
        Ok(())
    }
}