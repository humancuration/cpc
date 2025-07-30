# Currency Internationalization in Finance-Sheets

## Overview

This document describes the implementation of currency internationalization for Finance-Sheets, enabling users to work with multiple currencies, handle exchange rates, and format values according to locale-specific rules.

## Features Implemented

### 1. Comprehensive Currency Support
- Support for 150+ currencies with ISO 4217 codes
- Proper handling of decimal places for each currency
- Special support for the platform's internal currency (Dabloons)

### 2. Exchange Rate Management
- Real-time exchange rate integration with multiple providers
- Caching layer with configurable TTL
- Historical exchange rate tracking
- Automatic fallback between providers

### 3. Localization
- Locale-aware number formatting (decimal separators, grouping)
- Currency symbol positioning based on locale
- Support for different decimal place requirements

### 4. User Preferences
- Per-user default currency settings
- Preferred locale for formatting
- Option to show currency symbols or codes

### 5. Integration
- Full sync infrastructure compatibility
- Database schema for persistent storage
- gRPC adapters for service communication

## Architecture

### Domain Layer
```
currency/
├── code.rs          # CurrencyCode type (ISO 4217 codes)
├── model.rs         # Currency struct with properties
├── registry.rs      # Registry of all supported currencies
└── exchange_rate.rs # Exchange rate management
```

### Application Layer
```
currency/
├── service.rs       # Main currency service
├── user_prefs.rs    # User preferences management
└── mock_repo.rs     # Mock repository for testing
```

### Presentation Layer
```
localization/
├── mod.rs
└── formatter.rs     # Locale-aware currency formatting
```

### Infrastructure Layer
```
database/
└── currency_repository.rs  # Database implementation

adapters/
└── currency/
    └── grpc.rs             # gRPC adapter
```

## Database Schema

### currencies Table
Stores all supported currencies with their properties:
- `code` (CHAR(3)): ISO 4217 currency code (Primary Key)
- `name` (VARCHAR): Full currency name
- `symbol` (VARCHAR): Currency symbol for display
- `decimal_places` (SMALLINT): Number of decimal places
- `is_dabloon` (BOOLEAN): Whether this is the platform's internal currency

### exchange_rates Table
Stores exchange rates between currencies:
- `id` (UUID): Unique identifier
- `from_currency` (CHAR(3)): Base currency code (Foreign Key)
- `to_currency` (CHAR(3)): Target currency code (Foreign Key)
- `rate` (DECIMAL): Exchange rate value
- `provider` (VARCHAR): Provider that supplied this rate
- `fetched_at` (TIMESTAMP): When this rate was fetched

### user_currency_preferences Table
Stores user-specific currency preferences:
- `user_id` (UUID): User identifier (Primary Key, Foreign Key)
- `default_currency` (CHAR(3)): User's default currency (Foreign Key)
- `preferred_locale` (VARCHAR): User's preferred locale
- `show_currency_symbols` (BOOLEAN): Whether to show symbols or codes

## Usage Examples

### Currency Conversion
```rust
let mut service = CurrencyService::new(registry, exchange_service, repo);
let converted = service.convert_currency(dec!(100), &CurrencyCode::new("USD"), &CurrencyCode::new("EUR")).await?;
```

### Locale-Aware Formatting
```rust
let formatter = CurrencyFormatter::new();
let currency: Currency = CurrencyCode::new("EUR").into();
let formatted = formatter.format_currency(dec!(1234.56), &currency, &"de-DE".into());
// Result: "1.234,56 €"
```

### User Preferences
```rust
let preferences = service.get_user_preferences(user_id).await?;
let formatted = service.format_currency_for_user(amount, &currency, user_id).await?;
```

## Sync Infrastructure Integration

The currency system is fully integrated with the CPC sync infrastructure:
- User preference changes are queued for sync
- Exchange rate updates are cached locally
- Offline operations are supported with automatic recovery

## Performance Considerations

- Exchange rates are cached with configurable TTL (default 60 seconds)
- Database queries are optimized with appropriate indexes
- Formatting operations are lightweight (<< 10ms)

## Testing

Comprehensive test coverage is provided:
- Unit tests for all components
- Integration tests with database
- Mock implementations for external dependencies
- Performance benchmarks

## Future Enhancements

- Additional exchange rate providers
- More comprehensive locale support
- Advanced historical analysis features
- Currency conversion graphs and trends