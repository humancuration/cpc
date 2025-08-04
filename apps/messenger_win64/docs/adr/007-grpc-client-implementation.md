# ADR-007: IdentityService gRPC Client

## Context
Need real gRPC integration instead of mock.

## Decision
1. Generate client from proto files:
```proto
service IdentityService {
    rpc ValidateToken(ValidateTokenRequest) returns (ValidateTokenResponse);
    rpc RefreshToken(RefreshTokenRequest) returns (RefreshTokenResponse);
}
```
2. Implement AuthService trait with real client
3. Add connection pooling
4. Implement retry logic with exponential backoff

## Consequences
- Production-ready authentication
- Dependency on IdentityService availability
- Added network error handling