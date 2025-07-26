use cpc_core::cooperative::{
    models::{Cooperative, CooperativeMember},
    repository::{AddMemberData, CooperativeRepository, CreateCooperativeData, RepositoryError, UpdateCooperativeData},
};
use std::sync::Arc;
use uuid::Uuid;
use tracing::instrument;

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Repository error: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Not found")]
    NotFound,
    #[error("An unexpected error occurred: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Clone)]
pub struct CooperativeService {
    repository: Arc<dyn CooperativeRepository>,
}

impl CooperativeService {
    pub fn new(repository: Arc<dyn CooperativeRepository>) -> Self {
        Self { repository }
    }

    #[instrument(skip(self, data))]
    pub async fn create_cooperative(&self, data: &CreateCooperativeData) -> Result<Cooperative, ServiceError> {
        if data.name.is_empty() {
            return Err(ServiceError::InvalidInput("Cooperative name cannot be empty".to_string()));
        }
        self.repository.create_cooperative(data).await.map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn get_cooperative(&self, id: Uuid) -> Result<Cooperative, ServiceError> {
        if id.is_nil() {
            return Err(ServiceError::InvalidInput("Cooperative ID cannot be nil".to_string()));
        }

        match self.repository.get_cooperative(id).await {
            Ok(Some(cooperative)) => Ok(cooperative),
            Ok(None) => Err(ServiceError::NotFound),
            Err(e) => Err(ServiceError::Repository(e)),
        }
    }
    
    #[instrument(skip(self))]
    pub async fn list_cooperatives(&self) -> Result<Vec<Cooperative>, ServiceError> {
        self.repository.list_cooperatives().await.map_err(ServiceError::from)
    }

    #[instrument(skip(self, data))]
    pub async fn update_cooperative(&self, id: Uuid, data: &UpdateCooperativeData) -> Result<Cooperative, ServiceError> {
        if id.is_nil() {
            return Err(ServiceError::InvalidInput("Cooperative ID cannot be nil".to_string()));
        }
        self.repository.update_cooperative(id, data).await.map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn delete_cooperative(&self, id: Uuid) -> Result<(), ServiceError> {
        if id.is_nil() {
            return Err(ServiceError::InvalidInput("Cooperative ID cannot be nil".to_string()));
        }
        self.repository.delete_cooperative(id).await.map_err(ServiceError::from)
    }
    
    #[instrument(skip(self, data))]
    pub async fn add_member(&self, data: &AddMemberData) -> Result<CooperativeMember, ServiceError> {
        if data.cooperative_id.is_nil() || data.user_id.is_nil() {
             return Err(ServiceError::InvalidInput("Cooperative and User IDs cannot be nil".to_string()));
        }
        self.repository.add_member(data).await.map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn list_members(&self, cooperative_id: Uuid) -> Result<Vec<CooperativeMember>, ServiceError> {
        if cooperative_id.is_nil() {
            return Err(ServiceError::InvalidInput("Cooperative ID cannot be nil".to_string()));
        }
        self.repository.list_members(cooperative_id).await.map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn remove_member(&self, member_id: Uuid) -> Result<(), ServiceError> {
        if member_id.is_nil() {
             return Err(ServiceError::InvalidInput("Member ID cannot be nil".to_string()));
        }
        self.repository.remove_member(member_id).await.map_err(ServiceError::from)
    }

    #[instrument(skip(self))]
    pub async fn update_member_role(&self, member_id: Uuid, role: &str) -> Result<CooperativeMember, ServiceError> {
        if member_id.is_nil() {
             return Err(ServiceError::InvalidInput("Member ID cannot be nil".to_string()));
        }
        if role.is_empty() {
            return Err(ServiceError::InvalidInput("Role cannot be empty".to_string()));
        }
        self.repository.update_member_role(member_id, role).await.map_err(ServiceError::from)
    }
}