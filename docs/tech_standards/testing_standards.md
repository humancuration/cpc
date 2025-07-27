# CPC Testing Standards

## Introduction
This document establishes comprehensive testing standards for the CPC ecosystem, ensuring consistent quality and reliability across all modules. These standards align with our architectural principles (hexagonal architecture, screaming architecture) and must be followed for all new development.

## Domain Layer Testing Requirements
*All domain logic must achieve 100% test coverage with validation-focused tests*

### Value Object Testing
- **Validation Coverage**:
  - Test all valid input scenarios with representative examples
  - Test *all* invalid cases (format, length, character set, boundaries)
  - Include edge cases (minimum/maximum values, empty strings)
  
- **Example Pattern** (from value objects):
```rust
#[test]
fn test_color_hex_valid() {
    let valid_hex = "#FF0000";
    let color = ColorHex::new(valid_hex);
    assert!(color.is_ok());
    assert_eq!(color.unwrap().as_str(), valid_hex);
}

#[test]
fn test_color_hex_invalid_format() {
    let invalid_hex = "FF0000"; // Missing #
    assert!(ColorHex::new(invalid_hex).is_err());
}

#[test]
fn test_color_hex_invalid_length() {
    assert!(ColorHex::new("#FF00").is_err());    // Too short
    assert!(ColorHex::new("#FF000000").is_err()); // Too long
}
```

- **Required Test Scenarios**:
  - Minimum/maximum length boundaries
  - Invalid character sets
  - Format validation (regex patterns)
  - Empty string handling
  - Case sensitivity (if applicable)

## Repository Integration Testing Patterns
*SQLx-based integration tests using real database connections*

### Test Structure
```rust
#[tokio::test]
async fn test_create_and_get_site() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create test data
    let site = Site { /* minimum valid data */ };
    
    // Test create operation
    let created_site = repository.create_site(site.clone()).await.unwrap();
    
    // Test retrieval
    let retrieved_site = repository.get_site_by_id(site.id).await.unwrap();
    
    // Assertions
    assert_eq!(retrieved_site.id, site.id);
    
    // Cleanup
    cleanup_test_data(&pool).await;
}
```

### Critical Requirements
1. **Database Setup**:
   - Use `TEST_DATABASE_URL` environment variable
   - Implement `setup_test_db()` and `cleanup_test_data()` helpers
   - Clean up in dependency order (child entities first)

2. **Test Coverage**:
   - All CRUD operations
   - Business logic specific to repository (e.g., `increment_link_click_count`)
   - Race conditions (where applicable)
   - Transaction boundaries

3. **Edge Cases**:
   - Non-existent IDs
   - Invalid foreign key relationships
   - Concurrency scenarios
   - Database constraint violations

## GraphQL API Testing Methodology
*Verification of API contracts and resolver logic*

### Mutation Testing
```rust
#[tokio::test]
async fn test_create_site_mutation_with_link_in_bio() {
    let schema = create_test_schema_with_mocks(); // Must include mocked services
    
    let mutation = r#"
        mutation {
            createSite(input: {
                name: "Test Link Site"
                siteType: {
                    linkInBio: {
                        headline: "My Links"
                        description: "Check out my links"
                    }
                }
            }) {
                id
                name
                siteType {
                    linkInBio {
                        headline
                    }
                }
            }
        }
    "#;
    
    let result = schema.execute(mutation).await;
    assert!(result.is_ok());
    
    // Verify data was created through service layer
    let data = result.data.into_json().unwrap();
    assert_eq!(data["createSite"]["name"], "Test Link Site");
}
```

### Critical Requirements
1. **Mocked Dependencies**:
   - *Must* mock application services (never test with real database)
   - Verify service method calls with expected parameters
   - Test error propagation from services

2. **Validation Testing**:
   - Test all validation rules (e.g., empty site name)
   - Verify appropriate error messages
   - Test invalid UUID formats
   - Test out-of-range values

3. **Subscription Verification**:
   - Test subscription setup and teardown
   - Verify real-time updates with multiple clients
   - Test error handling in subscription streams

## Async Testing Patterns
*Standardized approach for asynchronous operations*

### Best Practices
- Always use `#[tokio::test]` for async tests
- Implement proper shutdown handling for resources
- Use `tokio::time::timeout` for operations that might hang
- Avoid `.unwrap()` in tests - use specific error assertions

### Example Pattern
```rust
#[tokio::test]
async fn test_async_operation_with_timeout() {
    let timeout = Duration::from_secs(5);
    
    let result = tokio::time::timeout(timeout, async {
        // Async operation under test
        repository.process_long_running_task().await
    }).await;
    
    assert!(result.is_ok(), "Operation timed out");
    assert!(result.unwrap().is_ok());
}
```

## Edge Case Testing Requirements
*Systematic identification of boundary conditions*

