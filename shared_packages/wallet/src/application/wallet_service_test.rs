//! Service layer tests for wallet functionality

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::wallet_service::{WalletServiceImpl, WalletRepository};
    use crate::domain::wallet::{Wallet, WalletTransaction, TransactionType};
    use crate::domain::primitives::{Money, Currency, FinancialError};
    use uuid::Uuid;
    use rust_decimal::Decimal;
    use common_utils::error::CommonError;
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    // Mock repository implementation
    struct MockWalletRepository {
        should_fail: bool,
        insufficient_funds: bool,
    }
    
    #[async_trait::async_trait]
    impl WalletRepository for MockWalletRepository {
        async fn save_wallet(&self, _wallet: &Wallet) -> Result<(), FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Database error".to_string()))
            } else {
                Ok(())
            }
        }
        
        async fn find_wallet_by_user_id(&self, _user_id: Uuid) -> Result<Option<Wallet>, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Database error".to_string()))
            } else if self.insufficient_funds {
                let mut wallet = Wallet::new(_user_id);
                // Set balance to 0 to simulate insufficient funds
                Ok(Some(wallet))
            } else {
                let mut wallet = Wallet::new(_user_id);
                // Add some funds for testing
                let amount = Money::new(Decimal::from(100), Currency::Dabloons);
                wallet.add_dabloons(amount).unwrap();
                Ok(Some(wallet))
            }
        }
        
        async fn save_transaction(&self, _transaction: &WalletTransaction) -> Result<(), FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Database error".to_string()))
            } else {
                Ok(())
            }
        }
        
        async fn find_transactions_by_wallet_id(&self, _wallet_id: Uuid) -> Result<Vec<WalletTransaction>, FinancialError> {
            if self.should_fail {
                Err(FinancialError::ServiceError("Database error".to_string()))
            } else {
                Ok(vec![])
            }
        }
    }
    
    #[tokio::test]
    async fn test_get_or_create_wallet_new() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        
        // Execute
        let result = service.get_or_create_wallet(Uuid::new_v4()).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_or_create_wallet_existing() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        
        // Execute
        let user_id = Uuid::new_v4();
        let result = service.get_or_create_wallet(user_id).await;
        
        // Assert
        assert!(result.is_ok());
        let wallet = result.unwrap();
        assert_eq!(wallet.user_id, user_id);
    }
    
    #[tokio::test]
    async fn test_get_or_create_wallet_database_error() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: true, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        
        // Execute
        let result = service.get_or_create_wallet(Uuid::new_v4()).await;
        
        // Assert
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_add_dabloons_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Execute
        let amount = Money::new(Decimal::from(50), Currency::Dabloons);
        let result = service.add_dabloons(user_id, amount, Some("Test deposit".to_string())).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_add_dabloons_wrong_currency() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Execute with wrong currency
        let amount = Money::new(Decimal::from(50), Currency::USD);
        let result = service.add_dabloons(user_id, amount, Some("Test deposit".to_string())).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::CurrencyMismatch { .. } => {}, // Expected
            _ => panic!("Expected CurrencyMismatch error"),
        }
    }
    
    #[tokio::test]
    async fn test_subtract_dabloons_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
