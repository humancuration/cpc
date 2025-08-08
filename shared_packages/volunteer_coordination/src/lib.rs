//! Volunteer Coordination crate
//!
//! Domain layer, repository traits, services, and infrastructure for managing
//! volunteer opportunities, applications, and contributions. Integrates with the
//! shared SocialEventBus for cross-app notifications and feeds.

pub mod domain {
    pub mod models;
    pub mod repository;
    pub mod service;
}

pub mod infrastructure {
    pub mod postgres_repository;
    pub mod event_bus_integration;
    pub mod reputation_stub;
}

pub mod application {
    pub mod volunteer_service;
    pub mod reputation_port;
}

pub mod optimization;
pub mod ml;

pub use domain::{models, repository, service};

use std::sync::Arc;
use application::reputation_port::ReputationPort;
use infrastructure::reputation_stub::ReputationStub;

/// Helper constructor for enabling the dev/test reputation stub from composition code.
/// Production defaults should pass `None` for the reputation port unless explicitly enabled.
pub fn reputation_stub() -> Arc<dyn ReputationPort + Send + Sync> {
    Arc::new(ReputationStub::new())
}