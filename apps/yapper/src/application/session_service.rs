use crate::domain::session_management::SessionManagement;
use crate::infrastructure::auth_service_client::YapperAuthServiceClient;
use crate::domain::session::Session;
use crate::domain::auth_error::SessionError;
use uuid::Uuid;
use std::sync::Arc;

pub struct SessionServiceImpl {
    auth_client: Arc<YapperAuthServiceClient>,
}

impl SessionServiceImpl {
    pub fn new(auth_client: Arc<YapperAuthServiceClient>) -> Self {
        Self { auth_client }
    }
}

#[async_trait::async_trait]
impl SessionManagement for SessionServiceImpl {
    async fn create_session(&self, user_id: Uuid, device_info: String) -> Result<Session, SessionError> {
        self.auth_client.create_session(user_id, device_info).await
            .map_err(|e| SessionError::DatabaseError(e.to_string()))
    }

    async fn invalidate_session(&self, session_id: Uuid) -> Result<(), SessionError> {
        self.auth_client.invalidate_session(session_id).await
            .map_err(|e| SessionError::DatabaseError(e.to_string()))
    }

    async fn find_session(&self, session_id: Uuid) -> Result<Option<Session>, SessionError> {
        match self.auth_client.validate_session(session_id).await {
            Ok(_) => {
                // In a real implementation, we would return the full session object
                // For now, we'll just return None since we don't have the full session data
                Ok(None)
            }
            Err(_) => Ok(None),
        }
    }
}