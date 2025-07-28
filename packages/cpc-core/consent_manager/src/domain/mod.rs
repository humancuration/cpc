//! Domain layer for the consent manager.
//!
//! Contains core business logic and entities related to consent management.

pub mod consent;
pub mod audit;
pub mod errors;

#[cfg(test)]
mod consent_tests;
#[cfg(test)]
mod audit_tests;

pub use consent::{DataSharingLevel, Domain, ConsentProfile};
pub use audit::{AuditEvent, Actor};
pub use errors::ConsentError;