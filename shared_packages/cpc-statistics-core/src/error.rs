//! Statistical error types and handling
//!
//! This module provides standardized error handling for statistical operations
//! with user-friendly messages that align with cooperative values.

use thiserror::Error;
use statrs::StatsError;
use polars::error::PolarsError;

/// Statistical error types
#[derive(Error, Debug)]
pub enum StatisticalError {
    /// Insufficient data for analysis
    #[error("Insufficient data: observed {0} samples, required {1}")]
    InsufficientData(usize, usize),
    
    /// Data does not follow a normal distribution
    #[error("Data does not follow a normal distribution")]
    NonNormalDistribution,
    
    /// Data is contaminated by outliers
    #[error("Data is contaminated by outliers ({0:.2}% affected)")]
    OutlierContamination(f64),
    
    /// Model failed to converge
    #[error("Model failed to converge")]
    ConvergenceFailure,
    
    /// Model divergence during computation
    #[error("Model divergence in {0}")]
    ModelDivergence(String),
    
    /// Error from statrs library
    #[error("Statistical computation error: {0}")]
    StatrsError(#[from] StatsError),
    
    /// Error from polars library
    #[error("Data processing error: {0}")]
    PolarsError(#[from] PolarsError),
    
    /// Invalid input parameters
    #[error("Invalid input parameters: {0}")]
    InvalidInput(String),
}

impl StatisticalError {
    /// Provide user-friendly message with actionable steps
    pub fn user_message(&self) -> String {
        match self {
            StatisticalError::InsufficientData(observed, required) => {
                format!(
                    "Analysis requires {} data points (only {} available). \
                     To resolve this issue: \
                     1) Collect more data points \
                     2) Use a different statistical method that works with smaller datasets \
                     3) Contact the statistics support team for guidance",
                    required, observed
                )
            },
            StatisticalError::NonNormalDistribution => {
                format!(
                    "The data does not follow a normal distribution, which is required for this analysis. \
                     To resolve this issue: \
                     1) Check for data entry errors \
                     2) Transform the data (e.g., logarithmic transformation) \
                     3) Use non-parametric statistical methods \
                     4) Consult the cooperative's statistics guide for more options"
                )
            },
            StatisticalError::OutlierContamination(percentage) => {
                format!(
                    "The data is contaminated by outliers ({:.2}% of data points). \
                     To resolve this issue: \
                     1) Review data for entry errors \
                     2) Use robust statistical methods that are less sensitive to outliers \
                     3) Consider removing outliers if they are data errors \
                     4) Document any data exclusions in the audit trail",
                    percentage
                )
            },
            StatisticalError::ConvergenceFailure => {
                format!(
                    "The statistical model failed to converge to a solution. \
                     To resolve this issue: \
                     1) Check input data for anomalies \
                     2) Adjust model parameters \
                     3) Try a different statistical approach \
                     4) Contact the cooperative's statistics team for assistance"
                )
            },
            StatisticalError::ModelDivergence(model_name) => {
                format!(
                    "The {} model is diverging during computation. \
                     To resolve this issue: \
                     1) Verify model assumptions are met \
                     2) Check input data quality \
                     3) Adjust model hyperparameters \
                     4) Consult the cooperative's model documentation",
                    model_name
                )
            },
            StatisticalError::StatrsError(_) => {
                format!(
                    "A statistical computation error occurred. \
                     To resolve this issue: \
                     1) Check input data validity \
                     2) Verify parameter ranges \
                     3) Review the technical documentation \
                     4) Report this issue to the development team"
                )
            },
            StatisticalError::PolarsError(_) => {
                format!(
                    "A data processing error occurred. \
                     To resolve this issue: \
                     1) Check data format and structure \
                     2) Verify data types match expectations \
                     3) Ensure sufficient memory is available \
                     4) Contact the data engineering team for support"
                )
            },
            StatisticalError::InvalidInput(details) => {
                format!(
                    "Invalid input parameters: {}. \
                     To resolve this issue: \
                     1) Review parameter values \
                     2) Check parameter documentation \
                     3) Ensure values are within valid ranges \
                     4) Contact support if the issue persists",
                    details
                )
            },
        }
    }
    
    /// Provide methodology source information
    pub fn methodology_source(&self) -> String {
        match self {
            StatisticalError::InsufficientData(_, _) => {
                "Sample Size Requirements - CPC Statistical Guidelines v1.2".to_string()
            },
            StatisticalError::NonNormalDistribution => {
                "Distribution Testing - CPC Statistical Methods Handbook".to_string()
            },
            StatisticalError::OutlierContamination(_) => {
                "Outlier Detection and Handling - CPC Data Quality Standards".to_string()
            },
            StatisticalError::ConvergenceFailure => {
                "Model Convergence - CPC Machine Learning Best Practices".to_string()
            },
            StatisticalError::ModelDivergence(_) => {
                "Model Stability - CPC Statistical Modeling Guidelines".to_string()
            },
            StatisticalError::StatrsError(_) => {
                "Statistical Computation - Statrs Library Documentation".to_string()
            },
            StatisticalError::PolarsError(_) => {
                "Data Processing - Polars Library Documentation".to_string()
            },
            StatisticalError::InvalidInput(_) => {
                "Parameter Validation - CPC API Documentation".to_string()
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insufficient_data_error() {
        let error = StatisticalError::InsufficientData(50, 100);
        assert!(error.user_message().contains("Analysis requires 100 data points (only 50 available)"));
        assert!(error.methodology_source().contains("Sample Size Requirements"));
    }

    #[test]
    fn test_non_normal_distribution_error() {
        let error = StatisticalError::NonNormalDistribution;
        assert!(error.user_message().contains("does not follow a normal distribution"));
        assert!(error.methodology_source().contains("Distribution Testing"));
    }

    #[test]
    fn test_outlier_contamination_error() {
        let error = StatisticalError::OutlierContamination(15.5);
        assert!(error.user_message().contains("contaminated by outliers (15.50%"));
        assert!(error.methodology_source().contains("Outlier Detection"));
    }
}