# Allat Enhancement Architecture

## Overview

This document outlines the architectural plans for implementing four key missing features in the Allat app:
1. Advanced search functionality
2. Community analytics dashboard
3. Integration with notification system
4. Cross-posting to Yapper

These enhancements will follow hexagonal architecture principles and integrate with the existing CPC ecosystem.

## 1. Advanced Search Functionality

### Current State
The Allat app currently has placeholder search methods in both `CommunityService` and `PostService` that are not implemented.

### Requirements
- Full-text search for posts (title, content)
- Search communities by name and description
- Filtering by community, date range, and user
- Search result ranking based on relevance
- Pagination support

### Domain Layer Changes

#### Post Entity Enhancement
We'll add search-related fields to the Post entity to support indexing:

```rust
// src/domain/post.rs
impl Post {
    pub fn get_searchable_text(&self) -> String {
        format!("{} {}", self.title, self.content)
    }
}
```

#### Community Entity Enhancement
Similarly for Community:

```rust
// src/domain/community.rs
impl Community {
    pub fn get_searchable_text(&self) -> String {
        format!("{} {}", self.name, self.description)
    }
}
```

### Application Layer Changes

#### Search Service
We'll create a dedicated search service to handle search operations:

```rust
// src/application/search_service.rs
pub struct SearchCriteria {
    pub query: String,
    pub community_id: Option<Uuid>,
    pub author_id: Option<Uuid>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

#[async_trait]
pub trait SearchService: Send + Sync {
    async fn search_posts(&self, criteria: SearchCriteria) -> Result<Vec<Post>, ApplicationError>;
    async fn search_communities(&self, query: String) -> Result<Vec<Community>, ApplicationError>;
}

pub struct SearchServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    community_repo: Arc<dyn CommunityRepository>,
}
```

### Infrastructure Layer Changes

#### PostgreSQL Full-Text Search
We'll enhance the database schema to support full-text search:

1. Add a `tsvector` column to posts and communities tables
2. Create GIN indexes for efficient searching
3. Implement search queries using PostgreSQL's full-text search capabilities

```sql
-- Migration script
ALTER TABLE posts ADD COLUMN search_vector tsvector;
ALTER TABLE communities ADD COLUMN search_vector tsvector;

UPDATE posts SET search_vector = 
    setweight(to_tsvector('english', title), 'A') || 
    setweight(to_tsvector('english', content), 'B');

UPDATE communities SET search_vector = 
    setweight(to_tsvector('english', name), 'A') || 
    setweight(to_tsvector('english', description), 'B');

CREATE INDEX idx_posts_search ON posts USING GIN(search_vector);
CREATE INDEX idx_communities_search ON communities USING GIN(search_vector);

CREATE OR REPLACE FUNCTION update_post_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', NEW.title), 'A') || 
        setweight(to_tsvector('english', NEW.content), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_community_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', NEW.name), 'A') || 
        setweight(to_tsvector('english', NEW.description), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_post_search_vector_trigger
    BEFORE INSERT OR UPDATE ON posts
    FOR EACH ROW EXECUTE FUNCTION update_post_search_vector();

CREATE TRIGGER update_community_search_vector_trigger
    BEFORE INSERT OR UPDATE ON communities
    FOR EACH ROW EXECUTE FUNCTION update_community_search_vector();
```

#### Repository Enhancements
We'll add search methods to the existing repositories:

```rust
// src/infrastructure/repositories/post_repo.rs
#[async_trait]
pub trait PostRepository: Send + Sync {
    // ... existing methods
    async fn search(&self, criteria: SearchCriteria) -> Result<Vec<Post>, PostRepositoryError>;
}

// Implementation in PgPostRepository
async fn search(&self, criteria: SearchCriteria) -> Result<Vec<Post>, PostRepositoryError> {
    // Implementation using PostgreSQL full-text search
}
```

```rust
// src/infrastructure/repositories/community_repo.rs
#[async_trait]
pub trait CommunityRepository: Send + Sync {
    // ... existing methods
    async fn search(&self, query: String) -> Result<Vec<Community>, CommunityRepositoryError>;
}
```

### API Layer Changes

#### GraphQL Schema Extensions
We'll extend the GraphQL schema to support advanced search:

