//! Rewards domain models for the CPC platform
//!
//! This module provides functionality for managing Universal Income distribution
//! of dabloons to all federation members.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use crate::domain::primitives::{Money, Currency, FinancialError};

/// Universal Income distribution configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UniversalIncomeConfig {
    /// Amount of dabloons distributed daily to each federation member
    pub daily_amount: Money,
    
    /// Date when the Universal Income program started
    pub start_date: NaiveDate,
    
    /// Whether the program is currently active
    pub is_active: bool,
}

impl UniversalIncomeConfig {
    /// Create a new Universal Income configuration
    pub fn new(daily_amount: Money, start_date: NaiveDate) -> Result<Self, FinancialError> {
        if daily_amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: daily_amount.currency.code().to_string(),
            });
        }
        
        Ok(Self {
            daily_amount,
            start_date,
            is_active: true,
        })
    }
    
    /// Check if the program is active
    pub fn is_active(&self) -> bool {
        self.is_active
    }
    
    /// Get the daily amount of dabloons distributed
    pub fn daily_amount(&self) -> &Money {
        &self.daily_amount
    }
}

/// Record of a Universal Income distribution to a user
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UIDistribution {
    /// Unique identifier for the distribution record
    pub id: Uuid,
    
    /// User who received the distribution
    pub user_id: Uuid,
    
    /// Amount distributed
    pub amount: Money,
    
    /// Date of distribution
    pub distribution_date: NaiveDate,
    
    /// When the record was created
    pub created_at: DateTime<Utc>,
}

impl UIDistribution {
    /// Create a new Universal Income distribution record
    pub fn new(user_id: Uuid, amount: Money, distribution_date: NaiveDate) -> Result<Self, FinancialError> {
        if amount.currency != Currency::Dabloons {
            return Err(FinancialError::CurrencyMismatch {
                expected: Currency::Dabloons.code().to_string(),
                actual: amount.currency.code().to_string(),
            });
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            distribution_date,
            created_at: Utc::now(),
        })
    }
}

/// Service trait for Universal Income operations
pub trait UIService {
    /// Calculate the amount of dabloons a user should receive for a given date
    fn calculate_daily_amount(&self, user_id: Uuid, date: NaiveDate) -> Result<Money, FinancialError>;
    
    /// Check if a user has already received their Universal Income for a given date
    fn has_received_today(&self, user_id: Uuid, date: NaiveDate) -> Result<bool, FinancialError>;
    
    /// Distribute Universal Income to a user for a specific date
    fn distribute_daily_income(&self, user_id: Uuid, date: NaiveDate) -> Result<UIDistribution, FinancialError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use chrono::NaiveDate;

    #[test]
    fn test_universal_income_config_creation() {
        let daily_amount = Money::new(dec!(10), Currency::Dabloons);
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let config = UniversalIncomeConfig::new(daily_amount.clone(), start_date).unwrap();
        
        assert_eq!(config.daily_amount, daily_amount);
        assert_eq!(config.start_date, start_date);
        assert!(config.is_active());
    }
    
    #[test]
    fn test_universal_income_config_wrong_currency() {
        let daily_amount = Money::new(dec!(10), Currency::USD);
        let start_date = NaiveDate::from_ymd_opt(2025, 1, 1).unwrap();
        let result = UniversalIncomeConfig::new(daily_amount, start_date);
        
        assert!(result.is_err());
    }
    
    #[test]
    fn test_ui_distribution_creation() {
        let user_id = Uuid::new_v4();
        let amount = Money::new(dec!(10), Currency::Dabloons);
        let distribution_date = NaiveDate::from_ymd_opt(2025, 7, 28).unwrap();
        let distribution = UIDistribution::new(user_id, amount.clone(), distribution_date).unwrap();
        
        assert_eq!(distribution.user_id, user_id);
        assert_eq!(distribution.amount, amount);
        assert_eq!(distribution.distribution_date, distribution_date);
    }
    
    #[test]
    fn test_ui_distribution_wrong_currency() {
        let user_id = Uuid::new_v4();
        let amount = Money::new(dec!(10), Currency::USD);
        let distribution_date = NaiveDate::from_ymd_opt(2025, 7, 28).unwrap();
        let result = UIDistribution::new(user_id, amount, distribution_date);
        
        assert!(result.is_err());
    }
}