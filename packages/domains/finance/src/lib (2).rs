//! Personal Finance Module
//!
//! A comprehensive suite of financial management tools designed as a single vertical slice
//! within CPC's architecture. The module integrates tightly with the BI Visualization Toolkit
//! to provide actionable financial insights through standardized, beautiful data visualizations.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types for convenience
pub use domain::primitives::*;
pub use domain::budget::*;
pub use domain::expense::*;
pub use domain::subscription::*;
pub use domain::savings_goal::*;
pub use domain::investment::*;
pub use domain::debt::*;