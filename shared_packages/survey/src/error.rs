//! Error types for the survey module
//!
//! This module defines custom error types used throughout the survey system.

use thiserror::Error;
use feedback_core::FeedbackError;

/// Custom error type for analysis errors
#[derive(Debug, Clone, Error)]
pub enum AnalysisError {
    #[error("E301: Insufficient data for analysis")]
    InsufficientData,
    
    #[error("E302: Invalid question index: {0}")]
    InvalidQuestionIndex(usize),
    
    #[error("E303: Question type not supported for this analysis")]
    UnsupportedQuestionType,
    
    #[error("E304: Correlation calculation failed: {0}")]
    CorrelationError(String),
    
    #[error("E305: Trend analysis failed: {0}")]
    TrendAnalysisError(String),
    
    #[error("E306: Sentiment analysis failed: {0}")]
    SentimentAnalysisError(String),
    
    #[error("E307: Comparative analysis failed: {0}")]
    ComparativeAnalysisError(String),
    
    #[error("E308: Sampling error: {0}")]
    SamplingError(String),
    
    #[error("E309: Caching error: {0}")]
    CachingError(String),
    
    #[error("E399: Chained analysis error: {source}")]
    ChainedError {
        source: Box<AnalysisError>,
        context: String,
    },
}

impl From<AnalysisError> for FeedbackError {
    fn from(err: AnalysisError) -> Self {
        FeedbackError::Analysis(err.to_string())
    }
}