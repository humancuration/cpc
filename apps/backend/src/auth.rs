use axum::{
    async_trait,
    extract::{FromRequestParts, Request, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
    RequestPartsExt, TypedHeader,
};
use axum_extra::headers::authorization::{Bearer, Authorization};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::env;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use tower_governor::{GovernorConfigBuilder, governor::middleware::NoOpMiddleware};
use std::time::Duration;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub exp: usize,
    pub iat: usize,
    pub roles: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct AuthState {
    pub decoding_key: DecodingKey,
    pub jwt_secret: String,
}

impl AuthState {
    pub fn new() -> Self {
        let jwt_secret = env::var("CPC_JWT_SECRET")
            .expect("CPC_JWT_SECRET must be set");
        
        let decoding_key = DecodingKey::from_secret(jwt_secret.as_bytes());
        
        Self {
            decoding_key,
            jwt_secret,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: Uuid,
    pub roles: Vec<String>,
    pub exp: DateTime<Utc>,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<AuthenticatedUser>()
            .cloned()
            .ok_or(StatusCode::UNAUTHORIZED)
    }
}

pub async fn auth_middleware(
    State(auth_state): State<Arc<AuthState>>,
    mut req: Request,
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
    let token_data = decode::<Claims>(&token, &auth_state.decoding_key, &Validation::new(Algorithm::HS256))
        .map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    // Check if token is expired
    let now = chrono::Utc::now().timestamp() as usize;
    if token_data.claims.exp < now {
        return Err(StatusCode::UNAUTHORIZED);
    }
    
    // Create authenticated user
    let authenticated_user = AuthenticatedUser {
        user_id: token_data.claims.user_id,
        roles: token_data.claims.roles,
        exp: DateTime::from_timestamp(token_data.claims.exp as i64, 0)
            .unwrap_or_else(|| Utc::now()),
    };
    
    // Insert authenticated user into request extensions
    parts.extensions.insert(authenticated_user);
    
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}

pub async fn optional_auth_middleware(
    State(auth_state): State<Arc<AuthState>>,
    mut req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let (mut parts, body) = req.into_parts();
    
    // Try to extract token from Authorization header
    if let Ok(Some(auth_header)) = parts.extract::<Option<TypedHeader<Authorization<Bearer>>>>().await {
        if let Ok(token_data) = decode::<Claims>(auth_header.token(), &auth_state.decoding_key, &Validation::new(Algorithm::HS256)) {
            let now = chrono::Utc::now().timestamp() as usize;
            if token_data.claims.exp >= now {
                let authenticated_user = AuthenticatedUser {
                    user_id: token_data.claims.user_id,
                    roles: token_data.claims.roles,
                    exp: DateTime::from_timestamp(token_data.claims.exp as i64, 0)
                        .unwrap_or_else(|| Utc::now()),
                };
                parts.extensions.insert(authenticated_user);
            }
        }
    }
    
    let req = Request::from_parts(parts, body);
    Ok(next.run(req).await)
}

pub fn require_role(required_role: &'static str) -> impl Fn(AuthenticatedUser) -> Result<AuthenticatedUser, StatusCode> + Clone {
    move |user: AuthenticatedUser| {
        if user.roles.contains(&required_role.to_string()) {
            Ok(user)
        } else {
            Err(StatusCode::FORBIDDEN)
        }
    }
}

pub fn create_rate_limiter() -> tower_governor::GovernorLayer<
    tower_governor::governor::DefaultKeyExtractor,
    NoOpMiddleware<tower_governor::governor::clock::QuantaInstant>
> {
    let governor_conf = Box::new(
        GovernorConfigBuilder::default()
            .per_second(10) // 10 requests per second
            .burst_size(20) // Allow bursts up to 20 requests
            .finish()
            .unwrap(),
    );
    
    tower_governor::GovernorLayer::new(governor_conf)
}

pub fn create_security_middleware() -> ServiceBuilder<
    tower::layer::util::Stack<
        tower::layer::util::Stack<
            RequestBodyLimitLayer,
            tower_http::timeout::TimeoutLayer,
        >,
        tower_http::cors::CorsLayer,
    >
> {
    ServiceBuilder::new()
        .layer(tower_http::cors::CorsLayer::new()
            .allow_origin(tower_http::cors::Any)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::PUT,
                axum::http::Method::DELETE,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers(tower_http::cors::Any))
        .layer(tower_http::timeout::TimeoutLayer::new(Duration::from_secs(30)))
        .layer(RequestBodyLimitLayer::new(10 * 1024 * 1024)) // 10MB limit
}