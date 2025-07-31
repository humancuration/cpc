//! Sled/PostgreSQL adapters
//! 
//! This module provides storage adapters for different backend systems.

use async_trait::async_trait;
use crate::domain::{
    event::AuditEvent,
    AuditError,
};
use crate::application::service::{AuditStorage, AuditQuery};
use storage_abstraction::{DataStore, StorageError};
use std::sync::Arc;

/// Sled-based audit storage implementation
pub struct SledAuditStorage {
    storage: Arc<dyn DataStore>,
}

impl SledAuditStorage {
    /// Create a new Sled audit storage
    pub fn new(storage: Arc<dyn DataStore>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl AuditStorage for SledAuditStorage {
    async fn store_event(&self, event: &AuditEvent) -> Result<(), AuditError> {
        let key = format!("audit:{}:{}", event.domain, event.event_id);
        let value = serde_json::to_vec(event)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;
        
        self.storage.set(&key, value)
            .await
            .map_err(|e| AuditError::StorageError(e.to_string()))
    }
    
    async fn get_events(&self, query: AuditQuery) -> Result<Vec<AuditEvent>, AuditError> {
        // This is a simplified implementation
        // In a real implementation, we would have a more sophisticated query mechanism
        todo!("Implement audit event querying for Sled storage")
    }
}

/// PostgreSQL-based audit storage implementation
pub struct PostgresAuditStorage {
    storage: Arc<dyn DataStore>,
}

impl PostgresAuditStorage {
    /// Create a new PostgreSQL audit storage
    pub fn new(storage: Arc<dyn DataStore>) -> Self {
        Self { storage }
    }
}

#[async_trait]
impl AuditStorage for PostgresAuditStorage {
    async fn store_event(&self, event: &AuditEvent) -> Result<(), AuditError> {
        let key = format!("audit:{}:{}", event.domain, event.event_id);
        let value = serde_json::to_vec(event)
            .map_err(|e| AuditError::SerializationError(e.to_string()))?;
        
        self.storage.set(&key, value)
            .await
            .map_err(|e| AuditError::StorageError(e.to_string()))
    }
    
    async fn get_events(&self, query: AuditQuery) -> Result<Vec<AuditEvent>, AuditError> {
        // This is a simplified implementation
        // In a real implementation, we would have a more sophisticated query mechanism
        todo!("Implement audit event querying for PostgreSQL storage")
    }
}