//! # Consent Manager
//!
//! A unified consent management system for the CPC ecosystem that provides
//! centralized control over data sharing preferences across all applications.

// Include the generated gRPC code
#[allow(dead_code)]
#[allow(clippy::all)]
mod consent_manager_proto {
    include!(concat!(env!("OUT_DIR"), "/consent_manager.rs"));
}

/// Domain layer containing core business logic and entities
pub mod domain;

/// Application layer containing use cases and service orchestration
pub mod application;

/// Infrastructure layer containing adapters for external systems
pub mod infrastructure;

/// Presentation layer containing user interfaces and API endpoints
pub mod presentation;

/// Migration utilities for converting existing consent data
pub mod migration;

/// Re-export commonly used types
pub use domain::{consent::DataSharingLevel, consent::Domain, consent::ConsentProfile};
pub use application::service::ConsentService;