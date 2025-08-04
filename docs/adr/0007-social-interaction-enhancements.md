# ADR 0007: Social Interaction Enhancements

## Status
Proposed

## Date
2025-08-03

## Context
To increase user engagement and foster a more community-driven platform, we need to enhance our social features with reactions, comments, content sharing, and customizable notification preferences. Currently, our platform has basic social capabilities through the social_enhancements package, but lacks interactive features that are common in modern social platforms.

Key challenges include:
- Limited user interaction capabilities beyond automatic social feed posts
- No way for users to express reactions to content
- No threaded commenting system
- No content sharing between users
- Limited notification customization for social activities

## Decision
We will implement a comprehensive social interaction system with the following components:

### 1. Database Schema
New tables to support social interactions:

```sql
-- Reactions table
CREATE TABLE reactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    target_id UUID NOT NULL,  -- ID of post/comment being reacted to
    target_type VARCHAR(20) NOT NULL,  -- 'post' or 'comment'
    reaction_type VARCHAR(20) NOT NULL,  -- 'like', 'heart', 'celebrate', etc.
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Comments table
CREATE TABLE comments (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    parent_id UUID,  -- For nested comments
    target_id UUID NOT NULL,  -- ID of post being commented on
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ
);

-- Shares table
CREATE TABLE shares (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    content_id UUID NOT NULL,  -- ID of shared content
    content_type VARCHAR(50) NOT NULL,  -- 'post', 'achievement', etc.
    shared_with UUID,  -- User ID or null for public
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

### 2. Hexagonal Architecture Structure
Create a new `social_interactions` package with vertical slices:

```
src/
├── domain/
│   ├── models.rs          # Reaction, Comment, Share entities
│   ├── repository.rs      # Trait definitions for repositories
│   └── service.rs         # Trait definitions for services
├── application/
│   ├── reaction_service.rs
│   ├── comment_service.rs
│   ├── share_service.rs
│   └── notification_integration.rs
├── infrastructure/
│   ├── postgres_repository.rs
│   ├── in_memory_repository.rs # For tests
│   └── event_bus.rs       # For real-time updates
└── lib.rs                 # Public API
```

### 3. Core Components

#### Reaction Service
- `add_reaction(user_id, target, reaction_type)`
- `remove_reaction(reaction_id)`
- `get_reactions(target_id)`

#### Comment Service
- `add_comment(user_id, target_id, content, parent_id)`
- `edit_comment(comment_id, new_content)`
- `delete_comment(comment_id)`
- `get_comments(target_id, depth)`

#### Share Service
- `share_content(user_id, content_id, content_type, visibility)`
- `get_shared_content(user_id)`

#### Notification Integration
Extend `notification_core` with new event types:
- `NotificationCategory::SocialReaction`
- `NotificationCategory::NewComment`
- `NotificationCategory::ContentShare`

### 4. API Endpoints (GraphQL)
Add new GraphQL mutations and queries in `apps/api_server/src/graphql/social_interactions.rs`:

```rust
#[derive(InputObject)]
struct AddReactionInput {
    target_id: Uuid,
    target_type: String,
    reaction_type: String,
}

#[Object]
impl SocialInteractionMutations {
    async fn add_reaction(&self, ctx: &Context<'_>, input: AddReactionInput) -> Result<Reaction> {
        // Implementation
    }
    
    // Similar for comments and shares
}
```

### 5. Notification Preferences
Extend `notification_core` preferences:

```rust
// In domain/preferences.rs
pub struct UserPreferences {
    pub social_preferences: SocialPreferences,
    // ... existing fields
}

pub struct SocialPreferences {
    pub reaction_notifications: bool,
    pub comment_notifications: bool,
    pub share_notifications: bool,
    // ... other preferences
}
```

## Consequences

### Positive
- Enhanced user engagement through interactive social features
- Improved community building capabilities
- Better notification customization for social activities
- Consistent hexagonal architecture implementation
- Real-time updates for social interactions
- Comprehensive test coverage with in-memory repositories

### Negative
- Additional complexity in the codebase
- Increased database storage requirements
- More notification types to manage
- Additional API endpoints to maintain

### Neutral
- Requires updates to existing GraphQL schema
- New dependencies for real-time event handling
- Extended testing scenarios

## Implementation Details

### Core Components
1. **SocialInteractions Package**: New shared package for social features
2. **Database Schema**: New tables for reactions, comments, and shares
3. **Notification Integration**: Extended notification types and preferences
4. **GraphQL API**: New endpoints for social interactions
5. **Event Bus**: Real-time updates for social activities

### Integration Points
- Social enhancements service: For existing achievement and challenge features
- Notification service: For sending social activity notifications
- Volunteer service: For sharing volunteer activities
- Skill exchange service: For sharing skill exchanges
- Wallet service: For sharing financial achievements

### Testing Strategy
- Unit tests for all new services and repositories
- Integration tests for social interaction flows
- Performance tests for large comment threads
- Cross-service integration tests for notification flows
- Real-time update tests with event bus

## Security Considerations
- Proper authorization checks for all social interactions
- Rate limiting for reactions and comments to prevent spam
- Content validation for comments to prevent XSS attacks
- Privacy controls for content sharing
- Secure event handling for real-time updates

## Future Considerations
- Rich text formatting for comments
- Comment moderation tools
- Advanced sharing options (groups, circles)
- Social analytics and insights
- Integration with p2panda network for distributed social features