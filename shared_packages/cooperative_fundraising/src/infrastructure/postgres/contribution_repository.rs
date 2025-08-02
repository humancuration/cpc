//! PostgreSQL implementation of ContributionRepository

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use crate::domain::{Contribution, VerificationStatus};
use crate::application::repository::{ContributionRepository, PaginatedContributions, RepositoryError};
use crate::application::ApplicationError;
use rust_decimal::Decimal;

pub struct PostgresContributionRepository {
    pool: PgPool,
}

impl PostgresContributionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ContributionRepository for PostgresContributionRepository {
    async fn save(&self, contribution: &Contribution) -> Result<(), ApplicationError> {
        sqlx::query!(
            r#"
            INSERT INTO contributions (
                id, campaign_id, user_id, created_at,
                cpay_transaction_id, amount, currency,
                opportunity_id, hours, verification_status
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            ON CONFLICT (id) DO UPDATE SET
                campaign_id = $2,
                user_id = $3,
                created_at = $4,
                cpay_transaction_id = $5,
                amount = $6,
                currency = $7,
                opportunity_id = $8,
                hours = $9,
                verification_status = $10
            "#,
            contribution.id,
            contribution.campaign_id,
            contribution.user_id,
            contribution.created_at,
            contribution.cpay_transaction_id,
            contribution.amount,
            contribution.currency,
            contribution.opportunity_id,
            contribution.hours,
            contribution.verification_status.as_ref().map(|s| s.to_string()),
        )
        .execute(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(())
    }
    
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Contribution>, ApplicationError> {
        let row = sqlx::query!(
            r#"
            SELECT id, campaign_id, user_id, created_at,
                   cpay_transaction_id, amount, currency,
                   opportunity_id, hours, verification_status
            FROM contributions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        let row = match row {
            Some(row) => row,
            None => return Ok(None),
        };
        
        // Parse verification status if present
        let verification_status = row.verification_status.as_ref().map(|status_str| {
            match status_str.as_str() {
                "pending" => VerificationStatus::Pending,
                "verified" => VerificationStatus::Verified,
                "disputed" => VerificationStatus::Disputed,
                "rejected" => VerificationStatus::Rejected,
                _ => VerificationStatus::Pending, // Default to pending if unknown
            }
        });
        
        Ok(Some(Contribution {
            id: row.id,
            campaign_id: row.campaign_id,
            user_id: row.user_id,
            created_at: row.created_at,
            cpay_transaction_id: row.cpay_transaction_id,
            amount: row.amount,
            currency: row.currency,
            opportunity_id: row.opportunity_id,
            hours: row.hours,
            verification_status,
        }))
    }
    
    async fn list_by_campaign(
        &self,
        campaign_id: Uuid,
        user_id: Option<Uuid>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<PaginatedContributions, ApplicationError> {
        // Build the query dynamically
        let mut query = "SELECT id, campaign_id, user_id, created_at, cpay_transaction_id, amount, currency, opportunity_id, hours, verification_status FROM contributions WHERE campaign_id = $1".to_string();
        let mut conditions = vec![];
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send>> = vec![Box::new(campaign_id)];
        
        if let Some(user_id) = user_id {
            conditions.push(format!("user_id = ${}", params.len() + 1));
            params.push(Box::new(user_id));
        }
        
        if !conditions.is_empty() {
            query.push_str(&format!(" AND {}", conditions.join(" AND ")));
        }
        
        query.push_str(" ORDER BY created_at DESC");
        
        // Add limit and offset if provided
        if let Some(limit) = limit {
            query.push_str(&format!(" LIMIT ${}", params.len() + 1));
            params.push(Box::new(limit));
        }
        
        if let Some(offset) = offset {
            query.push_str(&format!(" OFFSET ${}", params.len() + 1));
            params.push(Box::new(offset));
        }
        
        // Execute the query
        let mut query_builder = sqlx::query(&query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        
        let rows = query_builder
            .fetch_all(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        
        let contributions: Vec<Contribution> = rows
            .into_iter()
            .map(|row| {
                let id: Uuid = row.get("id");
                let verification_status_str: Option<String> = row.get("verification_status");
                
                // Parse verification status if present
                let verification_status = verification_status_str.as_ref().map(|status_str| {
                    match status_str.as_str() {
                        "pending" => VerificationStatus::Pending,
                        "verified" => VerificationStatus::Verified,
                        "disputed" => VerificationStatus::Disputed,
                        "rejected" => VerificationStatus::Rejected,
                        _ => VerificationStatus::Pending, // Default to pending if unknown
                    }
                });
                
                Contribution {
                    id,
                    campaign_id: row.get("campaign_id"),
                    user_id: row.get("user_id"),
                    created_at: row.get("created_at"),
                    cpay_transaction_id: row.get("cpay_transaction_id"),
                    amount: row.get("amount"),
                    currency: row.get("currency"),
                    opportunity_id: row.get("opportunity_id"),
                    hours: row.get("hours"),
                    verification_status,
                }
            })
            .collect();
        
        // Get total count
        let mut count_query = "SELECT COUNT(*) as count FROM contributions WHERE campaign_id = $1".to_string();
        let mut count_params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send>> = vec![Box::new(campaign_id)];
        
        if let Some(user_id) = user_id {
            count_query.push_str(&format!(" AND user_id = ${}", count_params.len() + 1));
            count_params.push(Box::new(user_id));
        }
        
        let mut count_query_builder = sqlx::query_scalar(&count_query);
        for param in count_params {
            count_query_builder = count_query_builder.bind(param);
        }
        
        let count: i64 = count_query_builder
            .fetch_one(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        
        Ok(PaginatedContributions {
            contributions,
            total_count: count,
        })
    }
    
    async fn get_total_monetary_contributions(
        &self,
        campaign_id: Uuid,
    ) -> Result<Decimal, ApplicationError> {
        let sum: Option<Decimal> = sqlx::query_scalar!(
            r#"
            SELECT SUM(amount) as "sum!"
            FROM contributions
            WHERE campaign_id = $1 AND amount IS NOT NULL
            "#,
            campaign_id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(RepositoryError::DatabaseError)?;
        
        Ok(sum.unwrap_or(Decimal::ZERO))
    }
    
    async fn delete(&self, id: Uuid) -> Result<(), ApplicationError> {
        sqlx::query!("DELETE FROM contributions WHERE id = $1", id)
            .execute(&self.pool)
            .await
            .map_err(RepositoryError::DatabaseError)?;
        
        
        Ok(())
    }
    
    async fn exists_for_campaign(&self, campaign_id: Uuid) -> Result<bool, ApplicationError> {
        let exists = sqlx::query_scalar!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM contributions
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
// Helper implementation for converting VerificationStatus to string
impl VerificationStatus {
    pub fn to_string(&self) -> &'static str {
        match self {
            VerificationStatus::Pending => "pending",
            VerificationStatus::Verified => "verified",
            VerificationStatus::Disputed => "disputed",
            VerificationStatus::Rejected => "rejected",
        }
    }
}