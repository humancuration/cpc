//! Project board service
//!
//! Responsibilities:
//! - Create/get boards
//! - Add columns
//! - Add/move tasks (publishing TaskMoved event)

use async_trait::async_trait;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::{ProjectBoard, ProjectColumn, ProjectTask};

#[derive(thiserror::Error, Debug)]
pub enum ProjectServiceError {
    #[error("not found: {0}")]
    NotFound(String),
    #[error("repository error: {0}")]
    Repository(String),
    #[error("event publish error: {0}")]
    Event(String),
    #[error("validation error: {0}")]
    Validation(String),
}

/// Repository trait expected by the service
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn create_board(&self, board: &ProjectBoard) -> Result<(), String>;
    async fn get_board(&self, board_id: Uuid) -> Result<ProjectBoard, String>;

    async fn add_column(&self, column: &ProjectColumn) -> Result<(), String>;
    async fn get_columns(&self, board_id: Uuid) -> Result<Vec<ProjectColumn>, String>;

    async fn add_task(&self, task: &ProjectTask) -> Result<(), String>;
    async fn get_task(&self, task_id: Uuid) -> Result<ProjectTask, String>;
    async fn move_task(
        &self,
        task_id: Uuid,
        new_column_id: Uuid,
        new_position: i32,
        updated_at: chrono::DateTime<Utc>,
    ) -> Result<ProjectTask, String>;
}

/// Event publisher trait for project-related events
#[async_trait]
pub trait ProjectEventPublisher: Send + Sync {
    async fn publish_task_moved(&self, event: TaskMoved) -> Result<(), String>;
}

/// Event when a task is moved
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMoved {
    pub task_id: Uuid,
    pub from_column_id: Uuid,
    pub to_column_id: Uuid,
    pub new_position: i32,
    pub moved_by: Uuid,
    pub moved_at: chrono::DateTime<Utc>,
}

#[async_trait]
pub trait ProjectService: Send + Sync {
    async fn create_board(&self, title: String, owner_id: Uuid) -> Result<ProjectBoard, ProjectServiceError>;
    async fn get_board(&self, board_id: Uuid) -> Result<ProjectBoard, ProjectServiceError>;
    async fn add_column(&self, board_id: Uuid, title: String, position: i32) -> Result<ProjectColumn, ProjectServiceError>;
    async fn add_task(&self, column_id: Uuid, title: String, description: Option<String>, position: i32) -> Result<ProjectTask, ProjectServiceError>;
    async fn move_task(&self, task_id: Uuid, new_column_id: Uuid, position: i32, moved_by: Uuid) -> Result<ProjectTask, ProjectServiceError>;
}

pub struct ProjectServiceImpl<R: ProjectRepository, P: ProjectEventPublisher> {
    repo: Arc<R>,
    publisher: Arc<P>,
}

impl<R: ProjectRepository, P: ProjectEventPublisher> ProjectServiceImpl<R, P> {
    pub fn new(repo: Arc<R>, publisher: Arc<P>) -> Self {
        Self { repo, publisher }
    }
}

#[async_trait]
impl<R: ProjectRepository, P: ProjectEventPublisher> ProjectService for ProjectServiceImpl<R, P> {
    async fn create_board(&self, title: String, owner_id: Uuid) -> Result<ProjectBoard, ProjectServiceError> {
        if title.trim().is_empty() {
            return Err(ProjectServiceError::Validation("title cannot be empty".into()));
        }
        let board = ProjectBoard {
            id: Uuid::new_v4(),
            title,
            owner_id,
            created_at: Utc::now(),
        };
        self.repo.create_board(&board).await.map_err(ProjectServiceError::Repository)?;
        Ok(board)
    }

    async fn get_board(&self, board_id: Uuid) -> Result<ProjectBoard, ProjectServiceError> {
        self.repo.get_board(board_id).await.map_err(|e| {
            if e.to_lowercase().contains("not found") {
                ProjectServiceError::NotFound(board_id.to_string())
            } else {
                ProjectServiceError::Repository(e)
            }
        })
    }

    async fn add_column(&self, board_id: Uuid, title: String, position: i32) -> Result<ProjectColumn, ProjectServiceError> {
        if title.trim().is_empty() {
            return Err(ProjectServiceError::Validation("column title cannot be empty".into()));
        }
        // Optional: verify board exists
        let _ = self.get_board(board_id).await?;
        let column = ProjectColumn {
            id: Uuid::new_v4(),
            board_id,
            title,
            position,
        };
        self.repo.add_column(&column).await.map_err(ProjectServiceError::Repository)?;
        Ok(column)
    }

