//! Sheets application for the CPC platform
//!
//! This module provides a collaborative spreadsheet tool designed as a top-level application
//! within CPC's ecosystem. The app integrates with the BI Visualization Toolkit to transform
//! spreadsheet data into insightful visualizations while providing robust collaboration features
//! and format compatibility.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types for convenience
pub use domain::sheet::*;
pub use domain::cell::*;
pub use domain::formula::*;
pub use domain::chart_spec::*;