//! Database models for personal finance module
//! These structs map directly to database tables and are used by repositories

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use crate::domain::primitives::{Money, Currency};
use cpc_wallet::domain::{
    wallet::{Wallet, WalletTransaction, TransactionType},
    primitives::{Money as WalletMoney, Currency as WalletCurrency},
};

/// Database model for budgets table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BudgetDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocated_amount: Decimal,
    pub spent_amount: Decimal,
    pub dabloons_allocated: Decimal,
    pub dabloons_spent: Decimal,
    pub currency_type: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BudgetDbModel {
    /// Convert domain Budget to database model
    pub fn from_domain(budget: &crate::domain::budget::Budget) -> Self {
        Self {
            id: budget.id,
            user_id: budget.user_id,
            category: budget.category.clone(),
            allocated_amount: budget.allocation.primary.amount,
            spent_amount: budget.spent.primary.amount,
            dabloons_allocated: budget.allocation.dabloons.amount,
            dabloons_spent: budget.spent.dabloons.amount,
            currency_type: match budget.currency_type {
                crate::domain::budget::BudgetCurrencyType::TraditionalOnly => "TraditionalOnly".to_string(),
                crate::domain::budget::BudgetCurrencyType::DabloonsOnly => "DabloonsOnly".to_string(),
                crate::domain::budget::BudgetCurrencyType::Mixed => "Mixed".to_string(),
            },
            period_start: budget.start_date,
            period_end: budget.end_date,
            created_at: budget.start_date, // Using start_date as created_at for now
            updated_at: Utc::now(),
        }
    }

    /// Convert database model to domain Budget
    pub fn to_domain(&self, currency: Currency) -> crate::domain::budget::Budget {
        let currency_type = match self.currency_type.as_str() {
            "TraditionalOnly" => crate::domain::budget::BudgetCurrencyType::TraditionalOnly,
            "DabloonsOnly" => crate::domain::budget::BudgetCurrencyType::DabloonsOnly,
            "Mixed" => crate::domain::budget::BudgetCurrencyType::Mixed,
            _ => crate::domain::budget::BudgetCurrencyType::TraditionalOnly, // Default
        };
        
        crate::domain::budget::Budget {
            id: self.id,
            user_id: self.user_id,
            category: self.category.clone(),
            allocation: crate::domain::budget::BudgetAllocation {
                primary: Money::new(self.allocated_amount, currency.clone()),
                dabloons: Money::new(self.dabloons_allocated, Currency::Dabloons),
            },
            spent: crate::domain::budget::BudgetAllocation {
                primary: Money::new(self.spent_amount, currency.clone()),
                dabloons: Money::new(self.dabloons_spent, Currency::Dabloons),
            },
            currency_type,
            period: crate::domain::budget::BudgetPeriod::Custom, // Simplified for now
            start_date: self.period_start,
            end_date: self.period_end,
        }
    }
}

