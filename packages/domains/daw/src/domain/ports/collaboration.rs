use crate::domain::models::Project;
use async_trait::async_trait;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CollaborationError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Sync conflict: {0}")]
    SyncConflict(String),
    
    #[error("Peer disconnected: {0}")]
    PeerDisconnected(String),
}

#[async_trait]
pub trait CollaborationPort: Send + Sync {
    async fn share_project(&self, project: &Project) -> Result<(), CollaborationError>;
    
    async fn join_project(&self, project_id: Uuid) -> Result<Project, CollaborationError>;
    
    async fn sync_changes(&self, project: &Project) -> Result<(), CollaborationError>;
    
    async fn leave_project(&self, project_id: Uuid) -> Result<(), CollaborationError>;
    
    fn get_connected_peers(&self) -> Vec<String>;
}