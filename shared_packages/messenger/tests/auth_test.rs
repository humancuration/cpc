//! Tests for the authentication service

use messenger_domain::auth::{AuthError, AuthService};
use uuid::Uuid;

// Mock authentication service for testing
struct MockAuthService;

#[async_trait::async_trait]
impl AuthService for MockAuthService {
    async fn validate_token(&self, token: &str) -> Result<Uuid, AuthError> {
        if token == "valid_token" {
            Ok(Uuid::new_v4())
        } else {
            Err(AuthError::InvalidToken)
        }
    }

    async fn refresh_token(&self, refresh_token: &str) -> Result<String, AuthError> {
        if refresh_token == "valid_refresh_token" {
            Ok("new_access_token".to_string())
        } else {
            Err(AuthError::InvalidToken)
        }
    }
}

#[tokio::test]
async fn test_valid_token() {
    let auth_service = MockAuthService;
    let result = auth_service.validate_token("valid_token").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_invalid_token() {
    let auth_service = MockAuthService;
    let result = auth_service.validate_token("invalid_token").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
}

#[tokio::test]
async fn test_valid_refresh_token() {
    let auth_service = MockAuthService;
    let result = auth_service.refresh_token("valid_refresh_token").await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "new_access_token");
}

#[tokio::test]
async fn test_invalid_refresh_token() {
    let auth_service = MockAuthService;
    let result = auth_service.refresh_token("invalid_refresh_token").await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), AuthError::InvalidToken));
}