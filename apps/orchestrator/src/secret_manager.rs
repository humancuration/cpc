use std::sync::{Arc, RwLock};
use valkey::ValkeyPool;
use jsonwebtoken::DecodingKey;
use thiserror::Error;
use tracing::info;

#[derive(Error, Debug)]
pub enum SecretError {
    #[error("Secret not found")]
    NotFound,
    #[error("Storage error: {0}")]
    StorageError(String),
}

pub enum SecretStorage {
    LocalEnv,
    Valkey(ValkeyPool),
    // Vault(VaultClient) - To be implemented later
}

pub struct SecretManager {
    current_secret: Arc<RwLock<String>>,
    previous_secrets: Arc<RwLock<Vec<String>>>,
    storage: SecretStorage,
}

impl SecretManager {
    pub fn new(storage: SecretStorage) -> Result<Self, SecretError> {
        let secret = Self::load_secret(&storage)?;
        Ok(Self {
            current_secret: Arc::new(RwLock::new(secret)),
            previous_secrets: Arc::new(RwLock::new(Vec::new())),
            storage,
        })
    }

    fn load_secret(storage: &SecretStorage) -> Result<String, SecretError> {
        match storage {
            SecretStorage::LocalEnv => {
                std::env::var("JWT_SECRET")
                    .map_err(|_| SecretError::NotFound)
            }
            SecretStorage::Valkey(pool) => {
                let mut conn = pool.get().map_err(|e| SecretError::StorageError(e.to_string()))?;
                let secret: String = conn.get("jwt_secrets:active")
                    .map_err(|e| SecretError::StorageError(e.to_string()))?;
                Ok(secret)
            }
        }
    }

    pub fn get_current_secret(&self) -> Arc<RwLock<String>> {
        self.current_secret.clone()
    }

    pub fn get_decoding_key(&self) -> DecodingKey {
        DecodingKey::from_secret(self.current_secret.read().unwrap().as_bytes())
    }

    pub async fn rotate_secret(&self, new_secret: String) -> Result<(), SecretError> {
        info!("Rotating JWT secret");
        let mut current = self.current_secret.write().unwrap();
        let mut previous = self.previous_secrets.write().unwrap();
        
        // Move current to previous secrets
        previous.push(current.clone());
        
        // Update current secret
        *current = new_secret.clone();
        
        // Update storage
        match &self.storage {
            SecretStorage::Valkey(pool) => {
                let mut conn = pool.get().map_err(|e| SecretError::StorageError(e.to_string()))?;
                conn.set("jwt_secrets:active", &new_secret)
                    .map_err(|e| SecretError::StorageError(e.to_string()))?;
            }
            _ => {} // Local env doesn't support rotation
        }
        
        Ok(())
    }
}