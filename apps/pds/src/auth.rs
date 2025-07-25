use axum::{
    async_trait,
    extract::{FromRequestParts, Request},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt, TypedHeader,
};
use axum_extra::headers::authorization::{Bearer, Authorization};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

static SECRET_KEY: Lazy<String> = Lazy::new(|| "my-secret-key".to_string());
static DECODING_KEY: Lazy<DecodingKey> = Lazy::new(|| {
    DecodingKey::from_secret(SECRET_KEY.as_bytes())
});

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
}

pub async fn auth_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();
    
    // Extract token from Authorization header
    let auth_header: Option<TypedHeader<Authorization<Bearer>>> = parts
        .extract()
        .await
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    let token = auth_header
        .ok_or(StatusCode::UNAUTHORIZED)?
        .token()
        .to_owned();

    // Decode and validate token
    let token_data = decode::<Claims>(&token, &DECODING_KEY, &Validation::new(Algorithm::HS256))
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Insert user_id into request extensions
    parts.extensions.insert(token_data.claims.user_id);
    
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}