```graphql
extend type Query {
    searchPosts(criteria: SearchCriteriaInput!): [Post!]!
    searchCommunities(query: String!): [Community!]!
}

input SearchCriteriaInput {
    query: String!
    communityId: ID
    authorId: ID
    dateFrom: DateTime
    dateTo: DateTime
    limit: Int
    offset: Int
}
```

## 2. Community Analytics Dashboard

### Requirements
- Community growth metrics (member count over time)
- Post and comment activity trends
- Top contributors identification
- Popular topics/tags analysis
- Engagement metrics (upvotes/downvotes)
- Export capabilities for reports

### Domain Layer Changes

#### Analytics Entities
We'll create analytics-specific entities:

```rust
// src/domain/analytics.rs
#[derive(Debug, Clone)]
pub struct CommunityGrowthMetrics {
    pub community_id: Uuid,
    pub member_count_history: Vec<TimeSeriesPoint>,
    pub post_count_history: Vec<TimeSeriesPoint>,
    pub comment_count_history: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Clone)]
pub struct UserContributionMetrics {
    pub user_id: Uuid,
    pub username: String,
    pub post_count: u32,
    pub comment_count: u32,
    pub karma: i32,
}

#[derive(Debug, Clone)]
pub struct EngagementMetrics {
    pub post_id: Uuid,
    pub title: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub comment_count: u32,
    pub engagement_score: f64,
}

#[derive(Debug, Clone)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: i32,
}
```

### Application Layer Changes

#### Analytics Service
We'll create an analytics service to compute metrics:

```rust
// src/application/analytics_service.rs
#[async_trait]
pub trait AnalyticsService: Send + Sync {
    async fn get_community_growth_metrics(&self, community_id: Uuid, days: u32) -> Result<CommunityGrowthMetrics, ApplicationError>;
    async fn get_top_contributors(&self, community_id: Uuid, limit: u32) -> Result<Vec<UserContributionMetrics>, ApplicationError>;
    async fn get_top_engaged_posts(&self, community_id: Uuid, limit: u32) -> Result<Vec<EngagementMetrics>, ApplicationError>;
    async fn get_community_overview(&self, community_id: Uuid) -> Result<CommunityOverview, ApplicationError>;
}

pub struct AnalyticsServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    community_repo: Arc<dyn CommunityRepository>,
    // We might need to add a user repository dependency
}
```

### Infrastructure Layer Changes

#### Analytics Repository
We'll need to create specialized repository methods for analytics queries:

```rust
// src/infrastructure/repositories/analytics_repo.rs
#[async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn get_member_growth_history(&self, community_id: Uuid, days: u32) -> Result<Vec<TimeSeriesPoint>, RepositoryError>;
    async fn get_post_activity_history(&self, community_id: Uuid, days: u32) -> Result<Vec<TimeSeriesPoint>, RepositoryError>;
    async fn get_top_contributors(&self, community_id: Uuid, limit: u32) -> Result<Vec<UserContributionMetrics>, RepositoryError>;
    async fn get_engagement_metrics(&self, community_id: Uuid, limit: u32) -> Result<Vec<EngagementMetrics>, RepositoryError>;
}
```

#### Database Queries for Analytics
We'll implement complex SQL queries to compute analytics:

```sql
-- Example query for member growth history
SELECT 
    DATE(created_at) as date,
    COUNT(*) as new_members
FROM community_members 
WHERE community_id = $1 
    AND created_at >= CURRENT_DATE - INTERVAL '$2 days'
GROUP BY DATE(created_at)
ORDER BY date;

-- Example query for top contributors
SELECT 
    u.id,
    u.username,
    COUNT(p.id) as post_count,
    COUNT(c.id) as comment_count,
    u.karma
FROM users u
LEFT JOIN posts p ON u.id = p.user_id AND p.community_id = $1
LEFT JOIN posts c ON u.id = c.user_id AND c.community_id = $1 AND c.parent_id IS NOT NULL
WHERE u.id IN (
    SELECT DISTINCT user_id 
    FROM posts 
    WHERE community_id = $1
)
GROUP BY u.id, u.username, u.karma
ORDER BY (COUNT(p.id) + COUNT(c.id)) DESC
LIMIT $2;
```

