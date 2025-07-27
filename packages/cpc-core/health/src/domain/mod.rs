pub mod audit_log;
#[cfg(test)]
pub mod audit_log_test;
//! Domain models for the health module
//!
//! This module contains the core business logic and entities for health management.

pub mod vital_signs;
pub mod health_condition;
pub mod primitives;