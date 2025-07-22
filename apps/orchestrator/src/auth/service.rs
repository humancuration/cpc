use crate::auth::errors::AuthError;
use crate::auth::jwt::JwtService;
use crate::models::{NewUser, User};
use cpc_core::models::token::NewToken;
use cpc_core::utils::password::{hash_password, verify_password};
use sqlx::{PgPool, postgres::PgQueryAs};
use uuid::Uuid;
use chrono::{Utc, Duration};

use valkey::Client;

pub struct AuthService {
    db: PgPool,
    valkey: Client,
    jwt_service: JwtService,
}

impl AuthService {
    pub fn new(db: PgPool, valkey: Client, access_secret: String, refresh_secret: String) -> Self {
        let jwt_service = JwtService::new(access_secret, refresh_secret);
        Self { db, valkey, jwt_service }
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
    pub async fn login(&self, email: &str, password: &str, device_fingerprint: &str) -> Result<(String, String), AuthError> {
        // Fetch user and verify password (existing logic)
        let user = self.verify_user_credentials(email, password).await?;

        // Generate tokens with proper expiration
        let access_token = self.jwt_service.generate_access_token(user.id);
        let refresh_token = self.jwt_service.generate_refresh_token(user.id);
        
        // Store refresh token in Valkey with device fingerprint
        let valkey_key = format!("refresh_token:{}", refresh_token);
        let expires_sec = 7 * 24 * 60 * 60; // 7 days in seconds
        
        let token_data = serde_json::json!({
            "user_id": user.id,
            "device_fingerprint": device_fingerprint,
            "created_at": Utc::now().to_rfc3339(),
            "expires_at": (Utc::now() + Duration::days(7)).to_rfc3339()
        });
        
        self.valkey.set_ex(
            &valkey_key,
            &serde_json::to_string(&token_data).map_err(|_| AuthError::TokenStorageError)?,
            expires_sec
        ).await.map_err(|_| AuthError::TokenStorageError)?;

        Ok((access_token, refresh_token))
    }

    pub async fn refresh_token(&self, refresh_token: &str, device_fingerprint: &str) -> Result<(String, String), AuthError> {
        // Validate refresh token
        let claims = self.jwt_service.validate_refresh_token(refresh_token)
            .map_err(|_| AuthError::InvalidToken)?;

        // Rate limiting: max 5 refreshes per hour per user
        let rate_limit_key = format!("refresh_rate_limit:{}", claims.user_id);
        let current_count: i64 = self.valkey.get(&rate_limit_key).await.unwrap_or(0);
        if current_count >= 5 {
            return Err(AuthError::RefreshRateLimitExceeded);
        }

        // Increment count and set expiration if new
        self.valkey.incr(&rate_limit_key, 1).await.map_err(|_| AuthError::InternalError)?;
        if current_count == 0 {
            self.valkey.expire(&rate_limit_key, 3600).await.map_err(|_| AuthError::InternalError)?;
        }

        // Check token existence in Valkey and validate device fingerprint
        let valkey_key = format!("refresh_token:{}", refresh_token);
        let token_data_str: String = self.valkey.get(&valkey_key)
            .await
            .map_err(|_| AuthError::TokenNotFound)?
            .ok_or(AuthError::TokenNotFound)?;
            
        let token_data: serde_json::Value = serde_json::from_str(&token_data_str)
            .map_err(|_| AuthError::TokenValidationError)?;
            
        let user_id: Uuid = serde_json::from_value(token_data["user_id"].clone())
            .map_err(|_| AuthError::TokenValidationError)?;
        let stored_fingerprint = token_data["device_fingerprint"].as_str()
            .ok_or(AuthError::TokenValidationError)?;
            
        if stored_fingerprint != device_fingerprint {
            return Err(AuthError::DeviceMismatch);
            pub async fn revoke_all_tokens_for_user(&self, user_id: Uuid) -> Result<(), AuthError> {
                let pattern = "refresh_token:*";
                let mut cursor = 0;
                let mut keys_to_delete = Vec::new();
        
                loop {
                    let (next_cursor, keys): (u64, Vec<String>) = self.valkey
                        .scan(cursor)
                        .pattern(pattern)
                        .get()
                        .await
                        .map_err(|_| AuthError::TokenRevocationError)?;
        
                    for key in keys {
                        let token_data_str: String = match self.valkey.get(&key).await {
                            Ok(Some(s)) => s,
                            Ok(None) => continue,
                            Err(_) => continue,
                        };
                        let token_data: serde_json::Value = match serde_json::from_str(&token_data_str) {
                            Ok(data) => data,
                            Err(_) => continue,
                        };
                        let token_user_id: Uuid = match token_data["user_id"].as_str() {
                            Some(id) => match Uuid::parse_str(id) {
                                Ok(uuid) => uuid,
                                Err(_) => continue,
                            },
                            None => continue,
                        };
                        if token_user_id == user_id {
                            keys_to_delete.push(key);
                        }
                    }
        
                    cursor = next_cursor;
                    if cursor == 0 {
                        break;
                    }
                }
        
                for key in keys_to_delete {
                    self.valkey.del(&key).await.map_err(|_| AuthError::TokenRevocationError)?;
                }
        
                Ok(())
            }
        
            pub async fn change_password(&self, user_id: Uuid, new_password: &str) -> Result<(), AuthError> {
                // Hash the new password
                let hashed_password = hash_password(new_password)
                    .map_err(|_| AuthError::PasswordHashingFailed)?;
        
                // Update the user's password in the database
                sqlx::query!(
                    r#"
                    UPDATE users
                    SET password_hash = $1
                    WHERE id = $2
                    "#,
                    hashed_password,
                    user_id
                )
                .execute(&self.db)
                .await
                .map_err(|_| AuthError::PasswordChangeFailed)?;
        
                // Revoke all tokens for this user
                self.revoke_all_tokens_for_user(user_id).await?;
        
                Ok(())
            }
        }

        // Validate user exists
        let user = self.get_user_by_id(user_id).await?;

        // Generate new tokens
        let new_access_token = self.jwt_service.generate_access_token(user.id);
        let new_refresh_token = self.jwt_service.generate_refresh_token(user.id);

        // Store new refresh token and delete old one
        let new_valkey_key = format!("refresh_token:{}", new_refresh_token);
        self.valkey.set_ex(
            &new_valkey_key,
            &serde_json::to_string(&token_data).map_err(|_| AuthError::TokenStorageError)?,
            7 * 24 * 60 * 60 // 7 days
        ).await.map_err(|_| AuthError::TokenStorageError)?;

        self.valkey.del(&valkey_key)
            .await
            .map_err(|_| AuthError::TokenRevocationError)?;

        Ok((new_access_token, new_refresh_token))
    }

    pub async fn logout(&self, refresh_token: &str) -> Result<(), AuthError> {
        let valkey_key = format!("refresh_token:{}", refresh_token);
        self.valkey.del(&valkey_key)
            .await
            .map_err(|_| AuthError::TokenRevocationError)?;
        Ok(())
    }

}