/// Database model for savings_goals table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SavingsGoalDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub target_dabloons: Decimal,
    pub current_dabloons: Decimal,
    pub currency_type: String,
    pub deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SavingsGoalDbModel {
    /// Convert domain SavingsGoal to database model
    pub fn from_domain(goal: &crate::domain::savings_goal::SavingsGoal) -> Self {
        Self {
            id: goal.id,
            user_id: goal.user_id,
            name: goal.name.clone(),
            target_amount: goal.target.primary.amount,
            current_amount: goal.current.primary.amount,
            target_dabloons: goal.target.dabloons.amount,
            current_dabloons: goal.current.dabloons.amount,
            currency_type: match goal.currency_type {
                crate::domain::savings_goal::SavingsCurrencyType::TraditionalOnly => "TraditionalOnly".to_string(),
                crate::domain::savings_goal::SavingsCurrencyType::DabloonsOnly => "DabloonsOnly".to_string(),
                crate::domain::savings_goal::SavingsCurrencyType::Mixed => "Mixed".to_string(),
            },
            deadline: Some(goal.target_date),
            created_at: goal.created_at,
            updated_at: goal.updated_at,
        }
    }

    /// Convert database model to domain SavingsGoal
    pub fn to_domain(&self, currency: Currency) -> crate::domain::savings_goal::SavingsGoal {
        let currency_type = match self.currency_type.as_str() {
            "TraditionalOnly" => crate::domain::savings_goal::SavingsCurrencyType::TraditionalOnly,
            "DabloonsOnly" => crate::domain::savings_goal::SavingsCurrencyType::DabloonsOnly,
            "Mixed" => crate::domain::savings_goal::SavingsCurrencyType::Mixed,
            _ => crate::domain::savings_goal::SavingsCurrencyType::TraditionalOnly, // Default
        };
        
        crate::domain::savings_goal::SavingsGoal {
            id: self.id,
            user_id: self.user_id,
            name: self.name.clone(),
            target: crate::domain::savings_goal::DualCurrencyTarget {
                primary: Money::new(self.target_amount, currency.clone()),
                dabloons: Money::new(self.target_dabloons, Currency::Dabloons),
            },
            current: crate::domain::savings_goal::DualCurrencyTarget {
                primary: Money::new(self.current_amount, currency.clone()),
                dabloons: Money::new(self.current_dabloons, Currency::Dabloons),
            },
            currency_type,
            target_date: self.deadline.unwrap_or_else(|| Utc::now() + chrono::Duration::days(365)),
            auto_deduct: false, // Default value
            deduction_percentage: rust_decimal::Decimal::ZERO, // Default value
            description: None, // Default value
            category: None, // Default value
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Database model for data_sharing_preferences table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct DataSharingPreferenceDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data_sharing_enabled: bool,
    pub anonymized_data: bool,
    pub preferred_currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DataSharingPreferenceDbModel {
    /// Convert domain model to database model
    pub fn from_domain(preference: &crate::application::savings_service::DataSharingPreference) -> Self {
        Self {
            id: preference.id,
            user_id: preference.user_id,
            data_sharing_enabled: preference.data_sharing_enabled,
            anonymized_data: preference.anonymized_data,
            preferred_currency: preference.preferred_currency.clone(),
            created_at: preference.created_at,
            updated_at: preference.updated_at,
        }
    }

    /// Convert database model to domain model
    pub fn to_domain(&self) -> crate::application::savings_service::DataSharingPreference {
        crate::application::savings_service::DataSharingPreference {
            id: self.id,
            user_id: self.user_id,
            data_sharing_enabled: self.data_sharing_enabled,
            anonymized_data: self.anonymized_data,
            preferred_currency: self.preferred_currency.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Database model for wallets table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WalletDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub balance: Decimal,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl WalletDbModel {
    /// Convert domain Wallet to database model
    pub fn from_domain(wallet: &Wallet) -> Self {
        Self {
            id: wallet.id,
            user_id: wallet.user_id,
            balance: wallet.balance.amount,
            created_at: wallet.created_at,
            updated_at: wallet.updated_at,
        }
    }

    /// Convert database model to domain Wallet
    pub fn to_domain(&self) -> Wallet {
        Wallet {
            id: self.id,
            user_id: self.user_id,
            balance: WalletMoney::new(self.balance, WalletCurrency::Dabloons),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Database model for wallet_transactions table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WalletTransactionDbModel {
    pub id: Uuid,
    pub wallet_id: Uuid,
    pub transaction_type: String, // "credit" or "debit"
    pub amount: Decimal,
    pub description: Option<String>,
    pub timestamp: DateTime<Utc>,
}

impl WalletTransactionDbModel {
    /// Convert domain WalletTransaction to database model
    pub fn from_domain(transaction: &WalletTransaction) -> Self {
        Self {
            id: transaction.id,
            wallet_id: transaction.wallet_id,
            transaction_type: match transaction.transaction_type {
                TransactionType::Credit => "credit".to_string(),
                TransactionType::Debit => "debit".to_string(),
            },
            amount: transaction.amount.amount,
            description: transaction.description.clone(),
            timestamp: transaction.timestamp,
        }
    }

    /// Convert database model to domain WalletTransaction
    pub fn to_domain(&self) -> WalletTransaction {
        WalletTransaction {
            id: self.id,
            wallet_id: self.wallet_id,
            transaction_type: match self.transaction_type.as_str() {
                "credit" => TransactionType::Credit,
                "debit" => TransactionType::Debit,
                _ => TransactionType::Credit, // Default to credit if unknown
            },
            amount: WalletMoney::new(self.amount, WalletCurrency::Dabloons),
            description: self.description.clone(),
            timestamp: self.timestamp,
        }
    }
}

/// Database model for ui_config table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UIConfigDbModel {
    pub id: Uuid,
    pub daily_amount: Decimal,
    pub start_date: chrono::NaiveDate,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UIConfigDbModel {
    /// Convert domain UniversalIncomeConfig to database model
    pub fn from_domain(config: &crate::domain::rewards::UniversalIncomeConfig) -> Self {
        Self {
            id: Uuid::new_v4(), // For new records
            daily_amount: config.daily_amount().amount,
            start_date: config.start_date,
            is_active: config.is_active(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    /// Convert database model to domain UniversalIncomeConfig
    pub fn to_domain(&self) -> crate::domain::rewards::UniversalIncomeConfig {
        crate::domain::rewards::UniversalIncomeConfig {
            daily_amount: crate::domain::primitives::Money::new(self.daily_amount, crate::domain::primitives::Currency::Dabloons),
            start_date: self.start_date,
            is_active: self.is_active,
        }
    }
}

/// Database model for ui_distributions table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UIDistributionDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub distribution_date: chrono::NaiveDate,
    pub created_at: DateTime<Utc>,
}

impl UIDistributionDbModel {
    /// Convert domain UIDistribution to database model
    pub fn from_domain(distribution: &crate::domain::rewards::UIDistribution) -> Self {
        Self {
            id: distribution.id,
            user_id: distribution.user_id,
            amount: distribution.amount.amount,
            distribution_date: distribution.distribution_date,
            created_at: distribution.created_at,
        }
    }

    /// Convert database model to domain UIDistribution
    pub fn to_domain(&self) -> crate::domain::rewards::UIDistribution {
        crate::domain::rewards::UIDistribution {
            id: self.id,
            user_id: self.user_id,
            amount: crate::domain::primitives::Money::new(self.amount, crate::domain::primitives::Currency::Dabloons),
            distribution_date: self.distribution_date,
            created_at: self.created_at,
        }
    }
}