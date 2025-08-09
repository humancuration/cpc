//! Basic usage example of the CPC Financial Core

use cpc_financial_core::{MonetaryAmount, CurrencyCode, RoundingStrategy, round_with_strategy};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

fn main() {
    println!("=== CPC Financial Core Basic Usage Example ===\n");
    
    // Create monetary amounts
    let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
    let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::USD);
    
    println!("Amount 1: {}", amount1);
    println!("Amount 2: {}", amount2);
    
    // Perform basic operations
    match amount1.add(&amount2) {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error adding amounts: {}", e),
    }
    
    match amount1.subtract(&amount2) {
        Ok(difference) => println!("Difference: {}", difference),
        Err(e) => println!("Error subtracting amounts: {}", e),
    }
    
    // Multiplication and division
    match amount1.multiply(dec!(2)) {
        Ok(product) => println!("Amount 1 * 2: {}", product),
        Err(e) => println!("Error multiplying amount: {}", e),
    }
    
    match amount1.divide(dec!(2)) {
        Ok(quotient) => println!("Amount 1 / 2: {}", quotient),
        Err(e) => println!("Error dividing amount: {}", e),
    }
    
    // Rounding examples
    let amount_with_many_decimals = MonetaryAmount::new(dec!(100.56789), CurrencyCode::USD);
    println!("\nOriginal amount: {}", amount_with_many_decimals);
    
    let bankers_rounded = amount_with_many_decimals.round(RoundingStrategy::Bankers);
    println!("Banker's rounded: {}", bankers_rounded);
    
    let ceiling_rounded = amount_with_many_decimals.round(RoundingStrategy::Ceiling);
    println!("Ceiling rounded: {}", ceiling_rounded);
    
    let floor_rounded = amount_with_many_decimals.round(RoundingStrategy::Floor);
    println!("Floor rounded: {}", floor_rounded);
    
    // Currency conversion (placeholder)
    let converted = amount1.convert_to(CurrencyCode::EUR);
    println!("\nConverted to EUR: {}", converted);
    
    // Direct rounding function usage
    let value = dec!(123.456);
    let rounded = round_with_strategy(value, 2, RoundingStrategy::Bankers);
    println!("\nDirect rounding: {} rounded to 2 decimal places = {}", value, rounded);
    
    println!("\n=== Example completed successfully ===");
}