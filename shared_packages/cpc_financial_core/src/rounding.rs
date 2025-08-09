//! Rounding strategies for financial calculations
//!
//! This module provides various rounding strategies commonly used in financial applications,
//! with Banker's rounding as the default for most financial calculations.

use rust_decimal::Decimal;
use rust_decimal_macros::dec;

/// Rounding strategies for financial calculations
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RoundingStrategy {
    /// Banker's rounding (round half to even) - default for financial calculations
    Bankers,
    
    /// Ceiling rounding (always round up)
    Ceiling,
    
    /// Floor rounding (always round down)
    Floor,
}

/// Round a decimal value using the specified strategy
/// 
/// # Arguments
/// * `value` - The value to round
/// * `decimal_places` - Number of decimal places to round to
/// * `strategy` - The rounding strategy to use
/// 
/// # Returns
/// The rounded decimal value
/// 
/// # Examples
/// ```
/// use cpc_financial_core::rounding::{round_with_strategy, RoundingStrategy};
/// use rust_decimal::Decimal;
/// use rust_decimal_macros::dec;
/// 
/// let value = dec!(100.565);
/// let rounded = round_with_strategy(value, 2, RoundingStrategy::Bankers);
/// assert_eq!(rounded, dec!(100.56)); // Rounds to even
/// ```
pub fn round_with_strategy(value: Decimal, decimal_places: u32, strategy: RoundingStrategy) -> Decimal {
    match strategy {
        RoundingStrategy::Bankers => {
            // Banker's rounding (round half to even)
            value.round_dp(decimal_places)
        },
        RoundingStrategy::Ceiling => {
            // Ceiling rounding (always round up)
            let multiplier = Decimal::from(10u64.pow(decimal_places));
            let scaled = value * multiplier;
            let ceiling = scaled.ceil();
            ceiling / multiplier
        },
        RoundingStrategy::Floor => {
            // Floor rounding (always round down)
            let multiplier = Decimal::from(10u64.pow(decimal_places));
            let scaled = value * multiplier;
            let floor = scaled.floor();
            floor / multiplier
        },
    }
}

/// Get the default rounding strategy for financial calculations
/// 
/// # Returns
/// Banker's rounding as recommended for financial calculations
pub fn default_financial_rounding() -> RoundingStrategy {
    RoundingStrategy::Bankers
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    
    #[test]
    fn test_bankers_rounding() {
        // Round half to even
        assert_eq!(round_with_strategy(dec!(100.565), 2, RoundingStrategy::Bankers), dec!(100.56));
        assert_eq!(round_with_strategy(dec!(100.575), 2, RoundingStrategy::Bankers), dec!(100.58));
        assert_eq!(round_with_strategy(dec!(100.566), 2, RoundingStrategy::Bankers), dec!(100.57));
    }
    
    #[test]
    fn test_ceiling_rounding() {
        assert_eq!(round_with_strategy(dec!(100.561), 2, RoundingStrategy::Ceiling), dec!(100.57));
        assert_eq!(round_with_strategy(dec!(100.569), 2, RoundingStrategy::Ceiling), dec!(100.57));
        assert_eq!(round_with_strategy(dec!(100.560), 2, RoundingStrategy::Ceiling), dec!(100.56));
    }
    
    #[test]
    fn test_floor_rounding() {
        assert_eq!(round_with_strategy(dec!(100.569), 2, RoundingStrategy::Floor), dec!(100.56));
        assert_eq!(round_with_strategy(dec!(100.561), 2, RoundingStrategy::Floor), dec!(100.56));
        assert_eq!(round_with_strategy(dec!(100.560), 2, RoundingStrategy::Floor), dec!(100.56));
    }
    
    #[test]
    fn test_default_financial_rounding() {
        assert_eq!(default_financial_rounding(), RoundingStrategy::Bankers);
    }
}