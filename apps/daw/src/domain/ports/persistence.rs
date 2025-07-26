use crate::domain::models::Project;
use async_trait::async_trait;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum PersistenceError {
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Project not found: {0}")]
    NotFound(Uuid),
}

#[async_trait]
pub trait PersistencePort: Send + Sync {
    async fn save_project(&self, project: &Project) -> Result<(), PersistenceError>;
    
    async fn load_project(&self, id: Uuid) -> Result<Option<Project>, PersistenceError>;
    
    async fn delete_project(&self, id: Uuid) -> Result<(), PersistenceError>;
    
    async fn list_projects(&self) -> Result<Vec<Project>, PersistenceError>;
}