# CPC Auth

This crate provides authentication services for CPC applications, including user registration, login, session management, and OAuth integration.

## Features

- User registration and authentication
- Password hashing with Argon2
- Session management with expiration
- OAuth provider abstraction
- Framework-agnostic design
- Comprehensive error handling

## Modules

- `models`: Data structures for credentials, users, and sessions
- `auth_service`: Authentication service trait and implementation
- `session_service`: Session management traits
- `oauth`: OAuth provider abstraction
- `middleware`: Authentication middleware utilities
- `error`: Error types for authentication operations

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
cpc_auth = { path = "../../packages/cpc_auth" }
```

Then use the re-exports in your domain modules:

```rust
// In your domain/credentials.rs
pub use cpc_auth::models::Credentials;

// In your domain/user.rs
pub use cpc_auth::models::{User, Role};

// In your domain/session.rs
pub use cpc_auth::models::Session;

// In your domain/auth_error.rs
pub use cpc_auth::error::AuthError;

// In your domain/auth_service.rs
use cpc_auth::models::{Credentials, User, Session};
use cpc_auth::error::AuthError;
```

## Integration Example

The Yapper application demonstrates how to integrate this crate:

1. Update Cargo.toml to include the cpc_auth dependency
2. Re-export models from cpc_auth in domain modules
3. Implement the AuthService trait using Yapper-specific repositories
4. Update main.rs to use the new service implementation

This approach maintains separation of concerns while providing a reusable authentication foundation.