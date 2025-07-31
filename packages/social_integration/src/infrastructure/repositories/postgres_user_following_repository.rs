//! PostgreSQL repository for user following relationships

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;

/// Repository trait for user following persistence
#[async_trait]
pub trait UserFollowingRepository: Send + Sync {
    /// Follow a user
    async fn follow(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Unfollow a user
    async fn unfollow(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Get all users that a user is following
    async fn get_following(&self, follower_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>>;
}

/// PostgreSQL repository for user following relationships
#[derive(Debug)]
pub struct PostgresUserFollowingRepository {
    pool: PgPool,
}

impl PostgresUserFollowingRepository {
    /// Create a new PostgreSQL user following repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserFollowingRepository for PostgresUserFollowingRepository {
    async fn follow(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO user_following (follower_id, followed_id, created_at)
            VALUES ($1, $2, NOW())
            ON CONFLICT (follower_id, followed_id) DO NOTHING
            "#,
            follower_id,
            followed_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn unfollow(&self, follower_id: Uuid, followed_id: Uuid) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            DELETE FROM user_following
            WHERE follower_id = $1 AND followed_id = $2
            "#,
            follower_id,
            followed_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_following(&self, follower_id: Uuid) -> Result<Vec<Uuid>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT followed_id
            FROM user_following
            WHERE follower_id = $1
            ORDER BY created_at DESC
            "#,
            follower_id
        )
        .fetch_all(&self.pool)
        .await?;
        
        let followed_ids: Vec<Uuid> = rows.into_iter().map(|row| row.followed_id).collect();
        
        Ok(followed_ids)
    }
}