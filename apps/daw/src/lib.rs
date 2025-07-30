//! Digital Audio Workstation (DAW) module for CPC
//!
//! This module provides the core functionality for a collaborative
//! digital audio workstation with real-time collaboration features.

pub mod domain;
pub mod infrastructure;
pub mod web;

// Re-export commonly used types
pub use domain::models::*;
pub use domain::services::*;
pub use web::graphql::*;

// Re-export automation types
pub use domain::models::automation::*;