//! High-precision financial calculations using fixed-point arithmetic
//!
//! This module provides the `MonetaryValue` type for precise financial calculations
//! using fixed-point arithmetic to avoid floating-point errors common in monetary operations.
//! It implements banker's rounding and other rounding strategies aligned with cooperative values.

use fixed::types::I9F23;
use fixed::types::I18F14; // 18 integer bits, 14 fractional bits
use fixed::types::I64F64; // 64 integer bits, 64 fractional bits for intermediate calculations
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use std::ops::{Add, Sub, Mul, Div};
use std::cmp::Ordering;

/// Error types for financial operations
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum FinancialError {
    #[error("Currency mismatch: {0} vs {1}")]
    CurrencyMismatch(String, String),
    
    #[error("Invalid monetary value: {0}")]
    InvalidValue(String),
    
    #[error("Overflow in monetary calculation")]
    Overflow,
    
    #[error("Division by zero")]
    DivisionByZero,
    
    #[error("Precision loss in conversion")]
    PrecisionLoss,
}

/// Result type for financial operations
pub type FinancialResult<T> = Result<T, FinancialError>;

/// Rounding strategies for monetary calculations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoundingStrategy {
    /// Banker's rounding (round half to even)
    Bankers,
    /// Round half up
    HalfUp,
    /// Round half down
    HalfDown,
    /// Round toward zero (truncate)
    TowardZero,
    /// Round away from zero
    AwayFromZero,
}

impl Default for RoundingStrategy {
    fn default() -> Self {
        RoundingStrategy::Bankers
    }
}

/// Represents a monetary value with high precision using fixed-point arithmetic
/// 
/// Uses I64F64 (64 integer bits, 64 fractional bits) for maximum precision
/// while providing safe conversion methods for different precision requirements.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct MonetaryValue {
    /// The monetary value as a fixed-point number (64.64 format)
    value: I64F64,
    
    /// The currency code (e.g., "USD", "EUR", "DABLOONS")
    currency: String,
}

