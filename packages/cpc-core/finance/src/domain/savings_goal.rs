//! Savings goals domain models

use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use super::primitives::{Money, Currency};

/// Savings goal model for tracking savings objectives
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SavingsGoal {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub target_amount: Money,
    pub current_amount: Money,
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
        target_amount: Money,
        target_date: DateTime<Utc>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            user_id,
            name,
            target_amount: target_amount.clone(),
            current_amount: Money::zero(target_amount.currency),
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
            let current = self.current_amount.amount;
            let target = self.target_amount.amount;
            if target.is_zero() {
                Decimal::ZERO
            } else {
                (current / target) * Decimal::from(100)
            }
        }
    }

    pub fn remaining_amount(&self) -> Money {
        self.target_amount.subtract(&self.current_amount).unwrap_or_else(|_| {
            Money::zero(self.target_amount.currency.clone())
        })
    }

    pub fn is_complete(&self) -> bool {
        self.current_amount.amount >= self.target_amount.amount
    }

    pub fn days_until_target(&self) -> i64 {
        (self.target_date - Utc::now()).num_days()
    }

    pub fn monthly_savings_needed(&self) -> Money {
        let remaining = self.remaining_amount();
        let days = self.days_until_target();
        if days <= 0 {
            remaining
        } else {
            let months = Decimal::from(days) / Decimal::from(30);
            if months.is_zero() {
                remaining
            } else {
                let amount = remaining.amount / months;
                Money::new(amount, remaining.currency)
            }
        }
    }
}

/// Savings progress tracking structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsProgress {
    pub goal_id: Uuid,
    pub current_amount: Money,
    pub target_amount: Money,
    pub percentage: Decimal,
    pub days_remaining: i64,
    pub monthly_savings_needed: Money,
}

impl SavingsProgress {
    pub fn from_goal(goal: &SavingsGoal) -> Self {
        Self {
            goal_id: goal.id,
            current_amount: goal.current_amount.clone(),
            target_amount: goal.target_amount.clone(),
            percentage: goal.progress_percentage(),
            days_remaining: goal.days_until_target(),
            monthly_savings_needed: goal.monthly_savings_needed(),
        }
    }
}