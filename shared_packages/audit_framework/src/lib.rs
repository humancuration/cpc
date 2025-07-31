//! # Unified Audit Framework
//!
//! Extends the audit capabilities from `consent_manager` into a comprehensive framework
//! for tracking all sensitive operations across the platform, with special attention
//! to regulatory compliance needs.

/// Domain layer containing core business logic and entities
pub mod domain;

/// Application layer containing use cases and service orchestration
pub mod application;

/// Infrastructure layer containing adapters for external systems
pub mod infrastructure;

// Re-export commonly used types
pub use domain::{event::AuditEvent, policy::Regulation, AuditError};
pub use application::service::AuditService;