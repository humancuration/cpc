use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::{PgPool, Row};
use uuid::Uuid;

use crate::domain::{Task, Project, Reminder, TaskWithProject, TaskStatus, Priority, Result};
use crate::application::ports::{TaskRepository, ProjectRepository, ReminderRepository};

pub struct PostgresTaskRepository {
    pool: PgPool,
}

impl PostgresTaskRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TaskRepository for PostgresTaskRepository {
    async fn create_task(&self, task: &Task) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO tasks (id, title, description, status, priority, due_date, 
                             created_at, updated_at, completed_at, project_id, 
                             assignee_id, tags, metadata)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)
            "#,
            task.id,
            task.title,
            task.description,
            task.status.to_string(),
            task.priority.to_string(),
            task.due_date,
            task.created_at,
            task.updated_at,
            task.completed_at,
            task.project_id,
            task.assignee_id,
            &task.tags,
            serde_json::to_value(&task.metadata)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_task(&self, id: Uuid) -> Result<Option<Task>> {
        let row = sqlx::query!(
            r#"
            SELECT id, title, description, status, priority, due_date,
                   created_at, updated_at, completed_at, project_id,
                   assignee_id, tags, metadata
            FROM tasks
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Task {
            id: row.id,
            title: row.title,
            description: row.description,
            status: TaskStatus::from_str(&row.status).unwrap_or(TaskStatus::Pending),
            priority: Priority::from_str(&row.priority).unwrap_or(Priority::Medium),
            due_date: row.due_date,
            created_at: row.created_at,
            updated_at: row.updated_at,
            completed_at: row.completed_at,
            project_id: row.project_id,
            assignee_id: row.assignee_id,
            tags: row.tags.unwrap_or_default(),
            metadata: serde_json::from_value(row.metadata.unwrap_or_default()).unwrap_or_default(),
        }))
    }

    async fn update_task(&self, task: &Task) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE tasks
            SET title = $2,
                description = $3,
                status = $4,
                priority = $5,
                due_date = $6,
                updated_at = $7,
                completed_at = $8,
                project_id = $9,
                assignee_id = $10,
                tags = $11,
                metadata = $12
            WHERE id = $1
            "#,
            task.id,
            task.title,
            task.description,
            task.status.to_string(),
            task.priority.to_string(),
            task.due_date,
            task.updated_at,
            task.completed_at,
            task.project_id,
            task.assignee_id,
            &task.tags,
            serde_json::to_value(&task.metadata)?
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_task(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM tasks WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_tasks(
        &self,
        project_id: Option<Uuid>,
        status: Option<String>,
        assignee_id: Option<String>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<TaskWithProject>> {
        let mut query = r#"
            SELECT t.*, p.id as project_id, p.name as project_name, p.description as project_description,
                   p.color as project_color, p.created_at as project_created_at,
                   p.updated_at as project_updated_at, p.owner_id as project_owner_id,
                   p.is_archived as project_is_archived
            FROM tasks t
            LEFT JOIN projects p ON t.project_id = p.id
            WHERE 1=1
        "#.to_string();

        let mut args = sqlx::postgres::PgArguments::default();
        
        if let Some(project_id) = project_id {
            query.push_str(" AND t.project_id = $1");
            args.add(project_id);
        }
        
        if let Some(status) = status {
            query.push_str(" AND t.status = $2");
            args.add(status);
        }
        
        if let Some(assignee_id) = assignee_id {
            query.push_str(" AND t.assignee_id = $3");
            args.add(assignee_id);
        }

        query.push_str(" ORDER BY t.created_at DESC LIMIT $4 OFFSET $5");
        args.add(limit);
        args.add(offset);

        let rows = sqlx::query_as_with::<_, TaskRow, _>(&query, args)
            .fetch_all(&self.pool)
            .await?;

        let mut tasks = Vec::new();
        for row in rows {
            let task = Task {
                id: row.id,
                title: row.title,
                description: row.description,
                status: TaskStatus::from_str(&row.status).unwrap_or(TaskStatus::Pending),
                priority: Priority::from_str(&row.priority).unwrap_or(Priority::Medium),
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                completed_at: row.completed_at,
                project_id: row.project_id,
                assignee_id: row.assignee_id,
                tags: row.tags.unwrap_or_default(),
                metadata: serde_json::from_value(row.metadata.unwrap_or_default()).unwrap_or_default(),
            };

            let project = row.project_id.map(|id| Project {
                id,
                name: row.project_name.unwrap_or_default(),
                description: row.project_description,
                color: row.project_color,
                created_at: row.project_created_at.unwrap_or(Utc::now()),
                updated_at: row.project_updated_at.unwrap_or(Utc::now()),
                owner_id: row.project_owner_id.unwrap_or_default(),
                is_archived: row.project_is_archived.unwrap_or(false),
            });

            let reminders = self.reminder_repo.get_reminders_for_task(task.id).await?;
            
            tasks.push(TaskWithProject {
                task,
                project,
                reminders,
            });
        }

        Ok(tasks)
    }

    async fn get_overdue_tasks(&self) -> Result<Vec<Task>> {
        let rows = sqlx::query!(
            r#"
            SELECT *
            FROM tasks
            WHERE due_date < NOW()
              AND status != 'completed'
              AND status != 'cancelled'
            ORDER BY due_date ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut tasks = Vec::new();
        for row in rows {
            tasks.push(Task {
                id: row.id,
                title: row.title,
                description: row.description,
                status: TaskStatus::from_str(&row.status).unwrap_or(TaskStatus::Pending),
                priority: Priority::from_str(&row.priority).unwrap_or(Priority::Medium),
                due_date: row.due_date,
                created_at: row.created_at,
                updated_at: row.updated_at,
                completed_at: row.completed_at,
                project_id: row.project_id,
                assignee_id: row.assignee_id,
                tags: row.tags.unwrap_or_default(),
                metadata: serde_json::from_value(row.metadata.unwrap_or_default()).unwrap_or_default(),
            });
        }

        Ok(tasks)
    }
}

pub struct PostgresProjectRepository {
    pool: PgPool,
}

impl PostgresProjectRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ProjectRepository for PostgresProjectRepository {
    async fn create_project(&self, project: &Project) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO projects (id, name, description, color, created_at, updated_at, owner_id, is_archived)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            project.id,
            project.name,
            project.description,
            project.color,
            project.created_at,
            project.updated_at,
            project.owner_id,
            project.is_archived
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_project(&self, id: Uuid) -> Result<Option<Project>> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, color, created_at, updated_at, owner_id, is_archived
            FROM projects
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|row| Project {
            id: row.id,
            name: row.name,
            description: row.description,
            color: row.color,
            created_at: row.created_at,
            updated_at: row.updated_at,
            owner_id: row.owner_id,
            is_archived: row.is_archived,
        }))
    }

    async fn update_project(&self, project: &Project) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE projects
            SET name = $2,
                description = $3,
                color = $4,
                updated_at = $5,
                is_archived = $6
            WHERE id = $1
            "#,
            project.id,
            project.name,
            project.description,
            project.color,
            project.updated_at,
            project.is_archived
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn delete_project(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM projects WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn list_projects(&self, owner_id: &str, include_archived: bool) -> Result<Vec<Project>> {
        let rows = if include_archived {
            sqlx::query!(
                r#"
                SELECT id, name, description, color, created_at, updated_at, owner_id, is_archived
                FROM projects
                WHERE owner_id = $1
                ORDER BY created_at DESC
                "#,
                owner_id
            )
            .fetch_all(&self.pool)
            .await?
        } else {
            sqlx::query!(
                r#"
                SELECT id, name, description, color, created_at, updated_at, owner_id, is_archived
                FROM projects
                WHERE owner_id = $1 AND is_archived = false
                ORDER BY created_at DESC
                "#,
                owner_id
            )
            .fetch_all(&self.pool)
            .await?
        };

        let mut projects = Vec::new();
        for row in rows {
            projects.push(Project {
                id: row.id,
                name: row.name,
                description: row.description,
                color: row.color,
                created_at: row.created_at,
                updated_at: row.updated_at,
                owner_id: row.owner_id,
                is_archived: row.is_archived,
            });
        }

        Ok(projects)
    }
}

