//! Sled database adapter for edge device storage.

use sled::Db;
use serde::{Deserialize, Serialize};
use crate::{
    domain::{
        consent::{ConsentProfile, Domain},
        audit::AuditEvent,
        errors::ConsentError,
    },
    application::service::ConsentStorage,
};
use async_trait::async_trait;

/// Sled storage adapter
pub struct SledAdapter {
    db: Db,
}

impl SledAdapter {
    /// Create a new Sled adapter
    pub fn new(db: Db) -> Self {
        Self { db }
    }
    
    /// Get the key for a consent profile
    fn consent_key(user_id: &str, domain: &Domain) -> String {
        format!("consent:{}:{:?}", user_id, domain)
    }
    
    /// Get the key prefix for audit events
    fn audit_prefix(user_id: &str) -> String {
        format!("audit:{}", user_id)
    }
    
    /// Get the key for an audit event
    fn audit_key(user_id: &str, event_id: &str) -> String {
        format!("audit:{}:{}", user_id, event_id)
    }
}

#[async_trait]
impl ConsentStorage for SledAdapter {
    async fn get_consent_profile(&self, user_id: &str, domain: &Domain) -> Result<Option<ConsentProfile>, ConsentError> {
        let key = Self::consent_key(user_id, domain);
        
        match self.db.get(key).map_err(|e| ConsentError::StorageError(e.to_string()))? {
            Some(bytes) => {
                let profile: ConsentProfile = serde_json::from_slice(&bytes)
                    .map_err(|e| ConsentError::StorageError(format!("Deserialization error: {}", e)))?;
                Ok(Some(profile))
            },
            None => Ok(None),
        }
    }
    
    async fn save_consent_profile(&self, profile: &ConsentProfile) -> Result<(), ConsentError> {
        let key = Self::consent_key(&profile.user_id, &profile.domain);
        let bytes = serde_json::to_vec(profile)
            .map_err(|e| ConsentError::StorageError(format!("Serialization error: {}", e)))?;
            
        self.db.insert(key, bytes)
            .map_err(|e| ConsentError::StorageError(e.to_string()))?;
            
        Ok(())
    }
    
    async fn revoke_domain(&self, user_id: &str, domain: &Domain) -> Result<(), ConsentError> {
        let key = Self::consent_key(user_id, domain);
        self.db.remove(key)
            .map_err(|e| ConsentError::StorageError(e.to_string()))?;
        Ok(())
    }
    
    async fn get_audit_events(&self, user_id: &str) -> Result<Vec<AuditEvent>, ConsentError> {
        let prefix = Self::audit_prefix(user_id);
        let mut events = Vec::new();
        
        for result in self.db.scan_prefix(&prefix) {
            let (_, bytes) = result.map_err(|e| ConsentError::StorageError(e.to_string()))?;
            let event: AuditEvent = serde_json::from_slice(&bytes)
                .map_err(|e| ConsentError::StorageError(format!("Deserialization error: {}", e)))?;
            events.push(event);
        }
        
        // Sort by timestamp
        events.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
        
        Ok(events)
    }
    
    async fn save_audit_event(&self, event: &AuditEvent) -> Result<(), ConsentError> {
        let key = Self::audit_key(&event.user_id, &event.id);
        let bytes = serde_json::to_vec(event)
            .map_err(|e| ConsentError::StorageError(format!("Serialization error: {}", e)))?;
            
        self.db.insert(key, bytes)
            .map_err(|e| ConsentError::StorageError(e.to_string()))?;
            
        Ok(())
    }
}