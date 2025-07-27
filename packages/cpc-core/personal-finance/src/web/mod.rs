//! Web layer
//!
//! This layer contains the HTTP API interface for the personal finance module:
//! - GraphQL resolvers and schema
//! - Request/response DTOs
//! - API routes and handlers

pub mod graphql;
pub mod handlers;
pub mod dto;

pub use graphql::*;
pub use handlers::*;
pub use dto::*;