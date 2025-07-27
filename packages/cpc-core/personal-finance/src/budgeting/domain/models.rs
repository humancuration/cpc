//! Budget domain models

use chrono::{DateTime, Utc};
use uuid::Uuid;
use cpc_core::finance::{Money, Currency};
use serde::{Deserialize, Serialize};

/// Budget model for tracking income allocation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Budget {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub allocated_amount: Money,
    pub spent_amount: Money,
    pub period: BudgetPeriod,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

impl Budget {
    pub fn new(
        user_id: Uuid,
        category: String,
        allocated_amount: Money,
        period: BudgetPeriod,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            category,
            allocated_amount: allocated_amount.clone(),
            spent_amount: Money::zero(allocated_amount.currency),
            period,
            start_date,
            end_date,
        }
    }

    pub fn remaining_amount(&self) -> Money {
        self.allocated_amount.subtract(&self.spent_amount).unwrap_or_else(|_| {
            Money::zero(self.allocated_amount.currency.clone())
        })
    }

    pub fn utilization_percentage(&self) -> rust_decimal::Decimal {
        if self.allocated_amount.is_zero() {
            rust_decimal::Decimal::ZERO
        } else {
            let spent = self.spent_amount.amount;
            let allocated = self.allocated_amount.amount;
            if allocated.is_zero() {
                rust_decimal::Decimal::ZERO
            } else {
                (spent / allocated) * rust_decimal::Decimal::from(100)
            }
        }
    }

    pub fn is_over_budget(&self) -> bool {
        self.spent_amount.amount > self.allocated_amount.amount
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