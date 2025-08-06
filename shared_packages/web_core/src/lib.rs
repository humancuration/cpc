//! Web Core - Shared functionality for CPC web applications
//!
//! This crate provides common utilities, components, and services
//! that can be reused across all web applications in the CPC ecosystem.

pub mod auth;
pub mod api_client;
pub mod components;
pub mod theme;
pub mod theme_manager;
pub mod utils;

// Re-export key types for convenience
pub use auth::AuthService;
pub use api_client::ApiClient;