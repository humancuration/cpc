# Consent Manager Design

## Overview

The Consent Manager is a core component of the CPC ecosystem that provides a unified approach to managing user consent for data sharing across all applications. It implements a hexagonal architecture with clear separation of concerns between domain logic, application services, infrastructure adapters, and presentation layers.

## Architecture

### Hexagonal Architecture Layers

1. **Domain Layer**: Core business logic and entities
2. **Application Layer**: Use cases and service orchestration
3. **Infrastructure Layer**: Adapters for external systems (databases, APIs)
4. **Presentation Layer**: User interfaces and API endpoints

### Key Components

#### Domain Model

- `DataSharingLevel`: Enum defining consent levels (None, Minimal, Standard, Full)
- `Domain`: Enum representing different application domains (FinancialData, HealthData, CalendarData, etc.)
- `ConsentProfile`: Aggregate root containing user consent preferences
- `AuditEvent`: Records of consent changes for compliance

#### Application Services

- `ConsentService`: Orchestrates consent operations
- Validators for input validation

#### Infrastructure Adapters

- `ConsentStorage`: Trait defining storage operations
- Sled adapter for edge device storage
- PostgreSQL adapter for centralized storage
- gRPC and REST API adapters

#### Presentation Components

- Yew components for web UI
- gRPC service implementation
- REST API handlers

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

### Audit Trail

```rust
struct AuditEvent {
    id: String,
    user_id: String,
    domain: Domain,
    action: ConsentAction,
    previous_level: Option<DataSharingLevel>,
    new_level: DataSharingLevel,
    actor: Actor,
    timestamp: DateTime,
}
```

## Integration Points

### Storage Adapters

1. **Sled Adapter**: For edge devices with local storage
2. **PostgreSQL Adapter**: For centralized systems

### API Interfaces

1. **gRPC**: For internal service-to-service communication
2. **REST**: For external integration and web clients

### Event System

Real-time event publishing for consent changes to enable immediate updates across the system.

## Migration Strategy

Utilities to migrate existing consent data from domain-specific implementations to the unified consent manager.

## Security Considerations

- All consent operations require authentication
- Audit trails for all changes
- Encryption at rest for sensitive data
- Rate limiting on API endpoints