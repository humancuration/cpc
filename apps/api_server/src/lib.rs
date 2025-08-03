//! API Server for CPC Platform
//!
//! This crate provides the main API server implementation for the CPC platform,
//! including GraphQL endpoints for all core features.

pub mod graphql;

// Re-export key types
pub use graphql::{VolunteerMutation, VolunteerQuery, SkillExchangeMutation, SkillExchangeQuery};