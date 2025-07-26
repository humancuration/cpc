use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::error::Error;

use crate::domain::models::{Task, Project};

#[async_trait]
pub trait P2pSyncService: Send + Sync {
    async fn broadcast_task_created(&self, task: &Task) -> Result<(), Box<dyn Error>>;
    async fn broadcast_task_updated(&self, task: &Task) -> Result<(), Box<dyn Error>>;
    async fn broadcast_task_deleted(&self, task_id: &str) -> Result<(), Box<dyn Error>>;
    async fn broadcast_project_created(&self, project: &Project) -> Result<(), Box<dyn Error>>;
    async fn broadcast_project_updated(&self, project: &Project) -> Result<(), Box<dyn Error>>;
    async fn broadcast_project_deleted(&self, project_id: &str) -> Result<(), Box<dyn Error>>;
    async fn sync_pending_changes(&self) -> Result<(), Box<dyn Error>>;
}

// Mock implementation for development/testing
pub struct MockP2pSyncService;

#[async_trait]
impl P2pSyncService for MockP2pSyncService {
    async fn broadcast_task_created(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Task created - {}", task.title);
        Ok(())
    }

    async fn broadcast_task_updated(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Task updated - {}", task.title);
        Ok(())
    }

    async fn broadcast_task_deleted(&self, task_id: &str) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Task deleted - {}", task_id);
        Ok(())
    }

    async fn broadcast_project_created(&self, project: &Project) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Project created - {}", project.name);
        Ok(())
    }

    async fn broadcast_project_updated(&self, project: &Project) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Project updated - {}", project.name);
        Ok(())
    }

    async fn broadcast_project_deleted(&self, project_id: &str) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Project deleted - {}", project_id);
        Ok(())
    }

    async fn sync_pending_changes(&self) -> Result<(), Box<dyn Error>> {
        tracing::info!("Mock P2P sync: Syncing pending changes");
        Ok(())
    }
}

// Real implementation using p2panda
#[cfg(feature = "p2panda")]
pub struct P2pandaSyncService {
    node: p2panda_rs::node::Node,
}

#[cfg(feature = "p2panda")]
impl P2pandaSyncService {
    pub async fn new(node: p2panda_rs::node::Node) -> Self {
        Self { node }
    }
}

#[cfg(feature = "p2panda")]
#[async_trait]
impl P2pSyncService for P2pandaSyncService {
    async fn broadcast_task_created(&self, task: &Task) -> Result<(), Box<dyn Error>> {
        // Implement actual p2panda sync logic
        let operation = SyncOperation::TaskCreated {
            id: task.id.to_string(),
            title: task.title.clone(),
            description: task.description.clone(),
            status: task.status.clone(),
            priority: task.priority.clone(),
            due_date: task.due_date,
            project_id: task.project_id.map(|id| id.to_string()),
            assignee_id: task.assignee_id.clone(),
            tags: task.tags.clone(),
            metadata: task.metadata.clone(),
        };
        
        // Broadcast via p2panda
        Ok(())
    }

    // ... implement other methods ...
}

// Sync operation types for CRDT synchronization
#[derive(Debug, Serialize, Deserialize)]
enum SyncOperation {
    TaskCreated {
        id: String,
        title: String,
        description: Option<String>,
        status: crate::domain::models::TaskStatus,
        priority: crate::domain::models::TaskPriority,
        due_date: Option<chrono::DateTime<chrono::Utc>>,
        project_id: Option<String>,
        assignee_id: Option<String>,
        tags: Option<Vec<String>>,
        metadata: Option<serde_json::Value>,
    },
    TaskUpdated {
        id: String,
        title: Option<String>,
        description: Option<String>,
        status: Option<crate::domain::models::TaskStatus>,
        priority: Option<crate::domain::models::TaskPriority>,
        due_date: Option<chrono::DateTime<chrono::Utc>>,
        tags: Option<Vec<String>>,
        metadata: Option<serde_json::Value>,
    },
    TaskDeleted {
        id: String,
    },
    ProjectCreated {
        id: String,
        name: String,
        description: Option<String>,
        color: Option<String>,
        owner_id: String,
    },
    ProjectUpdated {
        id: String,
        name: Option<String>,
        description: Option<String>,
        color: Option<String>,
    },
    ProjectDeleted {
        id: String,
    },
}