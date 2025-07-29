use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::domain::primitives::{FinancialCategory, Money, TimePeriod};

/// Budget entity representing a financial plan for a specific category and time period
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Budget {
    pub id: Uuid,
    pub name: String,
    pub category: FinancialCategory,
    pub amount: Money,
    pub period: TimePeriod,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Budget {
    pub fn new(
        name: String,
        category: FinancialCategory,
        amount: Money,
        period: TimePeriod,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            category,
            amount,
            period,
            created_at: now,
            updated_at: now,
        }
    }
    
    pub fn update_amount(&mut self, new_amount: Money) {
        self.amount = new_amount;
        self.updated_at = Utc::now();
    }
    
    pub fn update_period(&mut self, new_period: TimePeriod) {
        self.period = new_period;
        self.updated_at = Utc::now();
    }
    
    pub fn is_active(&self) -> bool {
        let now = Utc::now();
        self.period.contains(&now)
    }
}