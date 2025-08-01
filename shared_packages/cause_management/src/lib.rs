//! # Cause Management
//!
//! Cause management functionality for the CPC platform.
//!
//! This crate provides the core business logic for managing causes,
//! including creation, updating, deletion, and listing of causes
//! for donations within the CPC ecosystem.

pub mod models;
pub mod repository;
pub mod service;

// Include gRPC generated code
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("cpay");
}

use tracing::info;
use tonic::transport::Server;

// Re-export the service for convenience
pub use service::CauseServiceImpl;