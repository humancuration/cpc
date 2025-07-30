# Yapper - Twitter-style Microblogging App

Yapper is a Twitter-style microblogging application built with Rust, following a hexagonal architecture pattern. It now integrates with CPC's unified authentication system for identity management.

## Features
- **Unified Authentication** - Integrated with CPC's gRPC-based auth service
- **Role-Based Access Control** - Fine-grained permissions using CPC's RBAC system
- **Consent Management** - Domain-specific consent preferences for data sharing
- **Karma Reputation** - User reputation scoring across the CPC ecosystem
- **Event Publishing** - Asynchronous event system for domain events
- **Offline Support** - Local-first data storage with background sync

## API Endpoints

### Health Check
- `GET /health` - Health check endpoint

### Yap Operations
- `POST /api/yap` - Create a new yap
  - Request body: `{ "content": "Your yap content" }`
  - Response: Yap object with status 201

- `GET /api/yap/:id` - Get a specific yap by ID
  - Response: Yap object or null

- `POST /api/yap/:id/like` - Like a yap
  - Response: Status 200

- `POST /api/yap/:id/share` - Share a yap
  - Response: Status 200

### Authentication Operations

#### Register
- `POST /api/auth/register`
  - Request body: `{ "email": "user@example.com", "password": "password123" }`
  - Response: AuthResponse object with status 201

#### Login
- `POST /api/auth/login`
  - Request body: `{ "email": "user@example.com", "password": "password123" }`
  - Response: AuthResponse object with status 200

#### Logout
- `POST /api/auth/logout`
  - Request body: `{ "session_id": "uuid", "user_id": "uuid" }`
  - Response: Status 200

#### Password Reset
- `POST /api/auth/password-reset`
  - Request body: `{ "email": "user@example.com" }`
  - Response: Status 200

#### Password Reset Confirmation
- `POST /api/auth/password-reset/confirm`
  - Request body: `{ "token": "reset_token", "new_password": "newpassword123" }`
  - Response: Status 200

#### OAuth Initiate
- `POST /api/oauth/initiate`
  - Request body: `{ "provider": "google", "code": "authorization_code" }`
  - Response: Redirect to OAuth provider

#### OAuth Callback
- `POST /api/oauth/callback`
  - Request body: `{ "provider": "google", "code": "authorization_code" }`
  - Response: OAuthResponse object with status 200

### New Authentication Features
- `POST /api/consent` - Update consent preferences
  - Request body: `{ "domain": "yapper", "level": "standard" }`
  - Response: Status 200
  
- `GET /api/karma` - Get user karma score
  - Response: `{ "score": 150 }`

## Running the Application

To run the Yapper application:

```bash
cd apps/yapper
cargo run
```

The server will start on `http://localhost:3000`.

## Architecture

The application follows a hexagonal architecture with the following layers:

1. **Domain Layer** - Core business logic and entities
2. **Application Layer** - Use cases and service orchestration
3. **Infrastructure Layer** - Database implementations and external integrations
4. **API Layer** - HTTP API endpoints
5. **Unified Auth Layer** - gRPC-based authentication and authorization

## Technologies Used

- Rust
- Axum (web framework)
- Sled (embedded database)
- Redis (session storage)
- PostgreSQL (primary data store)
- Tokio (async runtime)
- Serde (serialization)
- gRPC (internal services)
- Tonic (gRPC implementation)
- UUID
- Chrono (date/time handling)
- Argon2 (password hashing)