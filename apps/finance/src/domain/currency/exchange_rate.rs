//! Exchange rate management
//!
//! This module provides functionality for managing exchange rates between currencies,
//! including fetching from external providers, caching, and historical tracking.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use super::{CurrencyCode, Currency};

/// Represents an exchange rate between two currencies
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExchangeRate {
    /// The base currency code
    pub from: CurrencyCode,
    
    /// The target currency code
    pub to: CurrencyCode,
    
    /// The exchange rate value
    pub rate: Decimal,
    
    /// When this rate was last updated
    pub updated_at: DateTime<Utc>,
}

impl ExchangeRate {
    /// Create a new exchange rate
    pub fn new(from: CurrencyCode, to: CurrencyCode, rate: Decimal) -> Self {
        Self {
            from,
            to,
            rate,
            updated_at: Utc::now(),
        }
    }

    /// Convert an amount using this exchange rate
    pub fn convert(&self, amount: Decimal) -> Decimal {
        amount * self.rate
    }
}

/// Historical exchange rate data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalExchangeRate {
    /// The exchange rate
    pub rate: ExchangeRate,
    
    /// The timestamp when this rate was recorded
    pub timestamp: DateTime<Utc>,
}

/// Exchange rate provider trait
#[async_trait::async_trait]
pub trait ExchangeRateProvider: Send + Sync {
    /// Get the latest exchange rate between two currencies
    async fn get_rate(&self, from: &CurrencyCode, to: &CurrencyCode) -> Result<ExchangeRate, ExchangeRateError>;
    
    /// Get historical exchange rates
    async fn get_historical_rates(
        &self,
        from: &CurrencyCode,
        to: &CurrencyCode,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
    ) -> Result<Vec<HistoricalExchangeRate>, ExchangeRateError>;
    
    /// Get the provider name
    fn name(&self) -> &'static str;
}

/// Exchange rate cache for storing recently fetched rates
pub struct ExchangeRateCache {
    rates: HashMap<(String, String), ExchangeRate>,
    ttl_seconds: i64,
}

impl ExchangeRateCache {
    /// Create a new exchange rate cache with the specified TTL in seconds
    pub fn new(ttl_seconds: i64) -> Self {
        Self {
            rates: HashMap::new(),
            ttl_seconds,
        }
    }

    /// Get a rate from the cache if it exists and is not expired
    pub fn get(&self, from: &CurrencyCode, to: &CurrencyCode) -> Option<&ExchangeRate> {
        let key = (from.as_str().to_string(), to.as_str().to_string());
        self.rates.get(&key).and_then(|rate| {
            let now = Utc::now();
            let elapsed = now.signed_duration_since(rate.updated_at);
            if elapsed.num_seconds() < self.ttl_seconds {
                Some(rate)
            } else {
                None
            }
        })
    }

    /// Store a rate in the cache
    pub fn put(&mut self, rate: ExchangeRate) {
        let key = (rate.from.as_str().to_string(), rate.to.as_str().to_string());
        self.rates.insert(key, rate);
    }

    /// Clear expired entries from the cache
    pub fn cleanup(&mut self) {
        let now = Utc::now();
        self.rates.retain(|_, rate| {
            let elapsed = now.signed_duration_since(rate.updated_at);
            elapsed.num_seconds() < self.ttl_seconds
        });
    }
}

/// Exchange rate service that manages multiple providers and caching
pub struct ExchangeRateService {
    providers: Vec<Box<dyn ExchangeRateProvider>>,
    cache: ExchangeRateCache,
}

impl ExchangeRateService {
    /// Create a new exchange rate service
    pub fn new(providers: Vec<Box<dyn ExchangeRateProvider>>, cache_ttl_seconds: i64) -> Self {
        Self {
            providers,
            cache: ExchangeRateCache::new(cache_ttl_seconds),
        }
    }

    /// Add a provider to the service
    pub fn add_provider(&mut self, provider: Box<dyn ExchangeRateProvider>) {
        self.providers.push(provider);
    }

