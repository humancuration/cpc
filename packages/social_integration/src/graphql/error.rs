//! Custom GraphQL errors for social integration

use async_graphql::ErrorExtensions;

/// Custom GraphQL error types
#[derive(Debug)]
pub enum GraphQLError {
    /// Unauthorized access
    Unauthorized(String),
    
    /// Forbidden access
    Forbidden(String),
    
    /// Invalid input
    InvalidInput(String),
    
    /// Internal server error
    InternalServerError(String),
    
    /// Not implemented
    NotImplemented(String),
}

impl From<GraphQLError> for async_graphql::Error {
    fn from(error: GraphQLError) -> Self {
        match error {
            GraphQLError::Unauthorized(message) => {
                async_graphql::Error::new(message).extend_with(|_, e| e.set("code", "UNAUTHORIZED"))
            }
            GraphQLError::Forbidden(message) => {
                async_graphql::Error::new(message).extend_with(|_, e| e.set("code", "FORBIDDEN"))
            }
            GraphQLError::InvalidInput(message) => {
                async_graphql::Error::new(message).extend_with(|_, e| e.set("code", "INVALID_INPUT"))
            }
            GraphQLError::InternalServerError(message) => {
                async_graphql::Error::new(message).extend_with(|_, e| e.set("code", "INTERNAL_SERVER_ERROR"))
            }
            GraphQLError::NotImplemented(message) => {
                async_graphql::Error::new(message).extend_with(|_, e| e.set("code", "NOT_IMPLEMENTED"))
            }
        }
    }
}