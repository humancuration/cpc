use async_trait::async_trait;
use crate::domain::auth::{Credentials, User, AuthError, community_role::CommunityRole};
use crate::infrastructure::repositories::user_repository::UserRepository;
use cpc_auth::auth_service::AuthServiceImpl as BaseAuthService;
use cpc_auth::models::Session;
use uuid::Uuid;
use std::sync::Arc;

#[async_trait]
pub trait AuthService: Send + Sync {
    async fn register(&self, credentials: Credentials) -> Result<User, AuthError>;
    async fn login(&self, credentials: Credentials) -> Result<Session, AuthError>;
    async fn logout(&self, session_id: Uuid) -> Result<(), AuthError>;
    async fn validate_session(&self, session_id: Uuid) -> Result<User, AuthError>;
    async fn refresh_token(&self, refresh_token: String) -> Result<Session, AuthError>;
    async fn initiate_password_reset(&self, email: String) -> Result<(), AuthError>;
    async fn confirm_password_reset(&self, token: String, new_password: String) -> Result<(), AuthError>;
    
    // Karma-specific methods
    async fn increment_karma(&self, user_id: Uuid, amount: i32) -> Result<(), AuthError>;
    async fn get_karma(&self, user_id: Uuid) -> Result<i32, AuthError>;
    
    // Community role methods
    async fn assign_community_role(&self, user_id: Uuid, role: CommunityRole) -> Result<(), AuthError>;
    async fn get_community_roles(&self, user_id: Uuid) -> Result<Vec<CommunityRole>, AuthError>;
    
    // Vote event handling
    async fn handle_vote_event(&self, event: crate::domain::vote::VoteEvent) -> Result<(), AuthError>;
}

pub struct AllatAuthService {
    base_service: Arc<BaseAuthService>,
    user_repository: Arc<dyn UserRepository>,
}

impl AllatAuthService {
    pub fn new(base_service: Arc<BaseAuthService>, user_repository: Arc<dyn UserRepository>) -> Self {
        Self { base_service, user_repository }
    }
}

#[async_trait]
impl AuthService for AllatAuthService {
    async fn register(&self, credentials: Credentials) -> Result<User, AuthError> {
        let base_user = self.base_service.register(credentials).await?;
        let user = User {
            base: base_user,
            karma: 0, // Initial karma
        };
        
        // Save the user with initial karma to our repository
        self.user_repository.save(&user).await.map_err(|e| AuthError::DatabaseError(e))?;
        
        Ok(user)
    }
    
    async fn login(&self, credentials: Credentials) -> Result<Session, AuthError> {
        self.base_service.login(credentials).await
    }
    
    async fn logout(&self, session_id: Uuid) -> Result<(), AuthError> {
        self.base_service.logout(session_id).await
    }
    
    async fn validate_session(&self, session_id: Uuid) -> Result<User, AuthError> {
        let base_user = self.base_service.validate_session(session_id).await?;
        
        // Try to fetch the user's karma from our repository
        match self.user_repository.find_by_id(base_user.id).await {
            Ok(Some(user)) => Ok(user),
            Ok(None) => {
                // User exists in base auth but not in our repository, create with default values
                let user = User {
                    base: base_user,
                    karma: 0,
                };
                self.user_repository.save(&user).await.map_err(|e| AuthError::DatabaseError(e))?;
                Ok(user)
            },
            Err(e) => Err(AuthError::DatabaseError(e)),
        }
    }
    
    async fn refresh_token(&self, refresh_token: String) -> Result<Session, AuthError> {
        self.base_service.refresh_token(refresh_token).await
    }
    
    async fn initiate_password_reset(&self, email: String) -> Result<(), AuthError> {
        self.base_service.initiate_password_reset(email).await
    }
    
    async fn confirm_password_reset(&self, token: String, new_password: String) -> Result<(), AuthError> {
        self.base_service.confirm_password_reset(token, new_password).await
    }
    
    async fn increment_karma(&self, user_id: Uuid, amount: i32) -> Result<(), AuthError> {
        self.user_repository.update_karma(user_id, amount).await.map_err(|e| AuthError::DatabaseError(e))
    }
    
    async fn get_karma(&self, user_id: Uuid) -> Result<i32, AuthError> {
        match self.user_repository.find_by_id(user_id).await {
            Ok(Some(user)) => Ok(user.karma),
            Ok(None) => Err(AuthError::UserNotFound),
            Err(e) => Err(AuthError::DatabaseError(e)),
        }
    }
    
    async fn assign_community_role(&self, user_id: Uuid, role: CommunityRole) -> Result<(), AuthError> {
        self.user_repository.assign_community_role(user_id, role).await.map_err(|e| AuthError::DatabaseError(e))
    }
    
    async fn get_community_roles(&self, user_id: Uuid) -> Result<Vec<CommunityRole>, AuthError> {
        self.user_repository.get_community_roles(user_id).await.map_err(|e| AuthError::DatabaseError(e))
    }
    
    async fn handle_vote_event(&self, event: crate::domain::vote::VoteEvent) -> Result<(), AuthError> {
        let delta = match event.vote_type {
            crate::domain::vote::VoteType::Upvote => 1,
            crate::domain::vote::VoteType::Downvote => -1,
        };
        
        self.increment_karma(event.user_id, delta).await
    }
}