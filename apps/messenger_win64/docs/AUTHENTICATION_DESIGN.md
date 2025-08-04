# Messenger Authentication Design

## WebSocket Authentication Flow
```mermaid
sequenceDiagram
    participant Client
    participant Messenger (WS Server)
    participant IdentityService (gRPC)
    
    Client->>Messenger: Connect to WebSocket (JWT in query)
    Messenger->>IdentityService: ValidateToken(gRPC)
    alt Valid Token
        IdentityService-->>Messenger: UserID + Expiry
        Messenger->>Client: Connection ACK
    else Invalid Token
        IdentityService-->>Messenger: AuthError
        Messenger->>Client: Close Connection (401)
    end
```

## gRPC Communication Flow
```mermaid
sequenceDiagram
    participant Messenger
    participant IdentityService
    
    Messenger->>IdentityService: ValidateTokenRequest
    IdentityService-->>Messenger: ValidateTokenResponse
    Messenger->>IdentityService: RefreshTokenRequest
    IdentityService-->>Messenger: RefreshTokenResponse
```

## Token Refresh Workflow
```mermaid
sequenceDiagram
    participant Client
    participant Messenger
    participant IdentityService
    
    Note over Messenger: Token nearing expiry
    Messenger->>Client: TokenExpiringSoon event
    Client->>Messenger: RefreshTokenRequest
    Messenger->>IdentityService: RefreshToken(gRPC)
    IdentityService-->>Messenger: NewAccessToken
    Messenger->>Client: NewAccessToken
```

## Key Components
- `AuthService` trait (validate_token, refresh_token)
- `GrpcAuthService` (gRPC client implementation)
- WebSocket auth middleware
- GraphQL auth middleware
## Related Documents

See these Architecture Decision Records for implementation details:
- [005: Websocket Token Refresh](adr/005-websocket-token-refresh.md)
- [006: GraphQL Auth Middleware](adr/006-graphql-auth-middleware.md)
- [007: gRPC Client Implementation](adr/007-grpc-client-implementation.md)