    async fn add_task(&self, column_id: Uuid, title: String, description: Option<String>, position: i32) -> Result<ProjectTask, ProjectServiceError> {
        if title.trim().is_empty() {
            return Err(ProjectServiceError::Validation("task title cannot be empty".into()));
        }
        let task = ProjectTask {
            id: Uuid::new_v4(),
            column_id,
            title,
            description,
            position,
            created_at: Utc::now(),
            updated_at: None,
        };
        self.repo.add_task(&task).await.map_err(ProjectServiceError::Repository)?;
        Ok(task)
    }

    async fn move_task(&self, task_id: Uuid, new_column_id: Uuid, position: i32, moved_by: Uuid) -> Result<ProjectTask, ProjectServiceError> {
        // fetch current to get from_column_id
        let existing = self.repo.get_task(task_id).await.map_err(|e| {
            if e.to_lowercase().contains("not found") {
                ProjectServiceError::NotFound(task_id.to_string())
            } else {
                ProjectServiceError::Repository(e)
            }
        })?;
        let moved_at = Utc::now();
        let updated = self.repo
            .move_task(task_id, new_column_id, position, moved_at)
            .await
            .map_err(ProjectServiceError::Repository)?;

        // publish TaskMoved
        let event = TaskMoved {
            task_id,
            from_column_id: existing.column_id,
            to_column_id: new_column_id,
            new_position: position,
            moved_by,
            moved_at,
        };
        self.publisher.publish_task_moved(event).await.map_err(ProjectServiceError::Event)?;

        Ok(updated)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct InMemRepo {
        boards: HashMap<Uuid, ProjectBoard>,
        columns: HashMap<Uuid, ProjectColumn>,
        tasks: HashMap<Uuid, ProjectTask>,
    }
    #[async_trait]
    impl ProjectRepository for InMemRepo {
        async fn create_board(&self, _board: &ProjectBoard) -> Result<(), String> { Ok(()) }
        async fn get_board(&self, board_id: Uuid) -> Result<ProjectBoard, String> {
            self.boards.get(&board_id).cloned().ok_or("not found".into())
        }
        async fn add_column(&self, _column: &ProjectColumn) -> Result<(), String> { Ok(()) }
        async fn get_columns(&self, board_id: Uuid) -> Result<Vec<ProjectColumn>, String> {
            Ok(self.columns.values().filter(|c| c.board_id == board_id).cloned().collect())
        }
        async fn add_task(&self, _task: &ProjectTask) -> Result<(), String> { Ok(()) }
        async fn get_task(&self, task_id: Uuid) -> Result<ProjectTask, String> {
            self.tasks.get(&task_id).cloned().ok_or("not found".into())
        }
        async fn move_task(&self, task_id: Uuid, new_column_id: Uuid, new_position: i32, updated_at: chrono::DateTime<Utc>) -> Result<ProjectTask, String> {
            let mut task = self.tasks.get(&task_id).cloned().ok_or("not found".into())?;
            task.column_id = new_column_id;
            task.position = new_position;
            task.updated_at = Some(updated_at);
            Ok(task)
        }
    }

    struct NoopPublisher;
    #[async_trait]
    impl ProjectEventPublisher for NoopPublisher {
        async fn publish_task_moved(&self, _event: TaskMoved) -> Result<(), String> { Ok(()) }
    }

    #[tokio::test]
    async fn move_task_publishes_event() {
        let repo = Arc::new(InMemRepo {
            boards: HashMap::new(),
            columns: HashMap::new(),
            tasks: HashMap::new(),
        });
        let publisher = Arc::new(NoopPublisher);
        let svc = ProjectServiceImpl::new(repo.clone(), publisher);

        // seed
        let col_a = Uuid::new_v4();
        let col_b = Uuid::new_v4();
        let task_id = Uuid::new_v4();
        repo.tasks.insert(task_id, ProjectTask {
            id: task_id,
            column_id: col_a,
            title: "T".into(),
            description: None,
            position: 0,
            created_at: Utc::now(),
            updated_at: None,
        });

        let res = svc.move_task(task_id, col_b, 2, Uuid::new_v4()).await.unwrap();
        assert_eq!(res.column_id, col_b);
        assert_eq!(res.position, 2);
        assert!(res.updated_at.is_some());
    }
}