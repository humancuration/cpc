use async_trait::async_trait;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::{Task, Project, Reminder, TaskWithProject, TaskStatus, Priority, TaskError, Result};
use super::ports::{TaskRepository, ProjectRepository, ReminderRepository, NotificationService, P2pSyncService, TaskService};

pub struct TaskServiceImpl {
    task_repo: Arc<dyn TaskRepository>,
    project_repo: Arc<dyn ProjectRepository>,
    reminder_repo: Arc<dyn ReminderRepository>,
    notifier: Arc<dyn NotificationService>,
    p2p_sync: Arc<dyn P2pSyncService>,
}

impl TaskServiceImpl {
    pub fn new(
        task_repo: Arc<dyn TaskRepository>,
        project_repo: Arc<dyn ProjectRepository>,
        reminder_repo: Arc<dyn ReminderRepository>,
        notifier: Arc<dyn NotificationService>,
        p2p_sync: Arc<dyn P2pSyncService>,
    ) -> Self {
        Self {
            task_repo,
            project_repo,
            reminder_repo,
            notifier,
            p2p_sync,
        }
    }

    fn parse_priority(priority: &str) -> Result<Priority> {
        match priority.to_lowercase().as_str() {
            "low" => Ok(Priority::Low),
            "medium" => Ok(Priority::Medium),
            "high" => Ok(Priority::High),
            "urgent" => Ok(Priority::Urgent),
            _ => Err(TaskError::ValidationError(format!("Invalid priority: {}", priority))),
        }
    }

    fn parse_status(status: &str) -> Result<TaskStatus> {
        match status.to_lowercase().as_str() {
            "pending" => Ok(TaskStatus::Pending),
            "in_progress" => Ok(TaskStatus::InProgress),
            "completed" => Ok(TaskStatus::Completed),
            "cancelled" => Ok(TaskStatus::Cancelled),
            _ => Err(TaskError::ValidationError(format!("Invalid status: {}", status))),
        }
    }
}

