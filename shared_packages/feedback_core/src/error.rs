//! Unified error types for the feedback system
//!
//! This module defines a common error type that can be used across all feedback-related packages.

use thiserror::Error;
use std::fmt;

/// Unified error type for the feedback system
#[derive(Error, Debug)]
pub enum FeedbackError {
    #[error("Analysis error: {0}")]
    Analysis(String),
    
    #[error("Visualization error: {0}")]
    Visualization(String),
    
    #[error("Data validation failed: {0}")]
    Validation(String),
    
    #[error("Federation error: {0}")]
    Federation(String),
}

impl From<reviews::ValidationError> for FeedbackError {
    fn from(err: reviews::ValidationError) -> Self {
        FeedbackError::Validation(err.to_string())
    }
}

impl From<survey::error::AnalysisError> for FeedbackError {
    fn from(err: survey::error::AnalysisError) -> Self {
        FeedbackError::Analysis(err.to_string())
    }
}