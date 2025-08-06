# API Client

The web_core API client provides a comprehensive solution for making API calls with support for GraphQL, gRPC-web, request batching, and rate limiting.

## Features

- GraphQL query and mutation support
- gRPC-web client integration
- Request batching for improved performance
- Rate limiting to prevent server overload
- Offline support with caching
- Automatic retries with exponential backoff

## Basic Usage

### GraphQL

```rust
use web_core::api_client::ApiClient;

let client = ApiClient::new("https://api.example.com".to_string());

// Execute a GraphQL query
let result = client.graphql_query::<serde_json::Value>(
    "query { users { id name } }",
    None
).await;

match result {
    Ok(response) => {
        if let Some(data) = response.data {
            // Handle the data
        }
    }
    Err(error) => {
        // Handle the error
    }
}
```

### gRPC-web

```rust
use web_core::api_client::ApiClient;

let client = ApiClient::new("https://api.example.com".to_string());

// Execute a gRPC call
let request = serde_json::json!({"id": 123});
let result = client.grpc_call::<serde_json::Value, serde_json::Value>(
    "UserService",
    "GetUser",
    request
).await;

match result {
    Ok(response) => {
        // Handle the response
    }
    Err(error) => {
        // Handle the error
    }
}
```

## Advanced Features

### Request Batching

The API client supports batching multiple requests together to reduce network overhead:

```rust
use web_core::api_client::{ApiClient, BatchRequest, HttpMethod};

let mut client = ApiClient::new("https://api.example.com".to_string());

// Queue requests for batching
let request = BatchRequest {
    id: "1".to_string(),
    endpoint: "/api/users".to_string(),
    method: HttpMethod::Get,
    body: None,
    headers: std::collections::HashMap::new(),
    queued_at: 0,
};

client.queue_request(request);

// Process the next batch
if let Some(result) = client.process_next_batch().await {
    match result {
        Ok(batch_result) => {
            // Handle successful batch
        }
        Err(error) => {
            // Handle batch error
        }
    }
}
```

### Rate Limiting

The API client includes built-in rate limiting to prevent overwhelming the server:

```rust
use web_core::api_client::{ApiClient, RateLimitConfig};

let mut client = ApiClient::new("https://api.example.com".to_string());

// Configure rate limiting
let config = RateLimitConfig {
    max_requests: 100,
    time_window_ms: 60000,
    queue_excess: true,
    max_queue_size: 50,
};

*client.rate_limiter_mut() = web_core::api_client::RateLimiter::new(config);
```

## Offline Support

The API client automatically caches responses for offline use:

```rust
use web_core::api_client::ApiClient;

let mut client = ApiClient::new("https://api.example.com".to_string());

// Enable offline mode
client.set_offline_mode(true);

// Requests will now use cached responses when available
let result = client.graphql_query::<serde_json::Value>(
    "query { users { id name } }",
    None
).await;
```

## Error Handling

The API client integrates with the web_core error handling system:

```rust
use web_core::api_client::ApiClient;
use web_core::utils::error_handling::WebError;

let client = ApiClient::new("https://api.example.com".to_string());

match client.graphql_query::<serde_json::Value>(
    "query { users { id name } }",
    None
).await {
    Ok(response) => {
        // Handle success
    }
    Err(error_string) => {
        // Convert to WebError for consistent handling
        let error = WebError::ApiError(error_string);
        // Handle error
    }
}