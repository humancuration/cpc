use jsonwebtoken::{encode, decode, Header, Algorithm, EncodingKey, DecodingKey, Validation};
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{Utc, Duration};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,      // user id
    pub exp: usize,     // expiration time
    pub token_type: String, // "access" or "refresh"
}

pub struct JwtService {
    access_secret: String,
    refresh_secret: String,
}

impl JwtService {
    pub fn new(access_secret: String, refresh_secret: String) -> Self {
        Self {
            access_secret,
            refresh_secret
        }
    }

    pub fn generate_access_token(&self, user_id: Uuid) -> String {
        self.generate_token(
            user_id,
            "access".to_string(),
            &self.access_secret,
            Duration::minutes(15)
        )
    }

    pub fn generate_refresh_token(&self, user_id: Uuid) -> String {
        self.generate_token(
            user_id,
            "refresh".to_string(),
            &self.refresh_secret,
            Duration::days(7)
        )
    }

    fn generate_token(
        &self,
        user_id: Uuid,
        token_type: String,
        secret: &str,
        expires_in: Duration
    ) -> String {
        let expiration = Utc::now()
            .checked_add_signed(expires_in)
            .expect("Invalid timestamp")
            .timestamp() as usize;

        let claims = Claims {
            sub: user_id,
            exp: expiration,
            token_type,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(secret.as_bytes()),
        )
        .unwrap_or_else(|_| "".to_string())
    }

    pub fn validate_access_token(&self, token: &str) -> Result<Claims, String> {
        self.validate_token(token, &self.access_secret, "access")
    }

    pub fn validate_refresh_token(&self, token: &str) -> Result<Claims, String> {
        self.validate_token(token, &self.refresh_secret, "refresh")
    }

    fn validate_token(
        &self,
        token: &str,
        secret: &str,
        expected_type: &str
    ) -> Result<Claims, String> {
        let decoding_key = DecodingKey::from_secret(secret.as_bytes());
        let validation = Validation::new(Algorithm::HS256);

        let claims = decode::<Claims>(token, &decoding_key, &validation)
            .map(|data| data.claims)
            .map_err(|err| format!("Invalid token: {}", err))?;
            
        if claims.token_type != expected_type {
            return Err("Invalid token type".to_string());
        }
            
        Ok(claims)
    }
}