### Integration with Visualization Packages
We'll leverage the existing visualization capabilities in the CPC ecosystem:

1. Use `shared_packages/bi_visualization` for chart generation
2. Use `shared_packages/plotters` for data visualization
3. Use `shared_packages/reviews/src/analytics.rs` as a reference implementation

### API Layer Changes

#### GraphQL Schema for Analytics
We'll extend the GraphQL schema with analytics queries:

```graphql
extend type Community {
    analytics(days: Int = 30): CommunityAnalytics!
}

type CommunityAnalytics {
    growthMetrics: CommunityGrowthMetrics!
    topContributors(limit: Int = 10): [UserContributionMetrics!]!
    topEngagedPosts(limit: Int = 10): [EngagementMetrics!]!
    overview: CommunityOverview!
}

type CommunityGrowthMetrics {
    memberCountHistory: [TimeSeriesPoint!]!
    postCountHistory: [TimeSeriesPoint!]!
    commentCountHistory: [TimeSeriesPoint!]!
}

type UserContributionMetrics {
    userId: ID!
    username: String!
    postCount: Int!
    commentCount: Int!
    karma: Int!
}

type EngagementMetrics {
    postId: ID!
    title: String!
    upvotes: Int!
    downvotes: Int!
    commentCount: Int!
    engagementScore: Float!
}

type CommunityOverview {
    totalMembers: Int!
    totalPosts: Int!
    totalComments: Int!
    averageKarma: Int!
    mostActiveDay: DateTime!
}

type TimeSeriesPoint {
    timestamp: DateTime!
    value: Int!
}
```

## 3. Notification System Integration

### Requirements
- Notify users of replies to their posts/comments
- Notify moderators of reported content
- Notify users of upvotes on their content
- Notify community owners of new posts
- Real-time delivery via WebSocket
- Configurable notification preferences

### Domain Layer Changes

#### Notification Events
We'll define domain events for notifications:

```rust
// src/domain/notification_events.rs
#[derive(Debug, Clone)]
pub enum NotificationEvent {
    PostReply {
        post_id: Uuid,
        post_title: String,
        replier_id: Uuid,
        replier_name: String,
        community_id: Uuid,
        community_name: String,
    },
    CommentReply {
        comment_id: Uuid,
        parent_comment_id: Uuid,
        replier_id: Uuid,
        replier_name: String,
        post_id: Uuid,
        post_title: String,
    },
    PostUpvoted {
        post_id: Uuid,
        post_title: String,
        voter_id: Uuid,
        voter_name: String,
        upvote_count: u32,
    },
    CommentUpvoted {
        comment_id: Uuid,
        voter_id: Uuid,
        voter_name: String,
        upvote_count: u32,
    },
    NewPostInCommunity {
        post_id: Uuid,
        post_title: String,
        author_id: Uuid,
        author_name: String,
        community_id: Uuid,
        community_name: String,
    },
    ContentReported {
        content_id: Uuid,
        content_type: ContentType,
        reporter_id: Uuid,
        reporter_name: String,
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub enum ContentType {
    Post,
    Comment,
}
```

### Application Layer Changes

#### Notification Service
We'll create a service to handle notification events:

```rust
// src/application/notification_service.rs
#[async_trait]
pub trait NotificationService: Send + Sync {
    async fn handle_event(&self, event: NotificationEvent) -> Result<(), ApplicationError>;
}

pub struct NotificationServiceImpl {
    notification_core_service: Arc<dyn NotificationCoreService>,
    user_repo: Arc<dyn UserRepository>,
}
```

### Infrastructure Layer Changes

#### Integration with Notification Core
We'll integrate with the existing `notification_core` shared package:

