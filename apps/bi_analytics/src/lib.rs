//! # CPC BI & Analytics Module
//!
//! Business Intelligence & Analytics module for the CPC platform.
//!
//! This module provides powerful, customizable dashboards that can pull data
//! from all other modules to provide deep insights into business operations,
//! market trends, and performance metrics.
//!
//! # Architecture
//!
//! This module follows hexagonal (clean) architecture with vertical slices:
//!
//! - **Dataset Slice**: Data ingestion and management
//! - **Report Slice**: Report generation and management
//! - **Dashboard Slice**: Dashboard creation and visualization
//! - **Compliance Slice**: GDPR and HIPAA compliance features
//!
//! # Key Features
//!
//! - Data ingestion from all CPC modules
//! - Report generation with customizable queries
//! - Dashboard builder with drag-and-drop interface
//! - GDPR-compliant data anonymization
//! - HIPAA-compliant PHI handling
//! - Audit trails for all data access operations
//! - Bevy visualization components
//! - GraphQL API for dashboard access

// Domain entities and business logic
pub mod domain;

// Application services for orchestrating domain logic
pub mod application;

// Infrastructure implementations (database, p2p, etc.)
pub mod infrastructure;

// Presentation layer (UI components, GraphQL API)
pub mod presentation;

// Test modules
#[cfg(test)]
mod tests;

// Re-export key types for convenience
pub use domain::{
    dataset::Dataset,
    report::Report,
    dashboard::Dashboard,
};