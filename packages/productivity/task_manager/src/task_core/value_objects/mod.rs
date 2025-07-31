use chrono::{NaiveDate, Utc};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct DueDate(NaiveDate);

impl DueDate {
    pub fn new(date: NaiveDate) -> Result<Self, DomainError> {
        if date < Utc::now().date_naive() {
            Err(DomainError::InvalidDueDate("Due date must be in the future"))
        } else {
            Ok(Self(date))
        }
    }
}

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Invalid due date: {0}")]
    InvalidDueDate(&'static str),
    // Add other error variants as needed
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TaskStatus {
    NotStarted,
    InProgress,
    Blocked,
    Completed,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ProgressPercentage(u8);

impl ProgressPercentage {
    pub fn new(value: u8) -> Result<Self, DomainError> {
        if value > 100 {
            Err(DomainError::InvalidProgress("Value must be between 0-100"))
        } else {
            Ok(Self(value))
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RecurrenceRule {
    // Implementation details
}

#[derive(Debug, Clone, PartialEq)]
pub struct DependencyGraph {
    // Implementation details
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    TeamOnly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamPermissions {
    // Implementation details
}