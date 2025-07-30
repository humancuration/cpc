use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    extract::Extension,
    http::StatusCode,
};
use cpc_rbac::RbacEngine;
use std::sync::Arc;

pub async fn authorization_middleware<B>(
    req: Request<B>,
    next: Next<B>,
    Extension(rbac_engine): Extension<Arc<RbacEngine>>,
    required_role: String,
    required_permission: String,
) -> Result<Response, StatusCode> {
    // In a real implementation, you would extract user ID from session/cookies
    // and get the user's roles from the database or session store
    // For now, we'll use a placeholder role
    let user_role = "user"; // This would come from the user's session/data
    
    if rbac_engine.check_permission(user_role, &required_permission) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}