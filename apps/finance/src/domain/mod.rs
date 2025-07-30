//! Domain layer for the finance module
//!
//! Contains the core business logic and models for personal finance management.
pub mod rewards;

pub mod budget;
pub mod savings_goal;
pub mod primitives;
pub mod wallet;
pub mod expense_tracker;
pub mod currency;

/// Common error types for the finance domain
#[derive(thiserror::Error, Debug)]
pub enum FinanceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Savings goal not found: {0}")]
    SavingsGoalNotFound(uuid::Uuid),
    
    #[error("Budget not found for user {0} and category {1}")]
    BudgetNotFound(uuid::Uuid, String),
    
    #[error("Invalid amount: {0}")]
    InvalidAmount(String),
    
    #[error("Financial error: {0}")]
    FinancialError(#[from] crate::domain::primitives::FinancialError),
    
    #[error("Wallet not found for user {0}")]
    WalletNotFound(uuid::Uuid),
    
    #[error("Insufficient wallet balance")]
    InsufficientWalletBalance,
    
    #[error("Insufficient funds in {0}")]
    InsufficientFunds(crate::domain::primitives::Currency),
    
    #[error("Budget exceeded for category {0}")]
    BudgetExceeded(String),
    
    #[error("p2p error: {0}")]
    #[cfg(feature = "p2p")]
    P2PError(String),
}