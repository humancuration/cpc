//! Services for the Messenger web application

/// Service for handling GraphQL API calls
pub mod graphql;

/// Service for handling authentication
pub mod auth;

/// Service for handling real-time messaging
pub mod messaging;

pub use graphql::GraphQLService;
pub use auth::AuthService;
pub use messaging::MessagingService;