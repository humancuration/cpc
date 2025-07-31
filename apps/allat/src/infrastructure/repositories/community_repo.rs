use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::community::Community;
use uuid::Uuid;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommunityRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Community not found")]
    NotFound,
}

#[async_trait]
pub trait CommunityRepository: Send + Sync {
    async fn create(&self, community: &Community) -> Result<(), CommunityRepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Community>, CommunityRepositoryError>;
    async fn find_by_name(&self, name: &str) -> Result<Option<Community>, CommunityRepositoryError>;
    async fn update(&self, community: &Community) -> Result<(), CommunityRepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), CommunityRepositoryError>;
}

pub struct PgCommunityRepository {
    pool: PgPool,
}

impl PgCommunityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommunityRepository for PgCommunityRepository {
    async fn create(&self, community: &Community) -> Result<(), CommunityRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO communities (id, name, description, rules, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            community.id,
            community.name,
            community.description,
            &community.rules,
            community.created_at
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Community>, CommunityRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, rules, created_at
            FROM communities
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let community = Community {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    rules: row.rules,
                    created_at: row.created_at,
                };
                Ok(Some(community))
            }
            None => Ok(None),
        }
    }

    async fn find_by_name(&self, name: &str) -> Result<Option<Community>, CommunityRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, rules, created_at
            FROM communities
            WHERE name = $1
            "#,
            name
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => {
                let community = Community {
                    id: row.id,
                    name: row.name,
                    description: row.description,
                    rules: row.rules,
                    created_at: row.created_at,
                };
                Ok(Some(community))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, community: &Community) -> Result<(), CommunityRepositoryError> {
        sqlx::query!(
            r#"
            UPDATE communities
            SET name = $1, description = $2, rules = $3
            WHERE id = $4
            "#,
            community.name,
            community.description,
            &community.rules,
            community.id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), CommunityRepositoryError> {
        sqlx::query!(
            r#"
            DELETE FROM communities
            WHERE id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }
}