//! Database migration utilities
//!
//! This module provides utilities for database schema versioning and migration.

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use crate::error::DbError;

/// Embedded migrations
pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

/// Run pending migrations
pub fn run_migrations<C: MigrationHarness<diesel::pg::Pg>>(conn: &mut C) -> Result<(), DbError> {
    conn.run_pending_migrations(MIGRATIONS)
        .map(|_| ())
        .map_err(|e| DbError::Database(diesel::result::Error::MigrationError(Box::new(e))))
}

/// Check if there are pending migrations
pub fn has_pending_migrations<C: MigrationHarness<diesel::pg::Pg>>(conn: &mut C) -> Result<bool, DbError> {
    let pending = conn.pending_migrations(MIGRATIONS)
        .map_err(|e| DbError::Database(diesel::result::Error::MigrationError(Box::new(e))))?;
    Ok(!pending.is_empty())
}

/// Get the latest migration version
pub fn latest_migration_version() -> Option<&'static str> {
    // This would return the latest migration version
    // In a real implementation, this would be determined from the embedded migrations
    None
}