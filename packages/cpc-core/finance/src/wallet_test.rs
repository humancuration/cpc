//! Integration tests for the wallet functionality

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            wallet::{Wallet, WalletTransaction, TransactionType},
            primitives::{Money, Currency},
        },
        application::{
            wallet_service::{WalletService, WalletServiceImpl},
        },
    };
    use uuid::Uuid;
    use std::sync::Arc;
    use rust_decimal_macros::dec;

    // Mock repository for testing
    struct MockWalletRepository {
        wallet: Option<Wallet>,
        transactions: Vec<WalletTransaction>,
    }

    impl MockWalletRepository {
        fn new() -> Self {
            Self {
                wallet: None,
                transactions: Vec::new(),
            }
        }
    }

    #[async_trait::async_trait]
    impl crate::application::wallet_service::WalletRepository for MockWalletRepository {
        async fn save_wallet(&self, _wallet: &crate::domain::wallet::Wallet) -> Result<(), crate::domain::FinanceError> {
            // In a real implementation, this would save to a database
            Ok(())
        }

        async fn find_wallet_by_user_id(&self, _user_id: Uuid) -> Result<Option<crate::domain::wallet::Wallet>, crate::domain::FinanceError> {
            Ok(self.wallet.clone())
        }

        async fn save_transaction(&self, _transaction: &crate::domain::wallet::WalletTransaction) -> Result<(), crate::domain::FinanceError> {
            // In a real implementation, this would save to a database
            Ok(())
        }

        async fn find_transactions_by_wallet_id(&self, _wallet_id: Uuid) -> Result<Vec<crate::domain::wallet::WalletTransaction>, crate::domain::FinanceError> {
            Ok(self.transactions.clone())
        }
    }

    #[tokio::test]
    async fn test_wallet_creation() {
        let user_id = Uuid::new_v4();
        let wallet = Wallet::new(user_id);
        
        assert_eq!(wallet.user_id, user_id);
        assert_eq!(wallet.balance.currency, Currency::Dabloons);
        assert_eq!(wallet.balance.amount, dec!(0));
    }

    #[tokio::test]
    async fn test_add_dabloons() {
        let user_id = Uuid::new_v4();
        let mut wallet = Wallet::new(user_id);
        let amount = Money::new(dec!(100), Currency::Dabloons);
        
        assert!(wallet.add_dabloons(amount.clone()).is_ok());
        assert_eq!(wallet.balance.amount, dec!(100));
    }

    #[tokio::test]
    async fn test_subtract_dabloons() {
        let user_id = Uuid::new_v4();
        let balance = Money::new(dec!(100), Currency::Dabloons);
        let mut wallet = Wallet::with_balance(user_id, balance).unwrap();
        let amount = Money::new(dec!(30), Currency::Dabloons);
        
        assert!(wallet.subtract_dabloons(amount.clone()).is_ok());
        assert_eq!(wallet.balance.amount, dec!(70));
    }

    #[tokio::test]
    async fn test_wallet_service_get_or_create() {
        let repo = Arc::new(MockWalletRepository::new());
        let wallet_service = WalletServiceImpl::new(repo);
        let user_id = Uuid::new_v4();
        
        let wallet = wallet_service.get_or_create_wallet(user_id).await.unwrap();
        
        assert_eq!(wallet.user_id, user_id);
        assert_eq!(wallet.balance.currency, Currency::Dabloons);
        assert_eq!(wallet.balance.amount, dec!(0));
    }

    #[tokio::test]
    async fn test_wallet_service_add_dabloons() {
        let repo = Arc::new(MockWalletRepository::new());
        let wallet_service = WalletServiceImpl::new(repo);
        let user_id = Uuid::new_v4();
        let amount = Money::new(dec!(50), Currency::Dabloons);
        
        let wallet = wallet_service.add_dabloons(user_id, amount, Some("Test deposit".to_string())).await.unwrap();
        
        assert_eq!(wallet.balance.amount, dec!(50));
    }

    #[tokio::test]
    async fn test_wallet_service_transfer_dabloons() {
        let repo = Arc::new(MockWalletRepository::new());
        let wallet_service = WalletServiceImpl::new(repo);
        let from_user_id = Uuid::new_v4();
        let to_user_id = Uuid::new_v4();
        let amount = Money::new(dec!(30), Currency::Dabloons);
        
        // Add some dabloons to the sender's wallet first
        wallet_service.add_dabloons(from_user_id, Money::new(dec!(100), Currency::Dabloons), None).await.unwrap();
        
        // Transfer dabloons
        let (from_wallet, to_wallet) = wallet_service.transfer_dabloons(
            from_user_id, 
            to_user_id, 
            amount, 
            Some("Test transfer".to_string())
        ).await.unwrap();
        
        assert_eq!(from_wallet.balance.amount, dec!(70));
        assert_eq!(to_wallet.balance.amount, dec!(30));
    }
}