pub struct PostgresReminderRepository {
    pool: PgPool,
}

impl PostgresReminderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ReminderRepository for PostgresReminderRepository {
    async fn create_reminder(&self, reminder: &Reminder) -> Result<()> {
        sqlx::query!(
            r#"
            INSERT INTO reminders (id, task_id, remind_at, message, is_sent, created_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            reminder.id,
            reminder.task_id,
            reminder.remind_at,
            reminder.message,
            reminder.is_sent,
            reminder.created_at
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_reminders_for_task(&self, task_id: Uuid) -> Result<Vec<Reminder>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task_id, remind_at, message, is_sent, created_at
            FROM reminders
            WHERE task_id = $1
            ORDER BY remind_at ASC
            "#,
            task_id
        )
        .fetch_all(&self.pool)
        .await?;

        let mut reminders = Vec::new();
        for row in rows {
            reminders.push(Reminder {
                id: row.id,
                task_id: row.task_id,
                remind_at: row.remind_at,
                message: row.message,
                is_sent: row.is_sent,
                created_at: row.created_at,
            });
        }

        Ok(reminders)
    }

    async fn update_reminder(&self, reminder: &Reminder) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE reminders
            SET remind_at = $2,
                message = $3,
                is_sent = $4
            WHERE id = $1
            "#,
            reminder.id,
            reminder.remind_at,
            reminder.message,
            reminder.is_sent
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_pending_reminders(&self) -> Result<Vec<Reminder>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, task_id, remind_at, message, is_sent, created_at
            FROM reminders
            WHERE is_sent = false AND remind_at <= NOW()
            ORDER BY remind_at ASC
            "#
        )
        .fetch_all(&self.pool)
        .await?;

