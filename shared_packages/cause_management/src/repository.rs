//! Repository for Cause Management
//!
//! This module provides the database access layer for managing causes
//! within the CPC platform.

use crate::models::{Cause, CauseError, ListCausesRequest, ListCausesResponse};
use sqlx::PgPool;
use uuid::Uuid;
use rust_decimal::Decimal;
use tracing::info;

/// Repository trait for cause management
#[async_trait::async_trait]
pub trait CauseRepository {
    /// Create a new cause
    async fn create_cause(&self, cause: &Cause) -> Result<(), CauseError>;
    
    /// Find a cause by ID
    async fn find_cause_by_id(&self, id: Uuid) -> Result<Option<Cause>, CauseError>;
    
    /// Update a cause
    async fn update_cause(&self, cause: &Cause) -> Result<(), CauseError>;
    
    /// Delete a cause
    async fn delete_cause(&self, id: Uuid) -> Result<(), CauseError>;
    
    /// List causes with pagination
    async fn list_causes(&self, request: ListCausesRequest) -> Result<ListCausesResponse, CauseError>;
    
    /// Add donation to a cause
    async fn add_donation_to_cause(&self, cause_id: Uuid, amount: Decimal) -> Result<(), CauseError>;
}

/// PostgreSQL implementation of the CauseRepository
#[derive(Debug, Clone)]
pub struct PostgresCauseRepository {
    pool: PgPool,
}

impl PostgresCauseRepository {
    /// Create a new PostgresCauseRepository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl CauseRepository for PostgresCauseRepository {
    async fn create_cause(&self, cause: &Cause) -> Result<(), CauseError> {
        info!("Creating cause: {}", cause.id);
        
        sqlx::query!(
            r#"
            INSERT INTO causes (id, name, description, image_url, total_donations, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            cause.id,
            cause.name,
            cause.description,
            cause.image_url,
            cause.total_donations,
            cause.created_at,
            cause.updated_at
        )
        .execute(&self.pool)
        .await
        .map_err(CauseError::from)?;
        
        Ok(())
    }
    
    async fn find_cause_by_id(&self, id: Uuid) -> Result<Option<Cause>, CauseError> {
        info!("Finding cause by ID: {}", id);
        
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, image_url, total_donations, created_at, updated_at
            FROM causes
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(CauseError::from)?;
        
        match row {
            Some(row) => {
                let cause = Cause {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    image_url: row.image_url,
                    total_donations: row.total_donations,
                    created_at: row.created_at,
                    updated_at: row.updated_at,
                };
                Ok(Some(cause))
            }
            None => Ok(None),
        }
    }
    
    async fn update_cause(&self, cause: &Cause) -> Result<(), CauseError> {
        info!("Updating cause: {}", cause.id);
        
        let rows_affected = sqlx::query!(
            r#"
            UPDATE causes
            SET name = $1, description = $2, image_url = $3, total_donations = $4, updated_at = $5
            WHERE id = $6
            "#,
            cause.name,
            cause.description,
            cause.image_url,
            cause.total_donations,
            cause.updated_at,
            cause.id
        )
        .execute(&self.pool)
        .await
        .map_err(CauseError::from)?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(CauseError::CauseNotFound(cause.id));
        }
        
        Ok(())
    }
    
    async fn delete_cause(&self, id: Uuid) -> Result<(), CauseError> {
        info!("Deleting cause: {}", id);
        
        let rows_affected = sqlx::query!(
            "DELETE FROM causes WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await
        .map_err(CauseError::from)?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(CauseError::CauseNotFound(id));
        }
        
        Ok(())
    }
    
    async fn list_causes(&self, request: ListCausesRequest) -> Result<ListCausesResponse, CauseError> {
        info!("Listing causes with limit: {:?}, offset: {:?}", request.limit, request.offset);
        
        let limit = request.limit.unwrap_or(50).max(1).min(1000) as i64;
        let offset = request.offset.unwrap_or(0).max(0) as i64;
        
        let causes = sqlx::query!(
            r#"
            SELECT id, name, description, image_url, total_donations, created_at, updated_at
            FROM causes
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await
        .map_err(CauseError::from)?
        .into_iter()
        .map(|row| Cause {
            id: row.id,
            name: row.name,
            description: row.description,
            image_url: row.image_url,
            total_donations: row.total_donations,
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
        .collect();
        
        let total_count: i64 = sqlx::query_scalar!("SELECT COUNT(*) FROM causes")
            .fetch_one(&self.pool)
            .await
            .map_err(CauseError::from)?
            .unwrap_or(0);
        
        Ok(ListCausesResponse {
            causes,
            total_count: total_count as i32,
        })
    }
    
    async fn add_donation_to_cause(&self, cause_id: Uuid, amount: Decimal) -> Result<(), CauseError> {
        info!("Adding donation of {} to cause: {}", amount, cause_id);
        
        let rows_affected = sqlx::query!(
            r#"
            UPDATE causes
            SET total_donations = total_donations + $1, updated_at = NOW()
            WHERE id = $2
            "#,
            amount,
            cause_id
        )
        .execute(&self.pool)
        .await
        .map_err(CauseError::from)?
        .rows_affected();
        
        if rows_affected == 0 {
            return Err(CauseError::CauseNotFound(cause_id));
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::PgPool;
    use std::env;
    
    // Note: These tests require a running PostgreSQL database
    // They are meant to be run in an integration test environment
    
    #[ignore] // Integration test - requires database
    #[tokio::test]
    async fn test_cause_repository() -> Result<(), Box<dyn std::error::Error>> {
        // This is a placeholder for integration tests
        // In a real implementation, you would:
        // 1. Set up a test database
        // 2. Create the causes table
        // 3. Run the repository operations
        // 4. Verify the results
        Ok(())
    }
}