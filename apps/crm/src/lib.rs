//! # CPC CRM Module
//!
//! Customer Relationship Management module for the CPC platform.
//!
//! This module provides a lightweight contact manager to track customer interactions,
//! notes, and sales pipelines. It's designed for small businesses and startups who
//! need simple CRM functionality without the complexity of enterprise solutions.

/// Domain entities and business logic
pub mod domain;

/// Application services for orchestrating domain logic
pub mod application;

/// Infrastructure implementations (database, p2p, etc.)
pub mod infrastructure;

/// Presentation layer (UI components)
pub mod presentation;

#[cfg(test)]
mod tests;

// Re-export key types for convenience
pub use domain::contact::Contact;
pub use domain::interaction::Interaction;
pub use domain::pipeline::Pipeline;
pub use domain::deal::Deal;