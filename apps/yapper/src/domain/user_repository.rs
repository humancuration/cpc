use async_trait::async_trait;
use crate::domain::user::User;
use crate::domain::auth_error::RepositoryError;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
}