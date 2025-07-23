# Social Feature Architecture Plan

**Author:** Elias Petrova
**Date:** 2025-07-23
**Status:** Proposed

## 1. Overview

This document outlines the architecture for implementing social media features within the CPC ecosystem. The goal is to port the functionality from the legacy Android application's `feature_social` module into our shared Rust `cpc-core` package, ensuring a robust, scalable, and maintainable design.

The architecture is divided into four main layers:
- **Data Models:** Core data structures representing social entities.
- **Repositories:** Traits and implementations for database interactions.
- **Services:** Business logic that orchestrates repository operations.
- **GraphQL API:** The public-facing API for clients to consume.

## 2. File Structure

The new social feature modules will be organized as follows:

```
apps/backend/src/
├── graphql/
│   └── social.rs      # GraphQL queries and mutations for social features
└── services/
    └── social_service.rs # Service containing business logic

packages/cpc-core/src/
├── models/
│   └── social/
│       ├── mod.rs
│       ├── post.rs        # Post, MediaItem, Visibility, etc.
│       └── relationship.rs
└── repositories/
    └── social/
        ├── mod.rs
        ├── post_repository.rs
        └── relationship_repository.rs
```

*Note: The `SocialService` has been initially placed in `apps/backend/src/services/` to be more tightly coupled with the backend application server. The repositories and models remain in `packages/cpc-core/` to be shared across the ecosystem.*

## 3. Data Models

Location: `packages/cpc-core/src/models/social/`

We will use `serde` for serialization/deserialization and `sqlx` for database mapping. Common types like `Uuid` and `DateTime<Utc>` will be used for identifiers and timestamps.
### `post.rs`

#### Phase 1: Initial Implementation

The initial version of the `Post` model implemented for the GraphQL API is a simplified structure. This allows for rapid initial development and will be expanded upon in Phase 2.

```rust
// packages/cpc-core/src/models/social/post.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, async_graphql::SimpleObject)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub visibility: String, // Simplified for Phase 1
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

#### Phase 2: Target Data Model

The target data model, to be implemented with the repository layer, includes more complex fields and relationships.

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

// The fully-featured Post model for Phase 2
#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Post {
    pub id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    #[sqlx(default)] // This field will be populated by the service layer
    pub media_items: Vec<MediaItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct MediaItem {
    pub id: Uuid,
    pub post_id: Uuid,
    pub url: String,
    pub media_type: MediaType,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "visibility", rename_all = "UPPERCASE")]
pub enum Visibility {
    Public,
    Cooperative,
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::Type, PartialEq)]
#[sqlx(type_name = "media_type", rename_all = "UPPERCASE")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Unknown,
}
```
```

### `relationship.rs`

```rust
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Relationship {
    pub id: Uuid,
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub created_at: DateTime<Utc>,
}
```

## 4. Repositories

Location: `packages/cpc-core/src/repositories/social/`

Repositories will define traits for database operations, with implementations using `sqlx`. We will use `async-trait` to support `async` functions in traits.

### `post_repository.rs`

```rust
use async_trait::async_trait;
use uuid::Uuid;
// ... other imports

pub struct CreatePostData {
    pub content: String,
    pub author_id: Uuid,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
}

pub struct CreateMediaItemData {
    pub url: String,
    pub media_type: MediaType,
}

pub struct TimelineArgs {
    pub current_user_id: Uuid,
    pub followed_user_ids: Vec<Uuid>,
    pub cooperative_ids: Vec<Uuid>,
    pub limit: i32,
    pub after_cursor: Option<String>, // Opaque cursor, likely post timestamp + ID
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        data: &CreatePostData
    ) -> Result<Post, sqlx::Error>;

    async fn add_media_items_to_post(
        &self,
        tx: &mut sqlx::Transaction<'_, sqlx::Sqlite>,
        post_id: Uuid,
        items: &[CreateMediaItemData]
    ) -> Result<(), sqlx::Error>;

    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>;
    
    async fn find_media_for_posts(&self, post_ids: &[Uuid]) -> Result<Vec<MediaItem>, sqlx::Error>;

    async fn find_posts_for_timeline(&self, args: TimelineArgs) -> Result<Vec<Post>, sqlx::Error>;
}
```

### `relationship_repository.rs`

```rust
use async_trait::async_trait;
use uuid::Uuid;
// ... other imports

