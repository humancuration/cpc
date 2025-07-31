//! Rewards service for managing Universal Income distribution
//!
//! This module provides functionality for distributing dabloons as Universal Income
//! to all federation members on a daily basis.

use async_trait::async_trait;
use uuid::Uuid;
use chrono::{NaiveDate, Utc};
use crate::domain::{
    rewards::{UniversalIncomeConfig, UIDistribution, UIService},
    primitives::{Money, Currency, FinancialError},
};
use cpc_wallet::domain::wallet::Wallet;
use std::sync::Arc;

/// Repository trait for Universal Income configuration persistence
#[async_trait]
pub trait UIConfigRepository {
    /// Get the current Universal Income configuration
    async fn get_config(&self) -> Result<Option<UniversalIncomeConfig>, FinancialError>;
    
    /// Save the Universal Income configuration
    async fn save_config(&self, config: &UniversalIncomeConfig) -> Result<(), FinancialError>;
}

/// Repository trait for Universal Income distribution records
#[async_trait]
pub trait UIDistributionRepository {
    /// Save a distribution record
    async fn save_distribution(&self, distribution: &UIDistribution) -> Result<(), FinancialError>;
    
    /// Check if a user has received their Universal Income for a specific date
    async fn has_received(&self, user_id: Uuid, date: NaiveDate) -> Result<bool, FinancialError>;
    
    /// Get distribution records for a user
    async fn get_distributions_for_user(&self, user_id: Uuid) -> Result<Vec<UIDistribution>, FinancialError>;
}

/// Service trait for wallet operations needed by the rewards system
#[async_trait]
pub trait RewardsWalletService {
    /// Add dabloons to a user's wallet
    async fn add_dabloons(&self, user_id: Uuid, amount: Money, description: Option<String>) -> Result<Wallet, FinancialError>;
    
    /// Distribute Universal Income to a user's wallet
    async fn distribute_universal_income(&self, user_id: Uuid, amount: Money, distribution_date: NaiveDate) -> Result<Wallet, FinancialError>;
}

/// Implementation of the Universal Income service
pub struct UIServiceImpl {
    config_repo: Arc<dyn UIConfigRepository>,
    distribution_repo: Arc<dyn UIDistributionRepository>,
    wallet_service: Arc<dyn RewardsWalletService>,
}

impl UIServiceImpl {
    /// Create a new Universal Income service
    pub fn new(
        config_repo: Arc<dyn UIConfigRepository>,
        distribution_repo: Arc<dyn UIDistributionRepository>,
        wallet_service: Arc<dyn RewardsWalletService>,
    ) -> Self {
        Self {
            config_repo,
            distribution_repo,
            wallet_service,
        }
    }
}

#[async_trait]
impl UIService for UIServiceImpl {
    /// Calculate the amount of dabloons a user should receive for a given date
    fn calculate_daily_amount(&self, _user_id: Uuid, _date: NaiveDate) -> Result<Money, FinancialError> {
        // In a more complex implementation, this could vary based on user status, participation, etc.
        // For now, we'll use the standard daily amount from the configuration
        match futures::executor::block_on(self.config_repo.get_config()) {
            Ok(Some(config)) => {
                if config.is_active() {
                    Ok(config.daily_amount().clone())
                } else {
                    Ok(Money::zero(Currency::Dabloons))
                }
            },
            Ok(None) => Ok(Money::zero(Currency::Dabloons)),
            Err(e) => Err(e),
        }
    }
    
    /// Check if a user has already received their Universal Income for a given date
    fn has_received_today(&self, user_id: Uuid, date: NaiveDate) -> Result<bool, FinancialError> {
        futures::executor::block_on(self.distribution_repo.has_received(user_id, date))
    }
    
