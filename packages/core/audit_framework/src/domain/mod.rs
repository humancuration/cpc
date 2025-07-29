//! Domain models for the audit framework
//!
//! This module contains the core business logic and entities for audit management.

pub mod event;
pub mod policy;
pub mod errors;

pub use errors::AuditError;