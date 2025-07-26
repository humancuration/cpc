use async_graphql::*;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::{TaskStatus, Priority};

#[derive(SimpleObject, Clone)]
pub struct TaskDto {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatusDto,
    pub priority: PriorityDto,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub project_id: Option<Uuid>,
    pub assignee_id: Option<String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TaskStatusDto {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

impl From<TaskStatus> for TaskStatusDto {
    fn from(status: TaskStatus) -> Self {
        match status {
            TaskStatus::Pending => TaskStatusDto::Pending,
            TaskStatus::InProgress => TaskStatusDto::InProgress,
            TaskStatus::Completed => TaskStatusDto::Completed,
            TaskStatus::Cancelled => TaskStatusDto::Cancelled,
        }
    }
}

impl From<TaskStatusDto> for TaskStatus {
    fn from(dto: TaskStatusDto) -> Self {
        match dto {
            TaskStatusDto::Pending => TaskStatus::Pending,
            TaskStatusDto::InProgress => TaskStatus::InProgress,
            TaskStatusDto::Completed => TaskStatus::Completed,
            TaskStatusDto::Cancelled => TaskStatus::Cancelled,
        }
    }
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PriorityDto {
    Low,
    Medium,
    High,
    Urgent,
}

impl From<Priority> for PriorityDto {
    fn from(priority: Priority) -> Self {
        match priority {
            Priority::Low => PriorityDto::Low,
            Priority::Medium => PriorityDto::Medium,
            Priority::High => PriorityDto::High,
            Priority::Urgent => PriorityDto::Urgent,
        }
    }
}

impl From<PriorityDto> for Priority {
    fn from(dto: PriorityDto) -> Self {
        match dto {
            PriorityDto::Low => Priority::Low,
            PriorityDto::Medium => Priority::Medium,
            PriorityDto::High => Priority::High,
            PriorityDto::Urgent => Priority::Urgent,
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct ProjectDto {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: String,
    pub is_archived: bool,
}

#[derive(SimpleObject, Clone)]
pub struct ReminderDto {
    pub id: Uuid,
    pub task_id: Uuid,
    pub remind_at: DateTime<Utc>,
    pub message: Option<String>,
    pub is_sent: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(SimpleObject, Clone)]
pub struct TaskWithProjectDto {
    pub task: TaskDto,
    pub project: Option<ProjectDto>,
    pub reminders: Vec<ReminderDto>,
}

#[derive(InputObject, Clone)]
pub struct CreateTaskInput {
    pub title: String,
    pub description: Option<String>,
    pub priority: PriorityDto,
    pub due_date: Option<DateTime<Utc>>,
    pub project_id: Option<Uuid>,
    pub assignee_id: Option<String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

#[derive(InputObject, Clone)]
pub struct UpdateTaskInput {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatusDto>,
    pub priority: Option<PriorityDto>,
    pub due_date: Option<DateTime<Utc>>,
    pub tags: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(InputObject, Clone)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(InputObject, Clone)]
pub struct UpdateProjectInput {
    pub name: Option<String>,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(InputObject, Clone)]
pub struct CreateReminderInput {
    pub task_id: Uuid,
    pub remind_at: DateTime<Utc>,
    pub message: Option<String>,
}