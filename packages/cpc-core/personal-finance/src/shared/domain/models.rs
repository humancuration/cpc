//! Shared financial primitives for the personal finance module

use cpc_core::finance::{Money, Currency, Amount};
use serde::{Deserialize, Serialize};

/// Re-export financial primitives
pub use cpc_core::finance::{Money, Currency, Amount};

/// Date range for financial periods
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DateRange {
    pub start: chrono::DateTime<chrono::Utc>,
    pub end: chrono::DateTime<chrono::Utc>,
}

impl DateRange {
    pub fn new(start: chrono::DateTime<chrono::Utc>, end: chrono::DateTime<chrono::Utc>) -> Result<Self, DateRangeError> {
        if end <= start {
            return Err(DateRangeError::InvalidRange);
        }
        
        Ok(Self { start, end })
    }
    
    pub fn duration_days(&self) -> i64 {
        (self.end - self.start).num_days()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum DateRangeError {
    #[error("Invalid date range: end must be after start")]
    InvalidRange,
}

/// Financial account representation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FinancialAccount {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub name: String,
    pub account_type: AccountType,
    pub currency: Currency,
    pub balance: Money,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccountType {
    Checking,
    Savings,
    Credit,
    Investment,
    Cash,
}

impl FinancialAccount {
    pub fn new(
        user_id: uuid::Uuid,
        name: String,
        account_type: AccountType,
        currency: Currency,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4(),
            user_id,
            name,
            account_type,
            currency,
            balance: Money::zero(currency),
            created_at: chrono::Utc::now(),
        }
    }
}