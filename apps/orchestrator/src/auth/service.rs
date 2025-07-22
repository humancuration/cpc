use crate::auth::errors::AuthError;
use crate::models::{NewUser, User};
use cpc_core::models::token::NewToken;
use cpc_core::utils::password::{hash_password, verify_password};
use sqlx::{PgPool, postgres::PgQueryAs};
use jsonwebtoken::{encode, EncodingKey, Header};
use uuid::Uuid;
use chrono::{Utc, Duration};

pub struct AuthService {
    db: PgPool,
    secret: String,
}

impl AuthService {
    pub fn new(db: PgPool, secret: String) -> Self {
        Self { db, secret }
    }

    pub async fn register_user(&self, new_user: NewUser) -> Result<Uuid, AuthError> {
        // Validate user input
        new_user.validate().map_err(|_| AuthError::ValidationFailed)?;
        
        // Hash password
        let hashed_password = hash_password(&new_user.password)
            .map_err(|_| AuthError::PasswordHashingFailed)?;
        
        // Insert user into database
        let user_id = sqlx::query_scalar!(
            r#"
            INSERT INTO users (username, email, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id
            "#,
            new_user.username,
            new_user.email,
            hashed_password
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| AuthError::UserCreationFailed)?;

        Ok(user_id)
    }

    pub async fn create_token(&self, new_token: NewToken) -> Result<(), AuthError> {
        sqlx::query!(
            r#"
            INSERT INTO tokens (user_id, refresh_token, device_info, expires_at)
            VALUES ($1, $2, $3, $4)
            "#,
            new_token.user_id,
            new_token.refresh_token,
            new_token.device_info,
            new_token.expires_at
        )
        .execute(&self.db)
        .await
        .map_err(|_| AuthError::TokenCreationFailed)?;

        Ok(())
    }

    pub async fn get_user_by_id(&self, user_id: Uuid) -> Result<User, AuthError> {
        let user = sqlx::query_as!(
            User,
            r#"
            SELECT id, username, email, password_hash, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.db)
        .await
        .map_err(|_| AuthError::UserNotFound)?;

        Ok(user)
    }

    // Existing functions remain unchanged but will be updated later
    pub async fn login(&self, email: &str, password: &str) -> Result<String, String> {
        unimplemented!()
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<String, String> {
        unimplemented!()
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<(), String> {
        unimplemented!()
    }

    fn generate_jwt(&self, user_id: Uuid) -> String {
        String::new()
    }
}