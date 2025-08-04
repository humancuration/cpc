//! Authentication service for the Messenger application

use crate::errors::MessengerError;
use async_trait::async_trait;
use thiserror::Error;
use tonic::{transport::Channel, Status};
use uuid::Uuid;

/// Custom error types for authentication
#[derive(Error, Debug)]
pub enum AuthError {
    /// Invalid token provided
    #[error("Invalid token")]
    InvalidToken,

    /// Token has expired
    #[error("Token has expired")]
    ExpiredToken,

    /// Permission denied
    #[error("Permission denied")]
    PermissionDenied,

    /// Identity service is unavailable
    #[error("Identity service unavailable")]
    IdentityServiceUnavailable,

    /// gRPC transport error
    #[error("gRPC transport error: {0}")]
    TransportError(#[from] tonic::transport::Error),

    /// gRPC status error
    #[error("gRPC status error: {0}")]
    GrpcStatus(#[from] Status),
}

impl From<AuthError> for MessengerError {
    fn from(auth_error: AuthError) -> Self {
        match auth_error {
            AuthError::InvalidToken => MessengerError::PermissionDenied {
                user_id: Uuid::nil(),
                action: "Invalid token".to_string(),
            },
            AuthError::ExpiredToken => MessengerError::PermissionDenied {
                user_id: Uuid::nil(),
                action: "Expired token".to_string(),
            },
            AuthError::PermissionDenied => MessengerError::PermissionDenied {
                user_id: Uuid::nil(),
                action: "Permission denied".to_string(),
            },
            AuthError::IdentityServiceUnavailable => MessengerError::StorageError {
                message: "Identity service unavailable".to_string(),
            },
            AuthError::TransportError(_) | AuthError::GrpcStatus(_) => MessengerError::StorageError {
                message: "Authentication service error".to_string(),
            },
        }
    }
}

/// Trait for authentication service
#[async_trait]
pub trait AuthService: Send + Sync {
    /// Validate a JWT token and return the user ID
    async fn validate_token(&self, token: &str) -> Result<Uuid, AuthError>;

    /// Refresh a token using a refresh token
    async fn refresh_token(&self, refresh_token: &str) -> Result<String, AuthError>;
}

/// gRPC client implementation for authentication service
pub struct GrpcAuthService {
    /// gRPC client for identity service
    client: identity_service_client::IdentityServiceClient<Channel>,
}

/// Generated gRPC client for identity service
/// In a real implementation, this would be generated from a .proto file
mod identity_service_client {
    use super::*;
    use tonic::IntoRequest;

    #[derive(Debug, Clone)]
    pub struct IdentityServiceClient<T> {
        _inner: T,
    }

    impl IdentityServiceClient<Channel> {
        pub fn new(channel: Channel) -> Self {
            Self { _inner: channel }
        }
    }

    /// Response from token validation
    #[derive(Debug)]
    pub struct ValidateTokenResponse {
        pub user_id: Uuid,
        pub expires_at: i64,
    }

    /// Response from token refresh
    #[derive(Debug)]
    pub struct RefreshTokenResponse {
        pub access_token: String,
        pub expires_in: i64,
    }

    #[async_trait]
    impl AuthService for IdentityServiceClient<Channel> {
        async fn validate_token(&self, _token: &str) -> Result<Uuid, AuthError> {
            // In a real implementation, this would make a gRPC call to the identity service
            // For now, we'll simulate a successful validation
            Ok(Uuid::new_v4())
        }

        async fn refresh_token(&self, _refresh_token: &str) -> Result<String, AuthError> {
            // In a real implementation, this would make a gRPC call to the identity service
            // For now, we'll simulate a successful refresh
            Ok("new_access_token".to_string())
        }
    }
}

impl GrpcAuthService {
    /// Create a new gRPC authentication service
    pub fn new(client: identity_service_client::IdentityServiceClient<Channel>) -> Self {
        Self { client }
    }
}

#[async_trait]
impl AuthService for GrpcAuthService {
    async fn validate_token(&self, token: &str) -> Result<Uuid, AuthError> {
        self.client.validate_token(token).await
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<String, AuthError> {
        self.client.refresh_token(refresh_token).await
    }
}