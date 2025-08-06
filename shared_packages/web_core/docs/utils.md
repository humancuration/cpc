# Utilities

The web_core utilities module provides helper functions and services for common tasks in CPC web applications.

## Storage

The `Storage` wrapper provides a unified interface for working with browser storage mechanisms:

```rust
use web_core::utils::storage::Storage;

let storage = Storage::new();

// Set an item
storage.set_item("key", "value").unwrap();

// Get an item
if let Ok(Some(value)) = storage.get_item("key") {
    // Use the value
}

// Remove an item
storage.remove_item("key").unwrap();

// Clear all items
storage.clear().unwrap();
```

## Error Handling

See [error_handling.md](./error_handling.md) for detailed documentation on error handling utilities.

## Error Reporting

The `ErrorReporter` service collects and reports errors to monitoring services:

```rust
use web_core::utils::error_reporting::{ErrorReporter, ErrorReportingConfig};
use web_core::utils::error_handling::WebError;
use std::collections::HashMap;

let config = ErrorReportingConfig::default();
let mut reporter = ErrorReporter::new(config);

let error = WebError::NetworkError("Connection failed".to_string());
let context = HashMap::new();

// In async context:
// reporter.report_error(error, "MyComponent", context).await;
```

## Error Recovery

The `ErrorRecovery` utilities provide retry logic and fallback mechanisms:

```rust
use web_core::utils::error_recovery::{ErrorRecovery, RecoveryConfig};

let config = RecoveryConfig::default();
let recovery = ErrorRecovery::new(config);

// Execute an operation with retry logic
// let result = recovery.execute_with_retry(|| async {
//     // Some fallible operation
// }).await;
```

## Mock Storage (Testing)

The `MockStorageAdapter` provides a mock implementation of browser storage for testing:

```rust
#[cfg(test)]
use web_core::tests::unit::utils::mock_storage::MockStorageAdapter;

#[cfg(test)]
fn test_storage() {
    let storage = MockStorageAdapter::new();
    
    // Set a value
    storage.set_item("key", "value").unwrap();
    
    // Get a value
    if let Ok(Some(value)) = storage.get_item("key") {
        assert_eq!(value, "value");
    }
}
```

## Base64 Encoding/Decoding

Utility functions for Base64 encoding and decoding:

```rust
// These would be implemented in a base64.rs module
// let encoded = base64_encode("hello world");
// let decoded = base64_decode(&encoded);
```

## JSON Utilities

Helper functions for working with JSON data:

```rust
// These would be implemented in a json.rs module
// let parsed: MyStruct = parse_json(json_string)?;
// let serialized = serialize_json(&my_struct);
```

## URL Utilities

Helper functions for working with URLs:

```rust
// These would be implemented in a url.rs module
// let params = parse_query_params("key=value&foo=bar");
// let url = build_url("https://example.com", params);
```

## Date/Time Utilities

Helper functions for working with dates and times:

```rust
// These would be implemented in a datetime.rs module
// let formatted = format_date(chrono::Utc::now());
// let timestamp = get_current_timestamp();
```

## Best Practices

1. **Use the Storage wrapper** - It provides a consistent interface and handles errors gracefully.

2. **Implement proper error handling** - Use the error handling utilities to provide consistent error management.

3. **Report errors appropriately** - Use ErrorReporter to collect error statistics and send reports to monitoring services.

4. **Implement retry logic** - Use ErrorRecovery for operations that might fail due to transient issues.

5. **Write tests with mock utilities** - Use MockStorageAdapter and other mock utilities for testing.

6. **Keep utilities focused** - Each utility module should have a single, well-defined purpose.

7. **Document utility functions** - Provide clear documentation and examples for all utility functions.

8. **Handle edge cases** - Consider null values, empty strings, and other edge cases in utility functions.