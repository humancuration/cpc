//! GraphQL API for social integration features

pub mod schema;
pub mod queries;
pub mod mutations;
pub mod types;
pub mod error;
#[cfg(test)]
pub mod integration_tests;

pub use schema::{create_schema, SocialIntegrationSchema};