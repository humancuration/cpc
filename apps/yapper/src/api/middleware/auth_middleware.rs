use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::domain::auth_service::AuthService;
use uuid::Uuid;

#[derive(Clone)]
pub struct AuthMiddleware {
    auth_service: Arc<dyn AuthService>,
}

impl AuthMiddleware {
    pub fn new(auth_service: Arc<dyn AuthService>) -> Self {
        Self { auth_service }
    }

    pub async fn auth_middleware<B>(
        &self,
        mut req: Request<B>,
        next: Next<B>,
    ) -> Result<Response, StatusCode> {
        // Get authorization header
        let auth_header = req.headers()
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|header| header.to_str().ok());

        // For now, we'll just check for a session ID in a cookie or header
        // In a real implementation, we would validate JWT tokens or session cookies
        let session_id = match auth_header {
            Some(header) if header.starts_with("Bearer ") => {
                let token = &header[7..];
                // Parse as UUID for session ID (simplified)
                Uuid::parse_str(token).ok()
            }
            _ => {
                // Check for session cookie
                // In a real implementation, we would parse cookies
                None
            }
        };

        // If we have a session ID, validate it
        if let Some(session_id) = session_id {
            match self.auth_service.validate_session(session_id).await {
                Ok(user) => {
                    // Add user to request extensions
                    req.extensions_mut().insert(user);
                    Ok(next.run(req).await)
                }
                Err(_) => Err(StatusCode::UNAUTHORIZED),
            }
        } else {
            // For demo purposes, we'll allow unauthenticated requests to pass through
            // In a real implementation, we would return Err(StatusCode::UNAUTHORIZED)
            Ok(next.run(req).await)
        }
    }
}