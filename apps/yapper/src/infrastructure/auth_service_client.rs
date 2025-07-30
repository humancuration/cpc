//! # Auth Service Client
//!
//! gRPC client for the Auth Service that provides session management for the Yapper app.

use cpc_auth::auth_service::auth_client::AuthServiceClient;
use cpc_auth::models::{Session, User};
use cpc_auth::error::AuthError;
use tonic::transport::Channel;
use uuid::Uuid;

/// Yapper Auth Service Client
///
/// This client communicates with the centralized Auth Service via gRPC
/// to handle session creation, validation, and invalidation using Redis.
pub struct YapperAuthServiceClient {
    client: AuthServiceClient<Channel>,
}

impl YapperAuthServiceClient {
    /// Create a new Auth Service client
    ///
    /// # Arguments
    /// * `addr` - The address of the Auth Service (e.g., "http://[::1]:50051")
    ///
    /// # Returns
    /// * `Result<Self, tonic::transport::Error>` - The client or a transport error
    pub async fn new(addr: String) -> Result<Self, tonic::transport::Error> {
        let client = AuthServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
    
    /// Validate a session
    ///
    /// # Arguments
    /// * `session_id` - The ID of the session to validate
    ///
    /// # Returns
    /// * `Result<User, AuthError>` - The user associated with the session or an error
    pub async fn validate_session(&self, session_id: Uuid) -> Result<User, AuthError> {
        let request = cpc_auth::auth_service::SessionRequest {
            session_id: session_id.to_string(),
        };
        
        let response = self.client.clone().validate_session(request).await
            .map_err(|e| AuthError::DatabaseError(format!("gRPC error: {}", e)))?
            .into_inner();
            
        if response.valid {
            let user_id = Uuid::parse_str(&response.user_id)
                .map_err(|_| AuthError::InvalidCredentials)?;
            Ok(User::new_with_id(user_id, "unknown@example.com".to_string(), "unknown".to_string()))
        } else {
            Err(AuthError::SessionExpired)
        }
    }
    
    /// Create a new session
    ///
    /// # Arguments
    /// * `user_id` - The ID of the user for whom to create a session
    /// * `device_info` - Information about the device creating the session
    ///
    /// # Returns
    /// * `Result<Session, AuthError>` - The created session or an error
    pub async fn create_session(&self, user_id: Uuid, device_info: String) -> Result<Session, AuthError> {
        let request = cpc_auth::auth_service::CreateSessionRequest {
            user_id: user_id.to_string(),
            device_info,
        };
        
        let response = self.client.clone().create_session(request).await
            .map_err(|e| AuthError::DatabaseError(format!("gRPC error: {}", e)))?
            .into_inner();
            
        if response.valid {
            let session_id = Uuid::parse_str(&response.session_id)
                .map_err(|_| AuthError::InvalidCredentials)?;
            let user_id = Uuid::parse_str(&response.user_id)
                .map_err(|_| AuthError::InvalidCredentials)?;
            Ok(Session::new_with_id(session_id, user_id, "unknown".to_string()))
        } else {
            Err(AuthError::DatabaseError(response.error_message))
        }
    }
    
    /// Invalidate a session
    ///
    /// # Arguments
    /// * `session_id` - The ID of the session to invalidate
    ///
    /// # Returns
    /// * `Result<(), AuthError>` - Success or an error
    pub async fn invalidate_session(&self, session_id: Uuid) -> Result<(), AuthError> {
        let request = cpc_auth::auth_service::InvalidateSessionRequest {
            session_id: session_id.to_string(),
        };
        
        let response = self.client.clone().invalidate_session(request).await
            .map_err(|e| AuthError::DatabaseError(format!("gRPC error: {}", e)))?
            .into_inner();
            
        if response.success {
            Ok(())
        } else {
            Err(AuthError::DatabaseError(response.error_message))
        }
    }
}