    /// Distribute Universal Income to a user for a specific date
    fn distribute_daily_income(&self, user_id: Uuid, date: NaiveDate) -> Result<UIDistribution, FinancialError> {
        // Check if the user has already received their income for this date
        if self.has_received_today(user_id, date)? {
            return Err(FinancialError::InvalidAmount("User has already received Universal Income for this date".to_string()));
        }
        
        // Calculate the amount to distribute
        let amount = self.calculate_daily_amount(user_id, date)?;
        
        // If the amount is zero, there's nothing to distribute
        if amount.is_zero() {
            return Err(FinancialError::InvalidAmount("No Universal Income to distribute".to_string()));
        }
        
        // Add the dabloons to the user's wallet
        futures::executor::block_on(self.wallet_service.distribute_universal_income(user_id, amount.clone(), date))?;
        
        // Create and save the distribution record
        let distribution = UIDistribution::new(user_id, amount, date)?;
        futures::executor::block_on(self.distribution_repo.save_distribution(&distribution))?;
        
        Ok(distribution)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::rewards::UniversalIncomeConfig;
    use rust_decimal_macros::dec;
    use chrono::NaiveDate;
    use std::collections::HashMap;
    use tokio;

    // Mock implementations for testing
    struct MockUIConfigRepository {
        config: Option<UniversalIncomeConfig>,
    }
    
    #[async_trait]
    impl UIConfigRepository for MockUIConfigRepository {
        async fn get_config(&self) -> Result<Option<UniversalIncomeConfig>, FinancialError> {
            Ok(self.config.clone())
        }
        
        async fn save_config(&self, _config: &UniversalIncomeConfig) -> Result<(), FinancialError> {
            Ok(())
        }
    }
    
    struct MockUIDistributionRepository {
        distributions: HashMap<(Uuid, NaiveDate), bool>,
    }
    
    #[async_trait]
    impl UIDistributionRepository for MockUIDistributionRepository {
        async fn save_distribution(&self, _distribution: &UIDistribution) -> Result<(), FinancialError> {
            Ok(())
        }
        
        async fn has_received(&self, user_id: Uuid, date: NaiveDate) -> Result<bool, FinancialError> {
            Ok(self.distributions.get(&(user_id, date)).cloned().unwrap_or(false))
        }
        
        async fn get_distributions_for_user(&self, _user_id: Uuid) -> Result<Vec<UIDistribution>, FinancialError> {
            Ok(vec![])
        }
    }
    
    struct MockRewardsWalletService;
    
    #[async_trait]
    impl RewardsWalletService for MockRewardsWalletService {
        async fn add_dabloons(&self, _user_id: Uuid, _amount: Money, _description: Option<String>) -> Result<Wallet, FinancialError> {
            // In a real implementation, this would create or update a wallet
            // For testing, we'll just return a mock wallet
            Ok(Wallet::new(_user_id))
        }
        
        async fn distribute_universal_income(&self, _user_id: Uuid, _amount: Money, _distribution_date: NaiveDate) -> Result<Wallet, FinancialError> {
            // In a real implementation, this would create or update a wallet
            // For testing, we'll just return a mock wallet
            Ok(Wallet::new(_user_id))
        }
    }
    
    #[tokio::test]
    async fn test_distribute_daily_income() {
        let daily_amount = Money::new(dec!(10), Currency::Dabloons);
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let config = UniversalIncomeConfig::new(daily_amount.clone(), start_date).unwrap();
        
        let config_repo = Arc::new(MockUIConfigRepository {
            config: Some(config),
        });
        
        let distribution_repo = Arc::new(MockUIDistributionRepository {
            distributions: HashMap::new(),
        });
        
        let wallet_service = Arc::new(MockRewardsWalletService);
        
        let ui_service = UIServiceImpl::new(config_repo, distribution_repo, wallet_service);
        let user_id = Uuid::new_v4();
        let date = NaiveDate::from_ymd_opt(2025, 7, 28).unwrap();
        
        let result = ui_service.distribute_daily_income(user_id, date);
        assert!(result.is_ok());
        
        let distribution = result.unwrap();
        assert_eq!(distribution.user_id, user_id);
        assert_eq!(distribution.amount, daily_amount);
        assert_eq!(distribution.distribution_date, date);
    }
}