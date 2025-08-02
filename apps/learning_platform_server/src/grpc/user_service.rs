use tonic::{Request, Response, Status};
use uuid::Uuid;
use bcrypt::{hash, DEFAULT_COST};
use chrono::Utc;
use crate::database::repository::DatabaseRepository;
use crate::database::models::User as DatabaseUser;
use crate::error::AppError;
use crate::utils::{validate_not_empty, validate_email, validate_password_strength};

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");
tonic::include_proto!("cpc.learning_platform_server");

pub struct UserService {
    repository: DatabaseRepository,
}

impl UserService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl user_service_server::UserService for UserService {
    async fn register_user(
        &self,
        request: Request<RegisterUserRequest>,
    ) -> Result<Response<RegisterUserResponse>, Status> {
        let req = request.into_inner();
        
        // Validate inputs
        validate_not_empty(&req.username, "Username")?;
        validate_not_empty(&req.email, "Email")?;
        validate_email(&req.email)?;
        validate_password_strength(&req.password)?;
        
        // Check if username already exists
        if self.repository.get_user_by_username(&req.username).await
            .map_err(AppError::from)?
            .is_some() {
            return Err(AppError::AlreadyExists("Username already exists".to_string()).into());
        }
        
        // In a real implementation, we would also check if email exists
        
        // Hash password
        let password_hash = hash(&req.password, DEFAULT_COST)
            .map_err(|_| AppError::Internal("Failed to hash password".to_string()))?;
        
        // Create user
        let user_id = Uuid::new_v4();
        let db_user = DatabaseUser {
            id: user_id,
            username: req.username,
            email: req.email,
            password_hash,
            created_at: Utc::now(),
        };
        
        // Save to database
        let saved_user = self.repository.create_user(&db_user).await
            .map_err(AppError::from)?;
        
        let response = RegisterUserResponse {
            user_id: saved_user.id.to_string(),
        };
        
        Ok(Response::new(response))
    }
}