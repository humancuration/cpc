//! Tests for the auth module
//!
//! This module contains tests for the authentication service functionality.

use wasm_bindgen_test::*;
use web_core::auth::{AuthService, User};

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_auth_service_creation() {
    let auth_service = AuthService::new();
    assert_eq!(std::mem::size_of_val(&auth_service), 0); // Empty struct
}

#[wasm_bindgen_test]
fn test_user_serialization() {
    let user = User {
        id: uuid::Uuid::nil(),
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
    };

    let serialized = serde_json::to_string(&user).unwrap();
    let deserialized: User = serde_json::from_str(&serialized).unwrap();

    assert_eq!(user.id, deserialized.id);
    assert_eq!(user.username, deserialized.username);
    assert_eq!(user.email, deserialized.email);
}