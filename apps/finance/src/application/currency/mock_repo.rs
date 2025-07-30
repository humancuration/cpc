//! Mock repository for testing currency preferences

use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::currency::{Currency, CurrencyCode};
use super::user_prefs::{UserCurrencyPreferences, UserCurrencyPreferencesRepository, CurrencyPreferencesError};

/// Mock repository for testing
pub struct MockCurrencyPreferencesRepository;

#[async_trait]
impl UserCurrencyPreferencesRepository for MockCurrencyPreferencesRepository {
    async fn get_preferences(&self, _user_id: Uuid) -> Result<UserCurrencyPreferences, CurrencyPreferencesError> {
        let currency: Currency = CurrencyCode::new("USD").into();
        Ok(UserCurrencyPreferences::new(
            Uuid::nil(),
            currency,
            "en-US".to_string(),
        ))
    }

    async fn save_preferences(&self, _preferences: &UserCurrencyPreferences) -> Result<(), CurrencyPreferencesError> {
        Ok(())
    }

    async fn set_default_currency(&self, _user_id: Uuid, _currency_code: CurrencyCode) -> Result<(), CurrencyPreferencesError> {
        Ok(())
    }
}