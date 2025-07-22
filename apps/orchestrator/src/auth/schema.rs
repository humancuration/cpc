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

        // Generate tokens
        let access_token = auth_service.generate_jwt(user_id);
        let refresh_token = Uuid::new_v4().to_string();
        
        // Store refresh token
        auth_service.create_token(NewToken {
            user_id,
            refresh_token: refresh_token.clone(),
            device_info: None,
            expires_at: Utc::now() + Duration::days(30),
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
        // TODO: Verify credentials and generate tokens
        unimplemented!()
    }

    async fn refresh_token(&self, ctx: &Context<'_>, refresh_token: String) -> Result<String> {
        let auth_service = ctx.data::<AuthService>()?;
        // TODO: Validate refresh token and generate new access token
        unimplemented!()
    }

    async fn logout(&self, ctx: &Context<'_>, refresh_token: String) -> Result<bool> {
        let auth_service = ctx.data::<AuthService>()?;
        // TODO: Invalidate refresh token
        unimplemented!()
    }
}