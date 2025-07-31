//! Storage adapters for OAuth tokens

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::AuthError;

/// Storage adapter trait for persisting OAuth tokens
#[async_trait]
pub trait StorageAdapter: Send + Sync {
    /// Store an encrypted token for a user
    async fn store_token(
        &self,
        user_id: Uuid,
        provider: &str,
        token: &str,
    ) -> Result<(), AuthError>;
    
    /// Retrieve an encrypted token for a user
    async fn get_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<String, AuthError>;
    
    /// Delete a token for a user
    async fn delete_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<(), AuthError>;
}

#[cfg(feature = "sled_storage")]
pub mod sled_storage;

#[cfg(feature = "postgres_storage")]
pub mod postgres_storage;