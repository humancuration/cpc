//! Monetary amount representation with precise decimal arithmetic
//!
//! This module provides the `MonetaryAmount` struct for handling monetary values
//! with exact decimal precision, avoiding floating-point errors common in financial calculations.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::fmt;
use std::str::FromStr;
use crate::currency::CurrencyCode;
use crate::rounding::{round_with_strategy, RoundingStrategy};

/// Error types for monetary operations
#[derive(Debug, thiserror::Error)]
pub enum MonetaryError {
    #[error("Currency mismatch: {0} vs {1}")]
    CurrencyMismatch(CurrencyCode, CurrencyCode),
    
    #[error("Invalid decimal value: {0}")]
    InvalidDecimal(String),
    
    #[error("Overflow in monetary calculation")]
    Overflow,
    
    #[error("Division by zero")]
    DivisionByZero,
}

/// Represents a monetary amount with precise decimal arithmetic
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MonetaryAmount {
    /// The monetary value as a precise decimal
    value: Decimal,
    
    /// The currency of this amount
    currency: CurrencyCode,
    
    /// The precision (decimal places) for this currency
    precision: u32,
}

impl MonetaryAmount {
    /// Create a new monetary amount
    /// 
    /// # Arguments
    /// * `value` - The monetary value
    /// * `currency` - The currency code
    /// 
    /// # Returns
    /// A new `MonetaryAmount` with the specified value and currency
    /// 
    /// # Examples
    /// ```
    /// use cpc_financial_core::monetary::MonetaryAmount;
    /// use cpc_financial_core::currency::CurrencyCode;
    /// use rust_decimal_macros::dec;
    /// 
    /// let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
    /// ```
    pub fn new(value: impl Into<Decimal>, currency: CurrencyCode) -> Self {
        let precision = currency.decimal_places();
        let value = value.into().round_dp(precision);
        Self { value, currency, precision }
    }
    
    /// Get the value of this monetary amount
    pub fn value(&self) -> Decimal {
        self.value
    }
    
    /// Get the currency of this monetary amount
    pub fn currency(&self) -> CurrencyCode {
        self.currency
    }
    
    /// Get the precision of this monetary amount
    pub fn precision(&self) -> u32 {
        self.precision
    }
    
    /// Add another monetary amount to this one
    /// 
    /// # Arguments
    /// * `other` - The amount to add
    /// 
    /// # Returns
    /// A new `MonetaryAmount` representing the sum, or an error if currencies don't match
    pub fn add(&self, other: &Self) -> Result<Self, MonetaryError> {
        if self.currency != other.currency {
            return Err(MonetaryError::CurrencyMismatch(self.currency, other.currency));
        }
        
        let result = self.value.checked_add(other.value)
            .ok_or(MonetaryError::Overflow)?;
        
        Ok(Self::new(result, self.currency))
    }
    
    /// Subtract another monetary amount from this one
    /// 
    /// # Arguments
    /// * `other` - The amount to subtract
    /// 
    /// # Returns
    /// A new `MonetaryAmount` representing the difference, or an error if currencies don't match
    pub fn subtract(&self, other: &Self) -> Result<Self, MonetaryError> {
        if self.currency != other.currency {
            return Err(MonetaryError::CurrencyMismatch(self.currency, other.currency));
        }
        
        let result = self.value.checked_sub(other.value)
            .ok_or(MonetaryError::Overflow)?;
        
        Ok(Self::new(result, self.currency))
    }
    
    /// Multiply this monetary amount by a decimal factor
    /// 
    /// # Arguments
    /// * `factor` - The factor to multiply by
    /// 
    /// # Returns
    /// A new `MonetaryAmount` representing the product
    pub fn multiply(&self, factor: Decimal) -> Result<Self, MonetaryError> {
        let result = self.value.checked_mul(factor)
            .ok_or(MonetaryError::Overflow)?;
        
        Ok(Self::new(result, self.currency))
    }
    
    /// Divide this monetary amount by a decimal divisor
    /// 
    /// # Arguments
    /// * `divisor` - The divisor to divide by
    /// 
    /// # Returns
    /// A new `MonetaryAmount` representing the quotient, or an error for division by zero
    pub fn divide(&self, divisor: Decimal) -> Result<Self, MonetaryError> {
        if divisor.is_zero() {
            return Err(MonetaryError::DivisionByZero);
        }
        
        let result = self.value.checked_div(divisor)
            .ok_or(MonetaryError::Overflow)?;
        
        Ok(Self::new(result, self.currency))
    }
    
