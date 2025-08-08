//! Financial Impact Tracking Implementation
//!
//! Core functionality for tracking financial transactions, donations, and economic activities
//! within the community platform.

use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use rust_decimal::Decimal;
use cpay_core::{Transaction, WalletId, Currency};
use cpc_financial_core::{FinancialEvent, FinancialCategory};
use crate::FinancialImpactError;

/// Financial impact tracking record
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct FinancialImpactRecord {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub transaction_id: Option<Uuid>,
    pub event_type: FinancialEventType,
    pub amount: Decimal,
    pub currency: Currency,
    pub category: FinancialCategory,
    pub description: String,
    pub metadata: serde_json::Value,
    pub impact_score: Decimal,
}

/// Types of financial events tracked
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "financial_event_type")]
pub enum FinancialEventType {
    Donation,
    Purchase,
    Investment,
    Grant,
    Fundraising,
    Expense,
    Transfer,
    CommunityTrade,
    ServicePayment,
    VolunteerReimbursement,
}

/// Financial impact tracker
pub struct FinancialImpactTracker {
    db_pool: PgPool,
}

impl FinancialImpactTracker {
    /// Create a new financial impact tracker
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    /// Record a financial transaction impact
    pub async fn record_transaction_impact(
        &self,
        transaction: &Transaction,
        category: FinancialCategory,
        impact_score: Decimal,
        metadata: serde_json::Value,
    ) -> Result<FinancialImpactRecord, FinancialImpactError> {
        let mut tx = self.db_pool.begin().await?;
        
        let record = sqlx::query_as::<_, FinancialImpactRecord>(
            r#"
            INSERT INTO financial_impact_records (
                id, timestamp, transaction_id, event_type, amount, currency,
                category, description, metadata, impact_score
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(Utc::now())
        .bind(Some(transaction.id))
        .bind(FinancialEventType::from_transaction_type(&transaction.transaction_type))
        .bind(transaction.amount)
        .bind(&transaction.currency)
        .bind(category)
        .bind(&transaction.description)
        .bind(metadata)
        .bind(impact_score)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    /// Record a financial event impact (not tied to a specific transaction)
    pub async fn record_event_impact(
        &self,
        event_type: FinancialEventType,
        amount: Decimal,
        currency: Currency,
        category: FinancialCategory,
        description: String,
        impact_score: Decimal,
        metadata: serde_json::Value,
    ) -> Result<FinancialImpactRecord, FinancialImpactError> {
        let mut tx = self.db_pool.begin().await?;
        
        let record = sqlx::query_as::<_, FinancialImpactRecord>(
            r#"
            INSERT INTO financial_impact_records (
                id, timestamp, transaction_id, event_type, amount, currency,
                category, description, metadata, impact_score
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(Utc::now())
        .bind(None::<Uuid>)
        .bind(event_type)
        .bind(amount)
        .bind(currency)
        .bind(category)
        .bind(description)
        .bind(metadata)
        .bind(impact_score)
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(record)
    }

    /// Get financial impact records for a specific time range
    pub async fn get_impact_records(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        category: Option<FinancialCategory>,
    ) -> Result<Vec<FinancialImpactRecord>, FinancialImpactError> {
        let records = if let Some(cat) = category {
            sqlx::query_as::<_, FinancialImpactRecord>(
                r#"
                SELECT * FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2 AND category = $3
                ORDER BY timestamp DESC
                "#
            )
            .bind(start_time)
            .bind(end_time)
            .bind(cat)
            .fetch_all(&self.db_pool)
            .await?
        } else {
            sqlx::query_as::<_, FinancialImpactRecord>(
                r#"
                SELECT * FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
                ORDER BY timestamp DESC
                "#
            )
            .bind(start_time)
            .bind(end_time)
            .fetch_all(&self.db_pool)
            .await?
        };

        Ok(records)
    }

    /// Calculate total financial impact for a time period
    pub async fn calculate_total_impact(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        category: Option<FinancialCategory>,
    ) -> Result<Decimal, FinancialImpactError> {
        let total: Decimal = if let Some(cat) = category {
            sqlx::query_scalar(
                r#"
                SELECT COALESCE(SUM(amount * impact_score), 0) 
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2 AND category = $3
                "#
            )
            .bind(start_time)
            .bind(end_time)
            .bind(cat)
            .fetch_one(&self.db_pool)
            .await?
            .unwrap_or(Decimal::ZERO)
        } else {
            sqlx::query_scalar(
                r#"
                SELECT COALESCE(SUM(amount * impact_score), 0) 
                FROM financial_impact_records
                WHERE timestamp >= $1 AND timestamp <= $2
                "#
            )
            .bind(start_time)
            .bind(end_time)
            .fetch_one(&self.db_pool)
            .await?
            .unwrap_or(Decimal::ZERO)
        };

        Ok(total)
    }

    /// Get top financial impact contributors
    pub async fn get_top_contributors(
        &self,
        start_time: DateTime<Utc>,
        end_time: DateTime<Utc>,
        limit: i64,
    ) -> Result<Vec<(String, Decimal)>, FinancialImpactError> {
        let contributors = sqlx::query_as::<_, (String, Decimal)>(
            r#"
            SELECT 
                metadata->>'contributor_id' as contributor,
                SUM(amount * impact_score) as total_impact
            FROM financial_impact_records
            WHERE timestamp >= $1 AND timestamp <= $2 
                AND metadata->>'contributor_id' IS NOT NULL
            GROUP BY metadata->>'contributor_id'
            ORDER BY total_impact DESC
            LIMIT $3
            "#
        )
        .bind(start_time)
        .bind(end_time)
        .bind(limit)
        .fetch_all(&self.db_pool)
        .await?;

        Ok(contributors)
    }
}

impl FinancialEventType {
    /// Convert cpay transaction type to financial event type
    fn from_transaction_type(transaction_type: &cpay_core::TransactionType) -> Self {
        match transaction_type {
            cpay_core::TransactionType::Donation => FinancialEventType::Donation,
            cpay_core::TransactionType::Purchase => FinancialEventType::Purchase,
            cpay_core::TransactionType::Investment => FinancialEventType::Investment,
            cpay_core::TransactionType::Grant => FinancialEventType::Grant,
            cpay_core::TransactionType::Fundraising => FinancialEventType::Fundraising,
            cpay_core::TransactionType::Expense => FinancialEventType::Expense,
            cpay_core::TransactionType::Transfer => FinancialEventType::Transfer,
            cpay_core::TransactionType::ServicePayment => FinancialEventType::ServicePayment,
            cpay_core::TransactionType::Reimbursement => FinancialEventType::VolunteerReimbursement,
        }
    }
}