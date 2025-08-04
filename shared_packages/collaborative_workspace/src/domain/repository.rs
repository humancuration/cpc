//! Repository traits for collaborative workspace
//!
//! Interfaces for data access to collaborative documents, project boards,
//! file versions, and meeting rooms.

use crate::domain::models::{
    CollaborativeDocument, FileVersion, MeetingRoom, ProjectBoard, ProjectColumn, ProjectTask,
};
use async_trait::async_trait;
use uuid::Uuid;

/// Error types for repository operations
#[derive(Debug, Clone, PartialEq)]
pub enum RepositoryError {
    /// Resource not found
    NotFound,
    /// Underlying database/storage error
    DatabaseError(String),
    /// Validation or constraint error
    ValidationError(String),
}

impl std::fmt::Display for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepositoryError::NotFound => write!(f, "Not found"),
            RepositoryError::DatabaseError(msg) => write!(f, "Database error: {}", msg),
            RepositoryError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
        }
    }
}

impl std::error::Error for RepositoryError {}

/// Repository for collaborative documents
#[async_trait]
pub trait DocumentRepository: Send + Sync {
    /// Get a document by its ID
    async fn get_by_id(
        &self,
        document_id: Uuid,
    ) -> Result<Option<CollaborativeDocument>, RepositoryError>;

    /// Persist a document (insert or update)
    async fn save(&self, document: &CollaborativeDocument) -> Result<(), RepositoryError>;

    /// Delete a document by its ID
    async fn delete(&self, document_id: Uuid) -> Result<(), RepositoryError>;
}

/// Repository for project boards, columns, and tasks
#[async_trait]
pub trait ProjectRepository: Send + Sync {
    /// Create a new board
    async fn create_board(&self, board: &ProjectBoard) -> Result<(), RepositoryError>;

    /// Fetch a board by ID
    async fn get_board_by_id(
        &self,
        board_id: Uuid,
    ) -> Result<Option<ProjectBoard>, RepositoryError>;

    /// Add a column to a board
    async fn add_column(&self, column: &ProjectColumn) -> Result<(), RepositoryError>;

    /// Add a task to a column
    async fn add_task(&self, task: &ProjectTask) -> Result<(), RepositoryError>;

    /// Move a task to a new column/position
    async fn move_task(
        &self,
        task_id: Uuid,
        new_column_id: Uuid,
        position: i32,
    ) -> Result<(), RepositoryError>;
}

/// Repository for file versioning
#[async_trait]
pub trait FileRepository: Send + Sync {
    /// Create a new file version entry
    async fn create_version(&self, version: &FileVersion) -> Result<(), RepositoryError>;

    /// Get all versions for a file (ordered by version ASC recommended)
    async fn get_versions(&self, file_id: Uuid) -> Result<Vec<FileVersion>, RepositoryError>;
}

/// Repository for meetings
#[async_trait]
pub trait MeetingRepository: Send + Sync {
    /// Create a new meeting room
    async fn create_room(&self, room: &MeetingRoom) -> Result<(), RepositoryError>;

    /// Mark a meeting as ended (set ended_at)
    async fn end_meeting(&self, room_id: Uuid) -> Result<(), RepositoryError>;
}