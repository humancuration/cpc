//! Secure key manager for payment processor API keys
//!
//! This module contains the functionality for securely storing and retrieving payment processor API keys using cpc-net encryption.

use crate::domain::payment::PaymentError;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Trait for encryption service
#[async_trait]
pub trait EncryptionService: Send + Sync {
    async fn encrypt(&self, plaintext: &str) -> Result<String, PaymentError>;
    async fn decrypt(&self, ciphertext: &str) -> Result<String, PaymentError>;
}

/// Secure key manager for storing API keys
pub struct SecureKeyManager {
    encryption_service: Box<dyn EncryptionService>,
    keys: HashMap<String, String>, // In a real implementation, this would be persistent storage
}

impl SecureKeyManager {
    pub fn new(encryption_service: Box<dyn EncryptionService>) -> Self {
        Self {
            encryption_service,
            keys: HashMap::new(),
        }
    }

    /// Store and encrypt an API key
    pub async fn store_key(&mut self, provider: &str, key: &str) -> Result<(), PaymentError> {
        let encrypted_key = self.encryption_service.encrypt(key).await?;
        self.keys.insert(provider.to_string(), encrypted_key);
        Ok(())
    }

    /// Retrieve and decrypt an API key
    pub async fn retrieve_key(&self, provider: &str) -> Result<String, PaymentError> {
        let encrypted_key = self.keys.get(provider).ok_or_else(|| {
            PaymentError::AuthenticationError(format!("No API key stored for provider: {}", provider))
        })?;
        self.encryption_service.decrypt(encrypted_key).await
    }

    /// Delete an API key
    pub fn delete_key(&mut self, provider: &str) {
        self.keys.remove(provider);
    }

    /// List all providers with stored keys
    pub fn list_providers(&self) -> Vec<String> {
        self.keys.keys().cloned().collect()
    }
}

/// Implementation of EncryptionService using cpc-net
pub struct CpcNetEncryptionService {
    // In a real implementation, this would contain cpc-net encryption components
}

#[async_trait]
impl EncryptionService for CpcNetEncryptionService {
    async fn encrypt(&self, plaintext: &str) -> Result<String, PaymentError> {
        // In a real implementation, this would use cpc-net encryption
        // For now, we'll just base64 encode as a mock (NOT secure for production)
        Ok(base64::encode(plaintext))
    }

    async fn decrypt(&self, ciphertext: &str) -> Result<String, PaymentError> {
        // In a real implementation, this would use cpc-net decryption
        // For now, we'll just base64 decode as a mock (NOT secure for production)
        base64::decode(ciphertext)
            .map(String::from_utf8)
            .map_err(|_| PaymentError::AuthenticationError("Failed to decode key".to_string()))?
            .map_err(|_| PaymentError::AuthenticationError("Invalid UTF-8 in key".to_string()))
    }
}

/// Key rotation policy
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyRotationPolicy {
    pub provider: String,
    pub rotation_interval_days: u32,
    pub last_rotation: Option<chrono::DateTime<chrono::Utc>>,
    pub auto_rotate: bool,
}

impl KeyRotationPolicy {
    pub fn new(provider: String, rotation_interval_days: u32, auto_rotate: bool) -> Self {
        Self {
            provider,
            rotation_interval_days,
            last_rotation: None,
            auto_rotate,
        }
    }

    /// Check if key rotation is needed
    pub fn needs_rotation(&self) -> bool {
        if !self.auto_rotate {
            return false;
        }

        if let Some(last_rotation) = self.last_rotation {
            let days_since_rotation = chrono::Utc::now().signed_duration_since(last_rotation).num_days();
            days_since_rotation >= self.rotation_interval_days as i64
        } else {
            true // Never rotated before
        }
    }

    /// Update last rotation timestamp
    pub fn update_rotation(&mut self) {
        self.last_rotation = Some(chrono::Utc::now());
    }
}

/// Key manager with rotation support
pub struct RotatingKeyManager {
    key_manager: SecureKeyManager,
    policies: HashMap<String, KeyRotationPolicy>,
}

impl RotatingKeyManager {
    pub fn new(key_manager: SecureKeyManager) -> Self {
        Self {
            key_manager,
            policies: HashMap::new(),
        }
    }

    /// Set key rotation policy for a provider
    pub fn set_rotation_policy(&mut self, policy: KeyRotationPolicy) {
        self.policies.insert(policy.provider.clone(), policy);
    }

    /// Check if any keys need rotation
    pub fn check_rotation_needed(&self) -> Vec<String> {
        self.policies
            .iter()
            .filter(|(_, policy)| policy.needs_rotation())
            .map(|(provider, _)| provider.clone())
            .collect()
    }

    /// Rotate a key (in a real implementation, this would involve generating a new key)
    pub async fn rotate_key(&mut self, provider: &str) -> Result<(), PaymentError> {
        if let Some(policy) = self.policies.get_mut(provider) {
            // In a real implementation, we would generate a new key and update it
            // For now, we'll just update the rotation timestamp
            policy.update_rotation();
            Ok(())
        } else {
            Err(PaymentError::AuthenticationError(
                format!("No rotation policy for provider: {}", provider)
            ))
        }
    }

    /// Delegate to the inner key manager
    pub async fn store_key(&mut self, provider: &str, key: &str) -> Result<(), PaymentError> {
        self.key_manager.store_key(provider, key).await
    }

    pub async fn retrieve_key(&self, provider: &str) -> Result<String, PaymentError> {
        self.key_manager.retrieve_key(provider).await
    }

    pub fn delete_key(&mut self, provider: &str) {
        self.key_manager.delete_key(provider);
    }

    pub fn list_providers(&self) -> Vec<String> {
        self.key_manager.list_providers()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    struct MockEncryptionService;

    #[async_trait]
    impl EncryptionService for MockEncryptionService {
        async fn encrypt(&self, plaintext: &str) -> Result<String, PaymentError> {
            Ok(format!("encrypted_{}", plaintext))
        }

        async fn decrypt(&self, ciphertext: &str) -> Result<String, PaymentError> {
            if ciphertext.starts_with("encrypted_") {
                Ok(ciphertext[10..].to_string())
            } else {
                Err(PaymentError::AuthenticationError("Invalid encrypted data".to_string()))
            }
        }
    }

    #[tokio::test]
    async fn test_secure_key_manager() {
        let encryption_service = Box::new(MockEncryptionService);
        let mut key_manager = SecureKeyManager::new(encryption_service);
        
        let provider = "stripe";
        let api_key = "sk_test_1234567890";
        
        // Store key
        assert!(key_manager.store_key(provider, api_key).await.is_ok());
        
        // Retrieve key
        let retrieved_key = key_manager.retrieve_key(provider).await;
        assert!(retrieved_key.is_ok());
        assert_eq!(retrieved_key.unwrap(), api_key);
        
        // List providers
        let providers = key_manager.list_providers();
        assert_eq!(providers, vec![provider]);
        
        // Delete key
        key_manager.delete_key(provider);
        let providers = key_manager.list_providers();
        assert!(providers.is_empty());
    }

    #[test]
    fn test_key_rotation_policy() {
        let mut policy = KeyRotationPolicy::new("stripe".to_string(), 30, true);
        assert!(policy.needs_rotation()); // Should need rotation initially
        
        policy.update_rotation();
        assert!(!policy.needs_rotation()); // Should not need rotation immediately after update
    }
}