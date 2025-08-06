//! Database abstraction layer using Diesel ORM
//!
//! This module provides a higher-level database abstraction using Diesel ORM
//! for type-safe database interactions.

pub mod schema;
pub mod models;
pub mod repositories;
pub mod migrations;
pub mod error;

pub use error::DbError;
pub use repositories::DbManager;
pub use diesel::prelude::*;

use db_pool::{Pool, PostgresConnectionManager, SqliteConnectionManager};
use diesel::r2d2::ConnectionManager;
use std::sync::Arc;

/// Database abstraction layer
pub struct DatabaseAbstraction;

impl DatabaseAbstraction {
    /// Create a new database abstraction instance
    pub fn new() -> Self {
        Self
    }
}