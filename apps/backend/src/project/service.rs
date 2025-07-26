use cpc_core::{
    business::project::{Project, ProjectStatus, UpdateProject},
    repositories::project_repository::ProjectRepository,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] sqlx::Error),
    #[error("Not found")]
    NotFound,
}

#[derive(Clone)]
pub struct ProjectService {
    repository: Arc<ProjectRepository>,
}

impl ProjectService {
    pub fn new(repository: Arc<ProjectRepository>) -> Self {
        Self { repository }
    }

    pub async fn create_project(&self, name: &str, description: Option<&str>, cooperative_id: Uuid) -> Result<Project, ServiceError> {
        self.repository
            .create(name, description, cooperative_id)
            .await
            .map_err(ServiceError::from)
    }

    pub async fn get_project(&self, id: Uuid) -> Result<Project, ServiceError> {
        match self.repository.find_by_id(id).await {
            Ok(Some(project)) => Ok(project),
            Ok(None) => Err(ServiceError::NotFound),
            Err(e) => Err(ServiceError::Repository(e)),
        }
    }

    pub async fn list_all_projects(&self) -> Result<Vec<Project>, ServiceError> {
        self.repository
            .list_all()
            .await
            .map_err(ServiceError::from)
    }

    pub async fn list_projects_by_cooperative(&self, cooperative_id: Uuid) -> Result<Vec<Project>, ServiceError> {
        self.repository
            .list_by_cooperative_id(cooperative_id)
            .await
            .map_err(ServiceError::from)
    }

    pub async fn update_project_status(&self, id: Uuid, status: ProjectStatus) -> Result<Project, ServiceError> {
        self.repository
            .update_status(id, status)
            .await
            .map_err(ServiceError::from)
    }

    pub async fn update_project(&self, id: Uuid, payload: UpdateProject) -> Result<Project, ServiceError> {
        self.repository
            .update(id, payload)
            .await
            .map_err(ServiceError::from)
    }
}