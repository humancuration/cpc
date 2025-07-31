# Social Apps GraphQL Contracts

## Overview
This document defines the GraphQL schemas and contracts for the Allat and Yapper social applications, as well as their integrated features.

## Allat GraphQL Schema

### Types
```graphql
type Community {
  id: ID!
  name: String!
  description: String!
  rules: [String!]!
  moderators: [User!]!
  posts(limit: Int, offset: Int): [Post!]!
  memberCount: Int!
  createdAt: DateTime!
}

type Post {
  id: ID!
  title: String!
  content: String!
  author: User!
  community: Community!
  votes: Int!
  userVote: VoteDirection
  comments(limit: Int, offset: Int): [Comment!]!
  media: [MediaAsset!]!
  createdAt: DateTime!
}

type Comment {
  id: ID!
  content: String!
  author: User!
  votes: Int!
  userVote: VoteDirection
  replies(limit: Int, offset: Int): [Comment!]!
  createdAt: DateTime!
}

type User {
  id: ID!
  username: String!
  karma: Int!
  communities(limit: Int): [Community!]!
  posts(limit: Int, offset: Int): [Post!]!
  comments(limit: Int, offset: Int): [Comment!]!
}

enum VoteDirection {
  UP
  DOWN
  NONE
}

type MediaAsset {
  id: ID!
  url: String!
  thumbnailUrl: String
  type: MediaType!
  altText: String
}

enum MediaType {
  IMAGE
  VIDEO
}

scalar DateTime
```

### Queries
```graphql
type Query {
  # Community queries
  communities(limit: Int, offset: Int): [Community!]!
  community(id: ID!): Community
  searchCommunities(query: String!): [Community!]!
  
  # Post queries
  posts(communityId: ID!, limit: Int, offset: Int): [Post!]!
  post(id: ID!): Post
  searchPosts(query: String!): [Post!]!
  
  # User queries
  currentUser: User
  user(id: ID!): User
  searchUsers(query: String!): [User!]!
  
  # Comment queries
  comments(postId: ID!, limit: Int, offset: Int): [Comment!]!
}
```

### Mutations
```graphql
type Mutation {
  # Community mutations
  createCommunity(input: CreateCommunityInput!): Community!
  updateCommunity(id: ID!, input: UpdateCommunityInput!): Community!
  deleteCommunity(id: ID!): Boolean!
  
  # Post mutations
  createPost(input: CreatePostInput!): Post!
  updatePost(id: ID!, input: UpdatePostInput!): Post!
  deletePost(id: ID!): Boolean!
  
  # Vote mutations
  votePost(postId: ID!, direction: VoteDirection!): Post!
  voteComment(commentId: ID!, direction: VoteDirection!): Comment!
  
  # Comment mutations
  createComment(input: CreateCommentInput!): Comment!
  updateComment(id: ID!, input: UpdateCommentInput!): Comment!
  deleteComment(id: ID!): Boolean!
  
  # Media mutations
  uploadMedia(file: Upload!): MediaAsset!
}

input CreateCommunityInput {
  name: String!
  description: String!
  rules: [String!]!
}

input UpdateCommunityInput {
  name: String
  description: String
  rules: [String!]
}

input CreatePostInput {
  title: String!
  content: String!
  communityId: ID!
  mediaIds: [ID!]
}

input UpdatePostInput {
  title: String
  content: String
  mediaIds: [ID!]
}

input CreateCommentInput {
  content: String!
  postId: ID!
  parentId: ID
}

input UpdateCommentInput {
  content: String!
}
```

### Subscriptions
```graphql
type Subscription {
  postAdded(communityId: ID!): Post!
  commentAdded(postId: ID!): Comment!
  voteUpdated(postId: ID!): Post!
}
```

## Yapper GraphQL Schema

