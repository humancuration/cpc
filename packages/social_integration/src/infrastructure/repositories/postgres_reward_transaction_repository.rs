// DEPRECATED: This file has been replaced by postgres_tip_transaction_repository.rs
//! PostgreSQL repository for reward transactions

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;
use cpc_wallet::domain::primitives::{Money, Currency};

/// Repository trait for reward transaction persistence
#[async_trait]
pub trait RewardTransactionRepository: Send + Sync {
    /// Record a reward transaction
    async fn record_transaction(
        &self, 
        user_id: Uuid, 
        amount: Money, 
        event_type: String, 
        description: String
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
}

/// PostgreSQL repository for reward transactions
#[derive(Debug)]
pub struct PostgresRewardTransactionRepository {
    pool: PgPool,
}

impl PostgresRewardTransactionRepository {
    /// Create a new PostgreSQL reward transaction repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl RewardTransactionRepository for PostgresRewardTransactionRepository {
    async fn record_transaction(
        &self,
        user_id: Uuid,
        amount: Money,
        event_type: String,
        description: String
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO reward_transactions (
                id, user_id, amount, currency, event_type, description, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, NOW()
            )
            "#,
            Uuid::new_v4(),
            user_id,
            amount.amount.to_string(),
            amount.currency.to_string(),
            event_type,
            description
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
}
}