```rust
// src/infrastructure/notification_adapter.rs
use notification_core::application::service::NotificationService as CoreNotificationService;
use notification_core::domain::types::{Notification, NotificationCategory, NotificationPriority, ChannelType};

pub struct NotificationAdapter {
    core_service: Arc<dyn CoreNotificationService>,
}

impl NotificationAdapter {
    pub async fn send_notification(&self, event: NotificationEvent) -> Result<(), ApplicationError> {
        match event {
            NotificationEvent::PostReply { post_id, post_title, replier_id, replier_name, community_id, community_name } => {
                let notification = Notification::new_immediate(
                    post_id.to_string(), // user_id would be the post author
                    NotificationCategory::Social,
                    NotificationPriority::Normal,
                    format!("{} replied to your post", replier_name),
                    format!("{} replied to your post '{}' in {}", replier_name, post_title, community_name),
                    serde_json::json!({
                        "post_id": post_id,
                        "replier_id": replier_id,
                        "community_id": community_id,
                        "type": "post_reply"
                    }),
                );
                
                self.core_service.send(notification).await
                    .map_err(|e| ApplicationError::ServiceError(format!("Failed to send notification: {}", e)))?;
            }
            // ... other event types
        }
        Ok(())
    }
}
```

#### Event Handlers
We'll modify existing services to publish notification events:

```rust
// In PostService::create_comment
// After successfully creating a comment:
if let Some(parent_comment) = parent_comment {
    let event = NotificationEvent::CommentReply {
        comment_id: comment.id,
        parent_comment_id: parent_comment.id,
        replier_id: comment.user_id,
        replier_name: user.username, // Need to fetch user
        post_id: comment.post_id,
        post_title: post.title, // Need to fetch post
    };
    self.notification_service.handle_event(event).await?;
}
```

### API Layer Changes

#### WebSocket Integration
We'll enhance the existing WebSocket implementation to handle real-time notifications:

```rust
// src/api/subscriptions.rs
// Extend with notification subscriptions
```

## 4. Cross-Posting to Yapper

### Requirements
- Allow users to cross-post Allat posts to Yapper
- Maintain link back to original Allat post
- Handle media assets correctly
- Track cross-post relationships
- Support cross-posting from Yapper to Allat as well

### Domain Layer Changes

#### Cross-Post Entities
We'll define entities to track cross-posting relationships:

```rust
// src/domain/cross_post.rs
#[derive(Debug, Clone)]
pub struct CrossPost {
    pub id: Uuid,
    pub source_app: SocialApp,
    pub source_id: Uuid,
    pub target_app: SocialApp,
    pub target_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub enum SocialApp {
    Allat,
    Yapper,
}
```

### Application Layer Changes

#### Cross-Post Service
We'll create a service to handle cross-posting:

```rust
// src/application/cross_post_service.rs
pub struct CrossPostInput {
    pub source_post_id: Uuid,
    pub target_community_id: Option<Uuid>, // For cross-posting to Allat
    pub user_id: Uuid,
}

#[async_trait]
pub trait CrossPostService: Send + Sync {
    async fn cross_post_to_yapper(&self, input: CrossPostInput) -> Result<Uuid, ApplicationError>;
    async fn cross_post_to_allat(&self, input: CrossPostInput) -> Result<Uuid, ApplicationError>;
    async fn get_cross_posts(&self, source_id: Uuid, source_app: SocialApp) -> Result<Vec<CrossPost>, ApplicationError>;
}

pub struct CrossPostServiceImpl {
    social_integration_service: Arc<dyn SocialIntegrationService>,
    post_repo: Arc<dyn PostRepository>,
    cross_post_repo: Arc<dyn CrossPostRepository>,
}
```

### Infrastructure Layer Changes

#### Integration with Social Integration Package
We'll integrate with the existing `social_integration` package:

```rust
// src/infrastructure/cross_post_adapter.rs
use social_integration::CrossPostingService;

pub struct CrossPostAdapter {
    cross_posting_service: CrossPostingService,
}

impl CrossPostAdapter {
    pub fn new() -> Self {
        Self {
            cross_posting_service: CrossPostingService::new(),
        }
    }
    
    pub fn cross_post_to_yapper(&self, allat_post_id: Uuid, user_id: Uuid) -> Result<Uuid, ApplicationError> {
        // In a real implementation, this would:
        // 1. Fetch the Allat post
        // 2. Transform it to Yapper format
        // 3. Call the Yapper API to create the post
        // 4. Return the Yapper post ID
        
        self.cross_posting_service.cross_post_to_yapper(allat_post_id, user_id)
            .map_err(|e| ApplicationError::ServiceError(format!("Cross-posting failed: {}", e)))?;
        
        // For now, we'll return a placeholder
        Ok(Uuid::new_v4())
    }
}
```

