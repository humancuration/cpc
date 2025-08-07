//! Currency codes and related functionality
//!
//! This module provides currency codes and helper functions for working with different currencies.

use std::fmt;
use std::str::FromStr;

/// Currency codes supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CurrencyCode {
    /// US Dollar
    USD,
    
    /// Euro
    EUR,
    
    /// British Pound
    GBP,
    
    /// Japanese Yen
    JPY,
    
    /// Canadian Dollar
    CAD,
    
    /// Australian Dollar
    AUD,
    
    /// Swiss Franc
    CHF,
    
    /// Chinese Yuan
    CNY,
    
    /// Internal CPC currency (Dabloons)
    DBL,
}

impl CurrencyCode {
    /// Get the number of decimal places for this currency
    /// 
    /// # Returns
    /// The number of decimal places typically used for this currency
    pub fn decimal_places(&self) -> u32 {
        match self {
            CurrencyCode::USD => 2,
            CurrencyCode::EUR => 2,
            CurrencyCode::GBP => 2,
            CurrencyCode::JPY => 0,  // Yen has no decimal places
            CurrencyCode::CAD => 2,
            CurrencyCode::AUD => 2,
            CurrencyCode::CHF => 2,
            CurrencyCode::CNY => 2,
            CurrencyCode::DBL => 2,  // Dabloons use 2 decimal places
        }
    }
    
    /// Get the currency code as a string
    /// 
    /// # Returns
    /// The ISO currency code as a string
    pub fn code(&self) -> &'static str {
        match self {
            CurrencyCode::USD => "USD",
            CurrencyCode::EUR => "EUR",
            CurrencyCode::GBP => "GBP",
            CurrencyCode::JPY => "JPY",
            CurrencyCode::CAD => "CAD",
            CurrencyCode::AUD => "AUD",
            CurrencyCode::CHF => "CHF",
            CurrencyCode::CNY => "CNY",
            CurrencyCode::DBL => "DBL",
        }
    }
}

impl fmt::Display for CurrencyCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.code())
    }
}

impl FromStr for CurrencyCode {
    type Err = String;
    
    /// Parse a currency code from a string
    /// 
    /// # Arguments
    /// * `s` - The currency code string (e.g., "USD")
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "USD" => Ok(CurrencyCode::USD),
            "EUR" => Ok(CurrencyCode::EUR),
            "GBP" => Ok(CurrencyCode::GBP),
            "JPY" => Ok(CurrencyCode::JPY),
            "CAD" => Ok(CurrencyCode::CAD),
            "AUD" => Ok(CurrencyCode::AUD),
            "CHF" => Ok(CurrencyCode::CHF),
            "CNY" => Ok(CurrencyCode::CNY),
            "DBL" => Ok(CurrencyCode::DBL),
            _ => Err(format!("Unknown currency code: {}", s)),
        }
    }
}

/// Helper function to get exchange rate between two currencies
/// 
/// Note: This is a placeholder implementation. In a real system, this would
/// fetch current exchange rates from a reliable source.
/// 
/// # Arguments
/// * `from` - The source currency
/// * `to` - The target currency
/// 
/// # Returns
/// A decimal representing the exchange rate (how many units of 'to' currency
/// you get for 1 unit of 'from' currency)
pub fn get_exchange_rate(from: CurrencyCode, to: CurrencyCode) -> rust_decimal::Decimal {
    // In a real implementation, this would fetch from an exchange rate API
    // For now, we'll return 1.0 for the same currency and placeholder values for others
    if from == to {
        rust_decimal_macros::dec!(1.0)
    } else {
        // Placeholder exchange rates - these would come from a real source
        match (from, to) {
            (CurrencyCode::USD, CurrencyCode::EUR) => rust_decimal_macros::dec!(0.85),
            (CurrencyCode::EUR, CurrencyCode::USD) => rust_decimal_macros::dec!(1.18),
            (CurrencyCode::USD, CurrencyCode::GBP) => rust_decimal_macros::dec!(0.73),
            (CurrencyCode::GBP, CurrencyCode::USD) => rust_decimal_macros::dec!(1.37),
            (CurrencyCode::USD, CurrencyCode::JPY) => rust_decimal_macros::dec!(110.0),
            (CurrencyCode::JPY, CurrencyCode::USD) => rust_decimal_macros::dec!(0.0091),
            _ => rust_decimal_macros::dec!(1.0), // Default fallback
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(CurrencyCode::USD.decimal_places(), 2);
        assert_eq!(CurrencyCode::EUR.decimal_places(), 2);
        assert_eq!(CurrencyCode::GBP.decimal_places(), 2);
        assert_eq!(CurrencyCode::JPY.decimal_places(), 0);
        assert_eq!(CurrencyCode::DBL.decimal_places(), 2);
    }
    
    #[test]
    fn test_currency_code() {
        assert_eq!(CurrencyCode::USD.code(), "USD");
        assert_eq!(CurrencyCode::EUR.code(), "EUR");
        assert_eq!(CurrencyCode::GBP.code(), "GBP");
        assert_eq!(CurrencyCode::JPY.code(), "JPY");
        assert_eq!(CurrencyCode::DBL.code(), "DBL");
    }
    
    #[test]
    fn test_currency_display() {
        assert_eq!(format!("{}", CurrencyCode::USD), "USD");
        assert_eq!(format!("{}", CurrencyCode::EUR), "EUR");
        assert_eq!(format!("{}", CurrencyCode::GBP), "GBP");
        assert_eq!(format!("{}", CurrencyCode::JPY), "JPY");
        assert_eq!(format!("{}", CurrencyCode::DBL), "DBL");
    }
    
    #[test]
    fn test_currency_from_str() {
        assert_eq!(CurrencyCode::from_str("USD").unwrap(), CurrencyCode::USD);
        assert_eq!(CurrencyCode::from_str("EUR").unwrap(), CurrencyCode::EUR);
        assert_eq!(CurrencyCode::from_str("GBP").unwrap(), CurrencyCode::GBP);
        assert_eq!(CurrencyCode::from_str("JPY").unwrap(), CurrencyCode::JPY);
        assert_eq!(CurrencyCode::from_str("DBL").unwrap(), CurrencyCode::DBL);
        assert!(CurrencyCode::from_str("XYZ").is_err());
    }
    
    #[test]
    fn test_exchange_rate() {
        assert_eq!(get_exchange_rate(CurrencyCode::USD, CurrencyCode::USD), dec!(1.0));
        assert_eq!(get_exchange_rate(CurrencyCode::USD, CurrencyCode::EUR), dec!(0.85));
        assert_eq!(get_exchange_rate(CurrencyCode::EUR, CurrencyCode::USD), dec!(1.18));
    }
}