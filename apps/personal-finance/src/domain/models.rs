//! Core domain models for personal finance

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Budget model for tracking income allocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocated_amount: Decimal,
    pub spent_amount: Decimal,
    pub period: BudgetPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl Budget {
    pub fn new(
        user_id: Uuid,
        category: String,
        allocated_amount: Decimal,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            category,
            allocated_amount,
            spent_amount: Decimal::ZERO,
            period,
            start_date,
            end_date,
        }
    }

    pub fn remaining_amount(&self) -> Decimal {
        self.allocated_amount - self.spent_amount
    }

    pub fn utilization_percentage(&self) -> Decimal {
        if self.allocated_amount.is_zero() {
            Decimal::ZERO
        } else {
            (self.spent_amount / self.allocated_amount) * Decimal::from(100)
        }
    }

    pub fn is_over_budget(&self) -> bool {
        self.spent_amount > self.allocated_amount
    }
}

/// Budget period enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BudgetPeriod {
    Monthly,
    Weekly,
    BiWeekly,
    Custom,
}

/// Expense model for tracking purchases and payments
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Expense {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: Decimal,
    pub currency: String,
    pub category: String,
    pub description: String,
    pub date: DateTime<Utc>,
    pub receipt_id: Option<Uuid>,
    pub payment_method: String,
    pub merchant: Option<String>,
    pub tags: Vec<String>,
}

impl Expense {
    pub fn new(
        user_id: Uuid,
        amount: Decimal,
        currency: String,
        description: String,
        payment_method: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            amount,
            currency,
            category: "Uncategorized".to_string(),
            description,
            date: Utc::now(),
            receipt_id: None,
            payment_method,
            merchant: None,
            tags: Vec::new(),
        }
    }
}

/// Savings goal model for tracking savings objectives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavingsGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Decimal,
    pub current_amount: Decimal,
    pub target_date: DateTime<Utc>,
    pub auto_deduct: bool,
    pub deduction_percentage: Decimal,
    pub description: Option<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SavingsGoal {
    pub fn new(
        user_id: Uuid,
        name: String,
        target_amount: Decimal,
        target_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            target_amount,
            current_amount: Decimal::ZERO,
            target_date,
            auto_deduct: false,
            deduction_percentage: Decimal::ZERO,
            description: None,
            category: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn progress_percentage(&self) -> Decimal {
        if self.target_amount.is_zero() {
            Decimal::ZERO
        } else {
            (self.current_amount / self.target_amount) * Decimal::from(100)
        }
    }

    pub fn remaining_amount(&self) -> Decimal {
        self.target_amount - self.current_amount
    }

    pub fn is_complete(&self) -> bool {
        self.current_amount >= self.target_amount
    }

    pub fn days_until_target(&self) -> i64 {
        (self.target_date - Utc::now()).num_days()
    }

    pub fn monthly_savings_needed(&self) -> Decimal {
        let days = self.days_until_target();
        if days <= 0 {
            self.remaining_amount()
        } else {
            let months = Decimal::from(days) / Decimal::from(30);
            self.remaining_amount() / months
        }
    }
}

/// Savings progress tracking structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsProgress {
    pub goal_id: Uuid,
    pub current_amount: Decimal,
    pub target_amount: Decimal,
    pub percentage: Decimal,
    pub days_remaining: i64,
    pub monthly_savings_needed: Decimal,
}

impl SavingsProgress {
    pub fn from_goal(goal: &SavingsGoal) -> Self {
        Self {
            goal_id: goal.id,
            current_amount: goal.current_amount,
            target_amount: goal.target_amount,
            percentage: goal.progress_percentage(),
            days_remaining: goal.days_until_target(),
            monthly_savings_needed: goal.monthly_savings_needed(),
        }
    }
}

/// Common error types for finance operations
#[derive(Debug, thiserror::Error)]
pub enum FinanceError {
    #[error("Budget not found: {0}")]
    BudgetNotFound(Uuid),
    
    #[error("Expense not found: {0}")]
    ExpenseNotFound(Uuid),
    
    #[error("Savings goal not found: {0}")]
    SavingsGoalNotFound(Uuid),
    
    #[error("Insufficient funds: required {required}, available {available}")]
    InsufficientFunds {
        required: Decimal,
        available: Decimal,
    },
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Invalid date range: start must be before end")]
    InvalidDateRange,
    
    #[error("UBI service error: {0}")]
    UbiServiceError(String),
    
    #[error("Storage error: {0}")]
    StorageError(String),
    
    #[error("OCR processing error: {0}")]
    OcrError(String),
    
    #[error("Categorization error: {0}")]
    CategorizationError(String),
}

/// Common categorization for expenses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub name: String,
    pub keywords: Vec<String>,
    pub parent_category: Option<String>,
}

impl ExpenseCategory {
    pub fn new(name: String) -> Self {
        Self {
            name,
            keywords: Vec::new(),
            parent_category: None,
        }
    }
}