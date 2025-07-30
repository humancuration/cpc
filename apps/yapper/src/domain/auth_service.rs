use async_trait::async_trait;
use cpc_auth::models::{Credentials, User, Session};
use cpc_auth::error::AuthError;
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