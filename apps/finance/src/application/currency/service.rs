//! Currency service
//!
//! This module provides the main currency service that coordinates currency conversion,
//! formatting, and user preferences.

use rust_decimal::Decimal;
use uuid::Uuid;
use crate::domain::currency::{
    Currency, CurrencyCode, CurrencyRegistry, ExchangeRateService,
    ExchangeRateError, CurrencyFormatter, Locale
};
use super::user_prefs::{UserCurrencyPreferences, UserCurrencyPreferencesRepository, CurrencyPreferencesError};
use super::mock_repo::MockCurrencyPreferencesRepository;

/// Main currency service that coordinates all currency-related functionality
pub struct CurrencyService<R: UserCurrencyPreferencesRepository> {
    /// The currency registry containing all supported currencies
    currency_registry: CurrencyRegistry,
    
    /// The exchange rate service for currency conversion
    exchange_rate_service: ExchangeRateService,
    
    /// The currency formatter for locale-aware formatting
    currency_formatter: CurrencyFormatter,
    
    /// The repository for user currency preferences
    preferences_repository: R,
}

impl<R: UserCurrencyPreferencesRepository> CurrencyService<R> {
    /// Create a new currency service
    pub fn new(
        currency_registry: CurrencyRegistry,
        exchange_rate_service: ExchangeRateService,
        preferences_repository: R,
    ) -> Self {
        Self {
            currency_registry,
            exchange_rate_service,
            currency_formatter: CurrencyFormatter::new(),
            preferences_repository,
        }
    }

    /// Get a currency by its code
    pub fn get_currency(&self, code: &str) -> Option<&Currency> {
        self.currency_registry.get(code)
    }

    /// Get all supported currencies
    pub fn get_all_currencies(&self) -> Vec<&Currency> {
        self.currency_registry.all()
    }

    /// Convert an amount from one currency to another
    pub async fn convert_currency(
        &mut self,
        amount: Decimal,
        from: &CurrencyCode,
        to: &CurrencyCode,
    ) -> Result<Decimal, CurrencyServiceError> {
        self.exchange_rate_service
            .convert(amount, from, to)
            .await
            .map_err(CurrencyServiceError::from)
    }

    /// Format a currency amount according to user preferences
    pub async fn format_currency_for_user(
        &self,
        amount: Decimal,
        currency: &Currency,
        user_id: Uuid,
    ) -> Result<String, CurrencyServiceError> {
        let preferences = self.preferences_repository
            .get_preferences(user_id)
            .await
            .map_err(CurrencyServiceError::from)?;
        
        let locale: Locale = preferences.preferred_locale.as_str().into();
        
        if preferences.show_currency_symbols {
            Ok(self.currency_formatter.format_currency(amount, currency, &locale))
        } else {
            Ok(self.currency_formatter.format_currency_with_code(amount, currency, &locale))
        }
    }

    /// Format a currency amount with a specific locale
    pub fn format_currency_with_locale(
        &self,
        amount: Decimal,
        currency: &Currency,
        locale_code: &str,
    ) -> String {
        let locale: Locale = locale_code.into();
        self.currency_formatter.format_currency(amount, currency, &locale)
    }

    /// Get user currency preferences
    pub async fn get_user_preferences(&self, user_id: Uuid) -> Result<UserCurrencyPreferences, CurrencyServiceError> {
        self.preferences_repository
            .get_preferences(user_id)
            .await
            .map_err(CurrencyServiceError::from)
    }

    /// Set user's default currency
    pub async fn set_user_default_currency(
        &self,
        user_id: Uuid,
        currency_code: CurrencyCode,
    ) -> Result<(), CurrencyServiceError> {
        self.preferences_repository
            .set_default_currency(user_id, currency_code)
            .await
            .map_err(CurrencyServiceError::from)
    }

    /// Clean up expired cache entries
    pub fn cleanup_cache(&mut self) {
        self.exchange_rate_service.cleanup_cache();
    }
}

/// Error types for currency service operations
#[derive(thiserror::Error, Debug)]
pub enum CurrencyServiceError {
    #[error("Exchange rate error: {0}")]
    ExchangeRateError(#[from] ExchangeRateError),
    
    #[error("Currency preferences error: {0}")]
    CurrencyPreferencesError(#[from] CurrencyPreferencesError),
    
    #[error("Currency not found: {0}")]
    CurrencyNotFound(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::currency::{MockExchangeRateProvider, ExchangeRate};
    use rust_decimal_macros::dec;
    use async_trait::async_trait;

    struct MockPreferencesRepository;

    #[async_trait]
    impl UserCurrencyPreferencesRepository for MockPreferencesRepository {
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

    #[tokio::test]
    async fn test_currency_service_creation() {
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let exchange_service = ExchangeRateService::new(vec![provider], 60);
        let repo = MockPreferencesRepository;
        
        let service = CurrencyService::new(registry, exchange_service, repo);
        assert!(!service.get_all_currencies().is_empty());
    }

    #[tokio::test]
    async fn test_get_currency() {
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let exchange_service = ExchangeRateService::new(vec![provider], 60);
        let repo = MockPreferencesRepository;
        
        let service = CurrencyService::new(registry, exchange_service, repo);
        let currency = service.get_currency("USD");
        assert!(currency.is_some());
        assert_eq!(currency.unwrap().code(), "USD");
    }

    #[tokio::test]
    async fn test_format_currency_with_locale() {
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let exchange_service = ExchangeRateService::new(vec![provider], 60);
        let repo = MockPreferencesRepository;
        
        let service = CurrencyService::new(registry, exchange_service, repo);
        let currency: Currency = CurrencyCode::new("USD").into();
        let formatted = service.format_currency_with_locale(dec!(1234.56), &currency, "en-US");
        assert_eq!(formatted, "$1,234.56");
    }
}