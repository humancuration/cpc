//! Currency API service
//!
//! This module provides a client for communicating with the currency service
//! backend API. It handles HTTP requests and responses for currency operations.

use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, Response};
use serde::{Deserialize, Serialize};
use serde_json;
use packages_domains_finance::domain::currency::{Currency, CurrencyCode};
use rust_decimal::Decimal;
use std::collections::HashMap;
use chrono;

/// Currency API service
pub struct CurrencyApiService;

/// Error type for currency API operations
#[derive(Debug, Clone)]
pub enum CurrencyApiError {
    /// Network error
    NetworkError(String),
    
    /// Parse error
    ParseError(String),
    
    /// API error
    ApiError(String),
}

impl std::fmt::Display for CurrencyApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CurrencyApiError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            CurrencyApiError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CurrencyApiError::ApiError(msg) => write!(f, "API error: {}", msg),
        }
    }
}

impl std::error::Error for CurrencyApiError {}

/// Response for getting all currencies
#[derive(Debug, Deserialize)]
struct GetAllCurrenciesResponse {
    currencies: Vec<CurrencyResponse>,
}

/// Response for a single currency
#[derive(Debug, Deserialize, Clone)]
struct CurrencyResponse {
    code: String,
    name: String,
    symbol: String,
    decimal_places: u32,
    is_dabloon: bool,
}

impl From<CurrencyResponse> for Currency {
    fn from(response: CurrencyResponse) -> Self {
        // In a real implementation, we would convert from the response format
        // to the domain model. For now, we'll create a mock conversion.
        Currency::traditional(
            CurrencyCode::new(&response.code),
            response.name,
            response.symbol,
            response.decimal_places,
        )
    }
}

/// Response for currency conversion
#[derive(Debug, Deserialize)]
struct ConvertCurrencyResponse {
    converted_amount: String,
}

impl CurrencyApiService {
    /// Get all supported currencies
    pub async fn get_all_currencies() -> Result<Vec<Currency>, CurrencyApiError> {
        // In a real implementation, this would make an HTTP request to the backend
        // For now, we'll return mock data
        let mock_currencies = vec![
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
                CurrencyCode::new("GBP"),
                "British Pound".to_string(),
                "£".to_string(),
                2,
            ),
            Currency::traditional(
                CurrencyCode::new("JPY"),
                "Japanese Yen".to_string(),
                "¥".to_string(),
                0,
            ),
            Currency::traditional(
                CurrencyCode::new("CAD"),
                "Canadian Dollar".to_string(),
                "CA$".to_string(),
                2,
            ),
            Currency::dabloon(),
        ];
        
        Ok(mock_currencies)
        
