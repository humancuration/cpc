# ADR 0006: Concurrency Handling and Transaction Management

## Status
Accepted

## Date
2025-08-03

## Context
As our CPC platform grows in complexity with multiple services interacting concurrently, we need a robust approach to handle concurrent operations, especially when multiple users or processes might attempt to modify the same data simultaneously. This is particularly important for features like volunteer hour conversions, skill exchange claims, and wallet transactions where data consistency is critical.

Current challenges include:
- Race conditions when multiple operations attempt to update the same volunteer activity
- Data inconsistency when concurrent transactions modify shared resources
- Lack of standardized concurrency control mechanisms across services
- Difficulty in testing concurrent scenarios

## Decision
We will implement a comprehensive concurrency handling strategy with the following components:

### 1. TransactionManager Trait
A standardized trait for managing transactions across all services:
```rust
#[async_trait::async_trait]
pub trait TransactionManager: Send + Sync {
    async fn begin_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn commit_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn rollback_transaction(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
```

### 2. EventBus for Cross-Service Communication
A mockable EventBus trait for handling events between services:
```rust
#[async_trait::async_trait]
pub trait EventBus: Send + Sync {
    async fn publish(&self, event: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
```

### 3. Enhanced Testing Utilities
New utilities in `test_utils` to support concurrency testing:
- `CurrencyValidator` for currency validation
- `LargeDatasetSeeder` for performance testing with large datasets
- Mock implementations for all new traits

### 4. Concurrency Control Patterns
Implementation of concurrency control using:
- Database-level locking where appropriate
- Mutex-based conflict simulation for testing
- Optimistic locking with version numbers for critical operations

## Consequences

### Positive
- Improved data consistency across concurrent operations
- Standardized approach to transaction management
- Better test coverage for concurrent scenarios
- Enhanced performance testing capabilities
- Clear separation of concerns with hexagonal architecture

### Negative
- Additional complexity in service implementations
- Potential performance overhead from transaction management
- Increased testing infrastructure requirements

### Neutral
- Requires updates to existing service implementations
- New dependencies for testing utilities

## Implementation Details

### Core Components
1. **TransactionManager**: Provides a consistent interface for transaction handling
2. **EventBus**: Enables loose coupling between services
3. **Testing Utilities**: Support comprehensive concurrency testing

### Integration Points
- Volunteer service: For handling concurrent volunteer hour conversions
- Skill exchange service: For managing concurrent skill claims
- Wallet service: For ensuring atomic financial transactions
- Achievement service: For consistent achievement awarding

### Testing Strategy
- Unit tests for all new utilities and traits
- Integration tests for concurrent operation scenarios
- Performance tests with large datasets (10,000+ records)
- Cross-service integration tests for event flows

## Security Considerations
- All transaction operations maintain data integrity
- Event-based communication ensures no direct service dependencies
- Proper error handling prevents data corruption

## Future Considerations
- Database-level advisory locks for more sophisticated concurrency control
- Distributed transaction support for multi-database operations
- Advanced conflict resolution strategies
- Integration with p2panda network for distributed concurrency handling