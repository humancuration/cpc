use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid due date: {0}")]
    InvalidDueDate(&'static str),
    #[error("Invalid progress value: {0}")]
    InvalidProgress(&'static str),
    #[error("Invalid dependency: {0}")]
    InvalidDependency(&'static str),
    #[error("Invalid operation: {0}")]
    InvalidOperation(&'static str),
    // Add other error variants as needed
}