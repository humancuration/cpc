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
        let user_id = Uuid::new_v4();
        
        // Execute
        let amount = Money::new(Decimal::from(50), Currency::Dabloons);
        let result = service.subtract_dabloons(user_id, amount, Some("Test withdrawal".to_string())).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_subtract_dabloons_wrong_currency() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Execute with wrong currency
        let amount = Money::new(Decimal::from(50), Currency::USD);
        let result = service.subtract_dabloons(user_id, amount, Some("Test withdrawal".to_string())).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::CurrencyMismatch { .. } => {}, // Expected
            _ => panic!("Expected CurrencyMismatch error"),
        }
    }
    
    #[tokio::test]
    async fn test_transfer_dabloons_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        
        // Execute
        let amount = Money::new(Decimal::from(25), Currency::Dabloons);
        let result = service.transfer_dabloons(
            from_user_id,
            to_user_id,
            amount,
            Some("Test transfer".to_string())
        ).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_transfer_dabloons_wrong_currency() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        
        // Execute with wrong currency
        let amount = Money::new(Decimal::from(25), Currency::USD);
        let result = service.transfer_dabloons(
            from_user_id,
            to_user_id,
            amount,
            Some("Test transfer".to_string())
        ).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::CurrencyMismatch { .. } => {}, // Expected
            _ => panic!("Expected CurrencyMismatch error"),
        }
    }
    
    #[tokio::test]
    async fn test_transfer_dabloons_insufficient_funds() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: true });
        let service = WalletServiceImpl::new(wallet_repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        
        // Execute
        let amount = Money::new(Decimal::from(25), Currency::Dabloons);
        let result = service.transfer_dabloons(
            from_user_id,
            to_user_id,
            amount,
            Some("Test transfer".to_string())
        ).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::InsufficientFunds(Currency::Dabloons) => {}, // Expected
            _ => panic!("Expected InsufficientFunds error"),
        }
    }
    
    #[tokio::test]
    async fn test_send_tip_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        
        // Execute
        let amount = Money::new(Decimal::from(10), Currency::Dabloons);
        let result = service.send_tip(
            from_user_id,
            to_user_id,
            amount,
            Some("Great work!".to_string())
        ).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_send_tip_wrong_currency() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        
        // Execute with wrong currency
        let amount = Money::new(Decimal::from(10), Currency::USD);
        let result = service.send_tip(
            from_user_id,
            to_user_id,
            amount,
            Some("Great work!".to_string())
        ).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::CurrencyMismatch { .. } => {}, // Expected
            _ => panic!("Expected CurrencyMismatch error"),
        }
    }
    
    #[tokio::test]
    async fn test_send_tip_insufficient_funds() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: true });
        let service = WalletServiceImpl::new(wallet_repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        
        // Execute
        let amount = Money::new(Decimal::from(10), Currency::Dabloons);
        let result = service.send_tip(
            from_user_id,
            to_user_id,
            amount,
            Some("Great work!".to_string())
        ).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::InsufficientFunds(Currency::Dabloons) => {}, // Expected
            _ => panic!("Expected InsufficientFunds error"),
        }
    }
    
    #[tokio::test]
    async fn test_get_transaction_history_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.get_transaction_history(user_id).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_transaction_history_database_error() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: true, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Execute
        let result = service.get_transaction_history(user_id).await;
        
        // Assert
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_distribute_universal_income_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        let amount = Money::new(Decimal::from(100), Currency::Dabloons);
        let distribution_date = chrono::Utc::now().date_naive();
        
        // Execute
        let result = service.distribute_universal_income(user_id, amount, distribution_date).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_distribute_universal_income_wrong_currency() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        let amount = Money::new(Decimal::from(100), Currency::USD);
        let distribution_date = chrono::Utc::now().date_naive();
        
        // Execute
        let result = service.distribute_universal_income(user_id, amount, distribution_date).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::CurrencyMismatch { .. } => {}, // Expected
            _ => panic!("Expected CurrencyMismatch error"),
        }
    }
    
    #[tokio::test]
    async fn test_credit_volunteer_dabloons_success() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        let amount = Money::new(Decimal::from(20), Currency::Dabloons);
        let hours_converted = Decimal::from(2);
        
        // Execute
        let result = service.credit_volunteer_dabloons(user_id, amount, hours_converted).await;
        
        // Assert
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_credit_volunteer_dabloons_wrong_currency() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        let amount = Money::new(Decimal::from(20), Currency::USD);
        let hours_converted = Decimal::from(2);
        
        // Execute
        let result = service.credit_volunteer_dabloons(user_id, amount, hours_converted).await;
        
        // Assert
        assert!(result.is_err());
        match result.unwrap_err() {
            FinancialError::CurrencyMismatch { .. } => {}, // Expected
            _ => panic!("Expected CurrencyMismatch error"),
        }
    }
    
    #[tokio::test]
    async fn test_wallet_service_tip_event_broadcasting() {
        // Setup
        let wallet_repo = Arc::new(MockWalletRepository { should_fail: false, insufficient_funds: false });
        let service = WalletServiceImpl::new(wallet_repo);
        
        // Subscribe to tip events
        let mut receiver = service.subscribe_tip_events();
        
        // Execute a tip operation
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        let amount = Money::new(Decimal::from(10), Currency::Dabloons);
        let result = service.send_tip(
            from_user_id,
            to_user_id,
            amount,
            Some("Great work!".to_string())
        ).await;
        
        // Assert the operation succeeded
        assert!(result.is_ok());
        
        // Check that a tip event was broadcast
        let event = receiver.try_recv();
        assert!(event.is_ok());
    }
}