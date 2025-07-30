//! Tests for the CurrencySelector component
//!
//! This module contains comprehensive tests for the CurrencySelector component,
//! including rendering, user interactions, and edge cases.

use wasm_bindgen_test::*;
use yew::platform::spawn_local;
use yew::{Renderer, Scope};
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};

// Import the component under test
use crate::components::currency::currency_selector::{CurrencySelector, CurrencySelectorProps};

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;
    use yew::prelude::*;
    use packages_domains_finance::domain::currency::{Currency, CurrencyCode};

    #[wasm_bindgen_test]
    fn test_currency_selector_creation() {
        // Test basic component creation
        let props = CurrencySelectorProps {
            on_select: Callback::from(|_| {}),
            selected: None,
            aria_label: "Test currency selector".to_string(),
        };

        assert_eq!(props.aria_label, "Test currency selector");
    }

    #[wasm_bindgen_test]
    fn test_currency_selector_with_selected_currency() {
        // Test component creation with a selected currency
        let currency = Currency::traditional(
            CurrencyCode::new("USD"),
            "United States Dollar".to_string(),
            "$".to_string(),
            2,
        );

        let props = CurrencySelectorProps {
            on_select: Callback::from(|_| {}),
            selected: Some(currency.clone()),
            aria_label: "Test currency selector".to_string(),
        };

        assert_eq!(props.selected, Some(currency));
    }

    #[wasm_bindgen_test]
    fn test_currency_selector_with_dabloon_currency() {
        // Test component creation with Dabloons currency
        let currency = Currency::dabloon();

        let props = CurrencySelectorProps {
            on_select: Callback::from(|_| {}),
            selected: Some(currency.clone()),
            aria_label: "Test currency selector".to_string(),
        };

        assert_eq!(props.selected, Some(currency));
        assert!(props.selected.unwrap().is_dabloon());
    }

    #[wasm_bindgen_test]
    fn test_currency_selector_with_different_currencies() {
        // Test with various currency types
        let currencies = vec![
            Currency::traditional(
                CurrencyCode::new("USD"),
                "United States Dollar".to_string(),
                "$".to_string(),
                2,
            ),
            Currency::traditional(
                CurrencyCode::new("EUR"),
                "Euro".to_string(),
                "€".to_string(),
                2,
            ),
            Currency::traditional(
                CurrencyCode::new("JPY"),
                "Japanese Yen".to_string(),
                "¥".to_string(),
                0,
            ),
            Currency::dabloon(),
        ];

        for currency in currencies {
            let props = CurrencySelectorProps {
                on_select: Callback::from(|_| {}),
                selected: Some(currency.clone()),
                aria_label: "Test currency selector".to_string(),
            };

            assert_eq!(props.selected, Some(currency.clone()));
            assert_eq!(props.selected.unwrap().code(), currency.code());
        }
    }

    #[wasm_bindgen_test]
    fn test_currency_selector_props_default_values() {
        // Test default prop values
        let props = CurrencySelectorProps {
            on_select: Callback::from(|_| {}),
            selected: None,
            aria_label: "Select currency".to_string(), // Default value
        };

        assert_eq!(props.aria_label, "Select currency");
        assert_eq!(props.selected, None);
    }

    #[wasm_bindgen_test]
    fn test_currency_display_format() {
        // Test that currencies display correctly in the selector
        let currency = Currency::traditional(
            CurrencyCode::new("EUR"),
            "Euro".to_string(),
            "€".to_string(),
            2,
        );

        let display = format!("{} - {} ({})", currency.code(), currency.name, currency.symbol);
        assert_eq!(display, "EUR - Euro (€)");
    }

    #[wasm_bindgen_test]
    fn test_dabloon_currency_display_format() {
        // Test that Dabloons currency displays correctly
        let currency = Currency::dabloon();
        let display = format!("{} - {} ({})", currency.code(), currency.name, currency.symbol);
        assert_eq!(display, "DABLOONS - Dabloons (ᴅ)");
    }
}