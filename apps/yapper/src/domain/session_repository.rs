use async_trait::async_trait;
use crate::domain::session::Session;
use crate::domain::auth_error::RepositoryError;
use uuid::Uuid;

#[async_trait]
pub trait SessionRepository: Send + Sync {
    async fn create(&self, session: &Session) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Session>, RepositoryError>;
    async fn delete(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Session>, RepositoryError>;
}