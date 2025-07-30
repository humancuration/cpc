use async_trait::async_trait;
use crate::domain::user::User;
use crate::domain::user::Role;
use crate::domain::auth_error::UserError;
use uuid::Uuid;
use cpc_karma::KarmaService;

#[async_trait]
pub trait UserService: Send + Sync {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, UserError>;
    async fn update_user(&self, user: User) -> Result<(), UserError>;
    async fn assign_role(&self, user_id: Uuid, role: Role) -> Result<(), UserError>;
    async fn get_user_karma(&self, user_id: Uuid) -> i32;
    async fn add_user_karma(&self, user_id: Uuid, amount: i32, reason: String) -> i32;
}