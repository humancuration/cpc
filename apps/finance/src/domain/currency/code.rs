//! ISO 4217 currency codes
//!
//! This module defines the `CurrencyCode` type which represents standardized
//! currency codes according to the ISO 4217 standard.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// Represents an ISO 4217 currency code
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CurrencyCode(String);

impl CurrencyCode {
    /// Create a new currency code
    pub fn new(code: impl Into<String>) -> Self {
        Self(code.into().to_uppercase())
    }

    /// Get the currency code as a string slice
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Get the number of decimal places for this currency
    pub fn decimal_places(&self) -> u32 {
        match self.as_str() {
            // Zero decimal currencies
            "JPY" | "KRW" | "VND" | "CLP" | "PYG" | "UYU" | "RWF" | "GNF" | "UGX" | "VUV" 
            | "XAF" | "XOF" | "XPF" => 0,
            // Three decimal currencies
            "BHD" | "IQD" | "JOD" | "KWD" | "LYD" | "OMR" | "TND" => 3,
            // Default to 2 decimal places
            _ => 2,
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for CurrencyCode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CurrencyCode::new(s))
    }
}

impl From<&str> for CurrencyCode {
    fn from(s: &str) -> Self {
        CurrencyCode::new(s)
    }
}

impl From<String> for CurrencyCode {
    fn from(s: String) -> Self {
        CurrencyCode::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_currency_code_creation() {
        let code = CurrencyCode::new("USD");
        assert_eq!(code.as_str(), "USD");
    }

    #[test]
    fn test_currency_code_case_insensitive() {
        let code = CurrencyCode::new("usd");
        assert_eq!(code.as_str(), "USD");
    }

    #[test]
    fn test_currency_code_decimal_places() {
        assert_eq!(CurrencyCode::new("USD").decimal_places(), 2);
        assert_eq!(CurrencyCode::new("JPY").decimal_places(), 0);
        assert_eq!(CurrencyCode::new("BHD").decimal_places(), 3);
    }

    #[test]
    fn test_currency_code_display() {
        let code = CurrencyCode::new("EUR");
        assert_eq!(format!("{}", code), "EUR");
    }

    #[test]
    fn test_currency_code_from_str() {
        let code: CurrencyCode = "GBP".parse().unwrap();
        assert_eq!(code.as_str(), "GBP");
    }
}