//! Integration tests for the wallet-budget integration functionality

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            wallet::{Wallet, WalletTransaction, TransactionType},
            budget::{Budget, BudgetPeriod},
            primitives::{Money, Currency},
        },
        application::{
            wallet_service::{WalletService, WalletServiceImpl},
            budget_service::{BudgetService, BudgetServiceImpl},
        },
    };
    use uuid::Uuid;
    use std::sync::Arc;
    use rust_decimal_macros::dec;
    use chrono::Utc;

    // Mock repositories for testing
    struct MockWalletRepository {
        wallet: Option<Wallet>,
        transactions: Vec<WalletTransaction>,
    }

    struct MockBudgetRepository {
        budget: Option<Budget>,
    }

    impl MockWalletRepository {
        fn new() -> Self {
            Self {
                wallet: None,
                transactions: Vec::new(),
            }
        }
    }

    impl MockBudgetRepository {
        fn new() -> Self {
            Self {
                budget: None,
            }
        }
    }

    #[async_trait::async_trait]
    impl crate::application::wallet_service::WalletRepository for MockWalletRepository {
        async fn save_wallet(&self, _wallet: &crate::domain::wallet::Wallet) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }

        async fn find_wallet_by_user_id(&self, _user_id: Uuid) -> Result<Option<crate::domain::wallet::Wallet>, crate::domain::FinanceError> {
            Ok(self.wallet.clone())
        }

        async fn save_transaction(&self, _transaction: &crate::domain::wallet::WalletTransaction) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }

        async fn find_transactions_by_wallet_id(&self, _wallet_id: Uuid) -> Result<Vec<crate::domain::wallet::WalletTransaction>, crate::domain::FinanceError> {
            Ok(self.transactions.clone())
        }
    }

    #[async_trait::async_trait]
    impl crate::application::budget_service::BudgetRepository for MockBudgetRepository {
        async fn save(&self, _budget: &crate::domain::budget::Budget) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }

        async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Vec<crate::domain::budget::Budget>, crate::domain::FinanceError> {
            Ok(self.budget.clone().map(|b| vec![b]).unwrap_or_else(Vec::new))
        }

        async fn find_by_user_and_category(&self, _user_id: Uuid, _category: &str) -> Result<Option<crate::domain::budget::Budget>, crate::domain::FinanceError> {
            Ok(self.budget.clone())
        }

        async fn reset_monthly_budgets(&self, _user_id: Uuid) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_wallet_link_to_budget() {
        let wallet_repo = Arc::new(MockWalletRepository::new());
        let wallet_service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Add some dabloons to the wallet first
        wallet_service.add_dabloons(user_id, Money::new(dec!(100), Currency::Dabloons), None).await.unwrap();
        
        // Try to link to a budget category
        let result = wallet_service.link_to_budget(
            user_id,
            "Entertainment",
            Money::new(dec!(50), Currency::Dabloons)
        ).await;
        
        // This should succeed (placeholder implementation)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wallet_link_to_budget_invalid_currency() {
        let wallet_repo = Arc::new(MockWalletRepository::new());
        let wallet_service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Try to link with traditional currency (should fail)
        let result = wallet_service.link_to_budget(
            user_id,
            "Entertainment",
            Money::new(dec!(50), Currency::USD)
        ).await;
        
        // This should fail
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::domain::FinanceError::InvalidCurrency => (),
            _ => panic!("Expected InvalidCurrency error"),
        }
    }

    #[tokio::test]
    async fn test_wallet_get_linked_budgets() {
        let wallet_repo = Arc::new(MockWalletRepository::new());
        let wallet_service = WalletServiceImpl::new(wallet_repo);
        let user_id = Uuid::new_v4();
        
        // Get linked budgets (placeholder implementation returns empty vec)
        let result = wallet_service.get_linked_budgets(user_id).await;
        
        assert!(result.is_ok());
        let budgets = result.unwrap();
        assert!(budgets.is_empty());
    }

    #[tokio::test]
    async fn test_budget_update_spent_with_dabloons() {
        let mut mock_budget_repo = MockBudgetRepository::new();
        let budget = Budget::new(
            Uuid::new_v4(),
            "Entertainment".to_string(),
            Money::new(dec!(100), Currency::USD),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        mock_budget_repo.budget = Some(budget);
        
        let budget_repo = Arc::new(mock_budget_repo);
        let budget_service = BudgetServiceImpl::new(budget_repo);
        let user_id = Uuid::new_v4();
        
        let result = budget_service.update_spent_with_dabloons(
            user_id,
            "Entertainment",
            Money::new(dec!(25), Currency::Dabloons)
        ).await;
        
        assert!(result.is_ok());
        let updated_budget = result.unwrap();
        assert_eq!(updated_budget.spent.dabloons.amount, dec!(25));
    }

    #[tokio::test]
    async fn test_budget_update_spent_with_dabloons_invalid_currency() {
        let mut mock_budget_repo = MockBudgetRepository::new();
        let budget = Budget::new(
            Uuid::new_v4(),
            "Entertainment".to_string(),
            Money::new(dec!(100), Currency::USD),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        mock_budget_repo.budget = Some(budget);
        
        let budget_repo = Arc::new(mock_budget_repo);
        let budget_service = BudgetServiceImpl::new(budget_repo);
        let user_id = Uuid::new_v4();
        
        let result = budget_service.update_spent_with_dabloons(
            user_id,
            "Entertainment",
            Money::new(dec!(25), Currency::USD) // Invalid - should be Dabloons
        ).await;
        
        assert!(result.is_err());
        match result.unwrap_err() {
            crate::domain::FinanceError::InvalidCurrency => (),
            _ => panic!("Expected InvalidCurrency error"),
        }
    }
}