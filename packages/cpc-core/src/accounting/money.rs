//! Money type for accounting operations

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Money type that stores amounts as cents to avoid floating point precision issues
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Money {
    pub amount: i64, // Amount in cents
    pub currency: String,
}

impl Money {
    pub fn new(amount: f64, currency: &str) -> Self {
        Self {
            amount: (amount * 100.0).round() as i64,
            currency: currency.to_string(),
        }
    }

    pub fn zero(currency: &str) -> Self {
        Self {
            amount: 0,
            currency: currency.to_string(),
        }
    }

    pub fn to_float(&self) -> f64 {
        self.amount as f64 / 100.0
    }

    pub fn add(&self, other: &Self) -> Result<Self, AccountingError> {
        if self.currency != other.currency {
            return Err(AccountingError::CurrencyMismatch {
                expected: self.currency.clone(),
                actual: other.currency.clone(),
            });
        }
        Ok(Self {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    pub fn subtract(&self, other: &Self) -> Result<Self, AccountingError> {
        if self.currency != other.currency {
            return Err(AccountingError::CurrencyMismatch {
                expected: self.currency.clone(),
                actual: other.currency.clone(),
            });
        }
        Ok(Self {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }

    pub fn negate(&self) -> Result<Self, AccountingError> {
        Ok(Self {
            amount: -self.amount,
            currency: self.currency.clone(),
        })
    }

    pub fn is_positive(&self) -> bool {
        self.amount > 0
    }

    pub fn is_negative(&self) -> bool {
        self.amount < 0
    }

    pub fn is_zero(&self) -> bool {
        self.amount == 0
    }
}

/// Error type for money operations
#[derive(Error, Debug)]
pub enum AccountingError {
    #[error("Currency mismatch - expected: {expected}, actual: {actual}")]
    CurrencyMismatch { expected: String, actual: String },

    #[error("Invalid amount - cannot be negative")]
    NegativeAmount,

    #[error("Arithmetic overflow")]
    Overflow,
}