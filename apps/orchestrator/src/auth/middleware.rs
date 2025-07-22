use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use jsonwebtoken::{decode, Algorithm, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::auth::service::AuthService;
use crate::auth::errors::AuthError;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: uuid::Uuid,
    pub exp: usize,
}

pub async fn jwt_auth<B>(
    mut req: Request<B>,
    next: Next<B>,
    secret: &str,
) -> Result<Response, StatusCode> {
    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let claims = decode::<Claims>(token, &decoding_key, &validation)
        .map(|data| data.claims)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Get AuthService from request extensions
    let auth_service = req.extensions().get::<AuthService>()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Fetch user from database
    let user = auth_service.get_user_by_id(claims.sub)
        .await
        .map_err(|e| match e {
            AuthError::UserNotFound => StatusCode::UNAUTHORIZED,
            _ => StatusCode::INTERNAL_SERVER_ERROR
        })?;

    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}