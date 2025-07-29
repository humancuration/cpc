//! Infrastructure layer for the consent manager.
//!
//! Contains adapters for external systems.

pub mod storage;
pub mod api;
pub mod events;

pub use storage::{sled_adapter::SledAdapter, postgres_adapter::PostgresAdapter};