//! Financial Impact Tracker
//!
//! Tracks and measures the financial impact of community activities and initiatives.
//! Integrates with cpay_core and cpc-financial-core to provide comprehensive
//! financial analytics and reporting.

pub mod tracker;
pub mod analytics;
pub mod reporting;
pub mod integration;

pub use tracker::FinancialImpactTracker;
pub use analytics::FinancialAnalytics;
pub use reporting::FinancialReportGenerator;
pub use integration::FinancialIntegration;

/// Financial impact tracking error types
#[derive(thiserror::Error, Debug)]
pub enum FinancialImpactError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("Financial calculation error: {0}")]
    FinancialError(#[from] rust_decimal::Error),
    
    #[error("Invalid transaction: {0}")]
    InvalidTransaction(String),
    
    #[error("Insufficient funds: {0}")]
    InsufficientFunds(String),
    
    #[error("Invalid currency: {0}")]
    InvalidCurrency(String),
    
    #[error("Time range error: {0}")]
    TimeRangeError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}