use async_trait::async_trait;
use crate::domain::auth_error::RepositoryError;
use uuid::Uuid;

#[async_trait]
pub trait PasswordResetRepository: Send + Sync {
    async fn create_token(&self, token: String, user_id: Uuid) -> Result<(), RepositoryError>;
    async fn find_user_id_by_token(&self, token: &str) -> Result<Option<Uuid>, RepositoryError>;
    async fn delete_token(&self, token: &str) -> Result<(), RepositoryError>;
}