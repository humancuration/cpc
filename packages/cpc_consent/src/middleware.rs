//! # Consent Middleware
//!
//! Middleware for checking user consent before allowing access to protected resources.

use axum::{
    http::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};
use crate::{ConsentService, Domain, ConsentLevel};

#[derive(Debug, thiserror::Error)]
pub enum ConsentError {
    #[error("Consent required for {0}")]
    ConsentRequired(Domain),
    #[error("Insufficient consent level")]
    InsufficientConsent,
}

pub async fn consent_middleware<B>(
    req: Request<B>,
    next: Next<B>,
    consent_service: &ConsentService,
    domain: Domain,
    required_level: ConsentLevel,
) -> Result<Response, StatusCode> {
    // In a real implementation, you would extract the user ID from the request
    // For now, we'll use a placeholder
    let user_id = "placeholder_user_id"; // This would come from the session or token
    
    if consent_service.allows(user_id, &domain, required_level) {
        Ok(next.run(req).await)
    } else {
        Err(StatusCode::FORBIDDEN)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{http::Request, middleware::Next};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_consent_middleware() {
        // This is a placeholder test
        // In a real implementation, you would test the middleware with actual requests
    }
}