//! Domain layer for OAuth2 authentication
//! 
//! Contains the core entities and value objects for the OAuth2 system.

pub mod oauth_provider;
pub mod oauth_token;
pub mod oauth_profile;
pub mod auth_error;
pub mod auth_config;

pub use oauth_provider::{OAuthProvider, ProviderAdapter};
pub use oauth_token::OAuthToken;
pub use oauth_profile::OAuthProfile;
pub use auth_error::AuthError;
pub use auth_config::AuthConfig;