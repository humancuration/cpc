use async_trait::async_trait;
use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use crate::models::{Credentials, User, Session};
use crate::error::AuthError;
use uuid::Uuid;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, credentials: Credentials) -> Result<User, AuthError>;
    async fn login(&self, credentials: Credentials) -> Result<Session, AuthError>;
    async fn logout(&self, session_id: Uuid) -> Result<(), AuthError>;
    async fn validate_session(&self, session_id: Uuid) -> Result<User, AuthError>;
    async fn refresh_token(&self, refresh_token: String) -> Result<Session, AuthError>;
    async fn initiate_password_reset(&self, email: String) -> Result<(), AuthError>;
    async fn confirm_password_reset(&self, token: String, new_password: String) -> Result<(), AuthError>;
}

pub struct AuthServiceImpl;

impl AuthServiceImpl {
    pub fn new() -> Self {
        Self
    }
    
    pub fn hash_password(&self, password: &str) -> Result<String, AuthError> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AuthError::PasswordHashError(e.to_string()))?;
        Ok(password_hash.to_string())
    }
    
    pub fn verify_password(&self, password: &str, hash: &str) -> Result<bool, AuthError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AuthError::PasswordHashError(e.to_string()))?;
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
}