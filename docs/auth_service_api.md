# Auth Service API Documentation

## Overview

The Auth Service provides centralized authentication and session management for the CPC ecosystem using gRPC. It implements a Redis-based session store and integrates with the unified RBAC and consent management systems.

## gRPC Service Definition

### AuthService

The AuthService provides the following RPC methods:

#### ValidateSession
Validates an existing session by its ID.

**Request**: `SessionRequest`
```protobuf
message SessionRequest {
  string session_id = 1;
}
```

**Response**: `SessionResponse`
```protobuf
message SessionResponse {
  bool valid = 1;
  string user_id = 2;
  string session_id = 3;
  string error_message = 4;
}
```

#### CreateSession
Creates a new session for a user.

**Request**: `CreateSessionRequest`
```protobuf
message CreateSessionRequest {
  string user_id = 1;
  string device_info = 2;
}
```

**Response**: `SessionResponse`
```protobuf
message SessionResponse {
  bool valid = 1;
  string user_id = 2;
  string session_id = 3;
  string error_message = 4;
}
```

#### InvalidateSession
Invalidates an existing session.

**Request**: `InvalidateSessionRequest`
```protobuf
message InvalidateSessionRequest {
  string session_id = 1;
}
```

**Response**: `InvalidateSessionResponse`
```protobuf
message InvalidateSessionResponse {
  bool success = 1;
  string error_message = 2;
}
```

## Authentication Flow

1. **User Login**: Client authenticates with credentials through the main auth service
2. **Session Creation**: On successful authentication, a session is created via `CreateSession`
3. **Session Storage**: Session is stored in Redis with a 30-minute TTL
4. **Session Validation**: Subsequent requests validate sessions using `ValidateSession`
5. **Session Invalidation**: Sessions can be invalidated using `InvalidateSession`

## Session Management

Sessions are stored in Redis with the following characteristics:
- 30-minute TTL (time-to-live)
- JSON serialized session objects
- Keyed by session UUID
- Automatic cleanup on expiration

## Error Handling

All RPC methods return appropriate gRPC status codes:
- `OK` for successful operations
- `INVALID_ARGUMENT` for malformed requests
- `UNAUTHENTICATED` for invalid sessions
- `INTERNAL` for server-side errors

## Integration with RBAC

The auth service integrates with the CPC RBAC system to provide role-based access control:
- Roles are stored with user profiles
- Permissions are checked against the RBAC engine
- Middleware enforces role requirements

## Integration with Consent Management

The auth service integrates with the CPC consent management system:
- User consent preferences are checked before accessing protected resources
- Domain-specific consent levels are enforced
- Middleware enforces consent requirements

## Security Considerations

- All session IDs are UUIDv4
- Sessions expire after 30 minutes of inactivity
- Redis connections should use authentication in production
- gRPC should be served over TLS in production