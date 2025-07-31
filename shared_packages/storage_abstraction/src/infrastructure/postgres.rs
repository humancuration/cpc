//! PostgreSQL implementation for storage abstraction
//! 
//! This module provides a PostgreSQL-based implementation of the DataStore trait.

use async_trait::async_trait;
use sqlx::{PgPool, Row};
use crate::domain::traits::{DataStore, StorageError};

/// PostgreSQL-based storage implementation
pub struct PostgresStore {
    pool: PgPool,
}

impl PostgresStore {
    /// Create a new PostgreSQL store
    pub async fn new(database_url: &str) -> Result<Self, StorageError> {
        let pool = PgPool::connect(database_url)
            .await
            .map_err(|e| StorageError::ConnectionError(format!("Failed to connect to PostgreSQL: {}", e)))?;
        
        // Create the table if it doesn't exist
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS storage_data (
                key TEXT PRIMARY KEY,
                value BYTEA NOT NULL,
                created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
                updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
            )"
        )
        .execute(&pool)
        .await
        .map_err(|e| StorageError::OperationFailed(format!("Failed to create table: {}", e)))?;
        
        Ok(Self { pool })
    }
}

#[async_trait]
impl DataStore for PostgresStore {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, StorageError> {
        let result = sqlx::query("SELECT value FROM storage_data WHERE key = $1")
            .bind(key)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| StorageError::OperationFailed(format!("Failed to get key: {}", e)))?;
        
        match result {
            Some(row) => {
                let value: Vec<u8> = row.try_get("value")
                    .map_err(|e| StorageError::SerializationError(format!("Failed to deserialize value: {}", e)))?;
                Ok(Some(value))
            },
            None => Ok(None),
        }
    }
    
    async fn set(&self, key: &str, value: Vec<u8>) -> Result<(), StorageError> {
        sqlx::query(
            "INSERT INTO storage_data (key, value, updated_at)
             VALUES ($1, $2, NOW())
             ON CONFLICT (key) DO UPDATE
             SET value = EXCLUDED.value, updated_at = NOW()"
        )
        .bind(key)
        .bind(&value)
        .execute(&self.pool)
        .await
        .map_err(|e| StorageError::OperationFailed(format!("Failed to set key: {}", e)))?;
        
        Ok(())
    }
    
    async fn delete(&self, key: &str) -> Result<(), StorageError> {
        sqlx::query("DELETE FROM storage_data WHERE key = $1")
            .bind(key)
            .execute(&self.pool)
            .await
            .map_err(|e| StorageError::OperationFailed(format!("Failed to delete key: {}", e)))?;
        
        Ok(())
    }
}