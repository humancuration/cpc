use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::auth_error::AuthError;

#[async_trait]
pub trait JwtService: Send + Sync {
    fn generate_access_token(&self, user_id: Uuid) -> Result<String, AuthError>;
    fn verify_access_token(&self, token: &str) -> Result<Uuid, AuthError>;
    fn generate_refresh_token(&self) -> String;
}