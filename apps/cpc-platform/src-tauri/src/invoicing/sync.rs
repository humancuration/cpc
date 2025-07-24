use cpc_core::invoicing::model::{Invoice, Customer};
use cpc_core::invoicing::repository::{
    InvoiceRepository, CustomerRepository, InvoiceSyncRepository, 
    RepositoryError, SyncChanges, SyncConflict, ChangeType, EntityType
};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error)]
pub enum SyncError {
    #[error("Repository error: {0}")]
    RepositoryError(#[from] RepositoryError),
    
    #[error("Conflict resolution error: {0}")]
    ConflictResolutionError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    pub last_sync_time: DateTime<Utc>,
    pub is_syncing: bool,
    pub conflicts: Vec<SyncConflict>,
    pub pending_changes: usize,
}

pub struct InvoiceSyncService {
    invoice_repo: Box<dyn InvoiceRepository>,
    customer_repo: Box<dyn CustomerRepository>,
    sync_repo: Box<dyn InvoiceSyncRepository>,
    conflict_resolver: Box<dyn ConflictResolver>,
}

#[async_trait::async_trait]
pub trait ConflictResolver: Send + Sync {
    async fn resolve(
        &self,
        conflict: &SyncConflict,
        local_data: &Invoice,
        remote_data: &Invoice,
    ) -> Result<Resolution, SyncError>;
}

#[derive(Debug, Clone)]
pub enum Resolution {
    UseLocal,
    UseRemote,
    Merge(Invoice),
}

impl InvoiceSyncService {
    pub fn new(
        invoice_repo: impl InvoiceRepository + 'static,
        customer_repo: impl CustomerRepository + 'static,
        sync_repo: impl InvoiceSyncRepository + 'static,
        conflict_resolver: impl ConflictResolver + 'static,
    ) -> Self {
        Self {
            invoice_repo: Box::new(invoice_repo),
            customer_repo: Box::new(customer_repo),
            sync_repo: Box::new(sync_repo),
            conflict_resolver: Box::new(conflict_resolver),
        }
    }

    pub async fn sync(&self) -> Result<SyncResult, SyncError> {
        let last_sync_time = self.sync_repo.get_last_sync_time().await?;
        let mut state = SyncState {
            last_sync_time,
            is_syncing: true,
            conflicts: Vec::new(),
            pending_changes: 0,
        };

        // Get local changes since last sync
        let local_changes = self.get_local_changes(last_sync_time).await?;
        
        // Get remote changes
        let remote_changes = self.get_remote_changes(last_sync_time).await?;

        // Detect conflicts
        let (conflicts, non_conflicts) = self.detect_conflicts(&local_changes, &remote_changes).await?;

        // Resolve conflicts
        state.conflicts = conflicts.clone();
        let resolved_changes = self.resolve_conflicts(conflicts).await?;

        // Apply non-conflicting changes
        self.apply_changes(non_conflicts).await?;
        self.apply_changes(resolved_changes).await?;

        // Update sync timestamp
        let new_sync_time = Utc::now();
        self.sync_repo.update_sync_time(new_sync_time).await?;

        state.last_sync_time = new_sync_time;
        state.is_syncing = false;

        Ok(SyncResult {
            success: true,
            conflicts_resolved: state.conflicts.len(),
            changes_applied: local_changes.len() + remote_changes.len(),
            sync_time: new_sync_time,
        })
    }

    async fn get_local_changes(&self, since: DateTime<Utc>) -> Result<Vec<LocalChange>, SyncError> {
        let invoices = self.invoice_repo.get_pending_sync(since).await?;
        let customers = self.customer_repo.list().await?; // Simplified
        
        let mut changes = Vec::new();
        
        for invoice in invoices {
            changes.push(LocalChange {
                entity_id: invoice.id,
                entity_type: EntityType::Invoice,
                change_type: ChangeType::Updated,
                data: serde_json::to_value(&invoice)?,
                version: invoice.sync_version,
            });
        }
        
        Ok(changes)
    }

    async fn get_remote_changes(&self, since: DateTime<Utc>) -> Result<SyncChanges, SyncError> {
        Ok(self.sync_repo.get_changes_since(since).await?)
    }

    async fn detect_conflicts(
        &self,
        local_changes: &[LocalChange],
        remote_changes: &SyncChanges,
    ) -> Result<(Vec<SyncConflict>, Vec<RemoteChange>), SyncError> {
        let mut conflicts = Vec::new();
        let mut non_conflicts = Vec::new();
        
        // Create lookup maps
        let local_map: HashMap<Uuid, &LocalChange> = local_changes
            .iter()
            .map(|c| (c.entity_id, c))
            .collect();
            
        let remote_map: HashMap<Uuid, &InvoiceChange> = remote_changes
            .invoices
            .iter()
            .map(|c| (c.invoice_id, c))
            .collect();
        
        // Check for conflicts
        for (entity_id, local_change) in &local_map {
            if let Some(remote_change) = remote_map.get(entity_id) {
                if local_change.version == remote_change.previous_version.unwrap_or(0) {
                    // No conflict, remote change is newer
                    non_conflicts.push(RemoteChange {
                        entity_id: *entity_id,
                        data: remote_change.invoice.clone(),
                        change_type: remote_change.change_type.clone(),
                    });
                } else {
                    // Conflict detected
                    let conflict = SyncConflict {
                        entity_id: *entity_id,
                        entity_type: EntityType::Invoice,
                        local_version: local_change.version,
                        remote_version: remote_change.previous_version.unwrap_or(0),
                        local_data: serde_json::to_vec(&local_change.data)?,
                        remote_data: serde_json::to_vec(&remote_change.invoice)?,
                    };
                    conflicts.push(conflict);
                }
            }
        }
        
        // Add non-conflicting remote changes
        for remote_change in &remote_changes.invoices {
            if !local_map.contains_key(&remote_change.invoice_id) {
                non_conflicts.push(RemoteChange {
                    entity_id: remote_change.invoice_id,
                    data: remote_change.invoice.clone(),
                    change_type: remote_change.change_type.clone(),
                });
            }
        }
        
        Ok((conflicts, non_conflicts))
    }

