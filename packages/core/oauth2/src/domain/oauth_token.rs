//! OAuth token representation with encryption capabilities

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm, Nonce, Key,
};
use rand::RngCore;
use base64::{Engine as _, engine::general_purpose};
use crate::domain::AuthError;

/// OAuth token with encryption capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthToken {
    /// Encrypted access token
    pub access_token: String,
    
    /// Encrypted refresh token
    pub refresh_token: Option<String>,
    
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    
    /// Scopes granted by this token
    pub scopes: Vec<String>,
    
    /// Provider that issued this token
    pub provider: String,
}

impl OAuthToken {
    /// Create a new OAuth token
    pub fn new(
        access_token: String,
        refresh_token: Option<String>,
        expires_at: DateTime<Utc>,
        scopes: Vec<String>,
        provider: String,
    ) -> Self {
        Self {
            access_token,
            refresh_token,
            expires_at,
            scopes,
            provider,
        }
    }
    
    /// Encrypt the token using AES-GCM
    pub fn encrypt(&self, encryption_key: &[u8; 32]) -> Result<EncryptedToken, AuthError> {
        let key = Key::<Aes256Gcm>::from_slice(encryption_key);
        let cipher = Aes256Gcm::new(key);
        
        // Generate a random nonce
        let mut nonce_bytes = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);
        
        // Encrypt access token
        let encrypted_access_token = cipher.encrypt(nonce, self.access_token.as_bytes())
            .map_err(|e| AuthError::EncryptionError(format!("Failed to encrypt access token: {}", e)))?;
        
        // Encrypt refresh token if present
        let encrypted_refresh_token = if let Some(refresh_token) = &self.refresh_token {
            let encrypted = cipher.encrypt(nonce, refresh_token.as_bytes())
                .map_err(|e| AuthError::EncryptionError(format!("Failed to encrypt refresh token: {}", e)))?;
            Some(encrypted)
        } else {
            None
        };
        
        Ok(EncryptedToken {
            access_token: encrypted_access_token,
            refresh_token: encrypted_refresh_token,
            nonce: nonce_bytes.to_vec(),
            expires_at: self.expires_at,
            scopes: self.scopes.clone(),
            provider: self.provider.clone(),
        })
    }
    
    /// Decrypt an encrypted token
    pub fn decrypt(encrypted: &EncryptedToken, encryption_key: &[u8; 32]) -> Result<Self, AuthError> {
        let key = Key::<Aes256Gcm>::from_slice(encryption_key);
        let cipher = Aes256Gcm::new(key);
        let nonce = Nonce::from_slice(&encrypted.nonce);
        
        // Decrypt access token
        let decrypted_access_token = cipher.decrypt(nonce, encrypted.access_token.as_slice())
            .map_err(|e| AuthError::EncryptionError(format!("Failed to decrypt access token: {}", e)))?;
        let access_token = String::from_utf8(decrypted_access_token)
            .map_err(|e| AuthError::EncryptionError(format!("Failed to decode access token: {}", e)))?;
        
        // Decrypt refresh token if present
        let refresh_token = if let Some(refresh_token_data) = &encrypted.refresh_token {
            let decrypted = cipher.decrypt(nonce, refresh_token_data.as_slice())
                .map_err(|e| AuthError::EncryptionError(format!("Failed to decrypt refresh token: {}", e)))?;
            Some(String::from_utf8(decrypted)
                .map_err(|e| AuthError::EncryptionError(format!("Failed to decode refresh token: {}", e)))?)
        } else {
            None
        };
        
        Ok(Self {
            access_token,
            refresh_token,
            expires_at: encrypted.expires_at,
            scopes: encrypted.scopes.clone(),
            provider: encrypted.provider.clone(),
        })
    }
    
    /// Check if the token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() >= self.expires_at
    }
}

/// Encrypted token representation for storage
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedToken {
    /// Encrypted access token data
    pub access_token: Vec<u8>,
    
    /// Encrypted refresh token data
    pub refresh_token: Option<Vec<u8>>,
    
    /// Nonce used for encryption
    pub nonce: Vec<u8>,
    
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    
    /// Scopes granted by this token
    pub scopes: Vec<String>,
    
    /// Provider that issued this token
    pub provider: String,
}

impl EncryptedToken {
    /// Encode to base64 string for storage
    pub fn encode(&self) -> Result<String, AuthError> {
        let json = serde_json::to_string(self)
            .map_err(|e| AuthError::SerializationError(format!("Failed to serialize encrypted token: {}", e)))?;
        Ok(general_purpose::STANDARD.encode(json))
    }
    
    /// Decode from base64 string
    pub fn decode(encoded: &str) -> Result<Self, AuthError> {
        let json_bytes = general_purpose::STANDARD.decode(encoded)
            .map_err(|e| AuthError::SerializationError(format!("Failed to decode encrypted token: {}", e)))?;
        let json = String::from_utf8(json_bytes)
            .map_err(|e| AuthError::SerializationError(format!("Failed to parse encrypted token JSON: {}", e)))?;
        serde_json::from_str(&json)
            .map_err(|e| AuthError::SerializationError(format!("Failed to deserialize encrypted token: {}", e)))
    }
}