impl MonetaryValue {
    /// Create a new monetary value
    /// 
    /// # Arguments
    /// * `value` - The monetary value as a fixed-point number
    /// * `currency` - The currency code
    /// 
    /// # Returns
    /// A new `MonetaryValue` with the specified value and currency
    pub fn new(value: impl Into<I64F64>, currency: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            currency: currency.into(),
        }
    }
    
    /// Create a monetary value from a decimal string
    /// 
    /// # Arguments
    /// * `value_str` - The monetary value as a string (e.g., "123.45")
    /// * `currency` - The currency code
    /// 
    /// # Returns
    /// A new `MonetaryValue` or an error if the string is invalid
    pub fn from_str(value_str: &str, currency: &str) -> FinancialResult<Self> {
        let value = I64F64::from_str(value_str)
            .map_err(|_| FinancialError::InvalidValue(value_str.to_string()))?;
        Ok(Self::new(value, currency))
    }
    
    /// Get the value of this monetary amount
    pub fn value(&self) -> I64F64 {
        self.value
    }
    
    /// Get the currency of this monetary amount
    pub fn currency(&self) -> &str {
        &self.currency
    }
    
    /// Add another monetary amount to this one
    /// 
    /// # Arguments
    /// * `other` - The amount to add
    /// 
    /// # Returns
    /// A new `MonetaryValue` representing the sum, or an error if currencies don't match
    pub fn add(&self, other: &Self) -> FinancialResult<Self> {
        if self.currency != other.currency {
            return Err(FinancialError::CurrencyMismatch(
                self.currency.clone(),
                other.currency.clone(),
            ));
        }
        
        let result = self.value.checked_add(other.value)
            .ok_or(FinancialError::Overflow)?;
        
        Ok(Self::new(result, self.currency.clone()))
    }
    
    /// Subtract another monetary amount from this one
    /// 
    /// # Arguments
    /// * `other` - The amount to subtract
    /// 
    /// # Returns
    /// A new `MonetaryValue` representing the difference, or an error if currencies don't match
    pub fn subtract(&self, other: &Self) -> FinancialResult<Self> {
        if self.currency != other.currency {
            return Err(FinancialError::CurrencyMismatch(
                self.currency.clone(),
                other.currency.clone(),
            ));
        }
        
        let result = self.value.checked_sub(other.value)
            .ok_or(FinancialError::Overflow)?;
        
        Ok(Self::new(result, self.currency.clone()))
    }
    
    /// Multiply this monetary amount by a factor
    /// 
    /// # Arguments
    /// * `factor` - The factor to multiply by (as a fixed-point number)
    /// 
    /// # Returns
    /// A new `MonetaryValue` representing the product
    pub fn multiply(&self, factor: I64F64) -> FinancialResult<Self> {
        let result = self.value.checked_mul(factor)
            .ok_or(FinancialError::Overflow)?;
        
        Ok(Self::new(result, self.currency.clone()))
    }
    
    /// Divide this monetary amount by a divisor
    /// 
    /// # Arguments
    /// * `divisor` - The divisor to divide by (as a fixed-point number)
    /// 
    /// # Returns
    /// A new `MonetaryValue` representing the quotient, or an error for division by zero
    pub fn divide(&self, divisor: I64F64) -> FinancialResult<Self> {
        if divisor.is_zero() {
            return Err(FinancialError::DivisionByZero);
        }
        
        let result = self.value.checked_div(divisor)
            .ok_or(FinancialError::Overflow)?;
        
        Ok(Self::new(result, self.currency.clone()))
    }
    
    /// Round this monetary amount using the specified strategy
    /// 
    /// # Arguments
    /// * `decimals` - The number of decimal places to round to
    /// * `strategy` - The rounding strategy to use
    /// 
    /// # Returns
    /// A new `MonetaryValue` with the rounded value
    pub fn round(&self, decimals: u32, strategy: RoundingStrategy) -> Self {
        let multiplier = I64F64::from(10u64.pow(decimals));
        let scaled = self.value * multiplier;
        
        let rounded = match strategy {
            RoundingStrategy::Bankers => {
                // Banker's rounding (round half to even)
                let floored = scaled.floor();
                let ceiled = scaled.ceil();
                let diff_floor = scaled - floored;
                let diff_ceil = ceiled - scaled;
                
                match diff_floor.cmp(&diff_ceil) {
                    Ordering::Less => floored,
                    Ordering::Greater => ceiled,
                    Ordering::Equal => {
                        // Halfway case - round to even
                        if floored.to_bits() & 1 == 0 {
                            floored
                        } else {
                            ceiled
                        }
                    }
                }
            },
            RoundingStrategy::HalfUp => {
                if scaled.frac() >= I64F64::from_num(0.5) {
                    scaled.ceil()
                } else {
                    scaled.floor()
                }
            },
            RoundingStrategy::HalfDown => {
                if scaled.frac() > I64F64::from_num(0.5) {
                    scaled.ceil()
                } else {
                    scaled.floor()
                }
            },
            RoundingStrategy::TowardZero => scaled.int(),
            RoundingStrategy::AwayFromZero => {
                if scaled.is_negative() {
                    scaled.floor()
                } else {
                    scaled.ceil()
                }
            },
        };
        
        let result = rounded / multiplier;
        Self::new(result, self.currency.clone())
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
    /// A new `MonetaryValue` in the target currency
    pub fn convert_to(&self, target_currency: &str) -> Self {
        // In a real implementation, this would use exchange rates
        // For now, we'll just change the currency code
        Self::new(self.value, target_currency)
    }
    
    /// Convert to a lower precision format (e.g., for display)
    /// 
    /// # Arguments
    /// * `decimals` - The number of decimal places to limit to
    /// * `strategy` - The rounding strategy to use
    /// 
    /// # Returns
    /// A new `MonetaryValue` with limited precision
    pub fn to_precision(&self, decimals: u32, strategy: RoundingStrategy) -> Self {
        self.round(decimals, strategy)
    }
    
    /// Check if the value is zero
    pub fn is_zero(&self) -> bool {
        self.value.is_zero()
    }
    
    /// Check if the value is positive
    pub fn is_positive(&self) -> bool {
        self.value.is_positive()
    }
    
    /// Check if the value is negative
    pub fn is_negative(&self) -> bool {
        self.value.is_negative()
    }
}

impl fmt::Display for MonetaryValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.value, self.currency)
    }
}

// Implement arithmetic operations
impl Add for MonetaryValue {
    type Output = FinancialResult<Self>;
    
    fn add(self, rhs: Self) -> Self::Output {
        self.add(&rhs)
    }
}

impl Sub for MonetaryValue {
    type Output = FinancialResult<Self>;
    
    fn sub(self, rhs: Self) -> Self::Output {
        self.subtract(&rhs)
    }
}

