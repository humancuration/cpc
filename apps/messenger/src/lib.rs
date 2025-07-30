//! # CPC Messenger Application
//!
//! This is the main library for the Messenger application, which provides
//! real-time communication capabilities with standard security implementation.
//!
//! The application follows hexagonal architecture principles with a strict
//! separation between domain logic, application use cases, and infrastructure concerns.

/// Re-export domain types
pub use messenger_domain::*;

/// Re-export application services
pub use messenger_app::services::*;

/// Re-export infrastructure implementations
pub use messenger_infrastructure::*;