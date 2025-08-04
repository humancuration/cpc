//! # CPC Messenger Application
//!
//! This is the main library for the Messenger application, which provides
//! real-time communication capabilities with standard security implementation.
//!
//! The application follows hexagonal architecture principles with a strict
//! separation between domain logic, application use cases, and infrastructure concerns.

/// Application services
pub mod services;

/// Repositories
pub mod repositories;

/// Infrastructure implementations
pub mod infrastructure;

/// UI components
pub mod ui;

/// Additional models
pub mod models;