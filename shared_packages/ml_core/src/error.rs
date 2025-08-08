//! Error types for the ML Core crate

use thiserror::Error;

/// Result type for ML operations
pub type MLResult<T> = std::result::Result<T, MLError>;

/// Error types for ML operations
#[derive(Error, Debug)]
pub enum MLError {
    /// Error from the linfa library
    #[error("Linfa error: {0}")]
    LinfaError(#[from] linfa::Error),

    /// Error from ndarray operations
    #[error("NDArray error: {0}")]
    NDArrayError(#[from] ndarray::ShapeError),

    /// Error from data processing
    #[error("Data processing error: {0}")]
    DataError(String),

    /// Error from model training
    #[error("Model training error: {0}")]
    TrainingError(String),

    /// Error from model evaluation
    #[error("Model evaluation error: {0}")]
    EvaluationError(String),

    /// Error from privacy constraints
    #[error("Privacy constraint violation: {0}")]
    PrivacyError(String),

    /// Error from bias detection
    #[error("Bias detection error: {0}")]
    BiasError(String),

    /// Error from consent management
    #[error("Consent management error: {0}")]
    ConsentError(String),

    /// Error from explainability features
    #[error("Explainability error: {0}")]
    ExplainabilityError(String),

    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParameters(String),

    /// Model not found
    #[error("Model not found: {0}")]
    ModelNotFound(String),

    /// Feature not implemented
    #[error("Feature not implemented: {0}")]
    NotImplemented(String),
}