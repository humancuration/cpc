use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    
    #[error("Invalid due date: {0}")]
    InvalidDueDate(String),
    
    #[error("Invalid task data: {0}")]
    ValidationError(String),
    
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    
    #[error("Sync error: {0}")]
    SyncError(String),
    
    #[error("Notification error: {0}")]
    NotificationError(String),
}

pub type Result<T> = std::result::Result<T, TaskError>;