### Types
```graphql
type YapperPost {
  id: ID!
  content: String!
  author: User!
  timestamp: DateTime!
  likes: Int!
  userLiked: Boolean!
  shares: Int!
  media: [MediaAsset!]!
  hashtags: [String!]!
  replyTo: YapperPost
}

type User {
  id: ID!
  displayName: String!
  username: String!
  bio: String
  avatarUrl: String
  following: [User!]!
  followers: [User!]!
  followingCount: Int!
  followersCount: Int!
  posts(limit: Int, offset: Int): [YapperPost!]!
}

type HashtagTrend {
  tag: String!
  count: Int!
  trending: Boolean!
}

type Feed {
  posts: [YapperPost!]!
  nextCursor: String
}

type DirectMessage {
  id: ID!
  sender: User!
  recipient: User!
  content: String!
  timestamp: DateTime!
  read: Boolean!
}
```

### Queries
```graphql
type Query {
  # Feed queries
  feed(cursor: String): Feed!
  userFeed(userId: ID!, cursor: String): Feed!
  hashtagFeed(tag: String!, cursor: String): Feed!
  
  # Post queries
  post(id: ID!): YapperPost
  searchPosts(query: String!): [YapperPost!]!
  
  # User queries
  currentUser: User
  user(id: ID!): User
  searchUsers(query: String!): [User!]!
  
  # Trend queries
  trendingHashtags(limit: Int): [HashtagTrend!]!
  
  # Message queries
  directMessages(userId: ID!, limit: Int, offset: Int): [DirectMessage!]!
}
```

### Mutations
```graphql
type Mutation {
  # Post mutations
  createPost(input: CreateYapperPostInput!): YapperPost!
  deletePost(id: ID!): Boolean!
  likePost(id: ID!): YapperPost!
  unlikePost(id: ID!): YapperPost!
  sharePost(id: ID!): YapperPost!
  
  # Follow mutations
  followUser(userId: ID!): User!
  unfollowUser(userId: ID!): User!
  
  # Message mutations
  sendDirectMessage(input: SendDirectMessageInput!): DirectMessage!
  markMessageAsRead(id: ID!): DirectMessage!
  
  # Media mutations
  uploadMedia(file: Upload!): MediaAsset!
}

input CreateYapperPostInput {
  content: String!
  mediaIds: [ID!]
  replyToId: ID
  hashtags: [String!]
}

input SendDirectMessageInput {
  recipientId: ID!
  content: String!
}
```

### Subscriptions
```graphql
type Subscription {
  postAdded: YapperPost!
  postLiked(postId: ID!): YapperPost!
  userFollowed(userId: ID!): User!
  directMessageReceived: DirectMessage!
}
```

## Cross-App Integration GraphQL Schema

### Extended Types
```graphql
extend type User {
  unifiedKarma: Int!
  crossPlatformActivity: CrossPlatformActivity!
}

type CrossPlatformActivity {
  allatKarma: Int!
  yapperFollowers: Int!
  totalPosts: Int!
  totalComments: Int!
  crossPosts: Int!
}

type UnifiedFeed {
  posts: [UnifiedPost!]!
  nextCursor: String
}

union UnifiedPost = Post | YapperPost

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

### Queries
```graphql
extend type Query {
  # Unified feed
  unifiedFeed(cursor: String, filter: FeedFilter): UnifiedFeed!
  
  # Cross-post queries
  crossPostConnections(postId: ID!, platform: SocialPlatform!): [CrossPostConnection!]!
}

input FeedFilter {
  platforms: [SocialPlatform!]
  includeCrossPosts: Boolean
}
```

### Mutations
```graphql
extend type Mutation {
  # Cross-posting
  crossPostToYapper(allatPostId: ID!): YapperPost!
  crossPostToAllat(yapperPostId: ID!, communityId: ID!): Post!
  
  # Unified actions
  unifiedFollow(userId: ID!): Boolean!
  unifiedLike(postId: ID!, platform: SocialPlatform!): Boolean!
}
```

## Shared Types

### Media Asset
```graphql
type MediaAsset {
  id: ID!
  url: String!
  thumbnailUrl: String
  type: MediaType!
  altText: String
  width: Int
  height: Int
}

enum MediaType {
  IMAGE
  VIDEO
}

scalar Upload
scalar DateTime
```

### Error Handling
```graphql
type Error {
  code: String!
  message: String!
  field: String
}

union Result = Success | Error

type Success {
  success: Boolean!
}