        let mut reminders = Vec::new();
        for row in rows {
            reminders.push(Reminder {
                id: row.id,
                task_id: row.task_id,
                remind_at: row.remind_at,
                message: row.message,
                is_sent: row.is_sent,
                created_at: row.created_at,
            });
        }

        Ok(reminders)
    }

    async fn delete_reminder(&self, id: Uuid) -> Result<()> {
        sqlx::query!(
            "DELETE FROM reminders WHERE id = $1",
            id
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}

// Helper struct for database rows
struct TaskRow {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: String,
    priority: String,
    due_date: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    completed_at: Option<DateTime<Utc>>,
    project_id: Option<Uuid>,
    assignee_id: Option<String>,
    tags: Option<Vec<String>>,
    metadata: Option<serde_json::Value>,
    project_id: Option<Uuid>,
    project_name: Option<String>,
    project_description: Option<String>,
    project_color: Option<String>,
    project_created_at: Option<DateTime<Utc>>,
    project_updated_at: Option<DateTime<Utc>>,
    project_owner_id: Option<String>,
    project_is_archived: Option<bool>,
}

impl TaskStatus {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "pending" => Some(TaskStatus::Pending),
            "in_progress" => Some(TaskStatus::InProgress),
            "completed" => Some(TaskStatus::Completed),
            "cancelled" => Some(TaskStatus::Cancelled),
            _ => None,
        }
    }
}

impl Priority {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "low" => Some(Priority::Low),
            "medium" => Some(Priority::Medium),
            "high" => Some(Priority::High),
            "urgent" => Some(Priority::Urgent),
            _ => None,
        }