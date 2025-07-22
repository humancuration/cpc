# Authentication System Improvements

## Enhanced JWT Claims
```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: Uuid,
    pub permissions: Vec<Permission>,
    pub exp: usize,
    pub refresh: bool, // True for refresh tokens
    pub scope: Option<String>, // For OAuth2
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Permission {
    CreateProject,
    EditProject,
    DeleteProject,
    PublishContent,
}
```

## Refresh Token Implementation
### Flow:
1. On login, issue both access_token (short-lived) and refresh_token (long-lived)
2. Store refresh tokens in Valkey with user ID and expiration
3. Implement refresh endpoint:
```rust
POST /auth/refresh
Authorization: Bearer <refresh_token>

Response:
{
    "access_token": "...",
    "expires_in": 3600
}
```
4. On refresh:
   - Validate refresh token signature
   - Check Valkey for token existence
   - Issue new access token
   - Rotate refresh token (optional)

## OAuth2 Integration
### Supported Providers:
- Google
- GitHub
- Custom OIDC

### Flow:
1. User redirected to provider's authorization endpoint
2. After consent, callback to `/auth/callback/:provider`
3. Exchange code for tokens
4. Create or update local user account
5. Issue CPC JWT tokens

### OpenID Connect Discovery:
```rust
pub struct OidcConfig {
    issuer: String,
    auth_endpoint: String,
    token_endpoint: String,
    jwks_uri: String,
}
```

## Rate Limiting Design
### Bucket Strategy:
```rust
use governor::{Quota, RateLimiter};
use std::num::NonZeroU32;

let quota = Quota::per_second(NonZeroU32::new(10).unwrap());
let limiter = RateLimiter::key_bucket(quota);
```

### Protected Endpoints:
| Endpoint | Limit | Scope |
|----------|-------|-------|
| /auth/login | 5/min | IP |
| /auth/refresh | 10/min | User |
| /publish | 30/min | User |
| /graphql | 100/min | User |

### Implementation:
```rust
async fn rate_limiting_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let key = req.headers()
        .get("X-Forwarded-For")
        .or_else(|| req.headers().get("X-Real-IP"))
        .map(|v| v.to_str().unwrap_or_default())
        .unwrap_or("unknown");
    
    if limiter.check_key(&key).is_err() {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }
    
    next.run(req).await
}
```

## Security Measures
- Token revocation list
- HTTPS enforcement
- Secure cookie flags
- SameSite strict for authentication cookies
- Token binding to client fingerprint