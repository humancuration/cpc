//! Database infrastructure modules for the music player

pub mod consent_repository;
pub mod pg_consent_repository;

// Re-export the main database module
pub use super::database::*;