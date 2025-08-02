//! PostgreSQL implementation of MembershipRepository

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::Membership;
use crate::application::repository::{MembershipRepository, RepositoryError};
use crate::application::ApplicationError;

pub struct PostgresMembershipRepository {
    pool: PgPool,
}

impl PostgresMembershipRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MembershipRepository for PostgresMembershipRepository {
    async fn save(&self, membership: &Membership) -> Result<(), ApplicationError> {
        sqlx::query!(
            r#"
            INSERT INTO user_shares (user_id, campaign_id, granted_at)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, campaign_id) DO UPDATE SET
                granted_at = $3
            "#,
            membership.user_id,
            membership.campaign_id,
            membership.join_date,
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn user_has_membership(&self, user_id: Uuid) -> Result<bool, ApplicationError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_shares 
                WHERE user_id = $1
            )
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(exists.unwrap_or(false))
    }
    
    async fn get_user_membership(&self, user_id: Uuid) -> Result<Option<Membership>, ApplicationError> {
        let row = sqlx::query!(
            r#"
            SELECT user_id, campaign_id, granted_at
            FROM user_shares
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        match row {
            Some(row) => Ok(Some(Membership {
                user_id: row.user_id,
                campaign_id: row.campaign_id,
                join_date: row.granted_at,
            })),
            None => Ok(None),
        }
    }
    
    async fn delete(&self, user_id: Uuid, campaign_id: Uuid) -> Result<(), ApplicationError> {
        sqlx::query!(
            "DELETE FROM user_shares WHERE user_id = $1 AND campaign_id = $2",
            user_id,
            campaign_id
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn exists_for_campaign(&self, campaign_id: Uuid) -> Result<bool, ApplicationError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM user_shares
                WHERE campaign_id = $1
            )
            "#,
            campaign_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(exists.unwrap_or(false))
    }
}