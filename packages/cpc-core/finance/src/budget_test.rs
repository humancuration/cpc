//! Integration tests for the budget functionality

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            budget::{Budget, BudgetPeriod, BudgetAllocation},
            primitives::{Money, Currency},
        },
        application::{
            budget_service::{BudgetService, BudgetServiceImpl},
        },
    };
    use uuid::Uuid;
    use std::sync::Arc;
    use rust_decimal_macros::dec;
    use chrono::Utc;

    // Mock repository for testing
    struct MockBudgetRepository {
        budget: Option<Budget>,
    }

    impl MockBudgetRepository {
        fn new() -> Self {
            Self {
                budget: None,
            }
        }
    }

    #[async_trait::async_trait]
    impl crate::application::budget_service::BudgetRepository for MockBudgetRepository {
        async fn save(&self, _budget: &crate::domain::budget::Budget) -> Result<(), crate::domain::FinanceError> {
            // In a real implementation, this would save to a database
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

    #[test]
    fn test_budget_allocation_creation() {
        let primary = Money::new(dec!(100), Currency::USD);
        let dabloons = Money::new(dec!(50), Currency::Dabloons);
        let allocation = BudgetAllocation::new(primary.clone(), dabloons.clone()).unwrap();
        
        assert_eq!(allocation.primary, primary);
        assert_eq!(allocation.dabloons, dabloons);
    }

    #[test]
    fn test_budget_allocation_creation_invalid_currency() {
        let primary = Money::new(dec!(100), Currency::Dabloons); // Invalid - should be traditional currency
        let dabloons = Money::new(dec!(50), Currency::Dabloons);
        
        let result = BudgetAllocation::new(primary, dabloons);
        assert!(result.is_err());
    }

    #[test]
    fn test_budget_allocation_addition() {
        let alloc1 = BudgetAllocation::new(
            Money::new(dec!(100), Currency::USD),
            Money::new(dec!(50), Currency::Dabloons)
        ).unwrap();
        
        let alloc2 = BudgetAllocation::new(
            Money::new(dec!(50), Currency::USD),
            Money::new(dec!(25), Currency::Dabloons)
        ).unwrap();
        
        let result = alloc1.add(&alloc2).unwrap();
        assert_eq!(result.primary.amount, dec!(150));
        assert_eq!(result.dabloons.amount, dec!(75));
    }

    #[test]
    fn test_budget_creation_traditional_only() {
        let user_id = Uuid::new_v4();
        let budget = Budget::new(
            user_id,
            "Groceries".to_string(),
            Money::new(dec!(500), Currency::USD),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        
        assert_eq!(budget.user_id, user_id);
        assert_eq!(budget.category, "Groceries");
        assert_eq!(budget.allocation.primary.amount, dec!(500));
        assert_eq!(budget.allocation.primary.currency, Currency::USD);
        assert_eq!(budget.allocation.dabloons.amount, dec!(0));
        assert_eq!(budget.currency_type, crate::domain::budget::BudgetCurrencyType::TraditionalOnly);
    }

    #[test]
    fn test_budget_creation_dabloons_only() {
        let user_id = Uuid::new_v4();
        let budget = Budget::new(
            user_id,
            "Entertainment".to_string(),
            Money::new(dec!(100), Currency::Dabloons),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        
        assert_eq!(budget.user_id, user_id);
        assert_eq!(budget.category, "Entertainment");
        assert_eq!(budget.allocation.primary.amount, dec!(0));
        assert_eq!(budget.allocation.dabloons.amount, dec!(100));
        assert_eq!(budget.allocation.dabloons.currency, Currency::Dabloons);
        assert_eq!(budget.currency_type, crate::domain::budget::BudgetCurrencyType::DabloonsOnly);
    }

    #[test]
    fn test_mixed_budget_creation() {
        let user_id = Uuid::new_v4();
        let result = Budget::new_mixed(
            user_id,
            "Shopping".to_string(),
            Money::new(dec!(200), Currency::USD),
            Money::new(dec!(50), Currency::Dabloons),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        
        assert!(result.is_ok());
        let budget = result.unwrap();
        assert_eq!(budget.user_id, user_id);
        assert_eq!(budget.category, "Shopping");
        assert_eq!(budget.allocation.primary.amount, dec!(200));
        assert_eq!(budget.allocation.primary.currency, Currency::USD);
        assert_eq!(budget.allocation.dabloons.amount, dec!(50));
        assert_eq!(budget.allocation.dabloons.currency, Currency::Dabloons);
        assert_eq!(budget.currency_type, crate::domain::budget::BudgetCurrencyType::Mixed);
    }

    #[test]
    fn test_mixed_budget_creation_invalid_currency() {
        let user_id = Uuid::new_v4();
        let result = Budget::new_mixed(
            user_id,
            "Shopping".to_string(),
            Money::new(dec!(200), Currency::Dabloons), // Invalid - should be traditional currency
            Money::new(dec!(50), Currency::Dabloons),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_budget_utilization_percentage() {
        let user_id = Uuid::new_v4();
        let mut budget = Budget::new_mixed(
            user_id,
            "Shopping".to_string(),
            Money::new(dec!(200), Currency::USD),
            Money::new(dec!(50), Currency::Dabloons),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        ).unwrap();
        
        // Update spent amounts
        budget.spent.primary = Money::new(dec!(100), Currency::USD);
        budget.spent.dabloons = Money::new(dec!(25), Currency::Dabloons);
        
        let percentage = budget.utilization_percentage();
        // Based on our implementation: (100 + 25*0.01) / (200 + 50*0.01) * 100
        // = (100 + 0.25) / (200 + 0.5) * 100
        // = 100.25 / 200.5 * 100
        // ≈ 49.99%
        assert!(percentage > dec!(49) && percentage < dec!(51));
    }

    #[tokio::test]
    async fn test_budget_service_create_mixed_budget() {
        let repo = Arc::new(MockBudgetRepository::new());
        let budget_service = BudgetServiceImpl::new(repo);
        let user_id = Uuid::new_v4();
        
        let result = budget_service.create_mixed_budget(
            user_id,
            "Shopping".to_string(),
            Money::new(dec!(200), Currency::USD),
            Money::new(dec!(50), Currency::Dabloons),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        ).await;
        
        assert!(result.is_ok());
        let budget = result.unwrap();
        assert_eq!(budget.user_id, user_id);
        assert_eq!(budget.category, "Shopping");
        assert_eq!(budget.allocation.primary.amount, dec!(200));
        assert_eq!(budget.allocation.dabloons.amount, dec!(50));
        assert_eq!(budget.currency_type, crate::domain::budget::BudgetCurrencyType::Mixed);
    }

    #[tokio::test]
    async fn test_budget_service_update_spent_with_dabloons() {
        let mut mock_repo = MockBudgetRepository::new();
        let budget = Budget::new_mixed(
            Uuid::new_v4(),
            "Shopping".to_string(),
            Money::new(dec!(200), Currency::USD),
            Money::new(dec!(50), Currency::Dabloons),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        ).unwrap();
        mock_repo.budget = Some(budget);
        
        let repo = Arc::new(mock_repo);
        let budget_service = BudgetServiceImpl::new(repo);
        let user_id = Uuid::new_v4();
        
        let result = budget_service.update_spent_with_dabloons(
            user_id,
            "Shopping",
            Money::new(dec!(25), Currency::Dabloons)
        ).await;
        
        assert!(result.is_ok());
        let updated_budget = result.unwrap();
        assert_eq!(updated_budget.spent.dabloons.amount, dec!(25));
    }
}