//! Integration tests for currency conversion functionality

use cpc_financial_core::currency::{CurrencyCode, get_exchange_rate};
use cpc_financial_core::monetary::MonetaryAmount;
use rust_decimal_macros::dec;

#[test]
fn test_exchange_rate_retrieval() {
    // Test same currency conversion
    let rate = get_exchange_rate(CurrencyCode::USD, CurrencyCode::USD);
    assert_eq!(rate, dec!(1.0));
    
    // Test USD to EUR conversion
    let rate = get_exchange_rate(CurrencyCode::USD, CurrencyCode::EUR);
    assert_eq!(rate, dec!(0.85));
    
    // Test EUR to USD conversion
    let rate = get_exchange_rate(CurrencyCode::EUR, CurrencyCode::USD);
    assert_eq!(rate, dec!(1.18));
    
    // Test USD to GBP conversion
    let rate = get_exchange_rate(CurrencyCode::USD, CurrencyCode::GBP);
    assert_eq!(rate, dec!(0.73));
    
    // Test GBP to USD conversion
    let rate = get_exchange_rate(CurrencyCode::GBP, CurrencyCode::USD);
    assert_eq!(rate, dec!(1.37));
}

#[test]
fn test_currency_decimal_places() {
    assert_eq!(CurrencyCode::USD.decimal_places(), 2);
    assert_eq!(CurrencyCode::EUR.decimal_places(), 2);
    assert_eq!(CurrencyCode::GBP.decimal_places(), 2);
    assert_eq!(CurrencyCode::JPY.decimal_places(), 0);
    assert_eq!(CurrencyCode::CAD.decimal_places(), 2);
    assert_eq!(CurrencyCode::AUD.decimal_places(), 2);
    assert_eq!(CurrencyCode::CHF.decimal_places(), 2);
    assert_eq!(CurrencyCode::CNY.decimal_places(), 2);
    assert_eq!(CurrencyCode::DBL.decimal_places(), 2);
}

#[test]
fn test_currency_code_parsing() {
    assert_eq!(CurrencyCode::from_str("USD").unwrap(), CurrencyCode::USD);
    assert_eq!(CurrencyCode::from_str("EUR").unwrap(), CurrencyCode::EUR);
    assert_eq!(CurrencyCode::from_str("GBP").unwrap(), CurrencyCode::GBP);
    assert_eq!(CurrencyCode::from_str("JPY").unwrap(), CurrencyCode::JPY);
    assert_eq!(CurrencyCode::from_str("DBL").unwrap(), CurrencyCode::DBL);
    
    // Test invalid currency code
    assert!(CurrencyCode::from_str("XYZ").is_err());
}

#[test]
fn test_currency_display() {
    assert_eq!(format!("{}", CurrencyCode::USD), "USD");
    assert_eq!(format!("{}", CurrencyCode::EUR), "EUR");
    assert_eq!(format!("{}", CurrencyCode::GBP), "GBP");
    assert_eq!(format!("{}", CurrencyCode::JPY), "JPY");
    assert_eq!(format!("{}", CurrencyCode::DBL), "DBL");
}