    /// Round this monetary amount using the specified strategy
    /// 
    /// # Arguments
    /// * `strategy` - The rounding strategy to use
    /// 
    /// # Returns
    /// A new `MonetaryAmount` with the rounded value
    pub fn round(&self, strategy: RoundingStrategy) -> Self {
        let rounded_value = round_with_strategy(self.value, self.precision, strategy);
        Self {
            value: rounded_value,
            currency: self.currency,
            precision: self.precision,
        }
    }
    
    /// Convert this monetary amount to a different currency
    /// 
    /// Note: This is a placeholder implementation. In a real system, this would
    /// require exchange rates and timestamp information.
    /// 
    /// # Arguments
    /// * `target_currency` - The currency to convert to
    /// 
    /// # Returns
    /// A new `MonetaryAmount` in the target currency
    pub fn convert_to(&self, target_currency: CurrencyCode) -> Self {
        // In a real implementation, this would use exchange rates
        // For now, we'll just change the currency code
        Self {
            value: self.value,
            currency: target_currency,
            precision: target_currency.decimal_places(),
        }
    }
}

impl fmt::Display for MonetaryAmount {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.currency)
    }
}

// Conversions from primitive types with warnings
impl From<f64> for MonetaryAmount {
    /// Convert from f64 to MonetaryAmount (USD)
    /// 
    /// # Warning
    /// This conversion may lose precision due to floating-point representation.
    /// Prefer using `Decimal` directly when possible.
    fn from(value: f64) -> Self {
        let decimal = Decimal::from_f64(value)
            .unwrap_or_else(|| {
                tracing::warn!("Potential precision loss converting f64 {} to Decimal", value);
                Decimal::from_f64_retain(value).unwrap_or(dec!(0))
            });
        Self::new(decimal, CurrencyCode::USD)
    }
}

impl From<Decimal> for MonetaryAmount {
    /// Convert from Decimal to MonetaryAmount (USD)
    fn from(value: Decimal) -> Self {
        Self::new(value, CurrencyCode::USD)
    }
}

impl FromStr for MonetaryAmount {
    type Err = MonetaryError;
    
    /// Parse a monetary amount from a string
    /// 
    /// # Arguments
    /// * `s` - String in the format "123.45 USD"
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split_whitespace().collect();
        if parts.len() != 2 {
            return Err(MonetaryError::InvalidDecimal("Invalid format".to_string()));
        }
        
        let value = Decimal::from_str(parts[0])
            .map_err(|e| MonetaryError::InvalidDecimal(e.to_string()))?;
        let currency = CurrencyCode::from_str(parts[1])
            .map_err(|_| MonetaryError::InvalidDecimal("Invalid currency".to_string()))?;
        
        Ok(Self::new(value, currency))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_monetary_amount_creation() {
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        assert_eq!(amount.value(), dec!(100.50));
        assert_eq!(amount.currency(), CurrencyCode::USD);
        assert_eq!(amount.precision(), 2);
    }
    
    #[test]
    fn test_monetary_addition() {
        let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::USD);
        let result = amount1.add(&amount2).unwrap();
        assert_eq!(result.value(), dec!(150.75));
    }
    
    #[test]
    fn test_monetary_addition_currency_mismatch() {
        let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::EUR);
        let result = amount1.add(&amount2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MonetaryError::CurrencyMismatch(_, _)));
    }
    
    #[test]
    fn test_monetary_subtraction() {
        let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::USD);
        let result = amount1.subtract(&amount2).unwrap();
        assert_eq!(result.value(), dec!(50.25));
    }
    
    #[test]
    fn test_monetary_multiplication() {
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let factor = dec!(2);
        let result = amount.multiply(factor).unwrap();
        assert_eq!(result.value(), dec!(201.00));
    }
    
    #[test]
    fn test_monetary_division() {
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let divisor = dec!(2);
        let result = amount.divide(divisor).unwrap();
        assert_eq!(result.value(), dec!(50.25));
    }
    
    #[test]
    fn test_monetary_division_by_zero() {
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        let divisor = dec!(0);
        let result = amount.divide(divisor);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), MonetaryError::DivisionByZero));
    }
    
    #[test]
    fn test_monetary_rounding() {
        let amount = MonetaryAmount::new(dec!(100.567), CurrencyCode::USD);
        let rounded = amount.round(RoundingStrategy::Bankers);
        assert_eq!(rounded.value(), dec!(100.57));
    }
    
    #[test]
    fn test_from_str() {
        let amount: Result<MonetaryAmount, _> = "100.50 USD".parse();
        assert!(amount.is_ok());
        let amount = amount.unwrap();
        assert_eq!(amount.value(), dec!(100.50));
        assert_eq!(amount.currency(), CurrencyCode::USD);
    }
    
    #[test]
    fn test_display() {
        let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
        assert_eq!(format!("{}", amount), "100.50 USD");
    }
}