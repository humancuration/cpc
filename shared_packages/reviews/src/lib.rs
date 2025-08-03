//! Reviews system for the CPC platform
//!
//! This crate provides functionality for creating, managing, and analyzing reviews.
//! It integrates with the survey system and supports federation of review data.

pub mod models;
pub mod analytics;
pub mod filters;

// Re-export commonly used types
pub use models::*;
pub use analytics::*;