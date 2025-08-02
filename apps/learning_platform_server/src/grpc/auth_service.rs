use tonic::{Request, Response, Status};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use crate::database::repository::DatabaseRepository;
use crate::database::models::User;
use crate::middleware::auth::Claims;

// Import the generated protobuf types
tonic::include_proto!("cpc.learning_platform");

pub struct AuthService {
    repository: DatabaseRepository,
}

impl AuthService {
    pub fn new(repository: DatabaseRepository) -> Self {
        Self { repository }
    }
}

#[tonic::async_trait]
impl auth_service_server::AuthService for AuthService {
    async fn authenticate(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let req = request.into_inner();
        
        // Look up user by username
        let user = self.repository.get_user_by_username(&req.username).await
            .map_err(|e| Status::internal(format!("Failed to lookup user: {}", e)))?
            .ok_or_else(|| Status::unauthenticated("Invalid username or password"))?;
        
        // Verify password
        let valid = verify(&req.password, &user.password_hash)
            .map_err(|_| Status::internal("Failed to verify password"))?;
        
        if !valid {
            return Err(Status::unauthenticated("Invalid username or password"));
        }
        
        // Create claims for JWT
        let exp = Utc::now()
            .checked_add_signed(Duration::hours(24))
            .expect("valid timestamp")
            .timestamp();
        
        let claims = Claims {
            sub: user.id,
            exp: exp as usize,
        };
        
        // Get JWT secret from environment or use default (in production, always use environment)
        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| "secret".to_string());
        
        // Create JWT token
        let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_bytes()))
            .map_err(|_| Status::internal("Failed to create token"))?;
        
        let response = AuthResponse {
            access_token: token,
            refresh_token: "".to_string(), // Not implemented in this example
            user_id: user.id.to_string(),
        };
        
        Ok(Response::new(response))
    }
}