use async_graphql::{
    Context, Object, Result, ID,
    futures_util::{stream::Stream, TryStreamExt},
};
use crate::{
    core::{
        services::{auth::AuthService, user::UserService},
        repositories::user_repository::UserRepository,
    },
    utils::graphql::{AppContext, extract_auth_token},
};
use uuid::Uuid;
use serde_json::Value as JsonValue;

/// GraphQL representation of a User
#[derive(Clone)]
pub struct UserType {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

#[Object]
impl UserType {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn email(&self) -> &str {
        &self.email
    }

    async fn display_name(&self) -> Option<&str> {
        self.display_name.as_deref()
    }

    async fn bio(&self) -> Option<&str> {
        self.bio.as_deref()
    }

    async fn avatar_url(&self) -> Option<&str> {
        self.avatar_url.as_deref()
    }

    async fn created_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.created_at
    }

    async fn updated_at(&self) -> chrono::DateTime<chrono::Utc> {
        self.updated_at
    }

    async fn friends(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = UserType>> {
        // Placeholder - will implement with data loaders
        Ok(futures_util::stream::empty())
    }

    async fn followers(&self, ctx: &Context<'_>) -> Result<impl Stream<Item = UserType>> {
        // Placeholder - will implement with data loaders
        Ok(futures_util::stream::empty())
    }

    /// Get dashboard preferences for the user
    async fn dashboard_preferences(&self, ctx: &Context<'_>) -> Result<Option<JsonValue>> {
        let app_ctx = AppContext::from(ctx);
        
        let preferences = app_ctx
            .user_service
            .get_user_preferences(self.id)
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to get preferences: {}", e)))?;
        
        Ok(preferences.map(|p| p.dashboard_preferences))
    }
}

/// Authentication payload returned after login/register
#[derive(Clone)]
pub struct AuthPayload {
    token: String,
    user: UserType,
}

#[Object]
impl AuthPayload {
    async fn token(&self) -> &str {
        &self.token
    }

    async fn user(&self) -> &UserType {
        &self.user
    }
}

/// Input for registration
#[derive(async_graphql::InputObject)]
struct RegisterInput {
    username: String,
    email: String,
    password: String,
    display_name: Option<String>,
}

/// Input for login
#[derive(async_graphql::InputObject)]
struct LoginInput {
    email: String,
    password: String,
}

/// Input for dashboard preferences
#[derive(async_graphql::InputObject)]
struct DashboardPreferencesInput {
    default_date_range: Option<String>,
    favorite_metrics: Option<Vec<String>>,
    chart_settings: Option<JsonValue>,
}

/// Identity-related GraphQL queries
pub struct IdentityQuery;

#[Object]
impl IdentityQuery {
    /// Get the current authenticated user
    async fn me(&self, ctx: &Context<'_>) -> Result<Option<UserType>> {
        let auth_data = extract_auth_token(ctx)?;
        let app_ctx = AppContext::from(ctx);
        
        let user = app_ctx.user_service.get_user_by_id(auth_data.user_id).await?;
        Ok(Some(user.into()))
    }

    /// Get a user by ID
    async fn user(&self, ctx: &Context<'_>, id: ID) -> Result<Option<UserType>> {
        let app_ctx = AppContext::from(ctx);
        let user_id = Uuid::parse_str(&id)?;
        
        app_ctx
            .user_service
            .get_user_by_id(user_id)
            .await
            .map(|u| Some(u.into()))
            .map_err(|e| e.into())
    }

    /// Get multiple users by their IDs
    async fn users(&self, ctx: &Context<'_>, ids: Vec<Uuid>) -> Result<Vec<UserType>> {
        let app_ctx = AppContext::from(ctx);
        let users = app_ctx.user_service.get_users_by_ids(&ids).await?;
        let user_types = users.into_iter().map(UserType::from).collect();
        Ok(user_types)
    }
}

/// Identity-related GraphQL mutations
pub struct IdentityMutation;

#[Object]
impl IdentityMutation {
    /// Register a new user
    async fn register(&self, ctx: &Context<'_>, input: RegisterInput) -> Result<AuthPayload> {
        let app_ctx = AppContext::from(ctx);
        
        let new_user = crate::core::models::user::NewUser {
            username: input.username,
            email: input.email,
            password: input.password,
            display_name: input.display_name,
        };
        
        let (user, token) = app_ctx.auth_service.register(new_user).await?;
        Ok(AuthPayload {
            token,
            user: user.into(),
        })
    }

    /// Login with email and password
    async fn login(&self, ctx: &Context<'_>, input: LoginInput) -> Result<AuthPayload> {
        let app_ctx = AppContext::from(ctx);
        
        let (user, token) = app_ctx
            .auth_service
            .login(&input.email, &input.password)
            .await?;
        
        Ok(AuthPayload {
            token,
            user: user.into(),
        })
    }

    /// Update user profile
    async fn update_profile(
        &self,
        ctx: &Context<'_>,
        display_name: Option<String>,
        bio: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<UserType> {
        let auth_data = extract_auth_token(ctx)?;
        let app_ctx = AppContext::from(ctx);
        
        let user = app_ctx
            .user_service
            .update_profile(auth_data.user_id, display_name, bio, avatar_url)
            .await?;
        
        Ok(user.into())
    }

    /// Update dashboard preferences
    async fn update_dashboard_preferences(
        &self,
        ctx: &Context<'_>,
        input: DashboardPreferencesInput,
    ) -> Result<UserPreferences> {
        let auth_data = extract_auth_token(ctx)?;
        let app_ctx = AppContext::from(ctx);
        
        let preferences = app_ctx
            .user_service
            .update_dashboard_preferences(
                auth_data.user_id,
                input.default_date_range,
                input.favorite_metrics,
                input.chart_settings,
            )
            .await
            .map_err(|e| async_graphql::Error::new(format!("Failed to update preferences: {}", e)))?;
        
        Ok(preferences)
    }
}

/// User preferences structure
#[derive(SimpleObject)]
struct UserPreferences {
    user_id: Uuid,
    dashboard_preferences: JsonValue,
}

/// Convert core User model to GraphQL UserType
impl From<crate::core::models::user::User> for UserType {
    fn from(user: crate::core::models::user::User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            display_name: user.display_name,
            bio: user.bio,
            avatar_url: user.avatar_url,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}