#### Cross-Post Repository
We'll create a repository to track cross-post relationships:

```rust
// src/infrastructure/repositories/cross_post_repo.rs
#[async_trait]
pub trait CrossPostRepository: Send + Sync {
    async fn create(&self, cross_post: &CrossPost) -> Result<(), RepositoryError>;
    async fn find_by_source(&self, source_id: Uuid, source_app: SocialApp) -> Result<Vec<CrossPost>, RepositoryError>;
    async fn find_by_target(&self, target_id: Uuid, target_app: SocialApp) -> Result<Vec<CrossPost>, RepositoryError>;
}
```

#### Database Schema for Cross-Posts
We'll add a table to track cross-post relationships:

```sql
-- Migration script
CREATE TABLE cross_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_app VARCHAR(20) NOT NULL, -- 'allat' or 'yapper'
    source_id UUID NOT NULL,
    target_app VARCHAR(20) NOT NULL, -- 'allat' or 'yapper'
    target_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    CONSTRAINT chk_source_app CHECK (source_app IN ('allat', 'yapper')),
    CONSTRAINT chk_target_app CHECK (target_app IN ('allat', 'yapper'))
);

CREATE INDEX idx_cross_posts_source ON cross_posts(source_app, source_id);
CREATE INDEX idx_cross_posts_target ON cross_posts(target_app, target_id);
CREATE INDEX idx_cross_posts_user ON cross_posts(user_id);
```

### API Layer Changes

#### GraphQL Schema Extensions
We'll extend the GraphQL schema to support cross-posting:

```graphql
extend type Post {
    crossPosts: [CrossPostConnection!]!
}

extend type Mutation {
    crossPostToYapper(allatPostId: ID!): YapperPost!
    crossPostToAllat(yapperPostId: ID!, communityId: ID!): Post!
}

type CrossPostConnection {
    allatPost: Post
    yapperPost: YapperPost
    originalPlatform: SocialPlatform!
}

enum SocialPlatform {
    ALLAT
    YAPPER
}
```

## Integration Points

### Event Bus Usage
We'll leverage the existing event bus for notification publishing:

```rust
// In infrastructure/event_bus.rs
impl EventBus {
    pub fn publish_notification(&self, event: NotificationEvent) -> Result<(), String> {
        // Serialize event and publish to event bus
        let event_json = serde_json::to_string(&event)
            .map_err(|e| format!("Failed to serialize event: {}", e))?;
        
        self.publish(&format!("notification:{}", event_json))
    }
}
```

### Shared Packages Utilization
We'll leverage several shared packages:
1. `notification_core` - For notification delivery
2. `social_integration` - For cross-app integration
3. `bi_visualization` - For analytics dashboard charts
4. `plotters` - For data visualization

## Security Considerations

1. **Access Control**: Ensure only authorized users can trigger cross-posting
2. **Rate Limiting**: Implement rate limits on search and cross-posting operations
3. **Data Privacy**: Respect user privacy settings when sharing content
4. **Input Validation**: Validate all inputs to prevent injection attacks

## Performance Considerations

1. **Caching**: Cache frequently accessed analytics data
2. **Indexing**: Proper database indexing for search operations
3. **Pagination**: Implement pagination for large result sets
4. **Async Processing**: Use background jobs for heavy analytics computations

## Testing Strategy

1. **Unit Tests**: Test each service method in isolation
2. **Integration Tests**: Test database interactions and cross-app integrations
3. **End-to-End Tests**: Test complete user flows through the API
4. **Load Tests**: Ensure performance under high load conditions

## Deployment Considerations

1. **Database Migrations**: Ensure smooth migration of existing data
2. **Backward Compatibility**: Maintain API compatibility during rollout
3. **Monitoring**: Add metrics for search performance and notification delivery
4. **Rollback Plan**: Prepare rollback procedures for each enhancement

## Future Enhancements

1. **Machine Learning**: Implement recommendation systems for communities
2. **Advanced Analytics**: Add predictive analytics for community growth
3. **Mobile Integration**: Optimize for mobile cross-posting
4. **Social Features**: Add more social interaction capabilities