//! Error types for the BI analytics framework

use thiserror::Error;
use polars::error::PolarsError;

/// Error types for analytics operations
#[derive(Error, Debug)]
pub enum AnalyticsError {
    #[error("Data processing error: {0}")]
    DataProcessing(String),
    
    #[error("Query execution error: {0}")]
    QueryExecution(String),
    
    #[error("Data source error: {0}")]
    DataSource(String),
    
    #[error("Privacy violation: {0}")]
    PrivacyViolation(String),
    
    #[error("Consent violation: {0}")]
    ConsentViolation(String),
    
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Polars error: {0}")]
    Polars(#[from] PolarsError),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Financial error: {0}")]
    Financial(String),
    
    #[error("Statistical error: {0}")]
    Statistical(String),
}

impl From<cpc_statistics_core::StatisticalError> for AnalyticsError {
    fn from(err: cpc_statistics_core::StatisticalError) -> Self {
        AnalyticsError::Statistical(err.to_string())
    }
}

impl From<consent_manager::domain::errors::ConsentError> for AnalyticsError {
    fn from(err: consent_manager::domain::errors::ConsentError) -> Self {
        AnalyticsError::ConsentViolation(err.to_string())
    }
}

impl From<common_utils::financial::FinancialError> for AnalyticsError {
    fn from(err: common_utils::financial::FinancialError) -> Self {
        AnalyticsError::Financial(err.to_string())
    }
}