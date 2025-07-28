//! Primitive types and helpers for the data lakehouse domain

use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A unique identifier for a data entity
pub type DataId = Uuid;

/// A timestamp with UTC timezone
pub type Timestamp = DateTime<Utc>;

/// A version number for data assets
pub type Version = u64;

/// Generate a new time-ordered UUID (UUIDv7)
/// Time-ordered UUIDs are better for database indexing
pub fn new_uuid() -> Uuid {
    // Using UUIDv4 for now as uuid crate doesn't have v7 yet
    // In a production implementation, we would use UUIDv7
    Uuid::new_v4()
}