use crate::domain::user_service::UserService;
use crate::domain::user_repository::UserRepository;
use crate::domain::user::User;
use crate::domain::user::Role;
use crate::domain::auth_error::UserError;
use uuid::Uuid;
use std::sync::Arc;
use cpc_karma::KarmaService;

pub struct UserServiceImpl {
    user_repo: Arc<dyn UserRepository>,
    karma_service: Arc<KarmaService>,
}

impl UserServiceImpl {
    pub fn new(user_repo: Arc<dyn UserRepository>, karma_service: Arc<KarmaService>) -> Self {
        Self { user_repo, karma_service }
    }
}

#[async_trait::async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, UserError> {
        self.user_repo.find_by_id(user_id).await
            .map_err(|e| UserError::DatabaseError(e.to_string()))?
            .ok_or(UserError::UserNotFound)
    }

    async fn update_user(&self, user: User) -> Result<(), UserError> {
        self.user_repo.update(&user).await
            .map_err(|e| UserError::DatabaseError(e.to_string()))
    }

    async fn assign_role(&self, user_id: Uuid, role: Role) -> Result<(), UserError> {
        let mut user = self.get_user_by_id(user_id).await?;
        if !user.roles.contains(&role) {
            user.roles.push(role);
            self.update_user(user).await?;
        }
        Ok(())
    }
    
    async fn get_user_karma(&self, user_id: Uuid) -> i32 {
        self.karma_service.get_karma(&user_id.to_string())
    }
    
    async fn add_user_karma(&self, user_id: Uuid, amount: i32, reason: String) -> i32 {
        self.karma_service.add_karma(user_id.to_string(), amount, reason)
    }
}