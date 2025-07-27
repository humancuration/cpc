//! Database models for personal finance module
//! These structs map directly to database tables and are used by repositories

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use cpc_core::finance::{Money, Currency};

/// Database model for budgets table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BudgetDbModel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocated_amount: Decimal,
    pub spent_amount: Decimal,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl BudgetDbModel {
    /// Convert domain Budget to database model
    pub fn from_domain(budget: &crate::budgeting::domain::models::Budget) -> Self {
        Self {
            id: budget.id,
            user_id: budget.user_id,
            category: budget.category.clone(),
            allocated_amount: budget.allocated_amount.amount,
            spent_amount: budget.spent_amount.amount,
            period_start: budget.start_date,
            period_end: budget.end_date,
            created_at: budget.start_date, // Using start_date as created_at for now
            updated_at: Utc::now(),
        }
    }

    /// Convert database model to domain Budget
    pub fn to_domain(&self, currency: Currency) -> crate::budgeting::domain::models::Budget {
        crate::budgeting::domain::models::Budget {
            id: self.id,
            user_id: self.user_id,
            category: self.category.clone(),
            allocated_amount: Money::new(self.allocated_amount, currency.clone()),
            spent_amount: Money::new(self.spent_amount, currency),
            period: crate::budgeting::domain::models::BudgetPeriod::Custom, // Simplified for now
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
    pub deadline: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SavingsGoalDbModel {
    /// Convert domain SavingsGoal to database model
    pub fn from_domain(goal: &crate::savings_goals::domain::models::SavingsGoal) -> Self {
        Self {
            id: goal.id,
            user_id: goal.user_id,
            name: goal.name.clone(),
            target_amount: goal.target_amount.amount,
            current_amount: goal.current_amount.amount,
            deadline: Some(goal.target_date),
            created_at: goal.created_at,
            updated_at: goal.updated_at,
        }
    }

    /// Convert database model to domain SavingsGoal
    pub fn to_domain(&self, currency: Currency) -> crate::savings_goals::domain::models::SavingsGoal {
        crate::savings_goals::domain::models::SavingsGoal {
            id: self.id,
            user_id: self.user_id,
            name: self.name.clone(),
            target_amount: Money::new(self.target_amount, currency.clone()),
            current_amount: Money::new(self.current_amount, currency),
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
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DataSharingPreferenceDbModel {
    /// Convert domain model to database model
    pub fn from_domain(preference: &DataSharingPreference) -> Self {
        Self {
            id: preference.id,
            user_id: preference.user_id,
            data_sharing_enabled: preference.data_sharing_enabled,
            anonymized_data: preference.anonymized_data,
            created_at: preference.created_at,
            updated_at: preference.updated_at,
        }
    }

    /// Convert database model to domain model
    pub fn to_domain(&self) -> DataSharingPreference {
        DataSharingPreference {
            id: self.id,
            user_id: self.user_id,
            data_sharing_enabled: self.data_sharing_enabled,
            anonymized_data: self.anonymized_data,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

/// Domain model for data sharing preferences
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DataSharingPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub data_sharing_enabled: bool,
    pub anonymized_data: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl DataSharingPreference {
    pub fn new(user_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            data_sharing_enabled: false,
            anonymized_data: false,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn enable_sharing(&mut self) {
        self.data_sharing_enabled = true;
        self.updated_at = Utc::now();
    }

    pub fn disable_sharing(&mut self) {
        self.data_sharing_enabled = false;
        self.updated_at = Utc::now();
    }

    pub fn enable_anonymization(&mut self) {
        self.anonymized_data = true;
        self.updated_at = Utc::now();
    }

    pub fn disable_anonymization(&mut self) {
        self.anonymized_data = false;
        self.updated_at = Utc::now();
    }
}