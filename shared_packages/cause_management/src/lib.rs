//! # Cause Management
//!
//! Cause management functionality for the CPC platform.
//!
//! This crate provides the core business logic for managing causes,
//! including creation, updating, deletion, and listing of causes
//! for donations within the CPC ecosystem.
//!
//! ## Features
//!
//! - Basic cause management (create, read, update, delete)
//! - Statistical analysis of donation patterns (with `statistics` feature)
//! - Impact measurement for causes (with `statistics` feature)
//! - Donation forecasting with confidence intervals (with `statistics` feature)
//! - BI visualization integration (with `statistics` and `visualization` features)
//!
//! ## Statistical Analysis Features
//!
//! When the `statistics` feature is enabled, the crate provides:
//!
//! - `StatisticalAnalysisService`: Forecasting and trend analysis for donations
//! - `ImpactMeasurementService`: Measuring real-world impact of causes
//! - Statistical domain models for representing analysis results
//!
//! ## Usage
//!
//! To use statistical features, enable the `statistics` feature in your Cargo.toml:
//!
//! ```toml
//! [dependencies]
//! cause_management = { path = "../cause_management", features = ["statistics"] }
//! ```
//!
//! For visualization integration, also enable the `visualization` feature:
//!
//! ```toml
//! [dependencies]
//! cause_management = { path = "../cause_management", features = ["statistics", "visualization"] }
//! ```

pub mod models;
pub mod repository;
pub mod service;
pub mod domain;
pub mod application;
pub mod ml;

// Include gRPC generated code
#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto {
    tonic::include_proto!("cpay");
}

use tracing::info;
use tonic::transport::Server;

// Re-export the service for convenience
pub use service::CauseServiceImpl;