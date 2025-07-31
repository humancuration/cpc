//! Sled storage adapter for OAuth tokens

use async_trait::async_trait;
use uuid::Uuid;
use sled::Db;
use crate::domain::AuthError;
use crate::infrastructure::storage::StorageAdapter;
use tracing::{info, debug, error};

/// Sled storage adapter for OAuth tokens
pub struct SledStorageAdapter {
    db: Db,
}

impl SledStorageAdapter {
    /// Create a new Sled storage adapter
    pub fn new(db: Db) -> Self {
        Self { db }
    }
    
    /// Generate a key for storing tokens
    fn generate_key(&self, user_id: Uuid, provider: &str) -> String {
        format!("oauth_token:{}:{}", user_id, provider)
    }
}

#[async_trait]
impl StorageAdapter for SledStorageAdapter {
    async fn store_token(
        &self,
        user_id: Uuid,
        provider: &str,
        token: &str,
    ) -> Result<(), AuthError> {
        debug!(user_id = %user_id, provider = provider, "Storing OAuth token in Sled");
        
        let key = self.generate_key(user_id, provider);
        self.db.insert(key.as_bytes(), token.as_bytes())
            .map_err(|e| AuthError::StorageError(format!("Failed to store token: {}", e)))?;
        
        self.db.flush_async().await
            .map_err(|e| AuthError::StorageError(format!("Failed to flush database: {}", e)))?;
        
        info!(user_id = %user_id, provider = provider, "OAuth token stored successfully in Sled");
        
        Ok(())
    }
    
    async fn get_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<String, AuthError> {
        debug!(user_id = %user_id, provider = provider, "Retrieving OAuth token from Sled");
        
        let key = self.generate_key(user_id, provider);
        let result = self.db.get(key.as_bytes())
            .map_err(|e| AuthError::StorageError(format!("Failed to retrieve token: {}", e)))?;
        
        match result {
            Some(value) => {
                let token = String::from_utf8(value.to_vec())
                    .map_err(|e| AuthError::StorageError(format!("Failed to decode token: {}", e)))?;
                info!(user_id = %user_id, provider = provider, "OAuth token retrieved successfully from Sled");
                Ok(token)
            }
            None => Err(AuthError::StorageError("Token not found".to_string())),
        }
    }
    
    async fn delete_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<(), AuthError> {
        debug!(user_id = %user_id, provider = provider, "Deleting OAuth token from Sled");
        
        let key = self.generate_key(user_id, provider);
        self.db.remove(key.as_bytes())
            .map_err(|e| AuthError::StorageError(format!("Failed to delete token: {}", e)))?;
        
        self.db.flush_async().await
            .map_err(|e| AuthError::StorageError(format!("Failed to flush database: {}", e)))?;
        
        info!(user_id = %user_id, provider = provider, "OAuth token deleted successfully from Sled");
        
        Ok(())
    }
}