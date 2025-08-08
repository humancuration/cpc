//! # Machine Learning Core for CPC Platform
//!
//! This crate provides a comprehensive machine learning framework built on top of
//! the `linfa` library, designed specifically for cooperative values and community impact.
//!
//! ## Features
//!
//! - Cooperative values-aware model training
//! - Privacy-preserving machine learning
//! - Explainable AI with community-focused interpretations
//! - Integration with CPC's mathematical ecosystem
//! - Bias detection and mitigation
//!
//! ## Modules
//!
//! - `engine`: Core ML engine with cooperative values integration
//! - `models`: Pre-built models for cooperative applications
//! - `evaluation`: Model evaluation with community impact metrics
//! - `explainability`: Tools for explaining ML predictions in accessible terms
//! - `privacy`: Privacy-preserving techniques for ML training
//! - `bias`: Bias detection and mitigation tools
//! - `error`: Common error types for the ML ecosystem

pub mod engine;
pub mod models;
pub mod evaluation;
pub mod explainability;
pub mod privacy;
pub mod bias;
pub mod error;
pub mod cooperative_values;

// Re-export commonly used items
pub use engine::MLEngine;
pub use models::ModelType;
pub use error::{MLError, MLResult};
pub use cooperative_values::CooperativeValues;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ml_core_creation() {
        let engine = MLEngine::new();
        assert!(true); // Creation should succeed
    }
}