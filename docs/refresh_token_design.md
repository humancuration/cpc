# Refresh Token System Design

## Overview
Secure token management system implementing JWT refresh tokens with token rotation, Valkey storage, and anti-replay protections.

## Components

### 1. Token Types
| Token Type | Secret | Expiration | Storage |
|------------|--------|------------|---------|
| Access Token | ACCESS_TOKEN_SECRET | 15 minutes | Client-side |
| Refresh Token | REFRESH_TOKEN_SECRET | 7 days | Valkey |

### 2. Token Rotation Flow
1. Client sends refresh token to `/refresh` endpoint
2. Server validates token signature and checks Valkey
3. If valid:
   - Generate new access token
   - Generate new refresh token
   - Store new refresh token in Valkey (with TTL)
   - Revoke old refresh token
   - Return new tokens to client
4. If invalid: Return 401 error

### 3. Valkey Storage Schema
```rust
struct StoredToken {
    user_id: Uuid,
    device_fingerprint: String, // SHA256 of user agent + IP
    created_at: DateTime<Utc>,
    expires_at: DateTime<Utc>,
}
```

### 4. Security Measures
- **Device Binding**: Tokens bound to client fingerprint
- **Token Revocation**: Immediate invalidation on logout
- **Refresh Limit**: Max 5 refreshes per hour per user
- **Short Expiration**: Access tokens expire in 15 minutes

### 5. Error Handling
- `REFRESH_TOKEN_EXPIRED`
- `REFRESH_TOKEN_REVOKED`
- `DEVICE_MISMATCH`
- `REFRESH_RATE_LIMIT_EXCEEDED`

## Implementation Tasks
1. Update `Token` model with `token_type`
2. Extend `JwtService` with refresh token handling
3. Integrate Valkey client into `AuthService`
4. Implement token rotation logic
5. Add security validations
6. Complete GraphQL mutations