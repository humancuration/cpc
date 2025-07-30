//! Finance module for the CPC platform
//!
//! This module provides comprehensive personal finance functionality including:
//! - Budget management
//! - Expense tracking
//! - Savings goals
//! - Financial insights
//! - Privacy-preserving data sharing
//!
//! # Architecture
//!
//! This module follows hexagonal (clean) architecture with vertical slices:
//!
//! - **Budgeting Slice**: Monthly allocation, category tracking, utilization analytics
//! - **Savings Goals Slice**: Goal planning, progress tracking, auto-deduction
//! - **Domain Module**: Cross-cutting financial primitives and shared models
//!
//! # Key Features
//!
//! - Budget creation and management
//! - Savings goal planning
//! - Financial insights and trends
//! - UBI (Universal Basic Income) integration
//! - Privacy-preserving data sharing with explicit user consent
//! - p2p data sharing using p2panda with Double Ratchet encryption
//! - Bevy visualization components
//! - Yew web components
//! - Expense tracking with receipt scanning

// Vertical slices
pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;
pub mod expense_tracker;

/// Re-export commonly used types
pub use domain::{
    budget::Budget,
    savings_goal::SavingsGoal,
    primitives::{Money, Currency, Amount},
    wallet::{Wallet, WalletTransaction, TransactionType},
    currency::{Currency as FullCurrency, CurrencyCode, CurrencyRegistry, ExchangeRate, ExchangeRateService},
};

pub use application::currency::{
    CurrencyService,
    UserCurrencyPreferences,
    CurrencyServiceError,
    CurrencyPreferencesError,
};

#[cfg(test)]
mod tests {
    use super::*;
    use domain::primitives::Currency;
    use domain::budget::{Budget, BudgetPeriod};
    use domain::savings_goal::SavingsGoal;
    use rust_decimal::Decimal;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_budget_creation() {
        let user_id = Uuid::new_v4();
        let budget = Budget::new(
            user_id,
            "Groceries".to_string(),
            Money::new(Decimal::new(500, 0), Currency::USD),
            BudgetPeriod::Monthly,
            Utc::now(),
            Utc::now() + chrono::Duration::days(30),
        );
        
        assert_eq!(budget.user_id, user_id);
        assert_eq!(budget.category, "Groceries");
        assert_eq!(budget.allocated_amount.amount, Decimal::new(500, 0));
        assert_eq!(budget.allocated_amount.currency, Currency::USD);
    }

    #[test]
    fn test_savings_goal_creation() {
        let user_id = Uuid::new_v4();
        let goal = SavingsGoal::new(
            user_id,
            "Vacation".to_string(),
            Money::new(Decimal::new(2000, 0), Currency::USD),
            Utc::now() + chrono::Duration::days(365),
        );
        
        assert_eq!(goal.user_id, user_id);
        assert_eq!(goal.name, "Vacation");
        assert_eq!(goal.target_amount.amount, Decimal::new(2000, 0));
        assert_eq!(goal.target_amount.currency, Currency::USD);
        assert_eq!(goal.current_amount.amount, Decimal::new(0, 0));
    }
}

#[cfg(test)]
mod wallet_test;

#[cfg(test)]
mod budget_test;

#[cfg(test)]
mod savings_goal_test;

#[cfg(test)]
mod wallet_budget_test;

#[cfg(test)]
mod expense_tracker_test;

#[cfg(test)]
mod currency_integration_test;