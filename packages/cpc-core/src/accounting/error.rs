//! Error types for accounting system

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AccountingError {
    #[error("Account not found: {0}")]
    AccountNotFound(uuid::Uuid),

    #[error("Inactive account: {0}")]
    InactiveAccount(uuid::Uuid),

    #[error("Currency mismatch - expected: {expected}, actual: {actual}")]
    CurrencyMismatch { expected: String, actual: String },

    #[error("Unbalanced transaction - debits: {debits}, credits: {credits}")]
    UnbalancedTransaction { debits: super::Money, credits: super::Money },

    #[error("Invalid transaction structure - must have at least one debit and one credit")]
    InvalidTransactionStructure,

    #[error("Invalid amount - cannot be negative: {0}")]
    InvalidAmount(super::Money),

    #[error("Arithmetic overflow in amount calculation")]
    ArithmeticOverflow,

    #[error("Ledger operation error: {0}")]
    LedgerError(String),

    #[error("Insufficient funds in account {0}")]
    InsufficientFunds(uuid::Uuid),

    #[error("Invalid account code: {0}")]
    InvalidAccountCode(String),

    #[error("Account code already exists: {0}")]
    DuplicateAccountCode(String),
}

impl From<std::num::ParseIntError> for AccountingError {
    fn from(_: std::num::ParseIntError) -> Self {
        AccountingError::ArithmeticOverflow
    }
}