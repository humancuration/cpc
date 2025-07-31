//! Application layer for the consent manager.
//!
//! Contains use cases and service orchestration.

pub mod service;
pub mod validators;

#[cfg(test)]
mod service_tests;

pub use service::ConsentService;