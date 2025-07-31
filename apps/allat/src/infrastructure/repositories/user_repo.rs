use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::auth::User;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum UserRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("User not found")]
    NotFound,
    #[error("Karma limit exceeded: max 10000")]
    KarmaLimitExceeded,
}

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError>;
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserRepositoryError>;
    async fn update_karma(&self, user_id: Uuid, delta: i32) -> Result<(), UserRepositoryError>;
}

pub struct PgUserRepository {
    pool: PgPool,
}

impl PgUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn save(&self, user: &User) -> Result<(), UserRepositoryError> {
        // In a real implementation, we would save the user to the database
        // For now, we'll just return Ok(())
        Ok(())
    }
    
    async fn find_by_id(&self, user_id: Uuid) -> Result<Option<User>, UserRepositoryError> {
        // In a real implementation, we would fetch the user from the database
        // For now, we'll just return None
        Ok(None)
    }
    
    async fn update_karma(&self, user_id: Uuid, delta: i32) -> Result<(), UserRepositoryError> {
        // Update user karma in the database
        let current_karma: i32 = sqlx::query_scalar!(
            "SELECT karma FROM users WHERE id = $1",
            user_id
        )
        .fetch_optional(&self.pool)
        .await?
        .unwrap_or(0);
        
        let new_karma = current_karma + delta;
        
        // Check for karma overflow
        if new_karma > 10000 {
            return Err(UserRepositoryError::KarmaLimitExceeded);
        }
        
        sqlx::query!(
            "UPDATE users SET karma = $1 WHERE id = $2",
            new_karma,
            user_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}