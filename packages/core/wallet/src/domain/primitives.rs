//! Financial primitives for the CPC platform
//!
//! This module provides basic financial types that can be used across the platform:
//! - Money: A monetary amount with currency
//! - Currency: A currency with all its properties
//! - Amount: A decimal amount without currency

use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use thiserror::Error;

/// Backwards compatibility type for the old currency enum
/// This will be deprecated in favor of the full Currency struct
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Currency {
    /// United States Dollar
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
    /// Swedish Krona
    SEK,
    /// New Zealand Dollar
    NZD,
    /// Mexican Peso
    MXN,
    /// Singapore Dollar
    SGD,
    /// Hong Kong Dollar
    HKD,
    /// Norwegian Krone
    NOK,
    /// South Korean Won
    KRW,
    /// Turkish Lira
    TRY,
    /// Russian Ruble
    RUB,
    /// Indian Rupee
    INR,
    /// Brazilian Real
    BRL,
    /// South African Rand
    ZAR,
    /// Internal currency for the CPC platform
    Dabloons,
}

impl Currency {
    /// Get the ISO 4217 currency code
    pub fn code(&self) -> &'static str {
        match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CHF => "CHF",
            Currency::CNY => "CNY",
            Currency::SEK => "SEK",
            Currency::NZD => "NZD",
            Currency::MXN => "MXN",
            Currency::SGD => "SGD",
            Currency::HKD => "HKD",
            Currency::NOK => "NOK",
            Currency::KRW => "KRW",
            Currency::TRY => "TRY",
            Currency::RUB => "RUB",
            Currency::INR => "INR",
            Currency::BRL => "BRL",
            Currency::ZAR => "ZAR",
            Currency::Dabloons => "DABLOONS",
        }
    }

    /// Get the number of decimal places for this currency
    pub fn decimal_places(&self) -> u32 {
        match self {
            Currency::JPY | Currency::KRW | Currency::Dabloons => 0,
            _ => 2,
        }
    }
    
    /// Check if the currency is Dabloons
    pub fn is_dabloon(&self) -> bool {
        matches!(self, Currency::Dabloons)
    }
    
    /// Check if the currency is a traditional currency (not Dabloons)
    pub fn is_traditional(&self) -> bool {
        !self.is_dabloon()
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.code())
    }
}

/// A monetary amount with currency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

impl Money {
    /// Create a new Money instance
    pub fn new(amount: Decimal, currency: Currency) -> Self {
        Self { amount, currency }
    }

