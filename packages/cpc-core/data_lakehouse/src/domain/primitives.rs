//! Primitive types and helpers for the data lakehouse domain

use uuid::Uuid;
use chrono::{DateTime, Utc};

/// A unique identifier for a data entity
pub type DataId = Uuid;

/// A timestamp with UTC timezone
pub type Timestamp = DateTime<Utc>;

/// A version number for data assets
pub type Version = u64;