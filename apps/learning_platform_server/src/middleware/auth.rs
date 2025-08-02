use std::sync::Arc;
use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::{HeaderMap, StatusCode},
};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,  // user_id
    pub exp: usize,
}

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        Self
    }

    pub async fn auth_middleware(
        headers: HeaderMap,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        // Get the Authorization header
        let auth_header = headers
            .get("authorization")
            .and_then(|header| header.to_str().ok())
            .ok_or(StatusCode::UNAUTHORIZED)?;

        // Check if it's a Bearer token
        if !auth_header.starts_with("Bearer ") {
            return Err(StatusCode::UNAUTHORIZED);
        }

        let token = &auth_header[7..]; // Skip "Bearer "

        // Decode the token
        let decoding_key = DecodingKey::from_secret("secret".as_bytes()); // In production, use a proper secret
        let validation = Validation::new(Algorithm::HS256);
        
        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                // Add user_id to request extensions
                request.extensions_mut().insert(token_data.claims.sub);
                Ok(next.run(request).await)
            }
            Err(_) => Err(StatusCode::UNAUTHORIZED),
        }
    }

    pub fn get_user_id_from_request(request: &Request) -> Option<Uuid> {
        request.extensions().get::<Uuid>().copied()
    }
}