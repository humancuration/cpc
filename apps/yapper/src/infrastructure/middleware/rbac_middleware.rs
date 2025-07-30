//! # RBAC Middleware
//!
//! Middleware for checking user permissions using the CPC RBAC system.

use cpc_rbac::RbacEngine;
use axum::{
    extract::State,
    http::Request,
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use crate::domain::auth_error::AuthError;

/// RBAC middleware for checking user permissions
///
/// This middleware checks if a user has the required permissions to access a resource
/// based on their role and the RBAC configuration.
///
/// # Arguments
/// * `rbac_engine` - The RBAC engine containing role and permission definitions
/// * `request` - The incoming HTTP request
/// * `next` - The next middleware in the chain
///
/// # Returns
/// * `Result<Response, AuthError>` - The response or an authentication error
pub async fn rbac_middleware<B>(
    State(rbac_engine): State<Arc<RbacEngine>>,
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, AuthError> {
    // Extract user role from request (this would typically come from the session or token)
    // For now, we'll use a placeholder
    let user_role = "user"; // This would come from the session or token
    
    // Extract required permission from request path or other metadata
    // For now, we'll use a placeholder
    let required_permission = "read"; // This would be determined based on the route
    
    // Check if the user has the required permission
    if rbac_engine.check_permission(user_role, required_permission) {
        Ok(next.run(request).await)
    } else {
        Err(AuthError::PermissionDenied)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::Request, middleware::Next};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_rbac_middleware() {
        // This is a placeholder test
        // In a real implementation, you would test the middleware with actual requests
    }
}