    async fn resolve_conflicts(
        &self,
        conflicts: Vec<SyncConflict>,
    ) -> Result<Vec<ResolvedChange>, SyncError> {
        let mut resolved = Vec::new();
        
        for conflict in conflicts {
            let local_invoice: Invoice = serde_json::from_slice(&conflict.local_data)?;
            let remote_invoice: Invoice = serde_json::from_slice(&conflict.remote_data)?;
            
            let resolution = self.conflict_resolver
                .resolve(&conflict, &local_invoice, &remote_invoice)
                .await?;
                
            resolved.push(ResolvedChange {
                entity_id: conflict.entity_id,
                invoice: match resolution {
                    Resolution::UseLocal => local_invoice,
                    Resolution::UseRemote => remote_invoice,
                    Resolution::Merge(merged) => merged,
                },
            });
        }
        
        Ok(resolved)
    }

    async fn apply_changes(&self, changes: Vec<RemoteChange>) -> Result<(), SyncError> {
        for change in changes {
            match change.change_type {
                ChangeType::Created | ChangeType::Updated => {
                    self.invoice_repo.update(change.data).await?;
                }
                ChangeType::Deleted => {
                    self.invoice_repo.delete(change.entity_id).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn get_sync_state(&self) -> Result<SyncState, SyncError> {
        let last_sync_time = self.sync_repo.get_last_sync_time().await?;
        let pending_changes = self.get_local_changes(last_sync_time).await?.len();
        
        Ok(SyncState {
            last_sync_time,
            is_syncing: false,
            conflicts: Vec::new(),
            pending_changes,
        })
    }
}

#[derive(Debug, Clone)]
struct LocalChange {
    entity_id: Uuid,
    entity_type: EntityType,
    change_type: ChangeType,
    data: serde_json::Value,
    version: u64,
}

#[derive(Debug, Clone)]
struct RemoteChange {
    entity_id: Uuid,
    data: Invoice,
    change_type: ChangeType,
}

#[derive(Debug, Clone)]
struct ResolvedChange {
    entity_id: Uuid,
    invoice: Invoice,
}

#[derive(Debug, Clone)]
pub struct SyncResult {
    pub success: bool,
    pub conflicts_resolved: usize,
    pub changes_applied: usize,
    pub sync_time: DateTime<Utc>,
}

// Default conflict resolver
pub struct LastWriterWinsResolver;

#[async_trait::async_trait]
impl ConflictResolver for LastWriterWinsResolver {
    async fn resolve(
        &self,
        _conflict: &SyncConflict,
        local_data: &Invoice,
        remote_data: &Invoice,
    ) -> Result<Resolution, SyncError> {
        // Simple last-writer-wins strategy
        if local_data.updated_at > remote_data.updated_at {
            Ok(Resolution::UseLocal)
        } else {
            Ok(Resolution::UseRemote)
        }
    }
}

// Manual conflict resolver for UI integration
pub struct ManualConflictResolver;

#[async_trait::async_trait]
impl ConflictResolver for ManualConflictResolver {
    async fn resolve(
        &self,
        conflict: &SyncConflict,
        _local_data: &Invoice,
        _remote_data: &Invoice,
    ) -> Result<Resolution, SyncError> {
        // This resolver would typically interact with the UI
        // For now, return an error indicating manual resolution needed
        Err(SyncError::ConflictResolutionError(
            format!("Manual resolution required for invoice {}", conflict.entity_id)
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_core::invoicing::model::{Invoice, Customer};
    use uuid::Uuid;
    
    // Mock implementations for testing
    struct MockSyncRepo;
    
    #[async_trait::async_trait]
    impl InvoiceSyncRepository for MockSyncRepo {
        async fn get_changes_since(&self, _since: DateTime<Utc>) -> Result<SyncChanges, RepositoryError> {
            Ok(SyncChanges {
                invoices: Vec::new(),
                customers: Vec::new(),
                sync_timestamp: Utc::now(),
            })
        }
        
        async fn apply_changes(&self, _changes: SyncChanges) -> Result<(), RepositoryError> {
            Ok(())
        }
        
        async fn resolve_conflicts(&self, _conflicts: Vec<SyncConflict>) -> Result<(), RepositoryError> {
            Ok(())
        }
        
        async fn get_last_sync_time(&self) -> Result<DateTime<Utc>, RepositoryError> {
            Ok(Utc::now())
        }
        
        async fn update_sync_time(&self, _sync_time: DateTime<Utc>) -> Result<(), RepositoryError> {
            Ok(())
        }
    }
}