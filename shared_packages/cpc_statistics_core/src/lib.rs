//! CPC Statistics Core
//!
//! A statistical framework for the CPC platform that provides:
//! - Standardized error handling for statistical operations
//! - Confidence interval calculations
//! - Statistical significance testing
//! - Effect size calculations
//! - Cooperative values-aligned result presentation

pub mod error;
pub mod confidence;
pub mod significance;

pub use error::StatisticalError;
pub use confidence::{ConfidenceInterval, ConfidenceMethod};
pub use significance::{SignificanceLevel, SignificanceResult};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_imports() {
        // This is a placeholder test to ensure modules compile
        // Actual tests will be in individual module test files
        assert!(true);
    }
}