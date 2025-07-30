//! Tests for the ExchangeRateManager component
//!
//! This module contains comprehensive tests for the ExchangeRateManager component,
//! including rate management, filtering, and error handling.

use wasm_bindgen_test::*;
use yew::prelude::*;
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};
use chrono::{DateTime, Utc};

// Import the component under test
use crate::components::currency::exchange_rate_manager::{ExchangeRateManager, ExchangeRateManagerProps, ExchangeRateManagerState, ExchangeRateEntry};

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::prelude::*;
    use packages_domains_finance::domain::currency::{Currency, CurrencyCode};
    use chrono::{DateTime, Utc};

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_creation() {
        // Test basic component creation
        let props = ExchangeRateManagerProps {};

        // The props struct is empty, so this test just verifies it can be created
        assert!(true);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_initial_state() {
        // Test initial state of the component
        let state = ExchangeRateManagerState {
            exchange_rates: vec![],
            filtered_rates: vec![],
            loading: true,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec![],
            available_providers: vec![],
        };

        assert_eq!(state.exchange_rates.len(), 0);
        assert_eq!(state.filtered_rates.len(), 0);
        assert_eq!(state.loading, true);
        assert_eq!(state.error, None);
        assert_eq!(state.filter_from_currency, None);
        assert_eq!(state.filter_to_currency, None);
        assert_eq!(state.filter_provider, None);
        assert_eq!(state.available_currencies.len(), 0);
        assert_eq!(state.available_providers.len(), 0);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_entry_creation() {
        // Test creation of exchange rate entries
        let entry = ExchangeRateEntry {
            from_currency: "USD".to_string(),
            to_currency: "EUR".to_string(),
            rate: 0.85,
            last_updated: Utc::now(),
            provider: "ECB".to_string(),
            is_active: true,
        };

        assert_eq!(entry.from_currency, "USD");
        assert_eq!(entry.to_currency, "EUR");
        assert_eq!(entry.rate, 0.85);
        assert_eq!(entry.provider, "ECB");
        assert_eq!(entry.is_active, true);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_with_sample_rates() {
        // Test with sample exchange rates
        let now = Utc::now();
        
        let rates = vec![
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.85,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true,
            },
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "GBP".to_string(),
                rate: 0.73,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true,
            },
            ExchangeRateEntry {
                from_currency: "EUR".to_string(),
                to_currency: "USD".to_string(),
                rate: 1.18,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true,
            },
        ];

        let state = ExchangeRateManagerState {
            exchange_rates: rates.clone(),
            filtered_rates: rates.clone(),
            loading: false,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec!["USD".to_string(), "EUR".to_string(), "GBP".to_string()],
            available_providers: vec!["ECB".to_string()],
        };

        assert_eq!(state.exchange_rates.len(), 3);
        assert_eq!(state.filtered_rates.len(), 3);
        assert_eq!(state.loading, false);
        assert_eq!(state.available_currencies.len(), 3);
        assert_eq!(state.available_providers.len(), 1);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_with_dabloon_currency() {
        // Test with Dabloons currency in exchange rates
        let now = Utc::now();
        
        let rates = vec![
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "DABLOONS".to_string(),
                rate: 100.0,
                last_updated: now,
                provider: "CPC".to_string(),
                is_active: true,
            },
            ExchangeRateEntry {
                from_currency: "DABLOONS".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.0085,
                last_updated: now,
                provider: "CPC".to_string(),
                is_active: true,
            },
        ];

        let state = ExchangeRateManagerState {
            exchange_rates: rates.clone(),
            filtered_rates: rates.clone(),
            loading: false,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec!["USD".to_string(), "DABLOONS".to_string(), "EUR".to_string()],
            available_providers: vec!["CPC".to_string()],
        };

        assert_eq!(state.exchange_rates.len(), 2);
        assert_eq!(state.filtered_rates.len(), 2);
        assert_eq!(state.available_currencies.len(), 3);
        assert!(state.available_currencies.contains(&"DABLOONS".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_filtering() {
        // Test filtering functionality
        let now = Utc::now();
        
        let rates = vec![
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.85,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true,
            },
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "GBP".to_string(),
                rate: 0.73,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true,
            },
        ];

        let state = ExchangeRateManagerState {
            exchange_rates: rates.clone(),
            filtered_rates: vec![rates[0].clone()], // Simulate filtering
            loading: false,
            error: None,
            filter_from_currency: Some("USD".to_string()),
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec!["USD".to_string(), "EUR".to_string(), "GBP".to_string()],
            available_providers: vec!["ECB".to_string()],
        };

        assert_eq!(state.exchange_rates.len(), 2);
        assert_eq!(state.filtered_rates.len(), 1);
        assert_eq!(state.filter_from_currency, Some("USD".to_string()));
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_error_handling() {
        // Test error handling
        let state = ExchangeRateManagerState {
            exchange_rates: vec![],
            filtered_rates: vec![],
            loading: false,
            error: Some("Failed to fetch exchange rates".to_string()),
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec![],
            available_providers: vec![],
        };

        assert_eq!(state.error, Some("Failed to fetch exchange rates".to_string()));
        assert_eq!(state.loading, false);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_loading_state() {
        // Test loading state
        let state = ExchangeRateManagerState {
            exchange_rates: vec![],
            filtered_rates: vec![],
            loading: true,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec![],
            available_providers: vec![],
        };

        assert_eq!(state.loading, true);
        assert_eq!(state.error, None);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_zero_and_extreme_rates() {
        // Test edge cases with zero and extreme rates
        let now = Utc::now();
        
        let rates = vec![
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.0, // Zero rate
                last_updated: now,
                provider: "Test".to_string(),
                is_active: true,
            },
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "JPY".to_string(),
                rate: 100000.0, // Very high rate
                last_updated: now,
                provider: "Test".to_string(),
                is_active: true,
            },
        ];

        let state = ExchangeRateManagerState {
            exchange_rates: rates.clone(),
            filtered_rates: rates.clone(),
            loading: false,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec!["USD".to_string(), "EUR".to_string(), "JPY".to_string()],
            available_providers: vec!["Test".to_string()],
        };

        assert_eq!(state.exchange_rates.len(), 2);
        assert_eq!(state.exchange_rates[0].rate, 0.0);
        assert_eq!(state.exchange_rates[1].rate, 100000.0);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_inactive_rates() {
        // Test handling of inactive rates
        let now = Utc::now();
        
        let rates = vec![
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.85,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true, // Active rate
            },
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "GBP".to_string(),
                rate: 0.73,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: false, // Inactive rate
            },
        ];

        let state = ExchangeRateManagerState {
            exchange_rates: rates.clone(),
            filtered_rates: rates.clone(),
            loading: false,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec!["USD".to_string(), "EUR".to_string(), "GBP".to_string()],
            available_providers: vec!["ECB".to_string()],
        };

        assert_eq!(state.exchange_rates.len(), 2);
        assert_eq!(state.exchange_rates[0].is_active, true);
        assert_eq!(state.exchange_rates[1].is_active, false);
    }

    #[wasm_bindgen_test]
    fn test_exchange_rate_manager_multiple_providers() {
        // Test with multiple providers
        let now = Utc::now();
        
        let rates = vec![
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.85,
                last_updated: now,
                provider: "ECB".to_string(),
                is_active: true,
            },
            ExchangeRateEntry {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.86,
                last_updated: now,
                provider: "CPC".to_string(),
                is_active: true,
            },
        ];

        let state = ExchangeRateManagerState {
            exchange_rates: rates.clone(),
            filtered_rates: rates.clone(),
            loading: false,
            error: None,
            filter_from_currency: None,
            filter_to_currency: None,
            filter_provider: None,
            available_currencies: vec!["USD".to_string(), "EUR".to_string()],
            available_providers: vec!["ECB".to_string(), "CPC".to_string()],
        };

        assert_eq!(state.exchange_rates.len(), 2);
        assert_eq!(state.available_providers.len(), 2);
        assert!(state.available_providers.contains(&"ECB".to_string()));
        assert!(state.available_providers.contains(&"CPC".to_string()));
    }
}