#[async_trait]
impl TaskService for TaskServiceImpl {
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
    ) -> Result<Task> {
        let mut task = Task::new(title, description, due_date);
        task.priority = Self::parse_priority(&priority)?;
        task.project_id = project_id;
        task.assignee_id = assignee_id;
        task.tags = tags;
        task.metadata = metadata;

        task.validate()?;

        self.task_repo.create_task(&task).await?;
        
        // Sync with p2p network
        if let Err(e) = self.p2p_sync.broadcast_task_update(&task).await {
            tracing::warn!("Failed to sync task creation: {}", e);
        }

        Ok(task)
    }

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
    ) -> Result<Task> {
        let mut task = self.task_repo.get_task(id).await?
            .ok_or_else(|| TaskError::TaskNotFound(id.to_string()))?;

        if let Some(title) = title {
            task.title = title;
        }
        if let Some(description) = description {
            task.description = description;
        }
        if let Some(status) = status {
            task.update_status(Self::parse_status(&status)?);
        }
        if let Some(priority) = priority {
            task.update_priority(Self::parse_priority(&priority)?);
        }
        if let Some(due_date) = due_date {
            task.due_date = Some(due_date);
        }
        if let Some(tags) = tags {
            task.tags = tags;
        }
        if let Some(metadata) = metadata {
            task.metadata.extend(metadata);
        }

        task.validate()?;
        
        self.task_repo.update_task(&task).await?;
        
        // Sync with p2p network
        if let Err(e) = self.p2p_sync.broadcast_task_update(&task).await {
            tracing::warn!("Failed to sync task update: {}", e);
        }

        Ok(task)
    }

    async fn complete_task(&self, id: Uuid) -> Result<Task> {
        let mut task = self.task_repo.get_task(id).await?
            .ok_or_else(|| TaskError::TaskNotFound(id.to_string()))?;

        task.complete();
        
        self.task_repo.update_task(&task).await?;
        
        // Sync with p2p network
        if let Err(e) = self.p2p_sync.broadcast_task_update(&task).await {
            tracing::warn!("Failed to sync task completion: {}", e);
        }

        Ok(task)
    }

    async fn delete_task(&self, id: Uuid) -> Result<()> {
        // Also delete associated reminders
        let reminders = self.reminder_repo.get_reminders_for_task(id).await?;
        for reminder in reminders {
            let _ = self.reminder_repo.delete_reminder(reminder.id).await;
        }

        self.task_repo.delete_task(id).await
    }

    async fn get_task(&self, id: Uuid) -> Result<TaskWithProject> {
        let task = self.task_repo.get_task(id).await?
            .ok_or_else(|| TaskError::TaskNotFound(id.to_string()))?;

        let project = match task.project_id {
            Some(project_id) => self.project_repo.get_project(project_id).await?,
            None => None,
        };

        let reminders = self.reminder_repo.get_reminders_for_task(id).await?;

        Ok(TaskWithProject {
            task,
            project,
            reminders,
        })
    }

    async fn list_tasks(
        &self,
        project_id: Option<Uuid>,
        status: Option<String>,
        assignee_id: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<TaskWithProject>> {
        self.task_repo.list_tasks(project_id, status, assignee_id, limit, offset).await
    }

    async fn create_project(
        &self,
        name: String,
        description: Option<String>,
        color: Option<String>,
        owner_id: String,
    ) -> Result<Project> {
        let mut project = Project::new(name, description, owner_id);
        project.color = color;
        
        project.validate()?;
        
        self.project_repo.create_project(&project).await?;
        
        // Sync with p2p network
        if let Err(e) = self.p2p_sync.broadcast_project_update(&project).await {
            tracing::warn!("Failed to sync project creation: {}", e);
        }

        Ok(project)
    }

    async fn update_project(
        &self,
        id: Uuid,
        name: Option<String>,
        description: Option<String>,
        color: Option<String>,
    ) -> Result<Project> {
        let mut project = self.project_repo.get_project(id).await?
            .ok_or_else(|| TaskError::ProjectNotFound(id.to_string()))?;

        if let Some(name) = name {
            project.name = name;
        }
        if let Some(description) = description {
            project.description = description;
        }
        if let Some(color) = color {
            project.color = color;
        }

        project.validate()?;
        
        self.project_repo.update_project(&project).await?;
        
        // Sync with p2p network
        if let Err(e) = self.p2p_sync.broadcast_project_update(&project).await {
            tracing::warn!("Failed to sync project update: {}", e);
        }

        Ok(project)
    }

    async fn delete_project(&self, id: Uuid) -> Result<()> {
        // Check if project has tasks
        let tasks = self.task_repo.list_tasks(Some(id), None, None, 1, 0).await?;
        if !tasks.is_empty() {
            return Err(TaskError::ValidationError(
                "Cannot delete project with existing tasks".to_string()
            ));
        }

        self.project_repo.delete_project(id).await
    }

    async fn get_project(&self, id: Uuid) -> Result<Project> {
        self.project_repo.get_project(id).await?
            .ok_or_else(|| TaskError::ProjectNotFound(id.to_string()))
    }

    async fn list_projects(&self, owner_id: &str, include_archived: bool) -> Result<Vec<Project>> {
        self.project_repo.list_projects(owner_id, include_archived).await
    }

    async fn create_reminder(
        &self,
        task_id: Uuid,
        remind_at: DateTime<Utc>,
        message: Option<String>,
    ) -> Result<Reminder> {
        // Verify task exists
        let task = self.task_repo.get_task(task_id).await?
            .ok_or_else(|| TaskError::TaskNotFound(task_id.to_string()))?;

        let reminder = Reminder::new(task_id, remind_at, message);
        
        self.reminder_repo.create_reminder(&reminder).await?;
        
        // Schedule notification
        if let Err(e) = self.notifier.schedule_reminder(&reminder, &task).await {
            tracing::warn!("Failed to schedule reminder notification: {}", e);
        }

        Ok(reminder)
    }

    async fn check_reminders(&self) -> Result<Vec<Reminder>> {
        let pending = self.reminder_repo.get_pending_reminders().await?;
        let mut triggered = Vec::new();

        for reminder in pending {
            if reminder.should_trigger() {
                let task = self.task_repo.get_task(reminder.task_id).await?
                    .ok_or_else(|| TaskError::TaskNotFound(reminder.task_id.to_string()))?;

                let message = reminder.message.as_ref()
                    .unwrap_or(&format!("Reminder: {}", task.title));

                if let Err(e) = self.notifier.send_notification(
                    "Task Reminder",
                    message,
                    task.id,
                ).await {
                    tracing::warn!("Failed to send reminder notification: {}", e);
                }

                let mut updated_reminder = reminder.clone();
                updated_reminder.mark_sent();
                self.reminder_repo.update_reminder(&updated_reminder).await?;
                
                triggered.push(reminder);
            }
        }

        Ok(triggered)
    }
}