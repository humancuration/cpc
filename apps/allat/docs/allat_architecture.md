# Allat Forum Architecture

## Overview
Allat is a decentralized forum application inspired by Reddit, following hexagonal architecture principles with a focus on community-driven content and integration with the CPC ecosystem.

## Core Components
- **CommunityService**: Manages communities/subreddits with customizable rules
- **PostService**: Handles post creation with rich text and media support
- **ThreadService**: Manages threaded conversations with nesting (up to 10 levels)
- **VoteService**: Voting system (upvote/downvote) with karma tracking
- **ModerationService**: Content moderation tools including post removal and user bans
- **SearchService**: Advanced search functionality for posts and communities
- **NotificationService**: Notification system for social interactions

## Integration Points
- **Identity**: Uses `cpc_oauth2` for authentication and unified identity
- **Media**: Integrates with media processing pipeline for image/video attachments
- **Task Manager**: Tracks rewards via dabloons system for content creators
- **Realtime**: WebSocket updates for new posts and comments
- **Consent Manager**: Privacy controls for data sharing

## Data Flow
```
User → Create Post → Media Processing → Store Post → Update Feed → Reward User
```

## Architectural Layers

### Domain Layer
- **Community**: Community entity with rules and settings
- **Post**: Post entity with content, media, and metadata
- **Vote**: Vote entity with upvote/downvote tracking
- **User**: Extended user model with karma field

### Application Layer
- **CommunityService**: Create, update, and manage communities
- **PostService**: Create and manage posts with media attachments
- **VoteService**: Handle voting and update karma accordingly
- **ModerationService**: Enforce community rules and content policies
- **SearchService**: Implement advanced search functionality for posts and communities
- **NotificationService**: Handle notification events and integrate with the notification core service

### Infrastructure Layer
- **PostgreSQL Repository**: Persistent storage for communities, posts, and votes
- **Redis Cache**: Caching for frequently accessed content
- **WebSocket Server**: Real-time updates for new content
- **gRPC Client**: Integration with media processing pipeline
- **PostgreSQL Full-Text Search**: Implementation of advanced search using PostgreSQL's tsvector
- **Notification Core Adapter**: Integration with the CPC notification system

## API Contracts

### GraphQL Schema
```graphql
type Community {
  id: ID!
  name: String!
  description: String!
  rules: [String!]!
  moderators: [User!]!
  createdAt: DateTime!
}

type Post {
  id: ID!
  title: String!
  content: String!
  author: User!
  community: Community!
  votes: Int!
  comments: [Comment!]!
  media: [MediaAsset!]!
  createdAt: DateTime!
}

type Comment {
  id: ID!
  content: String!
  author: User!
  votes: Int!
  replies: [Comment!]!
  createdAt: DateTime!
}

type User {
  id: ID!
  username: String!
  karma: Int!
  communities: [Community!]!
}
```

## Performance Considerations
- Use Redis for caching frequently accessed communities and posts
- Implement pagination for large threads
- Use database indexing for efficient search operations
- Batch karma updates to reduce database load

## Security Considerations
- Role-based access control for moderation actions
- Rate limiting for post creation and voting
- Content filtering for inappropriate material
- Secure media upload and processing

## TODO
- [x] Implement advanced search functionality
- [ ] Add community analytics dashboard
- [x] Integrate with notification system
- [ ] Implement cross-posting to Yapper