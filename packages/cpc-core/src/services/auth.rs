use crate::{
    models::user::{NewUser, User},
    repositories::user_repository::UserRepository,
    utils::{datetime::now_utc, password},
};
use anyhow::{anyhow, Result};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// JWT claims structure
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // User ID
    pub exp: usize, // Expiration time (as UTC timestamp)
}

/// Authentication service for handling user registration, login, and token management
pub struct AuthService {
    user_repo: UserRepository,
    jwt_secret: String,
}

impl AuthService {
    /// Creates a new AuthService instance
    pub fn new(user_repo: UserRepository, jwt_secret: String) -> Self {
        Self {
            user_repo,
            jwt_secret,
        }
    }

    /// Registers a new user and returns an authentication token
    pub async fn register(&self, new_user: NewUser) -> Result<(User, String)> {
        // Validate user input
        new_user.validate().map_err(|e| anyhow!(e))?;

        // Hash password
        let password_hash = password::hash_password(&new_user.password)?;

        // Create user in database
        let mut user = User {
            id: Uuid::new_v4(),
            username: new_user.username,
            email: new_user.email,
            password_hash,
            created_at: now_utc(),
            updated_at: now_utc(),
            display_name: new_user.display_name,
            bio: None,
            avatar_url: None,
            friends: Vec::new(),
            followers: Vec::new(),
        };

        self.user_repo.create(&mut user).await?;

        // Generate JWT token
        let token = self.generate_token(user.id)?;

        Ok((user, token))
    }

    /// Authenticates a user and returns an authentication token
    pub async fn login(&self, email: &str, password: &str) -> Result<(User, String)> {
        // Find user by email
        let user = self
            .user_repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        // Verify password
        if !password::verify_password(password, &user.password_hash)? {
            return Err(anyhow!("Invalid password"));
        }

        // Generate JWT token
        let token = self.generate_token(user.id)?;

        Ok((user, token))
    }

    /// Generates a JWT token for a user
    fn generate_token(&self, user_id: Uuid) -> Result<String> {
        let expiration = (now_utc() + chrono::Duration::days(30)).timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )
        .map_err(|e| anyhow!("Failed to generate token: {}", e))
    }

    /// Validates a JWT token and returns the user ID if valid
    pub fn validate_token(&self, token: &str) -> Result<Uuid> {
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_ref()),
            &Validation::new(Algorithm::HS256),
        )?;

        Ok(token_data.claims.sub)
    }
}