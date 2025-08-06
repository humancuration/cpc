//! Mock storage adapter for testing
//!
//! This module provides a mock storage implementation using gloo-mocks
//! for testing components that depend on browser storage.

use gloo_mocks::MockStorage;
use serde::{de::DeserializeOwned, Serialize};
use wasm_bindgen::JsValue;

/// Mock storage implementation for testing
pub struct MockStorageAdapter {
    storage: MockStorage,
}

impl MockStorageAdapter {
    /// Create a new mock storage adapter
    pub fn new() -> Self {
        Self {
            storage: MockStorage::new(),
        }
    }

    /// Get an item from mock storage
    pub fn get_item(&self, key: &str) -> Result<Option<String>, JsValue> {
        self.storage.get_item(key)
    }

    /// Set an item in mock storage
    pub fn set_item(&self, key: &str, value: &str) -> Result<(), JsValue> {
        self.storage.set_item(key, value)
    }

    /// Remove an item from mock storage
    pub fn remove_item(&self, key: &str) -> Result<(), JsValue> {
        self.storage.remove_item(key)
    }

    /// Clear all items from mock storage
    pub fn clear(&self) -> Result<(), JsValue> {
        self.storage.clear()
    }

    /// Get a value from mock storage and deserialize it
    pub fn get_value<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, JsValue> {
        if let Some(value) = self.get_item(key)? {
            let deserialized: T = serde_json::from_str(&value)
                .map_err(|e| JsValue::from_str(&format!("Deserialization error: {:?}", e)))?;
            Ok(Some(deserialized))
        } else {
            Ok(None)
        }
    }

    /// Serialize a value and set it in mock storage
    pub fn set_value<T: Serialize>(&self, key: &str, value: &T) -> Result<(), JsValue> {
        let serialized = serde_json::to_string(value)
            .map_err(|e| JsValue::from_str(&format!("Serialization error: {:?}", e)))?;
        self.set_item(key, &serialized)
    }
}

impl Default for MockStorageAdapter {
    fn default() -> Self {
        Self::new()
    }
}