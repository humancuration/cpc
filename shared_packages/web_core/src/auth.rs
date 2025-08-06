//! Authentication service for CPC web applications
//!
//! This module provides authentication functionality including
//! user management, login/logout, and JWT handling.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use wasm_bindgen::JsValue;
use web_sys::window;

/// Service for handling authentication
pub struct AuthService {
    // In a real implementation, this would hold auth state
}

/// User information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
}

/// JWT token information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: i64,
}

impl AuthService {
    /// Create a new auth service
    pub fn new() -> Self {
        Self {}
    }
    
    /// Get the current user from local storage
    pub fn get_current_user(&self) -> Option<User> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(user_json)) = storage.get_item("current_user") {
                        if let Ok(user) = serde_json::from_str(&user_json) {
                            return Some(user);
                        }
                    }
                }
            }
        }
        None
    }
    
    /// Save the current user to local storage
    pub fn save_current_user(&self, user: &User) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let user_json = serde_json::to_string(user).map_err(|_| JsValue::from_str("Failed to serialize user"))?;
                    storage.set_item("current_user", &user_json)?;
                }
            }
        }
        Ok(())
    }
    
    /// Clear the current user from local storage
    pub fn clear_current_user(&self) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    storage.remove_item("current_user")?;
                }
            }
        }
        Ok(())
    }
    
    /// Login a user
    pub async fn login(&self, username: &str, password: &str) -> Result<User, String> {
        // In a real implementation, this would authenticate with the backend
        // For now, we'll create a mock user
        let user = User {
            id: Uuid::new_v4(),
            username: username.to_string(),
            email: format!("{}@example.com", username),
        };
        
        // Save user to local storage
        if let Err(e) = self.save_current_user(&user) {
            return Err(format!("Failed to save user: {:?}", e));
        }
        
        Ok(user)
    }
    
    /// Logout the current user
    pub async fn logout(&self) -> Result<(), String> {
        // Clear user from local storage
        if let Err(e) = self.clear_current_user() {
            return Err(format!("Failed to clear user: {:?}", e));
        }
        Ok(())
    }
    
    /// Get JWT token from local storage
    pub fn get_token(&self) -> Option<Token> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    if let Ok(Some(token_json)) = storage.get_item("auth_token") {
                        if let Ok(token) = serde_json::from_str(&token_json) {
                            return Some(token);
                        }
                    }
                }
            }
        }
        None
    }
    
    /// Save JWT token to local storage
    pub fn save_token(&self, token: &Token) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    let token_json = serde_json::to_string(token).map_err(|_| JsValue::from_str("Failed to serialize token"))?;
                    storage.set_item("auth_token", &token_json)?;
                }
            }
        }
        Ok(())
    }
    
    /// Clear JWT token from local storage
    pub fn clear_token(&self) -> Result<(), JsValue> {
        if let Some(window) = window() {
            if let Ok(storage) = window.local_storage() {
                if let Some(storage) = storage {
                    storage.remove_item("auth_token")?;
                }
            }
        }
        Ok(())
    }
}

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}