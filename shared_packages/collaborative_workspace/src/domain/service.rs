//! Service traits for collaborative workspace
//!
//! Business logic interfaces for collaborative documents, project boards,
//! file versioning, and meetings. Focused on interfaces only.

use crate::domain::models::{
    CollaborativeDocument, FileVersion, MeetingRoom, ProjectBoard, ProjectColumn, ProjectTask,
};
use crate::domain::repository::RepositoryError;
use async_trait::async_trait;
use uuid::Uuid;

/// Error types for service operations
#[derive(Debug, Clone, PartialEq)]
pub enum ServiceError {
    /// Propagated repository/storage error
    RepositoryError(RepositoryError),
    /// Input validation error
    ValidationError(String),
    /// Authorization failure
    Unauthorized,
}

impl From<RepositoryError> for ServiceError {
    fn from(err: RepositoryError) -> Self {
        ServiceError::RepositoryError(err)
    }
}

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ServiceError::RepositoryError(e) => write!(f, "Repository error: {}", e),
            ServiceError::ValidationError(msg) => write!(f, "Validation error: {}", msg),
            ServiceError::Unauthorized => write!(f, "Unauthorized"),
        }
    }
}

impl std::error::Error for ServiceError {}

/// Service for collaborative documents
#[async_trait]
pub trait DocumentService: Send + Sync {
    /// Create a new document
    async fn create_document(
        &self,
        title: String,
        created_by: Uuid,
    ) -> Result<CollaborativeDocument, ServiceError>;

    /// Get a document by ID
    async fn get_document(
        &self,
        document_id: Uuid,
    ) -> Result<Option<CollaborativeDocument>, ServiceError>;

    /// Apply CRDT/operational transform operation to a document.
    /// Operation type is left abstract here (String placeholder).
    async fn apply_operation(
        &self,
        document_id: Uuid,
        operation: String,
        user_id: Uuid,
    ) -> Result<CollaborativeDocument, ServiceError>;
}

/// Service for project boards, columns, and tasks
#[async_trait]
pub trait ProjectService: Send + Sync {
    /// Create a project board
    async fn create_board(
        &self,
        title: String,
        owner_id: Uuid,
    ) -> Result<ProjectBoard, ServiceError>;

    /// Get a board by ID
    async fn get_board(&self, board_id: Uuid) -> Result<Option<ProjectBoard>, ServiceError>;

    /// Add a column to a board
    async fn add_column(
        &self,
        board_id: Uuid,
        title: String,
        position: i32,
    ) -> Result<ProjectColumn, ServiceError>;

    /// Add a task to a column
    async fn add_task(
        &self,
        column_id: Uuid,
        title: String,
        description: Option<String>,
        position: i32,
    ) -> Result<ProjectTask, ServiceError>;

    /// Move a task to another column/position
    async fn move_task(
        &self,
        task_id: Uuid,
        new_column_id: Uuid,
        position: i32,
    ) -> Result<ProjectTask, ServiceError>;
}

/// Service for file versioning
#[async_trait]
pub trait FileService: Send + Sync {
    /// Create a new version for a file
    async fn create_version(
        &self,
        file_id: Uuid,
        created_by: Uuid,
    ) -> Result<FileVersion, ServiceError>;

    /// Get a specific file version
    async fn get_version(
        &self,
        file_id: Uuid,
        version: i32,
    ) -> Result<Option<FileVersion>, ServiceError>;
}

/// Service for meetings
#[async_trait]
pub trait MeetingService: Send + Sync {
    /// Create a new meeting room
    async fn create_meeting(
        &self,
        title: String,
        owner_id: Uuid,
    ) -> Result<MeetingRoom, ServiceError>;

    /// End an active meeting
    async fn end_meeting(&self, room_id: Uuid) -> Result<MeetingRoom, ServiceError>;
}