### Mandatory Edge Cases
| Component        | Edge Cases to Test                          |
|------------------|-------------------------------------------|
| **Value Objects**| Min/max length, empty strings, invalid chars, boundary values |
| **Repositories** | Non-existent IDs, duplicate keys, constraint violations, race conditions |
| **API Layer**    | Invalid UUIDs, out-of-range dates, malformed JSON, large payloads |
| **Services**     | Null inputs, permission boundaries, rate limiting |

### Example: URL Validation Edge Cases
```rust
#[test]
fn test_valid_url_valid_minimum_length() {
    let valid_urls = vec![
        "http://ab.c",   // 10 chars (minimum)
        "https://a.bc",
        "http://a.com",
    ];
    
    for url in valid_urls {
        assert!(ValidUrl::new(url).is_ok());
    }
}

#[test]
fn test_valid_url_too_short() {
    let short_urls = vec![
        "http://a",      // 9 chars (invalid)
        "https://ab",
    ];
    
    for url in short_urls {
        assert!(ValidUrl::new(url).is_err());
    }
}
```

## Test Organization Structure
*Consistent project structure for test discoverability*

```
src/
├── domain/
│   ├── value_objects.rs
│   └── value_objects_test.rs   # 100% domain coverage
├── infrastructure/
│   ├── repository.rs
│   └── repository_test.rs      # SQLx integration tests
├── application/
│   ├── site_service.rs
│   └── site_service_test.rs    # Service logic with mocked dependencies
└── web/
    └── graphql/
        ├── resolvers.rs
        └── graphql_test.rs     # API contract verification
```

### Organization Rules
1. **Unit Tests**: Co-located with implementation (in same module/file)
2. **Integration Tests**: In dedicated `_test.rs` files at same directory level
3. **Test Types Separation**:
   - `*_domain_test.rs` - Domain validation
   - `*_integration_test.rs` - Database/external interactions
   - `*_api_test.rs` - API contract verification

## Mocking Strategies
*Isolating components for focused testing*

### Layer-Specific Approaches
| Layer            | Mocking Strategy                          | Tools               |
|------------------|------------------------------------------|---------------------|
| **Domain**       | Not required (pure logic)                | N/A                 |
| **Repository**   | Real database (SQLx integration tests)   | SQLx, PostgreSQL    |
| **Application**  | Mock repositories                        | `mockall`           |
| **Web/API**      | Mock application services                | `async_graphql` mocks |

### Example: Service Layer Mocking
```rust
use mockall::mock;

mock! {
    SiteRepository {
        fn create_site(&self, site: Site) -> Result<Site, DomainError>;
        fn get_site_by_id(&self, id: Uuid) -> Result<Option<Site>, DomainError>;
    }
}

#[tokio::test]
async fn test_site_service_create_site() {
    let mut mock_repo = MockSiteRepository::new();
    mock_repo.expect_create_site()
             .withf(|site| site.name == "Test Site")
             .returning(|site| Ok(site));
    
    let service = SiteService::new(mock_repo);
    let result = service.create_site(/* ... */).await;
    
    assert!(result.is_ok());
    // Verify side effects
}
```

## Analytics Tracking Verification
*Validating event emission and processing*

### Required Tests
1. **Event Generation**:
   - Verify correct event payload structure
   - Test all required metadata fields
   - Validate event naming conventions

2. **Error Handling**:
   - Test analytics service failures don't break main flow
   - Verify retry mechanisms
   - Test fallback logging

### Example Implementation
```rust
#[tokio::test]
async fn test_increment_link_click_tracking() {
    let (analytics_mock, rx) = create_analytics_mock();
    
    let repository = SiteRepository::new(
        Arc::new(mock_pool()),
        analytics_mock
    );
    
    repository.increment_link_click_count(link_id).await.unwrap();
    
    // Verify analytics event emitted
    let event = rx.try_recv().unwrap();
    assert_eq!(event.event_type, "LINK_CLICK");
    assert_eq!(event.properties.get("link_id"), Some(&link_id.to_string()));
}
```

## Performance Considerations
*Ensuring test suites remain maintainable*

### Critical Rules
1. **Test Execution Time**:
   - Unit tests: < 100ms per test
   - Integration tests: < 1s per test
   - Mark slow tests with `#[ignore]` and run separately

2. **Database Optimization**:
   - Use transactional tests where possible
   - Implement test data factories
   - Avoid full DB resets between tests when unnecessary

3. **Parallel Execution**:
   - Tests must be isolated (no shared state)
   - Use unique identifiers in test data
   - Avoid port conflicts in network tests

## Implementation Roadmap
1. All new features **must** include tests per these standards
2. Existing code will be retrofitted during module refactoring
3. PRs without sufficient test coverage will be rejected
4. Coverage reports required for all modules (100% domain, 90%+ integration)

---

*Document created per CPC testing standards initiative. Last updated: 2025-07-26*