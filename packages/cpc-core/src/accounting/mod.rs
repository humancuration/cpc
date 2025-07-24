//! Accounting module for business tools
//! Provides core accounting functionality including ledger management,
//! transaction processing, and financial reporting

use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod ledger;
pub mod transaction;
pub mod account;
pub mod report;
pub mod service;

pub use ledger::*;
pub use transaction::*;
pub use account::*;
pub use report::*;
pub use service::*;

/// Core accounting error type
#[derive(Debug, thiserror::Error)]
pub enum AccountingError {
    #[error("Account not found: {0}")]
    AccountNotFound(Uuid),
    
    #[error("Insufficient funds in account {0}")]
    InsufficientFunds(Uuid),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Duplicate transaction: {0}")]
    DuplicateTransaction(Uuid),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] rusqlite::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
}

/// Result type for accounting operations
pub type AccountingResult<T> = Result<T, AccountingError>;

/// Basic financial amount type
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64, // Store as cents to avoid floating point issues
    pub currency: String,
}

impl Money {
    pub fn new(amount: f64, currency: &str) -> Self {
        Self {
            amount: (amount * 100.0).round() as i64,
            currency: currency.to_string(),
        }
    }
    
    pub fn to_float(&self) -> f64 {
        self.amount as f64 / 100.0
    }
    
    pub fn zero(currency: &str) -> Self {
        Self {
            amount: 0,
            currency: currency.to_string(),
        }
    }
}

impl std::ops::Add for Money {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        assert_eq!(self.currency, other.currency, "Cannot add different currencies");
        Self {
            amount: self.amount + other.amount,
            currency: self.currency,
        }
    }
}

impl std::ops::Sub for Money {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        assert_eq!(self.currency, other.currency, "Cannot subtract different currencies");
        Self {
            amount: self.amount - other.amount,
            currency: self.currency,
        }
    }
}

/// Transaction status
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TransactionStatus {
    Draft,
    Pending,
    Posted,
    Reversed,
    Cancelled,
}

/// Account type classification
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

/// Period type for reporting
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PeriodType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

/// Configuration for accounting service
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingConfig {
    pub base_currency: String,
    pub fiscal_year_start: chrono::NaiveDate,
    pub enable_multi_currency: bool,
    pub enable_budgeting: bool,
    pub enable_tax_tracking: bool,
}

impl Default for AccountingConfig {
    fn default() -> Self {
        Self {
            base_currency: "USD".to_string(),
            fiscal_year_start: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
            enable_multi_currency: false,
            enable_budgeting: true,
            enable_tax_tracking: true,
        }
    }
}