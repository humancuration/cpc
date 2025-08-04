# Social Graph GraphQL API

This document describes the GraphQL API provided by the social_graph package.

## Schema Overview

The GraphQL schema provides queries for social interactions within the CPC ecosystem.

## Queries

### getFriends

Get the friends of a specific user.

```graphql
getFriends(user_id: String!): [User!]!
```

**Arguments:**
- `user_id`: The ID of the user to get friends for

**Returns:**
- An array of User objects representing the user's friends

### getActivityFeed

Get the universal activity feed for a specific user with filtering and pagination.

```graphql
getActivityFeed(
  userId: ID!
  after: String
  limit: Int = 20
  filters: [FeedFilter!]
): [ActivityFeedItem!]!
```

**Arguments:**
- `userId`: The ID of the user to get the activity feed for
- `after`: Cursor for pagination (timestamp + ID)
- `limit`: Maximum number of items to return (default: 20)
- `filters`: Filters to apply to the feed

**Returns:**
- An array of ActivityFeedItem objects representing the user's universal activity feed

### getRecommendations

Get recommended users for a specific user.

```graphql
getRecommendations(user_id: String!): [User!]!
```

**Arguments:**
- `user_id`: The ID of the user to get recommendations for

**Returns:**
- An array of User objects representing recommended users

## Types

### User

Represents a user in the social graph.

```graphql
type User {
  id: ID!
  username: String!
  displayName: String!
  email: String!
  createdAt: String!
  updatedAt: String!
  isActive: Boolean!
}
```

### Activity

Represents a user activity or interaction.

```graphql
type Activity {
  id: ID!
  userId: ID!
  activityType: ActivityType!
  targetId: ID
  targetType: String
  metadata: JSON
  createdAt: String!
  isPublic: Boolean!
}
```

### ActivityType

Enumeration of possible activity types.

```graphql
enum ActivityType {
  PROFILE_VIEW
  POST_CREATED
  POST_LIKED
  COMMENTED
  SHARED
  FOLLOWED
  UNFOLLOWED
  JOINED_GROUP
  LEFT_GROUP
}
```

### Relationship

Represents a relationship between two users.

```graphql
type Relationship {
  id: ID!
  sourceUserId: ID!
  targetUserId: ID!
  relationshipType: RelationshipType!
  createdAt: String!
  updatedAt: String!
  isActive: Boolean!
}
```

### RelationshipType

Enumeration of possible relationship types.

```graphql
enum RelationshipType {
  FRIEND
  FOLLOWER
  BLOCKED
  PENDING
}
```

### ActivityFeedItem

Represents a unified content item in the universal feed.

```graphql
type ActivityFeedItem {
  id: ID!
  contentType: ContentType!
  package: String!
  content: JSON!
  timestamp: String!
  visibility: Visibility!
}
```

### ContentType

Enumeration of possible content types in the universal feed.

```graphql
enum ContentType {
  SOCIAL_POST
  VIDEO
  JOB_POSTING
  COURSE_SNIPPET
  BUSINESS_PLAN
  COMMUNITY_EVENT
  CUSTOM
}
```

### Visibility

Enumeration of possible visibility levels for content.

```graphql
enum Visibility {
  PUBLIC
  FRIENDS_ONLY
  GROUP_MEMBERS
  PRIVATE
}
```

### FeedFilter

Filter criteria for the universal feed.

```graphql
input FeedFilter {
  contentType: ContentType
  package: String
  visibility: Visibility
}
```

## Usage Example

```graphql
query {
  getActivityFeed(
    userId: "123e4567-e89b-12d3-a456-426614174000"
    limit: 10
    filters: [
      { contentType: SOCIAL_POST }
      { visibility: PUBLIC }
    ]
  ) {
    id
    contentType
    package
    content
    timestamp
    visibility
  }
}
```

## Integration

To use the GraphQL schema in your application:

```rust
use social_graph::{create_schema, SocialGraphSchema};

let schema = create_schema();

// Use with async-graphql server
// For example, with axum:
// let schema = schema.into_inner();
// Router::new().route("/graphql", post(graphql(schema)))