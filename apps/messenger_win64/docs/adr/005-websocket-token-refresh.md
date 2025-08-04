# ADR-005: WebSocket Token Refresh Strategy

## Context
Long-lived WebSocket connections require token refresh without reauthentication.

## Decision
Implement proactive token refresh:
1. Server tracks token expiration time
2. 5 minutes before expiry, send TokenExpiringSoon event
3. Client initiates refresh with refresh_token
4. Server obtains new access token via gRPC
5. New token pushed to client via dedicated WebSocket message

## Consequences
- Added complexity to connection lifecycle
- Requires client-side refresh handling
- Reduces authentication interruptions