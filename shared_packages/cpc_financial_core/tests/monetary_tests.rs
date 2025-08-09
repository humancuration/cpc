//! Integration tests for monetary functionality

use cpc_financial_core::monetary::MonetaryAmount;
use cpc_financial_core::currency::CurrencyCode;
use cpc_financial_core::rounding::RoundingStrategy;
use rust_decimal_macros::dec;

#[test]
fn test_monetary_amount_basic_operations() {
    let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
    let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::USD);
    
    // Test addition
    let sum = amount1.add(&amount2).unwrap();
    assert_eq!(sum.value(), dec!(150.75));
    assert_eq!(sum.currency(), CurrencyCode::USD);
    
    // Test subtraction
    let difference = amount1.subtract(&amount2).unwrap();
    assert_eq!(difference.value(), dec!(50.25));
    assert_eq!(difference.currency(), CurrencyCode::USD);
    
    // Test multiplication
    let product = amount1.multiply(dec!(2)).unwrap();
    assert_eq!(product.value(), dec!(201.00));
    assert_eq!(product.currency(), CurrencyCode::USD);
    
    // Test division
    let quotient = amount1.divide(dec!(2)).unwrap();
    assert_eq!(quotient.value(), dec!(50.25));
    assert_eq!(quotient.currency(), CurrencyCode::USD);
}

#[test]
fn test_monetary_amount_rounding() {
    let amount = MonetaryAmount::new(dec!(100.567), CurrencyCode::USD);
    
    // Test Banker's rounding
    let rounded = amount.round(RoundingStrategy::Bankers);
    assert_eq!(rounded.value(), dec!(100.57));
    
    // Test ceiling rounding
    let ceiling = amount.round(RoundingStrategy::Ceiling);
    assert_eq!(ceiling.value(), dec!(100.57));
    
    // Test floor rounding
    let floor = amount.round(RoundingStrategy::Floor);
    assert_eq!(floor.value(), dec!(100.56));
}

#[test]
fn test_currency_conversion() {
    let amount = MonetaryAmount::new(dec!(100.00), CurrencyCode::USD);
    let converted = amount.convert_to(CurrencyCode::EUR);
    
    assert_eq!(converted.value(), dec!(100.00)); // Placeholder implementation
    assert_eq!(converted.currency(), CurrencyCode::EUR);
    assert_eq!(converted.precision(), 2);
}

#[test]
fn test_monetary_amount_currency_mismatch() {
    let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
    let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::EUR);
    
    // Adding different currencies should fail
    let result = amount1.add(&amount2);
    assert!(result.is_err());
    
    // Subtracting different currencies should fail
    let result = amount1.subtract(&amount2);
    assert!(result.is_err());
}