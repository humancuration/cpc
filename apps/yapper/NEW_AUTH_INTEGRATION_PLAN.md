# Yapper App - Unified Authentication Integration Plan

## Overview
This document outlines the integration of the unified authentication system into the Yapper app. The new system will replace the existing auth implementation while maintaining all existing features and adding new capabilities including RBAC, consent management, and Karma reputation.

## High-Level Architecture
```
┌───────────────────────┐       ┌───────────────────────┐
│       Yapper App      │       │     Auth Service      │
│                       │       │      (gRPC/Redis)     │
│ ┌───────────────────┐ │       ┌───────────────────────┐
│ │    API Handlers   │◀┼───────┤ Session Validation    │
│ └───────────────────┘ │       └───────────────────────┘
│           ▲            │
│           │            │
│ ┌───────────────────┐ │
│ │  Auth Middleware  │─┘
│ └───────────────────┘ 
│           ▲
│           │
│ ┌───────────────────┐ 
│ │  Unified Auth     │ 
│ │  Adapter Layer    │───┐
│ └───────────────────┘   │       ┌───────────────────────┐
│                         ├───────┤       RBAC System     │
│ ┌───────────────────┐   │       └───────────────────────┘
│ │  Consent Manager  │───┤
│ └───────────────────┘   │       ┌───────────────────────┐
│                         ├───────┤      Karma System     │
│ ┌───────────────────┐   │       └───────────────────────┘
│ │ Event Publisher   │◀──┘
│ └───────────────────┘ 
└───────────────────────┘
```

## Integration Steps

### 1. Update Dependencies
Add required packages to `apps/yapper/Cargo.toml`:
```toml
cpc_auth = { path = "../../packages/cpc_auth" }
cpc_rbac = { path = "../../packages/cpc_rbac" }
cpc_karma = { path = "../../packages/cpc_karma" }
cpc_consent = { path = "../../packages/cpc_consent" }
tonic = "0.9"
prost = "0.12"
redis = "0.23"
```

### 2. Implement gRPC Client
Create `apps/yapper/src/infrastructure/auth_service_client.rs`:
```rust
use cpc_auth::grpc::auth_service_client::AuthServiceClient;
use tonic::transport::Channel;

pub struct YapperAuthServiceClient {
    client: AuthServiceClient<Channel>,
}

impl YapperAuthServiceClient {
    pub async fn new(addr: String) -> Result<Self, tonic::transport::Error> {
        let client = AuthServiceClient::connect(addr).await?;
        Ok(Self { client })
    }
    
    // Implement session methods that call gRPC service
}
```

### 3. Update Session Management
Modify `apps/yapper/src/application/session_service.rs` to use Redis-backed sessions via gRPC:
```rust
use crate::infrastructure::auth_service_client::YapperAuthServiceClient;

pub struct SessionServiceImpl {
    auth_client: YapperAuthServiceClient,
    event_publisher: Arc<dyn EventPublisher>,
}

impl SessionService for SessionServiceImpl {
    async fn create_session(&self, user_id: Uuid) -> Result<Session, AuthError> {
        // Call auth_client.create_session()
        // Publish SessionCreated event
    }
    
    async fn validate_session(&self, session_id: &str) -> Result<Session, AuthError> {
        // Call auth_client.validate_session()
    }
}
```

### 4. Add RBAC Middleware
Create `apps/yapper/src/infrastructure/middleware/rbac_middleware.rs`:
```rust
use cpc_rbac::RBACEngine;
use axum::middleware::Next;
use axum::response::Response;

pub async fn rbac_middleware(
    rbac: Arc<RBACEngine>,
    request: Request,
    next: Next,
) -> Result<Response, AuthError> {
    let user_id = // extract from session
    let required_permission = // map route to permission
    
    if rbac.check_permission(user_id, required_permission).await {
        next.run(request).await
    } else {
        Err(AuthError::PermissionDenied)
    }
}
```

### 5. Implement Consent Management
Create `apps/yapper/src/infrastructure/consent_manager.rs`:
```rust
use cpc_consent::ConsentManager;

pub struct YapperConsentManager {
    consent_manager: ConsentManager,
}

impl YapperConsentManager {
    pub fn new() -> Self {
        Self {
            consent_manager: ConsentManager::new(),
        }
    }
    
    pub async fn check_consent(&self, user_id: Uuid, domain: &str, level: ConsentLevel) -> bool {
        self.consent_manager.check_consent(user_id, domain, level).await
    }
}
```

### 6. Integrate Karma System
Modify `apps/yapper/src/domain/user_service.rs` to include karma:
```rust
use cpc_karma::KarmaService;

pub struct UserServiceImpl {
    karma_service: Arc<dyn KarmaService>,
    // ... other dependencies
}

impl UserService for UserServiceImpl {
    async fn create_user(&self, credentials: Credentials) -> Result<User, AuthError> {
        // Create user
        self.karma_service.initialize_user(user.id).await?;
        // Publish UserCreated event
    }
}
```

### 7. Update Authentication Handlers
Modify `apps/yapper/src/api/handlers/auth_handler.rs`:
```rust
pub async fn register(
    Extension(consent_manager): Extension<Arc<YapperConsentManager>>,
    Json(payload): Json<RegisterRequest>
) -> Result<Json<UserResponse>> {
    // Create user
    consent_manager.set_consent(
        user.id, 
        "yapper", 
        payload.consent_level
    ).await?;
}
```

### 8. Migration Strategy
1. **Phase 1**: Run both auth systems in parallel
2. **Phase 2**: Migrate sessions using dual-write approach:
   - New sessions → Redis
   - Existing sessions remain in Sled until expiration
3. **Phase 3**: Fully switch to new system after all sessions expire

### Files to Modify/Create
| File Path | Action | Description |
|-----------|--------|-------------|
| `apps/yapper/Cargo.toml` | Modify | Add new dependencies |
| `apps/yapper/src/infrastructure/auth_service_client.rs` | Create | gRPC client implementation |
| `apps/yapper/src/application/session_service.rs` | Modify | Update to use gRPC client |
| `apps/yapper/src/infrastructure/middleware/rbac_middleware.rs` | Create | RBAC middleware |
| `apps/yapper/src/infrastructure/consent_manager.rs` | Create | Consent management adapter |
| `apps/yapper/src/domain/user_service.rs` | Modify | Add karma integration |
| `apps/yapper/src/api/handlers/auth_handler.rs` | Modify | Add consent handling |
| `apps/yapper/src/main.rs` | Modify | Initialize new services |

### Testing Strategy
1. **Unit Tests**:
   - Auth Service Client
   - RBAC Middleware
   - Consent Manager

2. **Integration Tests**:
   - Full authentication flow with gRPC mock
   - RBAC permission checks
   - Consent enforcement

3. **Load Testing**:
   - Session creation/validation under load
   - Concurrent RBAC checks

4. **Compatibility Tests**:
   - Verify existing OAuth providers still work
   - Backward compatibility with old clients

## Dependencies
### Add
- `cpc_auth`
- `cpc_rbac`
- `cpc_karma`
- `cpc_consent`
- `tonic`
- `prost`
- `redis`

### Remove
- Old session management implementation (after migration)

## Important Considerations
1. Maintain existing API endpoints for backward compatibility
2. Add new endpoints:
   - `POST /api/consent` - Update consent preferences
   - `GET /api/karma` - Get user karma score
3. All OAuth flows must be tested with new system
4. Event publishing must be maintained for all auth events