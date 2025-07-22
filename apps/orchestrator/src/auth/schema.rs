use async_graphql::{InputObject, Object, Context, Result};
use uuid::Uuid;
use crate::models::{NewUser, User};
use super::service::AuthService;

#[derive(InputObject)]
pub struct RegisterInput {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(InputObject)]
pub struct LoginInput {
    pub email: String,
    pub password: String,
    pub device_fingerprint: String,
}

#[derive(SimpleObject)]
pub struct AuthPayload {
    pub access_token: String,
    pub refresh_token: String,
    pub user: User,
}

pub struct AuthMutation;

#[Object]
impl AuthMutation {
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let auth_service = ctx.data::<AuthService>()?;
        
        // Create new user
        let new_user = NewUser {
            username: input.username,
            email: input.email,
            password: input.password,
        };
        
        let user_id = auth_service.register_user(new_user)
            .await
            .map_err(|e| e.into())?;

        // Generate tokens using JwtService
        let access_token = auth_service.jwt_service.generate_token(user_id);
        let refresh_token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + Duration::days(30);
        
        // Store refresh token
        auth_service.create_token(NewToken {
            user_id,
            refresh_token: refresh_token.clone(),
            device_info: Some("web".to_string()), // TODO: get actual device info
            expires_at,
        }).await.map_err(|e| e.into())?;

        // Get full user data
        let user = auth_service.get_user_by_id(user_id)
            .await
            .map_err(|e| e.into())?;

        Ok(AuthPayload {
            access_token,
            refresh_token,
            user,
        })
    }

    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let auth_service = ctx.data::<AuthService>()?;
        
        let (access_token, refresh_token) = auth_service.login(
            &input.email,
            &input.password,
            &input.device_fingerprint
        )
        .await
        .map_err(|e| e.into())?;
        
        // Get user by email
        let user = sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            input.email
        )
        .fetch_one(&auth_service.db)
        .await
        .map_err(|_| "User not found".into())?;

        Ok(AuthPayload {
            access_token,
            refresh_token,
            user,
        })
    }

    async fn refresh_token(&self, ctx: &Context<'_>, input: RefreshTokenInput) -> Result<AuthPayload> {
        let auth_service = ctx.data::<AuthService>()?;
        
        let (access_token, refresh_token) = auth_service.refresh_token(
            &input.refresh_token,
            &input.device_fingerprint
        )
        .await
        .map_err(|e| e.into())?;

        // Get user from token claims
        let claims = auth_service.jwt_service.validate_access_token(&access_token)
            .map_err(|_| "Invalid token".into())?;
            
        let user = auth_service.get_user_by_id(claims.sub)
            .await
            .map_err(|e| e.into())?;

        Ok(AuthPayload {
            access_token,
            refresh_token,
            user,
        })
    }

#[derive(InputObject)]
pub struct RefreshTokenInput {
    pub refresh_token: String,
    pub device_fingerprint: String,
}

    async fn logout(&self, ctx: &Context<'_>, refresh_token: String) -> Result<bool> {
        let auth_service = ctx.data::<AuthService>()?;
        auth_service.logout(&refresh_token)
            .await
            .map_err(|e| e.into())?;
        Ok(true)
    }
}