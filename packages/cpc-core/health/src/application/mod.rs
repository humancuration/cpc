//! Application services for the health module
//!
//! This module contains the service implementations that orchestrate domain logic.

pub mod monitoring_service;
pub mod condition_service;

// Error types for the health module
#[derive(Debug)]
pub enum HealthError {
    DatabaseError(String),
    ValidationError(String),
    P2PError(String),
    NotFound,
}