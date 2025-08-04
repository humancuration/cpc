//! Volunteer Core Module
//!
//! This module provides functionality for volunteer tracking, including:
//! - Logging volunteer hours
//! - Verification workflow
//! - Dabloon conversion
//! - Integration with wallet, notification, and social systems

pub mod models;
pub mod repositories;
pub mod services;

#[cfg(test)]
mod volunteer_service_test;

#[cfg(test)]
mod lib_test;

// Re-export key types
pub use models::{VolunteerActivity, VolunteerVerification, DabloonConversion};
pub use services::{VolunteerService, VolunteerServiceImpl};
pub use repositories::VolunteerRepository;