//! Web presentation layer for the BI & Analytics module

pub mod graphql;
pub mod routes;

// Re-export key types
pub use graphql::BiAnalyticsGraphQLSchema;