impl Mul<I64F64> for MonetaryValue {
    type Output = FinancialResult<Self>;
    
    fn mul(self, rhs: I64F64) -> Self::Output {
        self.multiply(rhs)
    }
}

impl Div<I64F64> for MonetaryValue {
    type Output = FinancialResult<Self>;
    
    fn div(self, rhs: I64F64) -> Self::Output {
        self.divide(rhs)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_monetary_value_creation() {
        let amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        assert_eq!(amount.value(), I64F64::from_num(100.50));
        assert_eq!(amount.currency(), "USD");
    }
    
    #[test]
    fn test_monetary_addition() {
        let amount1 = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        let amount2 = MonetaryValue::new(I64F64::from_num(50.25), "USD");
        let result = amount1.add(&amount2).unwrap();
        assert_eq!(result.value(), I64F64::from_num(150.75));
    }
    
    #[test]
    fn test_monetary_addition_currency_mismatch() {
        let amount1 = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        let amount2 = MonetaryValue::new(I64F64::from_num(50.25), "EUR");
        let result = amount1.add(&amount2);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FinancialError::CurrencyMismatch(_, _)));
    }
    
    #[test]
    fn test_monetary_subtraction() {
        let amount1 = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        let amount2 = MonetaryValue::new(I64F64::from_num(50.25), "USD");
        let result = amount1.subtract(&amount2).unwrap();
        assert_eq!(result.value(), I64F64::from_num(50.25));
    }
    
    #[test]
    fn test_monetary_multiplication() {
        let amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        let factor = I64F64::from_num(2);
        let result = amount.multiply(factor).unwrap();
        assert_eq!(result.value(), I64F64::from_num(201.00));
    }
    
    #[test]
    fn test_monetary_division() {
        let amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        let divisor = I64F64::from_num(2);
        let result = amount.divide(divisor).unwrap();
        assert_eq!(result.value(), I64F64::from_num(50.25));
    }
    
    #[test]
    fn test_monetary_division_by_zero() {
        let amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        let divisor = I64F64::from_num(0);
        let result = amount.divide(divisor);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), FinancialError::DivisionByZero));
    }
    
    #[test]
    fn test_bankers_rounding() {
        let amount = MonetaryValue::new(I64F64::from_num(100.565), "USD");
        let rounded = amount.round(2, RoundingStrategy::Bankers);
        // 100.565 should round to 100.56 (banker's rounding - round to even)
        assert_eq!(rounded.value(), I64F64::from_num(100.56));
        
        let amount2 = MonetaryValue::new(I64F64::from_num(100.575), "USD");
        let rounded2 = amount2.round(2, RoundingStrategy::Bankers);
        // 100.575 should round to 100.58 (banker's rounding - round to even)
        assert_eq!(rounded2.value(), I64F64::from_num(100.58));
    }
    
    #[test]
    fn test_half_up_rounding() {
        let amount = MonetaryValue::new(I64F64::from_num(100.565), "USD");
        let rounded = amount.round(2, RoundingStrategy::HalfUp);
        // 100.565 should round to 100.57 (round half up)
        assert_eq!(rounded.value(), I64F64::from_num(100.57));
    }
    
    #[test]
    fn test_from_str() {
        let amount = MonetaryValue::from_str("100.50", "USD").unwrap();
        assert_eq!(amount.value(), I64F64::from_num(100.50));
        assert_eq!(amount.currency(), "USD");
    }
    
    #[test]
    fn test_display() {
        let amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        assert_eq!(format!("{}", amount), "100.5 USD");
    }
    
    #[test]
    fn test_is_zero() {
        let amount = MonetaryValue::new(I64F64::from_num(0.0), "USD");
        assert!(amount.is_zero());
        
        let amount2 = MonetaryValue::new(I64F64::from_num(1.0), "USD");
        assert!(!amount2.is_zero());
    }
    
    #[test]
    fn test_is_positive() {
        let amount = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        assert!(amount.is_positive());
        
        let amount2 = MonetaryValue::new(I64F64::from_num(-100.50), "USD");
        assert!(!amount2.is_positive());
    }
    
    #[test]
    fn test_is_negative() {
        let amount = MonetaryValue::new(I64F64::from_num(-100.50), "USD");
        assert!(amount.is_negative());
        
        let amount2 = MonetaryValue::new(I64F64::from_num(100.50), "USD");
        assert!(!amount2.is_negative());
    }
}