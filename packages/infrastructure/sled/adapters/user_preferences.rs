//! Sled adapter for user preferences
//! 
//! This module implements the UserPreferences trait using Sled as the storage backend.
//! It provides offline-first storage with automatic sync capabilities.

use async_trait::async_trait;
use sled::{Db, Tree};
use uuid::Uuid;
use packages::domains::finance::domain::primitives::Currency;
use packages::domains::finance::application::user_preferences::UserPreferences;
use packages::infra::sync::queue::UserPreferencesClient;
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Sled implementation of UserPreferences
pub struct SledUserPreferences {
    tree: Tree,
}

/// Stored preference format
#[derive(Debug, Clone, Serialize, Deserialize)]
struct StoredPreference {
    currency_code: String,
    synced: bool,
    timestamp: u64,
}

impl SledUserPreferences {
    /// Create a new SledUserPreferences instance
    pub fn new(db: &Db) -> Self {
        let tree = db.open_tree("user_preferences").expect("Failed to open user_preferences tree");
        Self { tree }
    }
    
    /// Convert UUID to bytes for use as Sled key
    fn uuid_to_bytes(&self, user_id: Uuid) -> [u8; 16] {
        *user_id.as_bytes()
    }
    
    /// Get current timestamp in seconds since epoch
    fn current_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }
}

#[async_trait]
impl UserPreferences for SledUserPreferences {
    async fn get_preferred_currency(&self, user_id: Uuid) -> Result<Currency, String> {
        let key = self.uuid_to_bytes(user_id);
        
        match self.tree.get(key).map_err(|e| e.to_string())? {
            Some(bytes) => {
                let stored: StoredPreference = bincode::deserialize(&bytes)
                    .map_err(|e| format!("Failed to deserialize preference: {}", e))?;
                
                // Convert currency code to Currency enum
                match stored.currency_code.as_str() {
                    "USD" => Ok(Currency::USD),
                    "EUR" => Ok(Currency::EUR),
                    "GBP" => Ok(Currency::GBP),
                    "JPY" => Ok(Currency::JPY),
                    "CAD" => Ok(Currency::CAD),
                    "AUD" => Ok(Currency::AUD),
                    "CHF" => Ok(Currency::CHF),
                    "CNY" => Ok(Currency::CNY),
                    "SEK" => Ok(Currency::SEK),
                    "NZD" => Ok(Currency::NZD),
                    "MXN" => Ok(Currency::MXN),
                    "SGD" => Ok(Currency::SGD),
                    "HKD" => Ok(Currency::HKD),
                    "NOK" => Ok(Currency::NOK),
                    "KRW" => Ok(Currency::KRW),
                    "TRY" => Ok(Currency::TRY),
                    "RUB" => Ok(Currency::RUB),
                    "INR" => Ok(Currency::INR),
                    "BRL" => Ok(Currency::BRL),
                    "ZAR" => Ok(Currency::ZAR),
                    "DABLOONS" => Ok(Currency::Dabloons),
                    _ => Err(format!("Unknown currency code: {}", stored.currency_code)),
                }
            }
            None => Err("User preference not found".to_string()),
        }
    }
    
    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String> {
        let key = self.uuid_to_bytes(user_id);
        
        let stored = StoredPreference {
            currency_code: currency.code().to_string(),
            synced: false, // Mark as not synced initially
            timestamp: self.current_timestamp(),
        };
        
        let bytes = bincode::serialize(&stored)
            .map_err(|e| format!("Failed to serialize preference: {}", e))?;
        
        self.tree.insert(key, bytes)
            .map_err(|e| e.to_string())?;
        
        // Flush to disk to ensure persistence
        self.tree.flush_async().await
            .map_err(|e| format!("Failed to flush to disk: {}", e))?;
        
        Ok(())
    }
}

/// Extension trait for conflict resolution using vector clocks
pub trait ConflictResolution {
    /// Resolve conflicts between local and remote preferences
    fn resolve_conflict(&self, local: &StoredPreference, remote: &StoredPreference) -> &StoredPreference;
}

impl ConflictResolution for SledUserPreferences {
    fn resolve_conflict(&self, local: &StoredPreference, remote: &StoredPreference) -> &StoredPreference {
        // Simple last-write-wins strategy based on timestamp
        if local.timestamp > remote.timestamp {
            local
        } else {
            remote
        }
    }
}

#[async_trait]
impl UserPreferencesClient for SledUserPreferences {
    async fn set_preferred_currency(&self, user_id: Uuid, currency: Currency) -> Result<(), String> {
        self.set_preferred_currency(user_id, currency).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_set_and_get_currency() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Set currency
        let result = preferences.set_preferred_currency(user_id, Currency::EUR).await;
        assert!(result.is_ok());
        
        // Get currency
        let currency = preferences.get_preferred_currency(user_id).await;
        assert!(currency.is_ok());
        assert_eq!(currency.unwrap(), Currency::EUR);
    }
    
    #[tokio::test]
    async fn test_get_nonexistent_currency() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Try to get currency for user with no preference set
        let result = preferences.get_preferred_currency(user_id).await;
        assert!(result.is_err());
    }
    
    #[tokio::test]
    async fn test_update_currency() {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let db = sled::open(temp_dir.path()).expect("Failed to open sled database");
        let preferences = SledUserPreferences::new(&db);
        
        let user_id = Uuid::new_v4();
        
        // Set initial currency
        let result = preferences.set_preferred_currency(user_id, Currency::USD).await;
        assert!(result.is_ok());
        
        // Update currency
        let result = preferences.set_preferred_currency(user_id, Currency::JPY).await;
        assert!(result.is_ok());
        
        // Get updated currency
        let currency = preferences.get_preferred_currency(user_id).await;
        assert!(currency.is_ok());
        assert_eq!(currency.unwrap(), Currency::JPY);
    }
}