#[async_trait]
pub trait RelationshipRepository: Send + Sync {
    async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<Relationship, sqlx::Error>;
    async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<u64, sqlx::Error>;
    async fn get_followed_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn get_follower_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool, sqlx::Error>;
}
```

## 5. Services

Location: `packages/cpc-core/src/services/social/`

The `SocialService` will encapsulate the business logic, depending on the repository traits for data access.

### `social_service.rs`

```rust
use std::sync::Arc;
// ... other imports for models, repos, etc.

pub struct SocialService {
    db_pool: sqlx::SqlitePool,
    post_repo: Arc<dyn PostRepository>,
    relationship_repo: Arc<dyn RelationshipRepository>,
    // Other repos like UserRepository will be needed for validation
    // user_repo: Arc<dyn UserRepository>,
}

impl SocialService {
    // Constructor
    pub fn new(...) -> Self { ... }

    pub async fn create_post(&self, author_id: Uuid, input: CreatePostInput) -> Result<Post, Error> {
        // 1. Validate media URLs (domain, scheme, extension)
        // 2. Validate author_id and cooperative_id exist using other repositories
        // 3. Start a database transaction
        // 4. Call `post_repo.create_post`
        // 5. Call `post_repo.add_media_items_to_post`
        // 6. Commit transaction
        // 7. Return the newly created post
    }

    pub async fn get_timeline(&self, user_id: Uuid, args: GetTimelineArgs) -> Result<TimelinePayload, Error> {
        // 1. Get followed user IDs from `relationship_repo`
        // 2. Get cooperative IDs user belongs to (from a future CooperativeRepository)
        // 3. Construct `TimelineArgs` for the repository
        // 4. Call `post_repo.find_posts_for_timeline`
        // 5. Populate author and media details for each post
        // 6. Construct and return `TimelinePayload` with posts and next cursor
    }

    pub async fn follow_user(&self, current_user_id: Uuid, target_user_id: Uuid) -> Result<Relationship, Error> {
        // 1. Validate users exist
        // 2. Call `relationship_repo.follow_user`
    }

    pub async fn unfollow_user(&self, current_user_id: Uuid, target_user_id: Uuid) -> Result<(), Error> {
        // 1. Call `relationship_repo.unfollow_user`
    }
}
```

## 6. GraphQL API

Location: `apps/backend/src/graphql/`

The GraphQL schema will be extended to support the new social features. Resolvers will delegate logic to the `SocialService`.

### Schema Additions (`schema.graphql`)

```graphql
# --- TYPES ---

type Post {
  id: ID!
  content: String!
  author: User!
  mediaItems: [MediaItem!]!
  visibility: Visibility!
  cooperative: Cooperative
  createdAt: String! # ISO 8601
}

type MediaItem {
  id: ID!
  url: String!
  mediaType: MediaType!
}

type TimelinePayload {
  posts: [Post!]!
  hasNextPage: Boolean!
  nextCursor: String
}

enum Visibility {
  PUBLIC
  COOPERATIVE
  PRIVATE
}

enum MediaType {
  IMAGE
  VIDEO
  AUDIO
  UNKNOWN
}

# --- INPUTS ---

input CreatePostInput {
  content: String!
  mediaUrls: [String!]
  visibility: Visibility!
  cooperativeId: ID
}

# --- EXTENSIONS ---

extend type Query {
  """Fetches a user's timeline with pagination."""
  timeline(first: Int = 20, after: String, cooperativeOnly: Boolean = false): TimelinePayload! @auth

  """Fetches a single post by its ID."""
  post(id: ID!): Post @auth
}

extend type Mutation {
  """Creates a new post."""
  createPost(input: CreatePostInput!): Post! @auth

  """Follow a user."""
  followUser(userId: ID!): User! @auth

  """Unfollow a user."""
  unfollowUser(userId: ID!): User! @auth
}
```

### Resolvers

The GraphQL resolvers in `apps/backend/src/` will be updated. They will access the `SocialService` from the `async-graphql` context and call the relevant service methods.

- **`Query::timeline`**: Will call `social_service.get_timeline`.
- **`Mutation::createPost`**: Will call `social_service.create_post`.
- **`Mutation::followUser` / `unfollowUser`**: Will call the respective service methods.

A `Post::author` resolver will be needed to perform a dataloader-based lookup for the `User` object to prevent N+1 query problems. Similarly, `Post::mediaItems` will be resolved efficiently.