        /*
        // Real implementation would look like this:
        let mut opts = RequestInit::new();
        opts.method("GET");
        
        let request = Request::new_with_str_and_init("/api/currencies", &opts)
            .map_err(|e| CurrencyApiError::NetworkError(format!("Failed to create request: {:?}", e)))?;
        
        let window = web_sys::window().ok_or(CurrencyApiError::NetworkError("No window object".to_string()))?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| CurrencyApiError::NetworkError(format!("Failed to fetch: {:?}", e)))?;
        
        let resp: Response = resp_value.dyn_into().map_err(|_| CurrencyApiError::ParseError("Failed to parse response".to_string()))?;
        
        let text = JsFuture::from(resp.text()?)
            .await
            .map_err(|e| CurrencyApiError::ParseError(format!("Failed to get response text: {:?}", e)))?
            .as_string()
            .ok_or(CurrencyApiError::ParseError("Failed to convert response to string".to_string()))?;
        
        let response: GetAllCurrenciesResponse = serde_json::from_str(&text)
            .map_err(|e| CurrencyApiError::ParseError(format!("Failed to parse JSON: {:?}", e)))?;
        
        let currencies: Vec<Currency> = response.currencies.into_iter().map(Currency::from).collect();
        Ok(currencies)
        */
    }
    
    /// Convert currency from one type to another
    pub async fn convert_currency(
        amount: Decimal,
        from: &str,
        to: &str,
    ) -> Result<Decimal, CurrencyApiError> {
        // In a real implementation, this would make an HTTP request to the backend
        // For now, we'll return a mock conversion
        let rate = match (from, to) {
            ("USD", "EUR") => Decimal::new(85, 2), // 0.85
            ("USD", "GBP") => Decimal::new(73, 2), // 0.73
            ("EUR", "USD") => Decimal::new(118, 2), // 1.18
            ("GBP", "USD") => Decimal::new(137, 2), // 1.37
            ("USD", "USD") => Decimal::new(100, 2), // 1.00
            ("EUR", "EUR") => Decimal::new(100, 2), // 1.00
            ("GBP", "GBP") => Decimal::new(100, 2), // 1.00
            _ => Decimal::new(100, 2), // Default 1:1 rate
        };
        
        Ok(amount * rate / Decimal::new(100, 2))
        
        /*
        // Real implementation would look like this:
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.headers(&serde_json::json!({
            "Content-Type": "application/json"
        }).to_string());
        
        let body = serde_json::json!({
            "amount": amount.to_string(),
            "from_currency": from,
            "to_currency": to
        });
        
        opts.body(Some(&body.to_string().into()));
        
        let request = Request::new_with_str_and_init("/api/currencies/convert", &opts)
            .map_err(|e| CurrencyApiError::NetworkError(format!("Failed to create request: {:?}", e)))?;
        
        let window = web_sys::window().ok_or(CurrencyApiError::NetworkError("No window object".to_string()))?;
        let resp_value = JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| CurrencyApiError::NetworkError(format!("Failed to fetch: {:?}", e)))?;
        
        let resp: Response = resp_value.dyn_into().map_err(|_| CurrencyApiError::ParseError("Failed to parse response".to_string()))?;
        
        let text = JsFuture::from(resp.text()?)
            .await
            .map_err(|e| CurrencyApiError::ParseError(format!("Failed to get response text: {:?}", e)))?
            .as_string()
            .ok_or(CurrencyApiError::ParseError("Failed to convert response to string".to_string()))?;
        
        let response: ConvertCurrencyResponse = serde_json::from_str(&text)
            .map_err(|e| CurrencyApiError::ParseError(format!("Failed to parse JSON: {:?}", e)))?;
        
        let converted_amount = Decimal::from_str(&response.converted_amount)
            .map_err(|e| CurrencyApiError::ParseError(format!("Failed to parse converted amount: {:?}", e)))?;
        
        Ok(converted_amount)
        */
    }
    
    /// Format currency according to locale and preferences
    pub async fn format_currency(
        amount: Decimal,
        currency_code: &str,
        locale: &str,
        show_symbols: bool,
    ) -> Result<String, CurrencyApiError> {
        // In a real implementation, this would make an HTTP request to the backend
        // For now, we'll return a mock formatted string
        let currency: Currency = CurrencyCode::new(currency_code).into();
        
        if show_symbols {
            match locale {
                "de-DE" => Ok(format!("{} {:.2}", currency.symbol, amount)),
                "fr-FR" => Ok(format!("{} {:.2}", currency.symbol, amount)),
                "ja-JP" => {
                    if currency.decimal_places == 0 {
                        Ok(format!("{} {}", currency.symbol, amount))
                    } else {
                        Ok(format!("{} {:.2}", currency.symbol, amount))
                    }
                },
                _ => Ok(format!("{}{:.2}", currency.symbol, amount)),
            }
        } else {
            match locale {
                "de-DE" => Ok(format!("{} {:.2}", currency_code, amount)),
                "fr-FR" => Ok(format!("{} {:.2}", currency_code, amount)),
                "ja-JP" => {
                    if currency.decimal_places == 0 {
                        Ok(format!("{} {}", currency_code, amount))
                    } else {
                        Ok(format!("{} {:.2}", currency_code, amount))
                    }
                },
                _ => Ok(format!("{}{:.2}", currency_code, amount)),
            }
        }
    }
    
    /// Get exchange rates
    pub async fn get_exchange_rates() -> Result<Vec<ExchangeRateInfo>, CurrencyApiError> {
        // In a real implementation, this would make an HTTP request to the backend
        // For now, we'll return mock data
        let rates = vec![
            ExchangeRateInfo {
                from_currency: "USD".to_string(),
                to_currency: "EUR".to_string(),
                rate: 0.85,
                last_updated: chrono::Utc::now(),
                provider: "ECB".to_string(),
                is_active: true,
            },
            ExchangeRateInfo {
                from_currency: "USD".to_string(),
                to_currency: "GBP".to_string(),
                rate: 0.73,
                last_updated: chrono::Utc::now(),
                provider: "ECB".to_string(),
                is_active: true,
            },
        ];
        
        Ok(rates)
    }
    
    /// Override exchange rate
    pub async fn override_exchange_rate(
        from_currency: &str,
        to_currency: &str,
        rate: f64,
    ) -> Result<(), CurrencyApiError> {
        // In a real implementation, this would make an HTTP request to the backend
        // For now, we'll just return Ok
        Ok(())
    }
}

/// Exchange rate information
#[derive(Debug, Clone)]
pub struct ExchangeRateInfo {
    pub from_currency: String,
    pub to_currency: String,
    pub rate: f64,
    pub last_updated: chrono::DateTime<chrono::Utc>,
    pub provider: String,
    pub is_active: bool,
}