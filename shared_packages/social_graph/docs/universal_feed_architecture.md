# Universal Feed Architecture Design

## Overview
The universal feed aggregates and displays multiple content types from across the CPC ecosystem while respecting consent and privacy settings. The system is designed to be extensible, allowing new content types to be added without major architectural changes.

## Core Components

### 1. Content Aggregation System
```mermaid
graph TD
    A[Content Providers] --> B[Feed Aggregator]
    B --> C[Consent Manager]
    C --> D[Ranking Engine]
    D --> E[Feed Delivery]
```

#### Content Provider Interface
```rust
trait ContentProvider {
    fn content_type() -> ContentType;
    fn get_content(after: DateTime<Utc>, limit: usize) -> Vec<ContentItem>;
    fn should_include(item: &ContentItem, user_id: Uuid) -> bool;
}
```

### 2. Unified Data Model

#### ContentItem
```rust
pub enum ContentType {
    SocialPost,
    Video,
    JobPosting,
    CourseSnippet,
    BusinessPlan,
    CommunityEvent,
    Custom(String),
}

pub struct ContentItem {
    pub id: Uuid,
    pub content_type: ContentType,
    pub source_package: String,
    pub metadata: JsonValue,
    pub timestamp: DateTime<Utc>,
    pub visibility: Visibility,
    pub relevance_score: f32,
}

pub enum Visibility {
    Public,
    FriendsOnly,
    GroupMembers,
    Private,
}
```

### 3. Consent Integration
- All content items pass through consent_manager before display
- Three-tier consent check:
  1. Package-level consent
  2. Content-type consent
  3. Item-specific consent (via provider's `should_include`)

### 4. Ranking & Filtering
- Pluggable ranking system with default algorithms:
  - Chronological
  - Relevance (engagement-based)
  - Personalized (ML-based)
- User-defined filters:
  - By content type
  - By source package
  - By time range

### 5. Performance Considerations
- **Pagination**: cursor-based using timestamp + UUID
- **Caching**: 
  - Sled for edge caching of feed segments
  - Redis for hot content caching
- **Lazy Loading**:
  - Placeholders for rich media
  - Progressive content hydration

### 6. Database Schema Changes
```sql
ALTER TABLE social_activities
    ADD COLUMN content_type VARCHAR(50) NOT NULL DEFAULT 'SocialPost',
    ADD COLUMN source_package VARCHAR(255) NOT NULL DEFAULT 'social_graph',
    ADD COLUMN visibility VARCHAR(20) NOT NULL DEFAULT 'Public',
    ALTER COLUMN metadata TYPE JSONB USING metadata::jsonb;
```

### 7. Extended GraphQL API
```graphql
type ActivityFeedItem {
    id: ID!
    contentType: ContentType!
    package: String!
    content: Json!
    timestamp: String!
    visibility: Visibility!
}

enum ContentType {
    SOCIAL_POST
    VIDEO
    JOB_POSTING
    COURSE_SNIPPET
    BUSINESS_PLAN
    COMMUNITY_EVENT
    CUSTOM
}

enum Visibility {
    PUBLIC
    FRIENDS_ONLY
    GROUP_MEMBERS
    PRIVATE
}

extend type Query {
    getActivityFeed(
        userId: ID!
        after: String
        limit: Int = 20
        filters: [FeedFilter!]
    ): [ActivityFeedItem!]!
}

input FeedFilter {
    contentType: ContentType
    package: String
    visibility: Visibility
}
```

## Integration Points
1. **Content Providers**:
   - Each package implements ContentProvider trait
   - Register providers via dependency injection

2. **Consent Manager**:
   - Central consent verification point
   - Handles GDPR-compliant data filtering

3. **Federation**:
   - p2panda integration for decentralized content
   - Content signatures for verification

## Implementation Roadmap
1. Update domain model and database schema
2. Implement content provider interface
3. Build feed aggregation service
4. Add consent integration
5. Implement ranking algorithms
6. Extend GraphQL API
7. Add performance optimizations