// In apps/backend/src/graphql/project.rs

use async_graphql::{ID, ComplexObject, Context, Object, Result, InputObject};
use cpc_core::business::project::{Project, ProjectStatus, UpdateProject};
use crate::project::service::ProjectService;
use uuid::Uuid;

// This struct will represent the Project in the GraphQL schema.
#[derive(Debug, Clone)]
pub struct ProjectObject(pub Project);

#[Object]
impl ProjectObject {
    async fn id(&self) -> Uuid { self.0.id }
    async fn name(&self) -> &str { &self.0.name }
    async fn description(&self) -> Option<&str> { self.0.description.as_deref() }
    async fn cooperative_id(&self) -> Uuid { self.0.cooperative_id }
    async fn status(&self) -> ProjectStatus { self.0.status.clone() }
    async fn start_date(&self) -> Option<chrono::NaiveDate> { self.0.start_date }
    async fn end_date(&self) -> Option<chrono::NaiveDate> { self.0.end_date }
    async fn created_at(&self) -> chrono::DateTime<chrono::Utc> { self.0.created_at }
    async fn updated_at(&self) -> chrono::DateTime<chrono::Utc> { self.0.updated_at }
}

#[derive(Default)]
pub struct ProjectQuery;

#[Object]
impl ProjectQuery {
    async fn projects(&self, ctx: &Context<'_>) -> Result<Vec<ProjectObject>> {
        let project_service = ctx.data::<ProjectService>()?;
        let projects = project_service
            .list_all_projects()
            .await?
            .into_iter()
            .map(ProjectObject)
            .collect();
        Ok(projects)
    }

    pub async fn project(
        &self,
        ctx: &Context<'_>,
        id: ID,
    ) -> Result<ProjectObject> {
       let project_id = Uuid::parse_str(&id)
           .map_err(|e| async_graphql::Error::new(format!("Invalid project ID: {}", e)))?;
        let project_service = ctx.data::<ProjectService>()?;
        let project = project_service.get_project(project_id).await?;
        Ok(ProjectObject(project))
    }
}

#[derive(InputObject)]
pub struct CreateProjectInput {
    pub name: String,
    pub description: Option<String>,
    pub cooperative_id: Uuid,
}

#[derive(InputObject)]
pub struct UpdateProjectInput {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Default)]
pub struct ProjectMutation;

#[Object]
impl ProjectMutation {
    pub async fn create_project(
        &self,
        ctx: &Context<'_>,
        input: CreateProjectInput,
    ) -> Result<ProjectObject> {
        let project_service = ctx.data::<ProjectService>()?;
        let project = project_service
            .create_project(&input.name, input.description.as_deref(), input.cooperative_id)
            .await?;
        Ok(ProjectObject(project))
    }

    pub async fn update_project(
        &self,
        ctx: &Context<'_>,
        input: UpdateProjectInput,
    ) -> Result<ProjectObject> {
        let project_service = ctx.data::<ProjectService>()?;
        let payload = UpdateProject {
            name: input.name,
            description: input.description,
        };
        let project = project_service
            .update_project(input.id, payload)
            .await?;
        Ok(ProjectObject(project))
    }
}