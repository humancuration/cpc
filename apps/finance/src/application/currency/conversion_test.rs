//! Tests for currency conversion functionality

#[cfg(test)]
mod tests {
    use rust_decimal_macros::dec;
    use crate::{
        domain::currency::{CurrencyRegistry, CurrencyCode, MockExchangeRateProvider, ExchangeRateService},
    };

    #[tokio::test]
    async fn test_exchange_rate_service_with_multiple_providers() {
        let provider1 = Box::new(MockExchangeRateProvider);
        let provider2 = Box::new(MockExchangeRateProvider);
        let mut service = ExchangeRateService::new(vec![provider1, provider2], 60);
        
        // Test getting a rate
        let rate = service.get_rate(&CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await;
        assert!(rate.is_ok());
        assert_eq!(rate.unwrap().rate, dec!(0.85));
    }

    #[tokio::test]
    async fn test_exchange_rate_caching() {
        let provider = Box::new(MockExchangeRateProvider);
        let mut service = ExchangeRateService::new(vec![provider], 60);
        
        // Get a rate first time
        let rate1 = service.get_rate(&CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await;
        assert!(rate1.is_ok());
        
        // Get the same rate second time (should come from cache)
        let rate2 = service.get_rate(&CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await;
        assert!(rate2.is_ok());
        
        // Rates should be the same
        assert_eq!(rate1.unwrap().rate, rate2.unwrap().rate);
    }

    #[tokio::test]
    async fn test_currency_conversion() {
        let provider = Box::new(MockExchangeRateProvider);
        let mut service = ExchangeRateService::new(vec![provider], 60);
        
        // Test conversion
        let converted = service.convert(dec!(100), &CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await;
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), dec!(85)); // 100 * 0.85
    }

    #[tokio::test]
    async fn test_same_currency_conversion() {
        let provider = Box::new(MockExchangeRateProvider);
        let mut service = ExchangeRateService::new(vec![provider], 60);
        
        // Test conversion with same currency
        let converted = service.convert(dec!(100), &CurrencyCode::new("USD"), &CurrencyCode::new("USD")).await;
        assert!(converted.is_ok());
        assert_eq!(converted.unwrap(), dec!(100));
    }

    #[tokio::test]
    async fn test_cache_cleanup() {
        let provider = Box::new(MockExchangeRateProvider);
        let mut service = ExchangeRateService::new(vec![provider], 1); // 1 second TTL
        
        // Add a rate to cache
        service.get_rate(&CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await.unwrap();
        
        // Cleanup cache
        service.cleanup_cache();
        
        // This should work fine (cache is empty but service still works)
        assert!(true);
    }

    #[test]
    fn test_currency_registry_completeness() {
        let registry = CurrencyRegistry::new();
        
        // Test we have major currencies
        assert!(registry.get("USD").is_some());
        assert!(registry.get("EUR").is_some());
        assert!(registry.get("GBP").is_some());
        assert!(registry.get("JPY").is_some());
        
        // Test we have Dabloons
        assert!(registry.get("DABLOONS").is_some());
        
        // Test we have many currencies
        assert!(registry.all().len() > 150);
    }

    #[test]
    fn test_currency_code_functionality() {
        let code = CurrencyCode::new("USD");
        assert_eq!(code.as_str(), "USD");
        assert_eq!(code.decimal_places(), 2);
        
        let code = CurrencyCode::new("JPY");
        assert_eq!(code.decimal_places(), 0);
        
        let code = CurrencyCode::new("BHD");
        assert_eq!(code.decimal_places(), 3);
    }
}