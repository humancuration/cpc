//! Mock implementations of repositories for testing

use crate::repositories::{TraditionalCurrencyTransactionRepository, TraditionalCurrencyTransaction};
use uuid::Uuid;
use sqlx::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Mock implementation of TraditionalCurrencyTransactionRepository for testing
pub struct MockTraditionalCurrencyTransactionRepository {
    transactions: Arc<Mutex<Vec<TraditionalCurrencyTransaction>>>,
}

impl MockTraditionalCurrencyTransactionRepository {
    /// Create a new mock repository
    pub fn new() -> Self {
        Self {
            transactions: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl TraditionalCurrencyTransactionRepository for MockTraditionalCurrencyTransactionRepository {
    async fn save_transaction(&self, transaction: TraditionalCurrencyTransaction) -> Result<(), Error> {
        let mut transactions = self.transactions.lock().await;
        transactions.push(transaction);
        Ok(())
    }
    
    async fn find_transactions_by_user_id(&self, user_id: Uuid) -> Result<Vec<TraditionalCurrencyTransaction>, Error> {
        let transactions = self.transactions.lock().await;
        Ok(transactions.iter()
            .filter(|t| t.user_id == user_id)
            .cloned()
            .collect())
    }
    
    async fn find_transaction_by_id(&self, id: Uuid) -> Result<Option<TraditionalCurrencyTransaction>, Error> {
        let transactions = self.transactions.lock().await;
        Ok(transactions.iter()
            .find(|t| t.id == id)
            .cloned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::TraditionalCurrencyTransaction;
    use rust_decimal_macros::dec;
    use chrono::Utc;
    
    #[tokio::test]
    async fn test_mock_repository_save_and_find() {
        let repo = MockTraditionalCurrencyTransactionRepository::new();
        let user_id = Uuid::new_v4();
        let now = Utc::now();
        
        let transaction = TraditionalCurrencyTransaction {
            id: Uuid::new_v4(),
            user_id,
            transaction_type: "credit".to_string(),
            amount: dec!(100.50),
            currency: "USD".to_string(),
            external_reference: None,
            status: "completed".to_string(),
            description: Some("Test transaction".to_string()),
            social_post_id: None,
            volunteer_hours: None,
            created_at: now,
            updated_at: now,
        };
        
        // Save transaction
        assert!(repo.save_transaction(transaction.clone()).await.is_ok());
        
        // Find by user ID
        let transactions = repo.find_transactions_by_user_id(user_id).await.unwrap();
        assert_eq!(transactions.len(), 1);
        assert_eq!(transactions[0].id, transaction.id);
        
        // Find by ID
        let found = repo.find_transaction_by_id(transaction.id).await.unwrap();
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, transaction.id);
        
        // Find by non-existent ID
        let not_found = repo.find_transaction_by_id(Uuid::new_v4()).await.unwrap();
        assert!(not_found.is_none());
    }
}