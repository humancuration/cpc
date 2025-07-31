// This file has been deprecated as tip functionality has been moved to the wallet package
// The file is kept for reference but is no longer used in the codebase
//! PostgreSQL repository for tip transactions

use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::error::Error;
use cpc_wallet::domain::primitives::{Money, Currency};
use crate::domain::tip_transaction::TipTransaction;

/// Repository trait for tip transaction persistence
#[async_trait]
pub trait TipTransactionRepository: Send + Sync {
    /// Record a tip transaction
    async fn record_transaction(
        &self,
        sender_id: Uuid,
        recipient_id: Uuid,
        amount: Money,
        transaction_type: String,
        description: String
    ) -> Result<(), Box<dyn Error + Send + Sync>>;
    
    /// Get tip transactions for a user with pagination
    async fn get_transactions_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TipTransaction>, Box<dyn Error + Send + Sync>>;
}

/// PostgreSQL repository for tip transactions
#[derive(Debug)]
pub struct PostgresTipTransactionRepository {
    pool: PgPool,
}

impl PostgresTipTransactionRepository {
    /// Create a new PostgreSQL tip transaction repository
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TipTransactionRepository for PostgresTipTransactionRepository {
    async fn record_transaction(
        &self,
        sender_id: Uuid,
        recipient_id: Uuid,
        amount: Money,
        transaction_type: String,
        description: String
    ) -> Result<(), Box<dyn Error + Send + Sync>> {
        sqlx::query!(
            r#"
            INSERT INTO tip_transactions (
                id, sender_id, recipient_id, amount, currency, transaction_type, description, created_at
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, NOW()
            )
            "#,
            Uuid::new_v4(),
            sender_id,
            recipient_id,
            amount.amount.to_string(),
            amount.currency.to_string(),
            transaction_type,
            description
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn get_transactions_for_user(
        &self,
        user_id: Uuid,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TipTransaction>, Box<dyn Error + Send + Sync>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, sender_id, recipient_id, amount, currency, transaction_type, description, created_at
            FROM tip_transactions
            WHERE sender_id = $1 OR recipient_id = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            user_id,
            limit,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        
        let transactions = rows.into_iter().map(|row| {
            let amount = row.amount.parse::<rust_decimal::Decimal>()
                .unwrap_or_else(|_| rust_decimal::Decimal::ZERO);
            
            let currency = match row.currency.as_str() {
                "DAB" => Currency::DAB,
                "USD" => Currency::USD,
                "EUR" => Currency::EUR,
                "GBP" => Currency::GBP,
                "JPY" => Currency::JPY,
                _ => Currency::DAB, // Default to DAB if unknown
            };
            
            TipTransaction {
                id: row.id,
                sender_id: row.sender_id,
                recipient_id: row.recipient_id,
                amount: Money { amount, currency },
                transaction_type: row.transaction_type,
                description: row.description,
                created_at: row.created_at,
            }
        }).collect();
        
        Ok(transactions)
    }
}