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
use ed25519_dalek::{Keypair, PublicKey};
use std::sync::Arc;
use uuid::Uuid;
use base64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
    pub fn new(decoding_key: DecodingKey) -> Self {
        let signing_key = Self::load_signing_key();
        let verification_key = PublicKey::from(&signing_key);
        Self { decoding_key, signing_key, verification_key }
    }

    fn load_signing_key() -> Keypair {
        let key_bytes = base64::decode(env::var("IMPACT_SIGNING_KEY")
            .expect("IMPACT_SIGNING_KEY must be set"))
            .expect("Failed to decode IMPACT_SIGNING_KEY");
        Keypair::from_bytes(&key_bytes).expect("Invalid signing key")
    }
}

pub async fn auth_middleware(
    State(state): State<Arc<crate::AppState>>,
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
    let token_data = decode::<Claims>(&token, &state.decoding_key, &Validation::new(Algorithm::HS256))
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Insert user_id into request extensions
    parts.extensions.insert(token_data.claims.user_id);
    
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}