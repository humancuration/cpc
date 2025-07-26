use async_trait::async_trait;
use uuid::Uuid;

use crate::cooperative::models::{Cooperative, CooperativeMember};

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Database query failed: {0}")]
    QueryError(#[from] sqlx::Error),
    #[error("Data not found")]
    NotFound,
    #[error("An unexpected error occurred: {0}")]
    Other(#[from] anyhow::Error),
}

#[derive(Debug, Default)]
pub struct CreateCooperativeData {
    pub name: String,
    pub description: Option<String>,
    pub founded_date: chrono::NaiveDate,
    pub website: Option<String>,
}

#[derive(Debug, Default)]
pub struct UpdateCooperativeData {
    pub name: Option<String>,
    pub description: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug)]
pub struct AddMemberData {
  pub cooperative_id: Uuid,
  pub user_id: Uuid,
  pub role: String,
}

#[async_trait]
pub trait CooperativeRepository: Send + Sync {
    async fn create_cooperative(&self, data: &CreateCooperativeData) -> Result<Cooperative, RepositoryError>;
    async fn get_cooperative(&self, id: Uuid) -> Result<Option<Cooperative>, RepositoryError>;
    async fn update_cooperative(&self, id: Uuid, data: &UpdateCooperativeData) -> Result<Cooperative, RepositoryError>;
    async fn delete_cooperative(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn list_cooperatives(&self) -> Result<Vec<Cooperative>, RepositoryError>;

    async fn add_member(&self, data: &AddMemberData) -> Result<CooperativeMember, RepositoryError>;
    async fn get_member(&self, id: Uuid) -> Result<Option<CooperativeMember>, RepositoryError>;
    async fn remove_member(&self, id: Uuid) -> Result<(), RepositoryError>;
    async fn list_members(&self, cooperative_id: Uuid) -> Result<Vec<CooperativeMember>, RepositoryError>;
    async fn update_member_role(&self, member_id: Uuid, role: &str) -> Result<CooperativeMember, RepositoryError>;
}