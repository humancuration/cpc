use cpc_auth::auth_service::{AuthService as CpcAuthService, AuthServiceImpl};
use cpc_auth::models::{Credentials, User, Session};
use cpc_auth::error::AuthError;
use crate::domain::user_repository::UserRepository;
use crate::domain::session_repository::SessionRepository;
use crate::domain::password_reset_repository::PasswordResetRepository;
use crate::domain::events::{EventPublisher, YapperEvent};
use std::sync::Arc;
use uuid::Uuid;

pub struct YapperAuthService {
    auth_impl: AuthServiceImpl,
    user_repo: Arc<dyn UserRepository>,
    session_repo: Arc<dyn SessionRepository>,
    password_reset_repo: Arc<dyn PasswordResetRepository>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl YapperAuthService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        session_repo: Arc<dyn SessionRepository>,
        password_reset_repo: Arc<dyn PasswordResetRepository>,
        event_publisher: Arc<dyn EventPublisher>,
    ) -> Self {
        Self {
            auth_impl: AuthServiceImpl::new(),
            user_repo,
            session_repo,
            password_reset_repo,
            event_publisher,
        }
    }
}

#[async_trait::async_trait]
impl CpcAuthService for YapperAuthService {
    async fn register(&self, credentials: Credentials) -> Result<User, AuthError> {
        // Check if user already exists
        if self.user_repo.find_by_email(&credentials.email).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .is_some() {
            return Err(AuthError::DatabaseError("User already exists".to_string()));
        }

        // Hash password using the cpc_auth implementation
        let password_hash = self.auth_impl.hash_password(&credentials.password)?;

        // Create user
        let user = User::new(credentials.email.clone(), password_hash);

        // Save user
        self.user_repo.create(&user).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Publish event
        self.event_publisher.publish(YapperEvent::UserRegistered {
            user_id: user.id,
            email: user.email.clone(),
        });

        Ok(user)
    }

    async fn login(&self, credentials: Credentials) -> Result<Session, AuthError> {
        // Find user by email
        let user = self.user_repo.find_by_email(&credentials.email).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::InvalidCredentials)?;

        // Verify password using the cpc_auth implementation
        if !self.auth_impl.verify_password(&credentials.password, &user.password_hash)? {
            return Err(AuthError::InvalidCredentials);
        }

        // Check if account is verified
        if !user.is_verified {
            return Err(AuthError::AccountNotVerified);
        }

        // Create session
        let session = Session::new(user.id, "unknown".to_string()); // In a real implementation, we'd get device info from request

        // Save session
        self.session_repo.create(&session).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Publish event
        self.event_publisher.publish(YapperEvent::UserLoggedIn {
            user_id: user.id,
            device_info: session.device_info.clone(),
        });

        Ok(session)
    }

    async fn logout(&self, session_id: Uuid) -> Result<(), AuthError> {
        self.session_repo.delete(session_id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;
        Ok(())
    }

    async fn validate_session(&self, session_id: Uuid) -> Result<User, AuthError> {
        let session = self.session_repo.find_by_id(session_id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::SessionExpired)?;

        if session.is_expired() {
            // Clean up expired session
            let _ = self.session_repo.delete(session_id).await;
            return Err(AuthError::SessionExpired);
        }

        let user = self.user_repo.find_by_id(session.user_id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        Ok(user)
    }

    async fn refresh_token(&self, _refresh_token: String) -> Result<Session, AuthError> {
        // In a real implementation, we would validate the refresh token and create a new session
        // For now, we'll just return an error
        Err(AuthError::TokenInvalid)
    }

    async fn initiate_password_reset(&self, email: String) -> Result<(), AuthError> {
        // Find user by email
        let user = self.user_repo.find_by_email(&email).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        // Generate reset token (in a real implementation, this should be cryptographically secure)
        let token = uuid::Uuid::new_v4().to_string();

        // Save token
        self.password_reset_repo.create_token(token.clone(), user.id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Publish event
        self.event_publisher.publish(YapperEvent::PasswordResetRequested {
            user_id: user.id,
        });

        // In a real implementation, we would send an email with the token
        // For now, we'll just log it
        println!("Password reset token for user {}: {}", user.id, token);

        Ok(())
    }

    async fn confirm_password_reset(&self, token: String, new_password: String) -> Result<(), AuthError> {
        // Find user ID by token
        let user_id = self.password_reset_repo.find_user_id_by_token(&token).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::TokenInvalid)?;

        // Hash new password using the cpc_auth implementation
        let password_hash = self.auth_impl.hash_password(&new_password)?;

        // Find user
        let mut user = self.user_repo.find_by_id(user_id).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?
            .ok_or(AuthError::UserNotFound)?;

        // Update password
        user.password_hash = password_hash;

        // Save user
        self.user_repo.update(&user).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        // Delete token
        self.password_reset_repo.delete_token(&token).await
            .map_err(|e| AuthError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}