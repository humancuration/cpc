//! Error types for impact reporting

use thiserror::Error;

/// Error type for impact reporting operations
#[derive(Error, Debug)]
pub enum ImpactError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid metric value: {0}")]
    InvalidMetric(String),
    
    #[error("Organization not found: {0}")]
    OrganizationNotFound(uuid::Uuid),
    
    #[error("Invalid year: {0}")]
    InvalidYear(i32),
    
    #[error("Data validation error: {0}")]
    ValidationError(String),
    
    #[error("Calculation error: {0}")]
    CalculationError(String),
}

impl From<sqlx::Error> for ImpactError {
    fn from(error: sqlx::Error) -> Self {
        ImpactError::DatabaseError(error.to_string())
    }
}

impl From<anyhow::Error> for ImpactError {
    fn from(error: anyhow::Error) -> Self {
        ImpactError::CalculationError(error.to_string())
    }
}