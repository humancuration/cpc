use async_trait::async_trait;
use sqlx::{PgPool, Error};
use uuid::Uuid;
use cpc_core::models::social::relationship::Relationship;
use cpc_core::repositories::social::relationship_repository::RelationshipRepository;

pub struct RelationshipRepositoryImpl {
    pool: PgPool,
}

impl RelationshipRepositoryImpl {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RelationshipRepository for RelationshipRepositoryImpl {
    async fn follow(&self, user_id: Uuid, target_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
            INSERT INTO relationships (follower_id, followed_id)
            VALUES ($1, $2)
            ON CONFLICT (follower_id, followed_id) DO NOTHING
            "#,
            user_id,
            target_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn unfollow(&self, user_id: Uuid, target_id: Uuid) -> Result<(), Error> {
        sqlx::query!(
            r#"
            DELETE FROM relationships
            WHERE follower_id = $1 AND followed_id = $2
            "#,
            user_id,
            target_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    async fn get_followers(&self, user_id: Uuid) -> Result<Vec<Uuid>, Error> {
        let followers = sqlx::query!(
            r#"
            SELECT follower_id FROM relationships
            WHERE followed_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|r| r.follower_id)
        .collect();
        
        Ok(followers)
    }

    async fn get_following(&self, user_id: Uuid) -> Result<Vec<Uuid>, Error> {
        let following = sqlx::query!(
            r#"
            SELECT followed_id FROM relationships
            WHERE follower_id = $1
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|r| r.followed_id)
        .collect();
        
        Ok(following)
    }
}