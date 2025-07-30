//! User currency preferences
//!
//! This module provides functionality for managing user currency preferences,
//! including default currencies and formatting preferences.

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::domain::currency::{Currency, CurrencyCode};

/// User currency preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserCurrencyPreferences {
    /// The user ID
    pub user_id: Uuid,
    
    /// The user's default currency
    pub default_currency: Currency,
    
    /// The user's preferred locale for formatting
    pub preferred_locale: String,
    
    /// Whether to show currency symbols or codes
    pub show_currency_symbols: bool,
}

impl UserCurrencyPreferences {
    /// Create new user currency preferences
    pub fn new(user_id: Uuid, default_currency: Currency, preferred_locale: String) -> Self {
        Self {
            user_id,
            default_currency,
            preferred_locale,
            show_currency_symbols: true,
        }
    }

    /// Get the default currency code
    pub fn default_currency_code(&self) -> &str {
        self.default_currency.code()
    }

    /// Update the default currency
    pub fn set_default_currency(&mut self, currency: Currency) {
        self.default_currency = currency;
    }

    /// Update the preferred locale
    pub fn set_preferred_locale(&mut self, locale: String) {
        self.preferred_locale = locale;
    }
}

/// Repository trait for user currency preferences
#[async_trait::async_trait]
pub trait UserCurrencyPreferencesRepository: Send + Sync {
    /// Get user currency preferences
    async fn get_preferences(&self, user_id: Uuid) -> Result<UserCurrencyPreferences, CurrencyPreferencesError>;
    
    /// Save user currency preferences
    async fn save_preferences(&self, preferences: &UserCurrencyPreferences) -> Result<(), CurrencyPreferencesError>;
    
    /// Set the default currency for a user
    async fn set_default_currency(&self, user_id: Uuid, currency_code: CurrencyCode) -> Result<(), CurrencyPreferencesError>;
}

/// Error types for currency preferences operations
#[derive(thiserror::Error, Debug)]
pub enum CurrencyPreferencesError {
    #[error("User not found: {0}")]
    UserNotFound(Uuid),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Invalid currency code: {0}")]
    InvalidCurrencyCode(String),
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::currency::CurrencyCode;

    #[test]
    fn test_user_preferences_creation() {
        let user_id = Uuid::new_v4();
        let currency: Currency = CurrencyCode::new("USD").into();
        let preferences = UserCurrencyPreferences::new(user_id, currency, "en-US".to_string());
        
        assert_eq!(preferences.user_id, user_id);
        assert_eq!(preferences.default_currency_code(), "USD");
        assert_eq!(preferences.preferred_locale, "en-US");
        assert!(preferences.show_currency_symbols);
    }

    #[test]
    fn test_user_preferences_modification() {
        let user_id = Uuid::new_v4();
        let currency: Currency = CurrencyCode::new("USD").into();
        let mut preferences = UserCurrencyPreferences::new(user_id, currency, "en-US".to_string());
        
        let eur_currency: Currency = CurrencyCode::new("EUR").into();
        preferences.set_default_currency(eur_currency);
        assert_eq!(preferences.default_currency_code(), "EUR");
        
        preferences.set_preferred_locale("de-DE".to_string());
        assert_eq!(preferences.preferred_locale, "de-DE");
    }
}