    /// Get the latest exchange rate between two currencies
    /// Tries providers in order until one succeeds
    pub async fn get_rate(&mut self, from: &CurrencyCode, to: &CurrencyCode) -> Result<ExchangeRate, ExchangeRateError> {
        // Check cache first
        if let Some(cached_rate) = self.cache.get(from, to) {
            return Ok(cached_rate.clone());
        }

        // Try providers in order
        let mut last_error = None;
        for provider in &self.providers {
            match provider.get_rate(from, to).await {
                Ok(rate) => {
                    // Cache the result
                    self.cache.put(rate.clone());
                    return Ok(rate);
                }
                Err(e) => {
                    last_error = Some(e);
                }
            }
        }

        // If we get here, all providers failed
        Err(last_error.unwrap_or(ExchangeRateError::NoProvidersAvailable))
    }

    /// Convert an amount from one currency to another
    pub async fn convert(&mut self, amount: Decimal, from: &CurrencyCode, to: &CurrencyCode) -> Result<Decimal, ExchangeRateError> {
        if from == to {
            return Ok(amount);
        }

        let rate = self.get_rate(from, to).await?;
        Ok(rate.convert(amount))
    }

    /// Clean up expired cache entries
    pub fn cleanup_cache(&mut self) {
        self.cache.cleanup();
    }
}

/// Error types for exchange rate operations
#[derive(thiserror::Error, Debug)]
pub enum ExchangeRateError {
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Invalid currency pair: {from} to {to}")]
    InvalidCurrencyPair { from: String, to: String },
    
    #[error("Rate not available")]
    RateNotAvailable,
    
    #[error("No providers available")]
    NoProvidersAvailable,
    
    #[error("Provider error: {0}")]
    ProviderError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use chrono::Duration;

    /// Mock exchange rate provider for testing
    pub struct MockExchangeRateProvider;

    #[async_trait::async_trait]
    impl ExchangeRateProvider for MockExchangeRateProvider {
        async fn get_rate(&self, from: &CurrencyCode, to: &CurrencyCode) -> Result<ExchangeRate, ExchangeRateError> {
            if from.as_str() == "USD" && to.as_str() == "EUR" {
                Ok(ExchangeRate::new(
                    from.clone(),
                    to.clone(),
                    dec!(0.85),
                ))
            } else {
                Err(ExchangeRateError::RateNotAvailable)
            }
        }

        async fn get_historical_rates(
            &self,
            _from: &CurrencyCode,
            _to: &CurrencyCode,
            _start_date: DateTime<Utc>,
            _end_date: DateTime<Utc>,
        ) -> Result<Vec<HistoricalExchangeRate>, ExchangeRateError> {
            Ok(vec![])
        }

        fn name(&self) -> &'static str {
            "Mock"
        }
    }

    #[test]
    fn test_exchange_rate_creation() {
        let rate = ExchangeRate::new(
            CurrencyCode::new("USD"),
            CurrencyCode::new("EUR"),
            dec!(0.85),
        );
        
        assert_eq!(rate.from.as_str(), "USD");
        assert_eq!(rate.to.as_str(), "EUR");
        assert_eq!(rate.rate, dec!(0.85));
    }

    #[test]
    fn test_exchange_rate_conversion() {
        let rate = ExchangeRate::new(
            CurrencyCode::new("USD"),
            CurrencyCode::new("EUR"),
            dec!(0.85),
        );
        
        let result = rate.convert(dec!(100));
        assert_eq!(result, dec!(85));
    }

    #[test]
    fn test_cache_operations() {
        let mut cache = ExchangeRateCache::new(60); // 60 second TTL
        let rate = ExchangeRate::new(
            CurrencyCode::new("USD"),
            CurrencyCode::new("EUR"),
            dec!(0.85),
        );
        
        cache.put(rate.clone());
        let cached = cache.get(&CurrencyCode::new("USD"), &CurrencyCode::new("EUR"));
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().rate, dec!(0.85));
    }

    #[tokio::test]
    async fn test_exchange_rate_service() {
        let provider = Box::new(MockExchangeRateProvider);
        let mut service = ExchangeRateService::new(vec![provider], 60);
        
        let rate = service.get_rate(&CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await.unwrap();
        assert_eq!(rate.rate, dec!(0.85));
        
        let converted = service.convert(dec!(100), &CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await.unwrap();
        assert_eq!(converted, dec!(85));
    }
}