# Security Architecture

## Environment-based Configuration
```rust
// Proposed structure in config.rs
pub struct Config {
    pub jwt_secret: String,
    pub encryption_key: String,
    pub jwt_expiration: u64,
    pub refresh_token_expiration: u64,
    pub environment: Environment,
}

pub enum Environment {
    Dev,
    Test,
    Prod,
}
```

## Secret Management
- **JWT Secret**: Loaded from environment variable `JWT_SECRET`
- **Encryption Keys**: Rotated monthly using key derivation from master secret
- **Key Rotation**: Implemented via versioned keys stored in Valkey

## Key Rotation Mechanism
1. Generate new keys monthly with `keygen` command
2. Store keys in Valkey with version prefix
3. Support dual verification during rotation period
4. Automatically expire old keys after 30-day grace period

## Authentication Middleware
- **Enhanced Claims**:
```rust
pub struct Claims {
    pub user_id: Uuid,
    pub permissions: Vec<Permission>,
    pub exp: usize,
    pub refresh: bool, // True for refresh tokens
}
```
- **Middleware Flow**:
  1. Verify token signature
  2. Check token expiration
  3. Validate permissions against route requirements
  4. Reject tokens marked as compromised

## Defense-in-Depth
- Rate limiting (10req/s per endpoint)
- HSTS enforcement
- CSP headers
- Regular security audits