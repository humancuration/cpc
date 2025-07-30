//! Comprehensive tests for the currency service

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use uuid::Uuid;
    use crate::{
        domain::currency::{CurrencyRegistry, CurrencyCode, MockExchangeRateProvider},
        application::currency::{CurrencyService, MockCurrencyPreferencesRepository},
    };

    #[tokio::test]
    async fn test_currency_service_full_integration() {
        // Setup
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let exchange_service = crate::domain::currency::ExchangeRateService::new(vec![provider], 60);
        let repo = MockCurrencyPreferencesRepository;
        
        let mut service = CurrencyService::new(registry, exchange_service, repo);
        
        // Test getting a currency
        let usd_currency = service.get_currency("USD");
        assert!(usd_currency.is_some());
        assert_eq!(usd_currency.unwrap().code(), "USD");
        
        // Test getting all currencies
        let all_currencies = service.get_all_currencies();
        assert!(!all_currencies.is_empty());
        assert!(all_currencies.len() > 150); // We should have 150+ currencies
        
        // Test formatting with locale
        let eur_currency: crate::domain::currency::Currency = CurrencyCode::new("EUR").into();
        let formatted = service.format_currency_with_locale(dec!(1234.56), &eur_currency, "de-DE");
        assert_eq!(formatted, "1.234,56 €");
        
        // Test getting user preferences
        let user_id = Uuid::new_v4();
        let preferences = service.get_user_preferences(user_id).await;
        assert!(preferences.is_ok());
        
        // Test setting user currency
        let result = service.set_user_default_currency(user_id, CurrencyCode::new("JPY")).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_currency_conversion() {
        // Setup
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let exchange_service = crate::domain::currency::ExchangeRateService::new(vec![provider], 60);
        let repo = MockCurrencyPreferencesRepository;
        
        let mut service = CurrencyService::new(registry, exchange_service, repo);
        
        // Test conversion (our mock returns 0.85 for USD to EUR)
        let result = service.convert_currency(dec!(100), &CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await;
        assert!(result.is_ok());
        let converted = result.unwrap();
        assert_eq!(converted, dec!(85)); // 100 * 0.85
        
        // Test same currency conversion (should return same amount)
        let result = service.convert_currency(dec!(100), &CurrencyCode::new("USD"), &CurrencyCode::new("USD")).await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), dec!(100));
    }

    #[tokio::test]
    async fn test_currency_not_found() {
        // Setup
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let exchange_service = crate::domain::currency::ExchangeRateService::new(vec![provider], 60);
        let repo = MockCurrencyPreferencesRepository;
        
        let service = CurrencyService::new(registry, exchange_service, repo);
        
        // Test getting non-existent currency
        let currency = service.get_currency("XYZ");
        assert!(currency.is_none());
    }

    #[tokio::test]
    async fn test_cache_cleanup() {
        // Setup
        let registry = CurrencyRegistry::new();
        let provider = Box::new(MockExchangeRateProvider);
        let mut exchange_service = crate::domain::currency::ExchangeRateService::new(vec![provider], 60);
        let repo = MockCurrencyPreferencesRepository;
        
        let mut service = CurrencyService::new(registry, exchange_service, repo);
        
        // Test cache cleanup (should not panic)
        service.cleanup_cache();
        assert!(true); // Just testing it doesn't crash
    }
}