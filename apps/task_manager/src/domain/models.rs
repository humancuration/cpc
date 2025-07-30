use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Task {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub project_id: Option<Uuid>,
    pub assignee_id: Option<String>,
    pub tags: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl Task {
    pub fn new(title: String, description: Option<String>, due_date: Option<DateTime<Utc>>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            status: TaskStatus::Pending,
            priority: Priority::Medium,
            due_date,
            created_at: now,
            updated_at: now,
            completed_at: None,
            project_id: None,
            assignee_id: None,
            tags: Vec::new(),
            metadata: HashMap::new(),
        }
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.title.trim().is_empty() {
            return Err(crate::TaskError::ValidationError("Title cannot be empty".to_string()));
        }

        if let Some(due_date) = self.due_date {
            if due_date < Utc::now() {
                return Err(crate::TaskError::InvalidDueDate("Due date must be in the future".to_string()));
            }
        }

        Ok(())
    }

    pub fn complete(&mut self) {
        self.status = TaskStatus::Completed;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    pub fn update_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.updated_at = Utc::now();
    }

    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn is_overdue(&self) -> bool {
        match self.due_date {
            Some(due) => self.status != TaskStatus::Completed && due < Utc::now(),
            None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub owner_id: String,
    pub is_archived: bool,
}

impl Project {
    pub fn new(name: String, description: Option<String>, owner_id: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            color: None,
            created_at: now,
            updated_at: now,
            owner_id,
            is_archived: false,
        }
    }

    pub fn validate(&self) -> crate::Result<()> {
        if self.name.trim().is_empty() {
            return Err(crate::TaskError::ValidationError("Project name cannot be empty".to_string()));
        }
        Ok(())
    }

    pub fn archive(&mut self) {
        self.is_archived = true;
        self.updated_at = Utc::now();
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: Uuid,
    pub task_id: Uuid,
    pub remind_at: DateTime<Utc>,
    pub message: Option<String>,
    pub is_sent: bool,
    pub created_at: DateTime<Utc>,
}

impl Reminder {
    pub fn new(task_id: Uuid, remind_at: DateTime<Utc>, message: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            task_id,
            remind_at,
            message,
            is_sent: false,
            created_at: now,
        }
    }

    pub fn should_trigger(&self) -> bool {
        !self.is_sent && self.remind_at <= Utc::now()
    }

    pub fn mark_sent(&mut self) {
        self.is_sent = true;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskWithProject {
    pub task: Task,
    pub project: Option<Project>,
    pub reminders: Vec<Reminder>,
}