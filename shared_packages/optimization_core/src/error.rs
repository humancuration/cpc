//! Error types for the optimization framework

use thiserror::Error;
use argmin::core::Error as ArgminError;

/// Error types for optimization operations
#[derive(Debug, Error)]
pub enum OptimizationError {
    #[error("Optimization failed to converge: {0}")]
    ConvergenceFailure(String),
    
    #[error("Invalid optimization parameters: {0}")]
    InvalidParameters(String),
    
    #[error("Optimization problem is ill-defined: {0}")]
    IllDefinedProblem(String),
    
    #[error("Unsupported optimization algorithm: {0}")]
    UnsupportedAlgorithm(String),
    
    #[error("Optimization timed out: {0}")]
    Timeout(String),
    
    #[error("Insufficient data for optimization: {0}")]
    InsufficientData(String),
    
    #[error("Cooperative values constraint violation: {0}")]
    CooperativeValuesViolation(String),
    
    #[error("Internal optimization error: {0}")]
    InternalError(#[from] ArgminError),
    
    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),
    
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
}