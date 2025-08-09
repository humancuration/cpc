use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    pub id: Uuid,
    pub name: String,
    pub resource: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<Permission>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRole {
    pub user_id: Uuid,
    pub role_id: Uuid,
}

#[async_trait::async_trait]
pub trait RbacService {
    async fn check_permission(&self, user_id: Uuid, resource: &str, action: &str) -> Result<bool, RbacError>;
    async fn assign_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), RbacError>;
    async fn revoke_role(&self, user_id: Uuid, role_id: Uuid) -> Result<(), RbacError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RbacError {
    #[error("Permission denied")]
    PermissionDenied,
    #[error("Role not found")]
    RoleNotFound,
    #[error("User not found")]
    UserNotFound,
}