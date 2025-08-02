//! Skill Management
//!
//! Handles the creation, retrieval, and categorization of skills.

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub mod models;
pub mod repository;
pub mod service;