# Authentication Implementation Guide

## Using AuthService
```rust
// Example usage in WebSocket handler
async fn handle_connection(
    auth_service: Arc<dyn AuthService>,
    token: &str
) -> Result<Uuid, MessengerError> {
    auth_service.validate_token(token).await
        .map_err(|e| MessengerError::AuthError { message: e.to_string() })
}
```

## Error Handling Patterns
```rust
match auth_service.validate_token(token).await {
    Ok(user_id) => { /* proceed */ },
    Err(AuthError::InvalidToken) => { /* handle invalid token */ },
    Err(AuthError::ExpiredToken) => { /* handle expired token */ },
    Err(AuthError::IdentityServiceUnavailable) => { /* retry logic */ },
    // ...
}
```

## Testing Authentication
```rust
#[cfg(test)]
mod tests {
    struct MockAuthService;
    
    #[async_trait]
    impl AuthService for MockAuthService {
        async fn validate_token(&self, token: &str) -> Result<Uuid, AuthError> {
            if token == "valid" { Ok(Uuid::new_v4()) }
            else { Err(AuthError::InvalidToken) }
        }
        // ...
    }
}
```

## Integration Points
- WebSocket connection handler
- GraphQL middleware
- gRPC client configuration