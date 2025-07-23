# Identity Management Migration to Shared Rust Backend

## Background
We're migrating identity functionality from the old Android implementation to our shared Rust backend to enable cross-platform support.

## Requirements
1. User authentication (registration/login)
2. Profile management (CRUD operations)
3. Social features (friends/following)
4. JWT token handling
5. GraphQL API for UI clients
6. Database schema for identity data

## Architecture Decisions

### Data Model Extensions
```rust
// In packages/cpc-core/src/models/user.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    // Existing fields...
    pub display_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub friends: Vec<uuid::Uuid>, // List of friend IDs
    pub followers: Vec<uuid::Uuid>, // List of follower IDs
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewUser {
    // Existing fields...
    pub display_name: Option<String>,
}
```

### Service Layer
- **AuthService**: Handles registration, login, token generation
- **UserService**: Manages profile CRUD operations
- **SocialService**: Handles friend/follower relationships

### API Design
**GraphQL Schema:**
```graphql
type User {
    id: ID!
    username: String!
    email: String!
    displayName: String
    bio: String
    avatarUrl: String
    friends: [User!]!
    followers: [User!]!
    createdAt: DateTime!
    updatedAt: DateTime!
}

type AuthPayload {
    token: String!
    user: User!
}

input RegisterInput {
    username: String!
    email: String!
    password: String!
    displayName: String
}

input LoginInput {
    email: String!
    password: String!
}

type Mutation {
    register(input: RegisterInput!): AuthPayload!
    login(input: LoginInput!): AuthPayload!
    updateProfile(displayName: String, bio: String, avatarUrl: String): User!
    addFriend(friendId: ID!): User!
    removeFriend(friendId: ID!): User!
}

type Query {
    me: User!
    user(id: ID!): User
}
```

### Database Schema
```sql
-- Add to existing users table
ALTER TABLE users ADD COLUMN display_name TEXT;
ALTER TABLE users ADD COLUMN bio TEXT;
ALTER TABLE users ADD COLUMN avatar_url TEXT;
ALTER TABLE users ADD COLUMN friends JSONB DEFAULT '[]'::jsonb;
ALTER TABLE users ADD COLUMN followers JSONB DEFAULT '[]'::jsonb;
```

## Implementation Plan

### Phase 1: Core Service Implementation
1. Implement AuthService with JWT token handling
2. Create UserService with profile management
3. Build SocialService for relationship management

### Phase 2: GraphQL Integration
1. Create identity query and mutation resolvers
2. Add to backend GraphQL schema
3. Implement data loaders for relationships

### Phase 3: Android Integration
1. Update Android app to use shared GraphQL API
2. Remove old Kotlin identity implementation