use std::fmt;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("User not found")]
    UserNotFound,
    
    #[error("Account not verified")]
    AccountNotVerified,
    
    #[error("Session expired")]
    SessionExpired,
    
    #[error("Invalid token")]
    TokenInvalid,
    
    #[error("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[error("OAuth failure: {0}")]
    OAuthFailure(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Password hash error: {0}")]
    PasswordHashError(String),
    
    // New errors for Allat
    #[error("Role assignment conflict")]
    RoleAssignmentConflict,
    
    #[error("Karma limit exceeded: max {0}")]
    KarmaLimitExceeded(i32),
    
    #[error("Invalid vote operation")]
    InvalidVoteOperation,
    
    // New errors for unified auth system
    #[error("Permission denied")]
    PermissionDenied,
    
    #[error("Invalid session")]
    InvalidSession,
    
    #[error("Consent required")]
    ConsentRequired,
}

#[derive(Debug, Error)]
pub enum SessionError {
    #[error("Session not found")]
    SessionNotFound,
    
    #[error("Database error: {0}")]
    DatabaseError(String),
}

#[derive(Debug, Error)]
pub enum OAuthError {
    #[error("Invalid OAuth code")]
    InvalidCode,
    
    #[error("OAuth provider error: {0}")]
    ProviderError(String),
}