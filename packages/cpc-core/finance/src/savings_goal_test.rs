//! Integration tests for the savings goal functionality

#[cfg(test)]
mod tests {
    use crate::{
        domain::{
            savings_goal::{SavingsGoal, DualCurrencyTarget},
            primitives::{Money, Currency},
        },
        application::{
            savings_service::{SavingsService, SavingsServiceImpl},
        },
    };
    use uuid::Uuid;
    use std::sync::Arc;
    use rust_decimal_macros::dec;
    use chrono::Utc;

    // Mock repositories for testing
    struct MockSavingsRepository {
        goal: Option<SavingsGoal>,
    }

    struct MockDataSharingRepository {
        preference: Option<crate::application::savings_service::DataSharingPreference>,
    }

    impl MockSavingsRepository {
        fn new() -> Self {
            Self {
                goal: None,
            }
        }
    }

    impl MockDataSharingRepository {
        fn new() -> Self {
            Self {
                preference: None,
            }
        }
    }

    #[async_trait::async_trait]
    impl crate::application::savings_service::SavingsRepository for MockSavingsRepository {
        async fn save(&self, _goal: &crate::domain::savings_goal::SavingsGoal) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }

        async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Vec<crate::domain::savings_goal::SavingsGoal>, crate::domain::FinanceError> {
            Ok(self.goal.clone().map(|g| vec![g]).unwrap_or_else(Vec::new))
        }

        async fn find_active_by_user_id(&self, _user_id: Uuid) -> Result<Vec<crate::domain::savings_goal::SavingsGoal>, crate::domain::FinanceError> {
            Ok(self.goal.clone().map(|g| vec![g]).unwrap_or_else(Vec::new))
        }

        async fn find_by_id(&self, _id: Uuid) -> Result<Option<crate::domain::savings_goal::SavingsGoal>, crate::domain::FinanceError> {
            Ok(self.goal.clone())
        }

