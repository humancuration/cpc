use crate::auth_service::AuthService;
use crate::models::User;
use crate::error::AuthError;
use std::sync::Arc;
use uuid::Uuid;

pub struct AuthMiddleware {
    auth_service: Arc<dyn AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn validate_session(&self, session_id: Uuid) -> Result<User, AuthError> {
        self.auth_service.validate_session(session_id).await
    }
}