# CPC Financial Core

The CPC Financial Core is a precise decimal arithmetic library designed for financial calculations within the CPC ecosystem. It provides robust monetary operations with proper rounding strategies, currency handling, and audit capabilities.

## Features

- **Precise Decimal Arithmetic**: Uses `rust_decimal` for accurate financial calculations without floating-point errors
- **Multiple Rounding Strategies**: Banker's rounding (default), Ceiling, and Floor
- **Currency Support**: Comprehensive currency code enumeration with decimal place definitions
- **Audit Trail**: Built-in audit logging for all financial operations
- **Currency Conversion**: Framework for currency conversion operations
- **WebAssembly Compatible**: Works in both native and WASM environments

## Core Components

### MonetaryAmount
Represents a monetary value with currency information:
```rust
use cpc_financial_core::{MonetaryAmount, CurrencyCode};
use rust_decimal_macros::dec;

let amount = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
```

### Rounding Strategies
Three rounding strategies are supported:
- `RoundingStrategy::Bankers` - Banker's rounding (default, statistically unbiased)
- `RoundingStrategy::Ceiling` - Always rounds up
- `RoundingStrategy::Floor` - Always rounds down

### CurrencyCode
Comprehensive currency enumeration with ISO 4217 codes and decimal place information.

## Usage Example

```rust
use cpc_financial_core::{MonetaryAmount, CurrencyCode, RoundingStrategy};
use rust_decimal::Decimal;
use rust_decimal_macros::dec;

// Create monetary amounts
let amount1 = MonetaryAmount::new(dec!(100.50), CurrencyCode::USD);
let amount2 = MonetaryAmount::new(dec!(50.25), CurrencyCode::USD);

// Perform operations
let sum = amount1.add(&amount2)?;
let difference = amount1.subtract(&amount2)?;
let product = amount1.multiply(dec!(2))?;
let quotient = amount1.divide(dec!(2))?;

// Rounding
let rounded = amount1.round(RoundingStrategy::Bankers);
```

## Audit Trail

All financial operations automatically generate audit logs through the `FinancialAuditable` trait, ensuring transparency and traceability of all financial activities.

## Cooperative Values Integration

This library embodies cooperative values by:
1. **Transparency**: All operations are auditable
2. **Precision**: Eliminates floating-point errors that can cause financial discrepancies
3. **Fairness**: Uses statistically unbiased Banker's rounding by default
4. **Interoperability**: Designed to work seamlessly with other CPC components

## WebAssembly Support

The library is fully compatible with WebAssembly, making it suitable for both native and web-based cooperative applications.

## Examples

Run the basic usage example:
```bash
cargo run --example basic_usage
```

Run the cooperative values integration example:
```bash
cargo run --example cooperative_values
```

Run the data integration example:
```bash
cargo run --example data_integration
```

Run the full integration example:
```bash
cargo run --example full_integration
```

## Testing

Run tests with:
```bash
cargo test
```

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license, promoting sharing within the federation.