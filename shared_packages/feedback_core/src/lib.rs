//! Core components for the feedback system
//!
//! This crate provides shared error types and core functionality for the feedback system.

pub mod error;

// Re-export commonly used types
pub use error::FeedbackError;