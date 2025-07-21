use std::collections::HashMap;
use crate::metadata::FileMetadata;

pub struct FileConflict {
    pub file_id: String,
    pub local_version: u64,
    pub remote_version: u64,
    pub local_metadata: FileMetadata,
    pub remote_metadata: FileMetadata,
}

pub enum ConflictResolution {
    KeepLocal,
    KeepRemote,
    Merge,
}

pub struct ConflictResolver {
    pending_conflicts: HashMap<String, FileConflict>,
}

impl ConflictResolver {
    pub fn new() -> Self {
        Self { pending_conflicts: HashMap::new() }
    }
    
    pub fn detect_conflict(
        &mut self,
        local_metadata: &FileMetadata,
        remote_metadata: &FileMetadata
    ) -> Option<FileConflict> {
        if local_metadata.version != remote_metadata.version {
            let conflict = FileConflict {
                file_id: local_metadata.content_address.clone(),
                local_version: local_metadata.version,
                remote_version: remote_metadata.version,
                local_metadata: local_metadata.clone(),
                remote_metadata: remote_metadata.clone(),
            };
            self.pending_conflicts.insert(local_metadata.content_address.clone(), conflict.clone());
            Some(conflict)
        } else {
            None
        }
    }
    
    pub fn resolve_conflict(
        &mut self,
        file_id: &str,
        resolution: ConflictResolution
    ) -> Result<(), String> {
        self.pending_conflicts.remove(file_id)
            .ok_or_else(|| "Conflict not found".to_string())?;
        
        // In a real implementation, we would apply the resolution here
        // For now, we just remove the conflict from pending
        Ok(())
    }

    pub fn get_conflicts(&self) -> Vec<FileConflict> {
        self.pending_conflicts.values().cloned().collect()
    }
}