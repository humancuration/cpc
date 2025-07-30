//! Currency model
//!
//! This module defines the `Currency` struct which represents a currency with
//! all its properties including code, name, symbol, and formatting information.

use serde::{Deserialize, Serialize};
use super::CurrencyCode;

/// Represents a currency with all its properties
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Currency {
    /// The ISO 4217 currency code
    pub code: CurrencyCode,
    
    /// The full name of the currency
    pub name: String,
    
    /// The currency symbol (e.g., $, €, £)
    pub symbol: String,
    
    /// The number of decimal places
    pub decimal_places: u32,
    
    /// Whether this is the platform's internal currency
    pub is_dabloon: bool,
}

impl Currency {
    /// Create a new currency
    pub fn new(
        code: CurrencyCode,
        name: String,
        symbol: String,
        decimal_places: u32,
        is_dabloon: bool,
    ) -> Self {
        Self {
            code,
            name,
            symbol,
            decimal_places,
            is_dabloon,
        }
    }

    /// Create a traditional currency (not Dabloons)
    pub fn traditional(
        code: CurrencyCode,
        name: String,
        symbol: String,
        decimal_places: u32,
    ) -> Self {
        Self::new(code, name, symbol, decimal_places, false)
    }

    /// Create the Dabloons currency
    pub fn dabloon() -> Self {
        Self::new(
            CurrencyCode::new("DABLOONS"),
            "Dabloons".to_string(),
            "ᴅ".to_string(),
            2,
            true,
        )
    }

    /// Get the ISO 4217 currency code as a string
    pub fn code(&self) -> &str {
        self.code.as_str()
    }

    /// Check if the currency is Dabloons
    pub fn is_dabloon(&self) -> bool {
        self.is_dabloon
    }

    /// Check if the currency is a traditional currency (not Dabloons)
    pub fn is_traditional(&self) -> bool {
        !self.is_dabloon
    }
}

impl From<CurrencyCode> for Currency {
    fn from(code: CurrencyCode) -> Self {
        // This is a simplified implementation - in practice, we would look up
        // the currency details from the registry
        match code.as_str() {
            "USD" => Currency::traditional(
                code,
                "United States Dollar".to_string(),
                "$".to_string(),
                2,
            ),
            "EUR" => Currency::traditional(
                code,
                "Euro".to_string(),
                "€".to_string(),
                2,
            ),
            "GBP" => Currency::traditional(
                code,
                "British Pound".to_string(),
                "£".to_string(),
                2,
            ),
            "JPY" => Currency::traditional(
                code,
                "Japanese Yen".to_string(),
                "¥".to_string(),
                0,
            ),
            "DABLOONS" => Currency::dabloon(),
            _ => Currency::traditional(
                code,
                "Unknown Currency".to_string(),
                code.as_str().to_string(),
                code.decimal_places(),
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_creation() {
        let currency = Currency::traditional(
            CurrencyCode::new("USD"),
            "United States Dollar".to_string(),
            "$".to_string(),
            2,
        );
        
        assert_eq!(currency.code(), "USD");
        assert_eq!(currency.name, "United States Dollar");
        assert_eq!(currency.symbol, "$");
        assert_eq!(currency.decimal_places, 2);
        assert!(!currency.is_dabloon());
        assert!(currency.is_traditional());
    }

    #[test]
    fn test_dabloon_currency() {
        let currency = Currency::dabloon();
        
        assert_eq!(currency.code(), "DABLOONS");
        assert_eq!(currency.name, "Dabloons");
        assert_eq!(currency.symbol, "ᴅ");
        assert_eq!(currency.decimal_places, 2);
        assert!(currency.is_dabloon());
        assert!(!currency.is_traditional());
    }

    #[test]
    fn test_currency_from_code() {
        let currency: Currency = CurrencyCode::new("USD").into();
        assert_eq!(currency.code(), "USD");
        assert_eq!(currency.symbol, "$");
        
        let currency: Currency = CurrencyCode::new("JPY").into();
        assert_eq!(currency.code(), "JPY");
        assert_eq!(currency.decimal_places, 0);
    }
}