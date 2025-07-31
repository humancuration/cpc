//! Authentication configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::domain::OAuthProvider;

/// Authentication configuration for a provider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    /// Client ID for the OAuth provider
    pub client_id: String,
    
    /// Client secret for the OAuth provider
    pub client_secret: String,
    
    /// Authorization URL for the provider
    pub auth_url: String,
    
    /// Token URL for the provider
    pub token_url: String,
    
    /// Redirect URIs allowed for this provider
    pub redirect_uris: Vec<String>,
    
    /// Default scopes for this provider
    pub default_scopes: Vec<String>,
}

/// Authentication configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthConfig {
    /// Configuration for each provider
    pub providers: HashMap<String, ProviderConfig>,
    
    /// Encryption key for token storage (32 bytes for AES-256)
    pub encryption_key: [u8; 32],
    
    /// Default redirect URI if none is specified
    pub default_redirect_uri: String,
}

impl AuthConfig {
    /// Create a new authentication configuration
    pub fn new(default_redirect_uri: String, encryption_key: [u8; 32]) -> Self {
        Self {
            providers: HashMap::new(),
            encryption_key,
            default_redirect_uri,
        }
    }
    
    /// Add or update provider configuration
    pub fn set_provider_config(&mut self, provider: OAuthProvider, config: ProviderConfig) {
        self.providers.insert(provider.to_string(), config);
    }
    
    /// Get provider configuration
    pub fn get_provider_config(&self, provider: &OAuthProvider) -> Option<&ProviderConfig> {
        self.providers.get(provider.as_str())
    }
    
    /// Generate a random encryption key
    pub fn generate_encryption_key() -> [u8; 32] {
        let mut key = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut key);
        key
    }
}