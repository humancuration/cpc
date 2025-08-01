//! Repository traits for CPay data access
//!
//! This module defines the repository interfaces for accessing transaction data
//! in both traditional currency and Dabloons formats.

pub mod mock;

use crate::models::{Transaction, TransactionStatus};
use uuid::Uuid;
use rust_decimal::Decimal;
use chrono::{DateTime, Utc};
use sqlx::Error;

/// Repository trait for traditional currency transactions
#[async_trait::async_trait]
pub trait TraditionalCurrencyTransactionRepository {
    /// Save a traditional currency transaction
    async fn save_transaction(&self, transaction: TraditionalCurrencyTransaction) -> Result<(), Error>;
    
    /// Find transactions by user ID
    async fn find_transactions_by_user_id(&self, user_id: Uuid) -> Result<Vec<TraditionalCurrencyTransaction>, Error>;
    
    /// Find transaction by ID
    async fn find_transaction_by_id(&self, id: Uuid) -> Result<Option<TraditionalCurrencyTransaction>, Error>;
}

/// Traditional currency transaction model for database storage
#[derive(Debug, Clone)]
pub struct TraditionalCurrencyTransaction {
    /// Unique identifier for the transaction
    pub id: Uuid,
    
    /// User associated with the transaction
    pub user_id: Uuid,
    
    /// Type of transaction (credit or debit)
    pub transaction_type: String,
    
    /// Amount of the transaction
    pub amount: Decimal,
    
    /// Currency of the transaction
    pub currency: String,
    
    /// External reference (e.g., payment provider transaction ID)
    pub external_reference: Option<String>,
    
    /// Status of the transaction
    pub status: String,
    
    /// Optional description
    pub description: Option<String>,
    
    /// Optional social post ID
    pub social_post_id: Option<Uuid>,
    
    /// Optional volunteer hours associated with the transaction
    pub volunteer_hours: Option<Decimal>,
    
    /// When the transaction was created
    pub created_at: DateTime<Utc>,
    
    /// When the transaction was last updated
    pub updated_at: DateTime<Utc>,
}

impl TraditionalCurrencyTransaction {
    /// Create a new traditional currency transaction
    pub fn new(
        user_id: Uuid,
        transaction_type: String,
        amount: Decimal,
        currency: String,
        external_reference: Option<String>,
        description: Option<String>,
        social_post_id: Option<Uuid>,
        volunteer_hours: Option<Decimal>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            transaction_type,
            amount,
            currency,
            external_reference,
            status: "pending".to_string(),
            description,
            social_post_id,
            volunteer_hours,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Convert to a CPay Transaction model
    pub fn to_transaction(&self) -> Transaction {
        Transaction {
            id: self.id,
            sender_id: self.user_id,
            recipient_id: self.user_id, // This would need to be looked up in a real implementation
            amount: self.amount,
            currency: self.currency.parse().unwrap_or(crate::models::Currency::USD),
            status: match self.status.as_str() {
                "completed" => TransactionStatus::Completed,
                "failed" => TransactionStatus::Failed,
                "cancelled" => TransactionStatus::Cancelled,
                _ => TransactionStatus::Pending,
            },
            description: self.description.clone(),
            social_post_id: self.social_post_id,
            volunteer_hours: self.volunteer_hours,
            created_at: self.created_at,
            completed_at: if self.status == "completed" { Some(self.updated_at) } else { None },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_traditional_currency_transaction_creation() {
        let user_id = Uuid::new_v4();
        let amount = dec!(100.50);
        
        let transaction = TraditionalCurrencyTransaction::new(
            user_id,
            "debit".to_string(),
            amount,
            "USD".to_string(),
            Some("ext123".to_string()),
            Some("Test transaction".to_string()),
        );
        
        assert_eq!(transaction.user_id, user_id);
        assert_eq!(transaction.transaction_type, "debit");
        assert_eq!(transaction.amount, amount);
        assert_eq!(transaction.currency, "USD");
        assert_eq!(transaction.external_reference, Some("ext123".to_string()));
        assert_eq!(transaction.description, Some("Test transaction".to_string()));
        assert_eq!(transaction.status, "pending");
    }
    
    #[test]
    fn test_traditional_currency_transaction_to_transaction() {
        let user_id = Uuid::new_v4();
        let amount = dec!(100.50);
        let now = Utc::now();
        
        let traditional_transaction = TraditionalCurrencyTransaction {
            id: Uuid::new_v4(),
            user_id,
            transaction_type: "credit".to_string(),
            amount,
            currency: "USD".to_string(),
            external_reference: None,
            status: "completed".to_string(),
            description: Some("Test transaction".to_string()),
            created_at: now,
            updated_at: now,
        };
        
        let transaction = traditional_transaction.to_transaction();
        
        assert_eq!(transaction.sender_id, user_id);
        assert_eq!(transaction.recipient_id, user_id);
        assert_eq!(transaction.amount, amount);
        assert_eq!(transaction.currency, crate::models::Currency::USD);
        assert_eq!(transaction.status, TransactionStatus::Completed);
        assert_eq!(transaction.description, Some("Test transaction".to_string()));
        assert_eq!(transaction.completed_at, Some(now));
    }
}