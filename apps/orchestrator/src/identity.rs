use tonic::{Request, Response, Status};
use crate::cpc_orchestrator::*;
use sqlx::PgPool;
use std::sync::Arc;
use ring::signature::{Ed25519KeyPair, UnparsedPublicKey, VerificationAlgorithm};
use ring::rand::SystemRandom;
use ring::signature::KeyPair;
use jsonwebtoken::{encode, decode, Header, EncodingKey, DecodingKey, Validation, Algorithm};
use serde::{Serialize, Deserialize};
use chrono::{Utc, Duration as ChronoDuration};
use crate::secret_manager::SecretManager;

pub struct IdentityService {
    db_pool: Arc<PgPool>,
    secret_manager: Arc<SecretManager>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String, // User ID
    exp: usize,  // Expiration time
}

impl IdentityService {
    pub fn new(db_pool: Arc<PgPool>, secret_manager: Arc<SecretManager>) -> Self {
        Self { db_pool, secret_manager }
    }

    fn verify_signature(&self, public_key: &[u8], signature: &[u8], message: &[u8]) -> bool {
        let peer_public_key = UnparsedPublicKey::new(&ring::signature::ED25519, public_key);
        peer_public_key.verify(message, signature).is_ok()
    }
fn generate_jwt(&self, user_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(ChronoDuration::minutes(15))
        .expect("valid timestamp")
        .timestamp() as usize;
    
    let claims = Claims {
        sub: user_id.to_owned(),
        exp: expiration,
    };
    
    let secret = self.secret_manager.get_current_secret().read().unwrap().clone();
    
    encode(
        &Header::new(Algorithm::EdDSA),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
}
    }
}

#[tonic::async_trait]
impl IdentityService for IdentityService {
    async fn authenticate(
        &self,
        request: Request<AuthRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let auth_request = request.into_inner();
        
        // TODO: Implement actual authentication
        // 1. Fetch user public key from database
        // 2. Verify signature
        // 3. Generate JWT and refresh token
        
        let jwt = self.generate_jwt("user_id")
            .map_err(|e| Status::internal(e.to_string()))?;
        
        Ok(Response::new(AuthResponse {
            jwt,
            refresh_token: "refresh_token".to_string(),
            expires_in: 900, // 15 minutes
        }))
    }

    async fn refresh_token(
        &self,
        request: Request<RefreshRequest>,
    ) -> Result<Response<AuthResponse>, Status> {
        let refresh_request = request.into_inner();
        
        // TODO: Validate refresh token
        // 1. Check token in database
        // 2. If valid, generate new JWT
        
        let jwt = self.generate_jwt("user_id")
            .map_err(|e| Status::internal(e.to_string()))?;
        
        Ok(Response::new(AuthResponse {
            jwt,
            refresh_token: refresh_request.refresh_token,
            expires_in: 900,
        }))
    }

    async fn manage_friends(
        &self,
        request: Request<FriendRequest>,
    ) -> Result<Response<FriendResponse>, Status> {
        let friend_request = request.into_inner();
        
        // TODO: Implement friend management
        // 1. Verify user authentication
        // 2. Update friends list in database based on action
        
        Ok(Response::new(FriendResponse {
            success: true,
            friends: vec![],
        }))
    }
}