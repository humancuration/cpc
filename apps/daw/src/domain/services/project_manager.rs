use crate::domain::models::Project;
use crate::domain::ports::persistence::PersistencePort;
use uuid::Uuid;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectManagerError {
    #[error("Project not found: {0}")]
    ProjectNotFound(Uuid),
    
    #[error("Persistence error: {0}")]
    PersistenceError(String),
    
    #[error("Invalid project data")]
    InvalidProjectData,
}

pub struct ProjectManager {
    persistence: Box<dyn PersistencePort>,
}

impl ProjectManager {
    pub fn new(persistence: Box<dyn PersistencePort>) -> Self {
        Self { persistence }
    }
    
    pub async fn create_project(&self, name: String, sample_rate: u32) -> Result<Project, ProjectManagerError> {
        let project = Project::new(name, sample_rate);
        self.persistence.save_project(&project).await
            .map_err(|e| ProjectManagerError::PersistenceError(e.to_string()))?;
        Ok(project)
    }
    
    pub async fn load_project(&self, id: Uuid) -> Result<Project, ProjectManagerError> {
        self.persistence.load_project(id).await
            .map_err(|e| ProjectManagerError::PersistenceError(e.to_string()))?
            .ok_or_else(|| ProjectManagerError::ProjectNotFound(id))
    }
    
    pub async fn save_project(&self, project: &Project) -> Result<(), ProjectManagerError> {
        self.persistence.save_project(project).await
            .map_err(|e| ProjectManagerError::PersistenceError(e.to_string()))
    }
    
    pub async fn delete_project(&self, id: Uuid) -> Result<(), ProjectManagerError> {
        self.persistence.delete_project(id).await
            .map_err(|e| ProjectManagerError::PersistenceError(e.to_string()))
    }
    
    pub async fn list_projects(&self) -> Result<Vec<Project>, ProjectManagerError> {
        self.persistence.list_projects().await
            .map_err(|e| ProjectManagerError::PersistenceError(e.to_string()))
    }
}