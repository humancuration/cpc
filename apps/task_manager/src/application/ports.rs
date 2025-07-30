use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use uuid::Uuid;

use crate::domain::{Task, Project, Reminder, TaskWithProject, TaskError, Result};

#[async_trait]
pub trait TaskRepository: Send + Sync {
    async fn create_task(&self, task: &Task) -> Result<()>;
    async fn get_task(&self, id: Uuid) -> Result<Option<Task>>;
    async fn update_task(&self, task: &Task) -> Result<()>;
    async fn delete_task(&self, id: Uuid) -> Result<()>;
    async fn list_tasks(
        &self,
        project_id: Option<Uuid>,
        status: Option<String>,
        assignee_id: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<TaskWithProject>>;
    async fn get_overdue_tasks(&self) -> Result<Vec<Task>>;
}

#[async_trait]
pub trait ProjectRepository: Send + Sync {
    async fn create_project(&self, project: &Project) -> Result<()>;
    async fn get_project(&self, id: Uuid) -> Result<Option<Project>>;
    async fn update_project(&self, project: &Project) -> Result<()>;
    async fn delete_project(&self, id: Uuid) -> Result<()>;
    async fn list_projects(&self, owner_id: &str, include_archived: bool) -> Result<Vec<Project>>;
}

#[async_trait]
pub trait ReminderRepository: Send + Sync {
    async fn create_reminder(&self, reminder: &Reminder) -> Result<()>;
    async fn get_reminders_for_task(&self, task_id: Uuid) -> Result<Vec<Reminder>>;
    async fn update_reminder(&self, reminder: &Reminder) -> Result<()>;
    async fn get_pending_reminders(&self) -> Result<Vec<Reminder>>;
    async fn delete_reminder(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn send_notification(&self, title: &str, message: &str, task_id: Uuid) -> Result<()>;
    async fn schedule_reminder(&self, reminder: &Reminder, task: &Task) -> Result<()>;
}

#[async_trait]
pub trait P2pSyncService: Send + Sync {
    async fn broadcast_task_update(&self, task: &Task) -> Result<()>;
    async fn broadcast_project_update(&self, project: &Project) -> Result<()>;
    async fn start_sync_listener(&self) -> Result<()>;
    async fn handle_incoming_sync(&self, data: Vec<u8>) -> Result<()>;
}

#[async_trait]
pub trait TaskService: Send + Sync {
    async fn create_task(
        &self,
        title: String,
        description: Option<String>,
        due_date: Option<DateTime<Utc>>,
        project_id: Option<Uuid>,
        priority: String,
        assignee_id: Option<String>,
        tags: Vec<String>,
        metadata: HashMap<String, String>,
    ) -> Result<Task>;
    
    async fn update_task(
        &self,
        id: Uuid,
        title: Option<String>,
        description: Option<String>,
        status: Option<String>,
        priority: Option<String>,
        due_date: Option<DateTime<Utc>>,
        tags: Option<Vec<String>>,
        metadata: Option<HashMap<String, String>>,
    ) -> Result<Task>;
    
    async fn complete_task(&self, id: Uuid) -> Result<Task>;
    async fn delete_task(&self, id: Uuid) -> Result<()>;
    async fn get_task(&self, id: Uuid) -> Result<TaskWithProject>;
    async fn list_tasks(
        &self,
        project_id: Option<Uuid>,
        status: Option<String>,
        assignee_id: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<TaskWithProject>>;
    
    async fn create_project(
        &self,
        name: String,
        description: Option<String>,
        color: Option<String>,
        owner_id: String,
    ) -> Result<Project>;
    
    async fn update_project(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Project>;
    
    async fn delete_project(&self, id: Uuid) -> Result<()>;
    async fn get_project(&self, id: Uuid) -> Result<Project>;
    async fn list_projects(&self, owner_id: &str, include_archived: bool) -> Result<Vec<Project>>;
    
    async fn create_reminder(
        &self,
        task_id: Uuid,
        remind_at: DateTime<Utc>,
        message: Option<String>,
    ) -> Result<Reminder>;
    
    async fn check_reminders(&self) -> Result<Vec<Reminder>>;
}