        async fn delete(&self, _id: Uuid) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }
    }

    #[async_trait::async_trait]
    impl crate::application::savings_service::DataSharingRepository for MockDataSharingRepository {
        async fn save(&self, _preference: &crate::application::savings_service::DataSharingPreference) -> Result<(), crate::domain::FinanceError> {
            Ok(())
        }

        async fn find_by_user_id(&self, _user_id: Uuid) -> Result<Option<crate::application::savings_service::DataSharingPreference>, crate::domain::FinanceError> {
            Ok(self.preference.clone())
        }

        async fn create_default(&self, user_id: Uuid) -> Result<crate::application::savings_service::DataSharingPreference, crate::domain::FinanceError> {
            Ok(crate::application::savings_service::DataSharingPreference::new(user_id))
        }
    }

    #[test]
    fn test_dual_currency_target_creation() {
        let primary = Money::new(dec!(1000), Currency::USD);
        let dabloons = Money::new(dec!(200), Currency::Dabloons);
        let target = DualCurrencyTarget::new(primary.clone(), dabloons.clone()).unwrap();
        
        assert_eq!(target.primary, primary);
        assert_eq!(target.dabloons, dabloons);
    }

    #[test]
    fn test_dual_currency_target_creation_invalid_currency() {
        let primary = Money::new(dec!(1000), Currency::Dabloons); // Invalid - should be traditional currency
        let dabloons = Money::new(dec!(200), Currency::Dabloons);
        
        let result = DualCurrencyTarget::new(primary, dabloons);
        assert!(result.is_err());
    }

    #[test]
    fn test_dual_currency_target_addition() {
        let target = DualCurrencyTarget::new(
            Money::new(dec!(1000), Currency::USD),
            Money::new(dec!(200), Currency::Dabloons)
        ).unwrap();
        
        let amount = Money::new(dec!(100), Currency::USD);
        let result = target.add(&amount).unwrap();
        assert_eq!(result.primary.amount, dec!(1100));
        assert_eq!(result.dabloons.amount, dec!(200));
    }

    #[test]
    fn test_dual_currency_target_addition_dabloons() {
        let target = DualCurrencyTarget::new(
            Money::new(dec!(1000), Currency::USD),
            Money::new(dec!(200), Currency::Dabloons)
        ).unwrap();
        
        let amount = Money::new(dec!(50), Currency::Dabloons);
        let result = target.add(&amount).unwrap();
        assert_eq!(result.primary.amount, dec!(1000));
        assert_eq!(result.dabloons.amount, dec!(250));
    }

    #[test]
    fn test_savings_goal_creation_traditional_only() {
        let user_id = Uuid::new_v4();
        let goal = SavingsGoal::new(
            user_id,
            "Vacation".to_string(),
            Money::new(dec!(2000), Currency::USD),
            Utc::now() + chrono::Duration::days(365),
        );
        
        assert_eq!(goal.user_id, user_id);
        assert_eq!(goal.name, "Vacation");
        assert_eq!(goal.target.primary.amount, dec!(2000));
        assert_eq!(goal.target.primary.currency, Currency::USD);
        assert_eq!(goal.target.dabloons.amount, dec!(0));
        assert_eq!(goal.currency_type, crate::domain::savings_goal::SavingsCurrencyType::TraditionalOnly);
    }

    #[test]
    fn test_savings_goal_creation_dabloons_only() {
        let user_id = Uuid::new_v4();
        let goal = SavingsGoal::new(
            user_id,
            "Gaming".to_string(),
            Money::new(dec!(500), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(180),
        );
        
        assert_eq!(goal.user_id, user_id);
        assert_eq!(goal.name, "Gaming");
        assert_eq!(goal.target.primary.amount, dec!(0));
        assert_eq!(goal.target.dabloons.amount, dec!(500));
        assert_eq!(goal.target.dabloons.currency, Currency::Dabloons);
        assert_eq!(goal.currency_type, crate::domain::savings_goal::SavingsCurrencyType::DabloonsOnly);
    }

    #[test]
    fn test_mixed_savings_goal_creation() {
        let user_id = Uuid::new_v4();
        let result = SavingsGoal::new_mixed(
            user_id,
            "Dream Car".to_string(),
            Money::new(dec!(5000), Currency::USD),
            Money::new(dec!(1000), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(730),
        );
        
        assert!(result.is_ok());
        let goal = result.unwrap();
        assert_eq!(goal.user_id, user_id);
        assert_eq!(goal.name, "Dream Car");
        assert_eq!(goal.target.primary.amount, dec!(5000));
        assert_eq!(goal.target.primary.currency, Currency::USD);
        assert_eq!(goal.target.dabloons.amount, dec!(1000));
        assert_eq!(goal.target.dabloons.currency, Currency::Dabloons);
        assert_eq!(goal.currency_type, crate::domain::savings_goal::SavingsCurrencyType::Mixed);
    }

    #[test]
    fn test_mixed_savings_goal_creation_invalid_currency() {
        let user_id = Uuid::new_v4();
        let result = SavingsGoal::new_mixed(
            user_id,
            "Dream Car".to_string(),
            Money::new(dec!(5000), Currency::Dabloons), // Invalid - should be traditional currency
            Money::new(dec!(1000), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(730),
        );
        
        assert!(result.is_err());
    }

    #[test]
    fn test_savings_goal_progress_percentage() {
        let user_id = Uuid::new_v4();
        let mut goal = SavingsGoal::new_mixed(
            user_id,
            "Dream Car".to_string(),
            Money::new(dec!(5000), Currency::USD),
            Money::new(dec!(1000), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(730),
        ).unwrap();
        
        // Update current amounts
        goal.current.primary = Money::new(dec!(2500), Currency::USD);
        goal.current.dabloons = Money::new(dec!(500), Currency::Dabloons);
        
        let percentage = goal.progress_percentage();
        // Based on our implementation: (2500 + 500*0.01) / (5000 + 1000*0.01) * 100
        // = (2500 + 5) / (5000 + 10) * 100
        // = 2505 / 5010 * 100
        // ≈ 49.99%
        assert!(percentage > dec!(49) && percentage < dec!(51));
    }

    #[tokio::test]
    async fn test_savings_service_create_mixed_goal() {
        let savings_repo = Arc::new(MockSavingsRepository::new());
        let data_sharing_repo = Arc::new(MockDataSharingRepository::new());
        let savings_service = SavingsServiceImpl::new(savings_repo, data_sharing_repo);
        let user_id = Uuid::new_v4();
        
        let result = savings_service.create_mixed_goal(
            user_id,
            "Dream Car".to_string(),
            Money::new(dec!(5000), Currency::USD),
            Money::new(dec!(1000), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(730),
        ).await;
        
        assert!(result.is_ok());
        let goal = result.unwrap();
        assert_eq!(goal.user_id, user_id);
        assert_eq!(goal.name, "Dream Car");
        assert_eq!(goal.target.primary.amount, dec!(5000));
        assert_eq!(goal.target.dabloons.amount, dec!(1000));
        assert_eq!(goal.currency_type, crate::domain::savings_goal::SavingsCurrencyType::Mixed);
    }

    #[tokio::test]
    async fn test_savings_service_add_contribution() {
        let mut mock_savings_repo = MockSavingsRepository::new();
        let goal = SavingsGoal::new_mixed(
            Uuid::new_v4(),
            "Dream Car".to_string(),
            Money::new(dec!(5000), Currency::USD),
            Money::new(dec!(1000), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(730),
        ).unwrap();
        mock_savings_repo.goal = Some(goal);
        
        let savings_repo = Arc::new(mock_savings_repo);
        let data_sharing_repo = Arc::new(MockDataSharingRepository::new());
        let savings_service = SavingsServiceImpl::new(savings_repo, data_sharing_repo);
        let goal_id = Uuid::new_v4();
        
        let result = savings_service.add_contribution(
            goal_id,
            Money::new(dec!(100), Currency::USD)
        ).await;
        
        assert!(result.is_ok());
        let updated_goal = result.unwrap();
        assert_eq!(updated_goal.current.primary.amount, dec!(100));
    }

    #[tokio::test]
    async fn test_savings_service_add_dabloons_contribution() {
        let mut mock_savings_repo = MockSavingsRepository::new();
        let goal = SavingsGoal::new_mixed(
            Uuid::new_v4(),
            "Dream Car".to_string(),
            Money::new(dec!(5000), Currency::USD),
            Money::new(dec!(1000), Currency::Dabloons),
            Utc::now() + chrono::Duration::days(730),
        ).unwrap();
        mock_savings_repo.goal = Some(goal);
        
        let savings_repo = Arc::new(mock_savings_repo);
        let data_sharing_repo = Arc::new(MockDataSharingRepository::new());
        let savings_service = SavingsServiceImpl::new(savings_repo, data_sharing_repo);
        let goal_id = Uuid::new_v4();
        
        let result = savings_service.add_contribution(
            goal_id,
            Money::new(dec!(50), Currency::Dabloons)
        ).await;
        
        assert!(result.is_ok());
        let updated_goal = result.unwrap();
        assert_eq!(updated_goal.current.dabloons.amount, dec!(50));
    }
}