    /// Create a zero amount in the specified currency
    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: Decimal::ZERO,
            currency,
        }
    }

    /// Check if the amount is zero
    pub fn is_zero(&self) -> bool {
        self.amount.is_zero()
    }

    /// Check if the amount is positive
    pub fn is_positive(&self) -> bool {
        self.amount.is_sign_positive() && !self.amount.is_zero()
    }

    /// Check if the amount is negative
    pub fn is_negative(&self) -> bool {
        self.amount.is_sign_negative()
    }

    /// Add two Money amounts
    /// Returns an error if the currencies don't match
    pub fn add(&self, other: &Self) -> Result<Self, FinancialError> {
        if self.currency != other.currency {
            return Err(FinancialError::CurrencyMismatch {
                expected: self.currency.code().to_string(),
                actual: other.currency.code().to_string(),
            });
        }
        
        Ok(Self {
            amount: self.amount + other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Subtract two Money amounts
    /// Returns an error if the currencies don't match
    pub fn subtract(&self, other: &Self) -> Result<Self, FinancialError> {
        if self.currency != other.currency {
            return Err(FinancialError::CurrencyMismatch {
                expected: self.currency.code().to_string(),
                actual: other.currency.code().to_string(),
            });
        }
        
        Ok(Self {
            amount: self.amount - other.amount,
            currency: self.currency.clone(),
        })
    }

    /// Multiply the amount by a decimal factor
    pub fn multiply(&self, factor: Decimal) -> Self {
        Self {
            amount: self.amount * factor,
            currency: self.currency.clone(),
        }
    }

    /// Divide the amount by a decimal divisor
    pub fn divide(&self, divisor: Decimal) -> Result<Self, FinancialError> {
        if divisor.is_zero() {
            return Err(FinancialError::DivisionByZero);
        }
        
        Ok(Self {
            amount: self.amount / divisor,
            currency: self.currency.clone(),
        })
    }
}

/// A decimal amount without currency
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Amount(pub Decimal);

impl Amount {
    /// Create a new Amount instance
    pub fn new(value: Decimal) -> Self {
        Self(value)
    }

    /// Create a zero amount
    pub fn zero() -> Self {
        Self(Decimal::ZERO)
    }

    /// Check if the amount is zero
    pub fn is_zero(&self) -> bool {
        self.0.is_zero()
    }

    /// Check if the amount is positive
    pub fn is_positive(&self) -> bool {
        self.0.is_sign_positive() && !self.0.is_zero()
    }

    /// Check if the amount is negative
    pub fn is_negative(&self) -> bool {
        self.0.is_sign_negative()
    }
}

/// Error types for financial operations
#[derive(Error, Debug)]
pub enum FinancialError {
    #[error("Currency mismatch - expected: {expected}, actual: {actual}")]
    CurrencyMismatch { expected: String, actual: String },
    
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Invalid amount")]
    InvalidAmount,
    
    #[error("Insufficient funds in {0}")]
    InsufficientFunds(Currency),
    
    #[error("Invalid currency")]
    InvalidCurrency,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_currency_code() {
        assert_eq!(Currency::USD.code(), "USD");
        assert_eq!(Currency::EUR.code(), "EUR");
        assert_eq!(Currency::JPY.code(), "JPY");
    }

    #[test]
    fn test_currency_decimal_places() {
        assert_eq!(Currency::USD.decimal_places(), 2);
        assert_eq!(Currency::EUR.decimal_places(), 2);
        assert_eq!(Currency::JPY.decimal_places(), 0);
        assert_eq!(Currency::KRW.decimal_places(), 0);
    }

    #[test]
    fn test_money_creation() {
        let money = Money::new(dec!(100.50), Currency::USD);
        assert_eq!(money.amount, dec!(100.50));
        assert_eq!(money.currency, Currency::USD);
    }

    #[test]
    fn test_money_zero() {
        let zero = Money::zero(Currency::EUR);
        assert!(zero.is_zero());
        assert_eq!(zero.amount, Decimal::ZERO);
        assert_eq!(zero.currency, Currency::EUR);
    }

    #[test]
    fn test_money_addition() {
        let money1 = Money::new(dec!(100.50), Currency::USD);
        let money2 = Money::new(dec!(50.25), Currency::USD);
        let result = money1.add(&money2).unwrap();
        assert_eq!(result.amount, dec!(150.75));
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn test_money_addition_currency_mismatch() {
        let money1 = Money::new(dec!(100.50), Currency::USD);
        let money2 = Money::new(dec!(50.25), Currency::EUR);
        let result = money1.add(&money2);
        assert!(result.is_err());
    }

    #[test]
    fn test_money_subtraction() {
        let money1 = Money::new(dec!(100.50), Currency::USD);
        let money2 = Money::new(dec!(50.25), Currency::USD);
        let result = money1.subtract(&money2).unwrap();
        assert_eq!(result.amount, dec!(50.25));
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn test_money_multiplication() {
        let money = Money::new(dec!(100.50), Currency::USD);
        let result = money.multiply(dec!(2));
        assert_eq!(result.amount, dec!(201.00));
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn test_money_division() {
        let money = Money::new(dec!(100.50), Currency::USD);
        let result = money.divide(dec!(2)).unwrap();
        assert_eq!(result.amount, dec!(50.25));
        assert_eq!(result.currency, Currency::USD);
    }

    #[test]
    fn test_money_division_by_zero() {
        let money = Money::new(dec!(100.50), Currency::USD);
        let result = money.divide(Decimal::ZERO);
        assert!(result.is_err());
    }

    #[test]
    fn test_amount_creation() {
        let amount = Amount::new(dec!(100.50));
        assert_eq!(amount.0, dec!(100.50));
    }

    #[test]
    fn test_amount_zero() {
        let zero = Amount::zero();
        assert!(zero.is_zero());
        assert_eq!(zero.0, Decimal::ZERO);
    }
}