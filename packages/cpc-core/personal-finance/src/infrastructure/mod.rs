//! Infrastructure layer
//!
//! This layer contains all external concerns and implementation details:
//! - Database models and repositories for data persistence
//! - External service integrations (UBI, Treasury, OCR)
//! - Configuration and connection management

pub mod database;
pub mod repositories;
pub mod services;

pub use database::*;
pub use repositories::*;
pub use services::*;