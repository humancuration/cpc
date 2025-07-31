# Consent Manager

A unified consent management system for the CPC ecosystem that provides centralized control over data sharing preferences across all applications.

## Overview

The Consent Manager implements a hexagonal architecture with clear separation of concerns between domain logic, application services, infrastructure adapters, and presentation layers. It provides a consistent API for managing user consent for data sharing across all CPC applications.

## Features

- **Unified Consent Model**: Single interface for managing consent across all application domains
- **Multiple Storage Backends**: Sled for edge devices, PostgreSQL for centralized systems
- **Rich Audit Trail**: Comprehensive logging of all consent changes
- **Real-time Events**: Pub/sub system for immediate consent updates
- **Multiple APIs**: gRPC for service-to-service, REST for web clients
- **Web UI Components**: Yew components for consent management dashboard
- **Migration Utilities**: Tools for converting existing consent data

## Architecture

The consent manager follows a hexagonal architecture with the following layers:

1. **Domain Layer**: Core business logic and entities
2. **Application Layer**: Use cases and service orchestration
3. **Infrastructure Layer**: Adapters for external systems
4. **Presentation Layer**: User interfaces and API endpoints

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
consent_manager = { path = "../consent_manager" }
```

## Usage

### Basic Usage

```rust
use consent_manager::{
    domain::{
        consent::{DataSharingLevel, Domain},
        audit::Actor,
    },
    application::service::ConsentService,
    infrastructure::storage::sled_adapter::SledAdapter,
};
use sled::Config;

// Create storage adapter
let config = Config::new().temporary(true);
let db = config.open()?;
let sled_adapter = SledAdapter::new(db);

// Create service
let consent_service = ConsentService::new(Box::new(sled_adapter));

// Update consent level
consent_service
    .update_consent_level(
        "user123",
        Domain::FinancialData,
        DataSharingLevel::Standard,
        Actor::User("user123".to_string()),
    )
    .await?;

// Check consent level
let level = consent_service
    .get_consent_level("user123", Domain::FinancialData)
    .await?;
```

### Checking Consent in Applications

```rust
// Check if a requested level is allowed
let can_share_data = match current_level {
    DataSharingLevel::None => false,
    DataSharingLevel::Minimal => requested_level.priority() <= DataSharingLevel::Minimal.priority(),
    DataSharingLevel::Standard => requested_level.priority() <= DataSharingLevel::Standard.priority(),
    DataSharingLevel::Full => true,
};
```

## API Endpoints

### REST API

- `GET /consent/:user_id/:domain` - Get consent level
- `POST /consent` - Update consent level
- `DELETE /consent/revoke` - Revoke domain consent
- `GET /consent/audit/:user_id` - Get audit events

### gRPC API

See `proto/consent_manager.proto` for the full gRPC service definition.

## Data Model

### Consent Profile

```rust
struct ConsentProfile {
    user_id: String,
    domain: Domain,
    level: DataSharingLevel,
    created_at: DateTime,
    updated_at: DateTime,
}
```

### Data Sharing Levels

- `None`: No data sharing allowed
- `Minimal`: Minimal data sharing for core functionality
- `Standard`: Standard data sharing for enhanced features
- `Full`: Full data sharing for all purposes

### Domains

- FinancialData
- HealthData
- CalendarData
- CrmData
- ScmData
- DocumentData
- WebsiteData
- RecruitmentData
- DataLakehouse
- ForecastingData

## Testing

Run tests with:

```bash
cargo test
```

## Examples

Run the basic usage example:

```bash
cargo run --example basic_usage
```

Run the main example:

```bash
cargo run --bin consent_manager_example
```

## License

This project is licensed under the CPC License - see the LICENSE file for details.