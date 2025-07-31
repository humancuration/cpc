//! Postgres storage adapter for OAuth tokens

use async_trait::async_trait;
use uuid::Uuid;
use sqlx::PgPool;
use crate::domain::AuthError;
use crate::infrastructure::storage::StorageAdapter;
use tracing::{info, debug, error};

/// Postgres storage adapter for OAuth tokens
pub struct PostgresStorageAdapter {
    pool: PgPool,
}

impl PostgresStorageAdapter {
    /// Create a new Postgres storage adapter
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl StorageAdapter for PostgresStorageAdapter {
    async fn store_token(
        &self,
        user_id: Uuid,
        provider: &str,
        token: &str,
    ) -> Result<(), AuthError> {
        debug!(user_id = %user_id, provider = provider, "Storing OAuth token in Postgres");
        
        sqlx::query!(
            "INSERT INTO oauth_tokens (user_id, provider, token_data) 
             VALUES ($1, $2, $3)
             ON CONFLICT (user_id, provider) 
             DO UPDATE SET token_data = $3, updated_at = NOW()",
            user_id,
            provider,
            token
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::StorageError(format!("Failed to store token: {}", e)))?;
        
        info!(user_id = %user_id, provider = provider, "OAuth token stored successfully in Postgres");
        
        Ok(())
    }
    
    async fn get_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<String, AuthError> {
        debug!(user_id = %user_id, provider = provider, "Retrieving OAuth token from Postgres");
        
        let row = sqlx::query!(
            "SELECT token_data FROM oauth_tokens 
             WHERE user_id = $1 AND provider = $2",
            user_id,
            provider
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| AuthError::StorageError(format!("Failed to retrieve token: {}", e)))?;
        
        match row {
            Some(record) => {
                info!(user_id = %user_id, provider = provider, "OAuth token retrieved successfully from Postgres");
                Ok(record.token_data)
            }
            None => Err(AuthError::StorageError("Token not found".to_string())),
        }
    }
    
    async fn delete_token(
        &self,
        user_id: Uuid,
        provider: &str,
    ) -> Result<(), AuthError> {
        debug!(user_id = %user_id, provider = provider, "Deleting OAuth token from Postgres");
        
        sqlx::query!(
            "DELETE FROM oauth_tokens 
             WHERE user_id = $1 AND provider = $2",
            user_id,
            provider
        )
        .execute(&self.pool)
        .await
        .map_err(|e| AuthError::StorageError(format!("Failed to delete token: {}", e)))?;
        
        info!(user_id = %user_id, provider = provider, "OAuth token deleted successfully from Postgres");
        
        Ok(())
    }
}