# Error Handling

The web_core error handling system provides a consistent approach to handling errors across all CPC web applications.

## WebError Enum

The `WebError` enum defines all possible error types in the application:

```rust
use web_core::utils::error_handling::WebError;

let network_error = WebError::NetworkError("Connection failed".to_string());
let parse_error = WebError::ParseError("Invalid JSON".to_string());
let storage_error = WebError::StorageError("LocalStorage unavailable".to_string());
let auth_error = WebError::AuthenticationError("Invalid credentials".to_string());
let validation_error = WebError::ValidationError("Field is required".to_string());
let api_error = WebError::ApiError("Server error".to_string());
let rate_limit_error = WebError::RateLimitError("Too many requests".to_string());
let batch_error = WebError::BatchError("Batch processing failed".to_string());
let grpc_error = WebError::GrpcError("gRPC call failed".to_string());
let component_error = WebError::ComponentError("Component render failed".to_string());
let theme_error = WebError::ThemeError("Theme loading failed".to_string());
let unknown_error = WebError::UnknownError("Unexpected error".to_string());
```

## Error Logging

Log errors to the console using the `log_error` function:

```rust
use web_core::utils::error_handling::{WebError, log_error};

let error = WebError::NetworkError("Connection failed".to_string());
log_error(&error);
```

## Error Conversion

Convert JavaScript errors to WebError using the `js_error_to_web_error` function:

```rust
use web_core::utils::error_handling::{WebError, js_error_to_web_error};
use wasm_bindgen::JsValue;

let js_error = JsValue::from_str("JavaScript error");
let web_error = js_error_to_web_error(js_error);
```

## Error Boundaries

Use the `ErrorBoundary` component to catch and handle errors in child components:

```rust
use web_core::components::ErrorBoundary;
use web_core::utils::error_handling::WebError;
use yew::prelude::*;

#[function_component(MyApp)]
fn my_app() -> Html {
    let on_error = Callback::from(|error: WebError| {
        // Handle the error
        web_sys::console::error_1(&format!("Error caught: {:?}", error).into());
    });
    
    html! {
        <ErrorBoundary on_error={on_error}>
            <MyComponentThatMightFail />
        </ErrorBoundary>
    }
}
```

## Error Reporting

Report errors to monitoring services using the `ErrorReporter`:

```rust
use web_core::utils::error_reporting::{ErrorReporter, ErrorReportingConfig};
use web_core::utils::error_handling::WebError;
use std::collections::HashMap;

let config = ErrorReportingConfig {
    enabled: true,
    endpoint: Some("https://api.example.com/errors".to_string()),
    max_reports_per_session: 10,
    include_user_agent: true,
    include_url: true,
};

let mut reporter = ErrorReporter::new(config);

let error = WebError::NetworkError("Connection failed".to_string());
let mut context = HashMap::new();
context.insert("component".to_string(), "LoginForm".to_string());
context.insert("action".to_string(), "submit".to_string());

// Report the error (this would be async in real code)
// reporter.report_error(error, "LoginForm", context).await;
```

## Error Recovery

Implement retry logic and fallbacks using the `ErrorRecovery` utilities:

```rust
use web_core::utils::error_recovery::{ErrorRecovery, RecoveryConfig, WebErrorType};
use web_core::utils::error_handling::WebError;

let config = RecoveryConfig {
    max_attempts: 3,
    strategy: web_core::utils::error_recovery::RecoveryStrategy::ExponentialBackoff {
        initial_delay_ms: 1000,
        max_delay_ms: 30000,
        multiplier: 2.0,
    },
    retry_on: vec![
        WebErrorType::Network,
        WebErrorType::Api,
        WebErrorType::RateLimit,
    ],
};

let recovery = ErrorRecovery::new(config);

// Execute an operation with retry logic
// let result = recovery.execute_with_retry(|| async {
//     // Some fallible operation
//     Ok::<String, WebError>("Success".to_string())
// }).await;

// Recover with a fallback value
// let result = recovery.recover_with_fallback(
//     Err(WebError::NetworkError("Failed".to_string())),
//     || "Fallback value".to_string()
// );
```

## Circuit Breaker Pattern

Use the circuit breaker pattern to prevent cascading failures:

```rust
use web_core::utils::error_recovery::CircuitBreaker;

let mut circuit_breaker = CircuitBreaker::new(5, 60000); // 5 failures, 1 minute timeout

// Execute an operation with circuit breaker protection
// let result = circuit_breaker.execute(|| async {
//     // Some fallible operation
//     Ok::<String, WebError>("Success".to_string())
// }).await;
```

## Best Practices

1. **Always handle errors appropriately** - Don't ignore errors, even if you just log them.

2. **Use specific error types** - Choose the most appropriate WebError variant for each situation.

3. **Provide meaningful error messages** - Include enough context to help with debugging.

4. **Implement retry logic for transient errors** - Network errors and rate limits often benefit from retries.

5. **Use error boundaries to prevent crashes** - Wrap components that might fail in ErrorBoundary components.

6. **Report errors to monitoring services** - Use ErrorReporter to collect error statistics.

7. **Implement circuit breakers for external services** - Prevent cascading failures with the CircuitBreaker pattern.

8. **Provide fallbacks when possible** - Use recovery utilities to provide fallback values when operations fail.