use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::social::relationship::Relationship;

#[async_trait]
pub trait RelationshipRepository: Send + Sync {
    async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<Relationship, sqlx::Error>;
    async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<u64, sqlx::Error>;
    async fn get_followed_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn get_follower_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool, sqlx::Error>;
}

pub struct SqliteRelationshipRepository {
    pool: SqlitePool,
}

impl SqliteRelationshipRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RelationshipRepository for SqliteRelationshipRepository {
    async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<Relationship, sqlx::Error> {
        let relationship = sqlx::query_as!(
            Relationship,
            r#"
            INSERT INTO relationships (id, follower_id, followed_id)
            VALUES ($1, $2, $3)
            RETURNING id, follower_id, followed_id, created_at
            "#,
            Uuid::new_v4(),
            follower_id,
            followed_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(relationship)
    }

    async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM relationships WHERE follower_id = $1 AND followed_id = $2",
            follower_id,
            followed_id
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    async fn get_followed_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = sqlx::query!(
            "SELECT followed_id FROM relationships WHERE follower_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|rec| rec.followed_id)
        .collect();
        Ok(ids)
    }

    async fn get_follower_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = sqlx::query!(
            "SELECT follower_id FROM relationships WHERE followed_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|rec| rec.follower_id)
        .collect();
        Ok(ids)
    }

    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool, sqlx::Error> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM relationships WHERE follower_id = $1 AND followed_id = $2",
            follower_id,
            followed_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        Ok(count > 0)
    }
}