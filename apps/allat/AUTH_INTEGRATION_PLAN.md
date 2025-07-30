# Allat Authentication Integration Plan

## Overview
Integrate cpc_auth authentication system into Allat forum application, following the same pattern used in Yapper while accounting for Allat-specific features like community moderation and karma systems.

## File Modifications

### New Files
1. `domain/credentials.rs` - Re-exports Credentials from cpc_auth
2. `domain/user.rs` - Re-exports User and Role from cpc_auth
3. `domain/session.rs` - Re-exports Session from cpc_auth
4. `domain/auth_error.rs` - Re-exports AuthError from cpc_auth
5. `domain/auth_service.rs` - AuthService trait definition
6. `application/auth_service.rs` - AuthService implementation
7. `infrastructure/user_repository.rs` - User persistence
8. `infrastructure/session_repository.rs` - Session storage

### Modified Files
1. `Cargo.toml` - Add cpc_auth dependency
2. `domain/mod.rs` - Include new auth modules
3. `application/mod.rs` - Include auth_service
4. `infrastructure/mod.rs` - Include user/session repositories
5. `main.rs` - Initialize auth services

## Architectural Decisions

### User Model Extension
- Add `karma: i32` field to User model via new `AllatUser` struct:
  ```rust
  pub struct AllatUser {
      pub base: cpc_auth::models::User,
      pub karma: i32,
  }
  ```

### Role Enhancements
- Extend Role enum to include community-specific roles:
  ```rust
  pub enum Role {
      User,
      Moderator(Uuid), // Community ID
      Admin,
  }
  ```

### Karma Integration
- Implement karma tracking in AuthService:
  - Increment karma when posts receive upvotes
  - Decrement karma when posts receive downvotes
  - Add `update_karma(user_id: Uuid, delta: i32)` method

### Moderation Workflow
- Implement moderator authorization middleware:
  ```rust
  pub async fn moderator_only(
      session: Session,
      community_id: Uuid,
      auth_service: Arc<dyn AuthService>
  ) -> Result<(), AuthError> {
      let user = auth_service.validate_session(session.id).await?;
      if !user.roles.iter().any(|r| matches!(r, Role::Moderator(id) if *id == community_id)) {
          return Err(AuthError::PermissionDenied);
      }
      Ok(())
  }
  ```

### Session Persistence
- Use Redis for session storage via new SessionRepository
- Implement SessionService trait from cpc_auth

## Test Plan
1. Unit tests for:
   - User registration and login
   - Karma tracking
   - Moderator role verification
2. Integration tests:
   - Protected moderator endpoints
   - Session persistence
   - OAuth flow simulation
3. E2E tests:
   - Full user journey: registration → posting → moderation
   - Karma accumulation workflow

## Implementation Steps
1. Add cpc_auth dependency to Cargo.toml
2. Create new domain files for auth models
3. Implement UserRepository using PostgreSQL
4. Implement AuthService with karma tracking
5. Add session management middleware
6. Update main.rs to initialize auth services
7. Create test coverage
8. Update API endpoints to use auth middleware