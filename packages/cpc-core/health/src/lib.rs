//! Health module for the CPC platform
//!
//! This module provides comprehensive health management capabilities including
//! vital sign tracking, condition management, and health trend analysis.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types for convenience
pub use domain::vital_signs::VitalSign;
pub use domain::health_condition::HealthCondition;