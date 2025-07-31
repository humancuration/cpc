//! OAuth user profile information

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::OAuthProvider;

/// OAuth user profile information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProfile {
    /// Unique identifier for the user from the provider
    pub provider_id: String,
    
    /// User's display name
    pub name: String,
    
    /// User's email address
    pub email: Option<String>,
    
    /// Provider that issued this profile
    pub provider: OAuthProvider,
    
    /// User's avatar URL if available
    pub avatar_url: Option<String>,
    
    /// Raw profile data from the provider
    pub raw_data: Option<serde_json::Value>,
}

impl OAuthProfile {
    /// Create a new OAuth profile
    pub fn new(
        provider_id: String,
        name: String,
        email: Option<String>,
        provider: OAuthProvider,
        avatar_url: Option<String>,
        raw_data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            provider_id,
            name,
            email,
            provider,
            avatar_url,
            raw_data,
        }
    }
}