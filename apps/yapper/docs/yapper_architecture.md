# Yapper Microblog Architecture

## Overview
Yapper is a Twitter-style microblogging platform with character-limited posts, real-time feeds, and social engagement features. It follows screaming architecture principles organized by features.

## Core Components
- **FeedService**: Algorithmic/chronological feeds with performance optimization
- **PostService**: Micro-post creation with media attachments (280 character limit)
- **HashtagService**: Trend discovery and hashtag tracking
- **EngagementService**: Like/share tracking and engagement metrics
- **MessagingService**: Direct messages (integrated with Messenger)

## Optimization Strategies
- **Sled caching**: For feed generation and user profiles
- **Bevy visualization**: For engagement metrics and trend analysis
- **Horizontal scalability**: For handling large volumes of posts and users

## Architectural Layers

### Domain Layer
- **Post**: Micro-post entity with content, media, and engagement metrics
- **User**: User profile with display name, bio, and avatar
- **Hashtag**: Hashtag entity for trend tracking
- **Engagement**: Like and share tracking

### Application Layer
- **FeedService**: Generate personalized feeds with algorithmic and chronological options
- **PostService**: Create and manage micro-posts with character limit enforcement
- **HashtagService**: Track and analyze hashtag trends
- **EngagementService**: Track likes, shares, and other engagement metrics

### Infrastructure Layer
- **Sled Storage**: Edge caching for feed generation and user profiles
- **PostgreSQL Repository**: Persistent storage for posts and user data
- **WebSocket Server**: Real-time feed updates
- **Bevy Engine**: Visualization of engagement metrics and trends

## Integration Points
- **Identity**: Uses `cpc_oauth2` for authentication and unified identity
- **Messenger**: Direct messaging capability through Messenger integration
- **Media**: Integrates with media processing pipeline for image attachments
- **Task Manager**: Integration with dabloons system for tipping and rewards
- **Consent Manager**: Privacy controls for post visibility and data sharing

## API Contracts

### GraphQL Schema
```graphql
type YapperPost {
  id: ID!
  content: String!
  author: User!
  timestamp: DateTime!
  likes: Int!
  shares: Int!
  media: [MediaAsset!]!
  hashtags: [String!]!
}

type User {
  id: ID!
  displayName: String!
  username: String!
  bio: String
  avatarUrl: String
  following: [User!]!
  followers: [User!]!
}

type Feed {
  posts: [YapperPost!]!
  nextCursor: String
}

type HashtagTrend {
  tag: String!
  count: Int!
  trending: Boolean!
}
```

## Performance Considerations
- Use Sled for edge caching of user feeds and profiles
- Implement efficient feed generation algorithms
- Use database partitioning for large datasets
- Optimize media processing pipeline for quick uploads

## Security Considerations
- Content moderation for harmful posts
- Privacy controls for post visibility
- Rate limiting for post creation and engagement
- Secure direct messaging through Messenger integration

## TODO
- [ ] Implement advanced feed algorithms
- [ ] Add trending topics visualization with Bevy
- [ ] Integrate with notification system
- [ ] Implement cross-posting to Allat