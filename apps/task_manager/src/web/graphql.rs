use async_graphql::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::application::ports::TaskService;
use crate::web::dto::*;

pub type ServiceSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn task(&self, ctx: &Context<'_>, id: Uuid) -> Result<TaskWithProjectDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let task = service.get_task(id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(task.into())
    }

    async fn tasks(
        &self,
        ctx: &Context<'_>,
        project_id: Option<Uuid>,
        status: Option<String>,
        assignee_id: Option<String>,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> Result<Vec<TaskWithProjectDto>> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let tasks = service.list_tasks(
            project_id,
            status,
            assignee_id,
            limit.unwrap_or(50),
            offset.unwrap_or(0),
        ).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(tasks.into_iter().map(|t| t.into()).collect())
    }

    async fn project(&self, ctx: &Context<'_>, id: Uuid) -> Result<ProjectDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let project = service.get_project(id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(project.into())
    }

    async fn projects(
        &self,
        ctx: &Context<'_>,
        owner_id: String,
        include_archived: Option<bool>,
    ) -> Result<Vec<ProjectDto>> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let projects = service.list_projects(&owner_id, include_archived.unwrap_or(false)).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(projects.into_iter().map(|p| p.into()).collect())
    }
}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_task(
        &self,
        ctx: &Context<'_>,
        input: CreateTaskInput,
    ) -> Result<TaskDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let task = service.create_task(
            input.title,
            input.description,
            input.due_date,
            input.project_id,
            input.priority.to_string(),
            input.assignee_id,
            input.tags,
            input.metadata,
        ).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(task.into())
    }

    async fn update_task(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateTaskInput,
    ) -> Result<TaskDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let task = service.update_task(
            id,
            input.title,
            input.description,
            input.status.map(|s| s.to_string()),
            input.priority.map(|p| p.to_string()),
            input.due_date,
            input.tags,
            input.metadata,
        ).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(task.into())
    }

    async fn complete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<TaskDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let task = service.complete_task(id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(task.into())
    }

    async fn delete_task(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        service.delete_task(id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(true)
    }

    async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
        owner_id: String,
    ) -> Result<ProjectDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let project = service.create_project(
            input.name,
            input.description,
            input.color,
            owner_id,
        ).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(project.into())
    }

    async fn update_project(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
        input: UpdateProjectInput,
    ) -> Result<ProjectDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let project = service.update_project(
            id,
            input.name,
            input.description,
            input.color,
        ).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(project.into())
    }

    async fn delete_project(&self, ctx: &Context<'_>, id: Uuid) -> Result<bool> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        service.delete_project(id).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(true)
    }

    async fn create_reminder(
        &self,
        ctx: &Context<'_>,
        input: CreateReminderInput,
    ) -> Result<ReminderDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        let reminder = service.create_reminder(
            input.task_id,
            input.remind_at,
            input.message,
        ).await
            .map_err(|e| async_graphql::Error::new(e.to_string()))?;
        
        Ok(reminder.into())
    }
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
    async fn reminder_alert(&self, ctx: &Context<'_>) -> impl Stream<Item = ReminderDto> {
        let service = ctx.data_unchecked::<Arc<dyn TaskService>>();
        
        // This is a simplified implementation
        // In a real system, you'd use a proper pub/sub mechanism
        async_stream::stream! {
            loop {
                tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
                
                match service.check_reminders().await {
                    Ok(reminders) => {
                        for reminder in reminders {
                            yield reminder.into();
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to check reminders: {}", e);
                    }
                }
            }
        }
    }
}

// Conversion traits
impl From<crate::domain::Task> for TaskDto {
    fn from(task: crate::domain::Task) -> Self {
        TaskDto {
            id: task.id,
            title: task.title,
            description: task.description,
            status: task.status.into(),
            priority: task.priority.into(),
            due_date: task.due_date,
            created_at: task.created_at,
            updated_at: task.updated_at,
            completed_at: task.completed_at,
            project_id: task.project_id,
            assignee_id: task.assignee_id,
            tags: task.tags,
            metadata: task.metadata,
        }
    }
}

impl From<crate::domain::Project> for ProjectDto {
    fn from(project: crate::domain::Project) -> Self {
        ProjectDto {
            id: project.id,
            name: project.name,
            description: project.description,
            color: project.color,
            created_at: project.created_at,
            updated_at: project.updated_at,
            owner_id: project.owner_id,
            is_archived: project.is_archived,
        }
    }
}

impl From<