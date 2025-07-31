use uuid::Uuid;
use chrono::{DateTime, Utc};
use super::value_objects::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub due_date: Option<DueDate>,
    pub priority: TaskPriority,
    pub status: TaskStatus,
    pub progress: ProgressPercentage,
    pub assignee: Option<Uuid>,
    pub dependencies: Vec<Uuid>,
    pub recurrence: Option<RecurrenceRule>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub tasks: Vec<Uuid>,
    pub team: Uuid,
    pub visibility: Visibility,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Team {
    pub id: Uuid,
    pub name: String,
    pub members: Vec<Uuid>,
    pub permissions: TeamPermissions,
}