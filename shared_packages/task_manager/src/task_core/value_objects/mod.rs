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

// DomainError is defined in the errors module
pub use super::errors::DomainError;

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
    pub pattern: RecurrencePattern,
    pub interval: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecurrencePattern {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DependencyGraph {
    pub dependencies: Vec<Uuid>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }

    pub fn add_dependency(&mut self, task_id: Uuid) -> Result<(), DomainError> {
        if self.dependencies.contains(&task_id) {
            return Err(DomainError::InvalidDependency("Dependency already exists"));
        }
        self.dependencies.push(task_id);
        Ok(())
    }

    pub fn has_circular_dependency(&self, task_id: Uuid) -> bool {
        // In a real implementation, this would check for circular dependencies
        // in the dependency graph. For now, we'll just check if the task depends
        // on itself.
        self.dependencies.contains(&task_id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Visibility {
    Public,
    Private,
    TeamOnly,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TeamPermissions {
    pub can_create_tasks: bool,
    pub can_assign_tasks: bool,
    pub can_modify_tasks: bool,
}