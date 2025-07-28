//! Data Lakehouse Module
//!
//! This module implements a federation-wide data management solution that serves as our
//! primary big data repository, following hexagonal architecture with vertical slices.

pub mod domain;
pub mod application;
pub mod infrastructure;
pub mod presentation;

// Re-export key types for convenience
pub use domain::models::{DataAsset, IngestionJob, DataAssetType, StorageFormat, DataSource, JobSchedule};
pub use application::ingestion_service::IngestionService;