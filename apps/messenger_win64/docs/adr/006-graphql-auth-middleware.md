# ADR-006: GraphQL Authentication Middleware

## Context
GraphQL endpoints need consistent authentication.

## Decision
Implement async-graphql middleware:
1. Extract Authorization header
2. Validate token via AuthService
3. Inject user_id into request context
4. Add error handling for:
   - Missing token
   - Invalid token
   - Expired token

## Consequences
- Uniform auth across GraphQL endpoints
- Centralized error handling
- Easier permission management