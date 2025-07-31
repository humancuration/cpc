//! Token service for managing OAuth tokens

use std::sync::Arc;
use uuid::Uuid;
use crate::domain::{
    OAuthProvider, OAuthToken, EncryptedToken, AuthError, AuthConfig
};
use crate::infrastructure::storage::StorageAdapter;
use tracing::{info, debug, error};

/// Token service for managing OAuth tokens
pub struct TokenService {
    /// Storage adapter for persisting tokens
    storage: Arc<dyn StorageAdapter>,
    
    /// Authentication configuration
    config: AuthConfig,
}

impl TokenService {
    /// Create a new token service
    pub fn new(storage: Arc<dyn StorageAdapter>, config: AuthConfig) -> Self {
        Self { storage, config }
    }
    
    /// Store an OAuth token for a user
    pub async fn store_token(
        &self,
        user_id: Uuid,
        token: OAuthToken,
    ) -> Result<(), AuthError> {
        debug!(user_id = %user_id, provider = %token.provider, "Storing OAuth token");
        
        // Encrypt the token before storing
        let encrypted_token = token.encrypt(&self.config.encryption_key)?;
        
        // Encode for storage
        let encoded_token = encrypted_token.encode()?;
        
        // Store in the storage adapter
        self.storage.store_token(user_id, &token.provider, &encoded_token).await?;
        
        info!(user_id = %user_id, provider = %token.provider, "OAuth token stored successfully");
        
        Ok(())
    }
    
    /// Get an OAuth token for a user
    pub async fn get_token(
        &self,
        user_id: Uuid,
        provider: &OAuthProvider,
    ) -> Result<OAuthToken, AuthError> {
        debug!(user_id = %user_id, provider = %provider, "Retrieving OAuth token");
        
        // Retrieve from storage
        let encoded_token = self.storage.get_token(user_id, provider.as_str()).await?;
        
        // Decode the token
        let encrypted_token = EncryptedToken::decode(&encoded_token)?;
        
        // Decrypt the token
        let token = OAuthToken::decrypt(&encrypted_token, &self.config.encryption_key)?;
        
        // Check if token is expired
        if token.is_expired() {
            return Err(AuthError::TokenExpired);
        }
        
        info!(user_id = %user_id, provider = %provider, "OAuth token retrieved successfully");
        
        Ok(token)
    }
    
    /// Delete an OAuth token for a user
    pub async fn delete_token(
        &self,
        user_id: Uuid,
        provider: &OAuthProvider,
    ) -> Result<(), AuthError> {
        debug!(user_id = %user_id, provider = %provider, "Deleting OAuth token");
        
        self.storage.delete_token(user_id, provider.as_str()).await?;
        
        info!(user_id = %user_id, provider = %provider, "OAuth token deleted successfully");
        
        Ok(())
    }
    
    /// Check if a user has a valid token for a provider
    pub async fn has_valid_token(
        &self,
        user_id: Uuid,
        provider: &OAuthProvider,
    ) -> Result<bool, AuthError> {
        match self.get_token(user_id, provider).await {
            Ok(_) => Ok(true),
            Err(AuthError::TokenExpired) | Err(AuthError::StorageError(_)) => Ok(false),
            Err(e) => Err(e),
        }
    }
}