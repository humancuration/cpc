# Social Feature - Phase 4: Timeline Implementation Plan

**Author:** Elias Petrova
**Date:** 2025-07-23
**Status:** Proposed

## 1. Overview

This document outlines the architecture and implementation plan for the user timeline feature. Building upon the existing `PostRepository` and `RelationshipRepository`, this phase introduces a `TimelineService` responsible for aggregating posts from a user's followed accounts.

The primary goal is to create a `timeline` query in our GraphQL API that returns a chronologically sorted list of posts from the users that the current user follows.

This plan will guide the `ougcode` persona through the required steps.

## 2. Task Breakdown

### Task 4.1: Extend the Post Repository

To efficiently fetch timeline posts, we need to add a new method to our `PostRepository` that can retrieve all posts from a given list of author IDs.

**File to Modify:** `packages/cpc-core/src/repositories/social/post_repository.rs`

**Instructions:**

1.  Add a new method `find_posts_by_authors` to the `PostRepository` trait.
2.  Implement this method in `SqlitePostRepository`. The query should fetch all posts where `author_id` is in the provided list and order them by `created_at` in descending order.

```rust
// In packages/cpc-core/src/repositories/social/post_repository.rs

// ... existing code

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, data: CreatePostData) -> Result<Post, sqlx::Error>;
    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>;
    async fn find_posts_by_authors(&self, author_ids: Vec<Uuid>) -> Result<Vec<Post>, sqlx::Error>;
}

// ... SqlitePostRepository struct

#[async_trait]
impl PostRepository for SqlitePostRepository {
    // ... existing create_post and find_post_by_id methods

    async fn find_posts_by_authors(&self, author_ids: Vec<Uuid>) -> Result<Vec<Post>, sqlx::Error> {
        let posts = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, created_at, updated_at
            FROM posts
            WHERE author_id = ANY($1)
            ORDER BY created_at DESC
            "#,
            &author_ids
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(posts)
    }
}
```
*Note: `ANY($1)` is PostgreSQL syntax. For SQLite, we will need to dynamically construct the `IN` clause. The `ougcode` persona should be aware of this and implement it correctly.*

### Task 4.2: Create the Timeline Service

We will introduce a new service dedicated to the timeline logic. This keeps the concerns separated from the general `SocialService`.

**File to Create:** `packages/cpc-core/src/services/social/timeline_service.rs`

**File to Create:** `packages/cpc-core/src/services/mod.rs`

**File to Create:** `packages/cpc-core/src/services/social/mod.rs`

**Instructions:**

1.  Create the new module files to house the services within `cpc-core`.
2.  Create the `timeline_service.rs` file.
3.  Implement the `TimelineService` which will depend on `RelationshipRepository` and `PostRepository`.
4.  The `get_timeline` method will first fetch the IDs of followed users and then use the new `find_posts_by_authors` repository method to get their posts.

**`packages/cpc-core/src/services/mod.rs`:**
```rust
pub mod social;
```

**`packages/cpc-core/src/services/social/mod.rs`:**
```rust
pub mod timeline_service;
```

**`packages/cpc-core/src/services/social/timeline_service.rs`:**
```rust
use crate::models::social::post::Post;
use crate::repositories::social::post_repository::PostRepository;
use crate::repositories::social::relationship_repository::RelationshipRepository;
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum TimelineServiceError {
    DatabaseError(sqlx::Error),
}

impl From<sqlx::Error> for TimelineServiceError {
    fn from(e: sqlx::Error) -> Self {
        TimelineServiceError::DatabaseError(e)
    }
}

pub type Result<T> = std::result::Result<T, TimelineServiceError>;

pub struct TimelineService {
    post_repo: Arc<dyn PostRepository>,
    relationship_repo: Arc<dyn RelationshipRepository>,
}

impl TimelineService {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        relationship_repo: Arc<dyn RelationshipRepository>,
    ) -> Self {
        Self {
            post_repo,
            relationship_repo,
        }
    }

    pub async fn get_timeline(&self, user_id: Uuid) -> Result<Vec<Post>> {
        // 1. Get the list of users the current user is following.
        let followed_user_ids = self
            .relationship_repo
            .get_followed_user_ids(user_id)
            .await?;

        if followed_user_ids.is_empty() {
            return Ok(vec![]);
        }

        // 2. Fetch all posts from those users.
        let posts = self
            .post_repo
            .find_posts_by_authors(followed_user_ids)
            .await?;

        Ok(posts)
    }
}
```

### Task 4.3: Integrate TimelineService into the Backend

Now we will wire up the new service in our backend application.

**File to Modify:** `apps/backend/src/main.rs`
**File to Modify:** `apps/backend/src/graphql/schema.rs`

**Instructions:**

1.  **`main.rs`**: Instantiate the `TimelineService` and provide it to the GraphQL schema.
2.  **`schema.rs`**: Update `create_schema` to accept the `TimelineService` and add it to the schema data.

**`apps/backend/src/main.rs`:**
```rust
// ... other imports
use cpc_core::services::social::timeline_service::TimelineService;

#[tokio::main]
async fn main() {
    // ... db_pool and repository setup
    
    // Create service instances
    let social_service = Arc::new(SocialService::new(post_repo.clone(), relationship_repo.clone()));
    let timeline_service = Arc::new(TimelineService::new(post_repo.clone(), relationship_repo.clone()));

    // Create the schema, passing in dependencies
    let schema = crate::graphql::schema::create_schema(social_service, timeline_service);
    
    // ... rest of main
}
```

**`apps/backend/src/graphql/schema.rs`:**
```rust
// ... other imports
use cpc_core::services::social::timeline_service::TimelineService;
use std::sync::Arc;

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_schema(
    social_service: Arc<SocialService>,
    timeline_service: Arc<TimelineService>,
) -> AppSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(social_service)
        .data(timeline_service)
        .finish()
}
```

### Task 4.4: Expose Timeline via GraphQL

Finally, we'll add the `timeline` query to our GraphQL API.

**File to Modify:** `apps/backend/src/graphql/social.rs`
**File to Modify:** `schema.graphql` (or equivalent if managed in code)

**Instructions:**

1.  Add the `timeline` query to `SocialQuery`. This resolver will call the `TimelineService`.
2.  Define the `timeline` query in the GraphQL schema. For now, it will return a simple list of posts.

**`apps/backend/src/graphql/social.rs`:**
```rust
// ... other imports
use cpc_core::services::social::timeline_service::TimelineService;

// In SocialQuery impl
#[Object]
impl SocialQuery {
    // ... existing post query

    async fn timeline(&self, ctx: &Context<'_>) -> Result<Vec<Post>> {
        let service = ctx.data_unchecked<Arc<TimelineService>>();
        // Placeholder for getting the current user's ID from the context
        let current_user_id = Uuid::new_v4(); 

        let posts = service.get_timeline(current_user_id).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to fetch timeline: {:?}", e)))?;
        
        Ok(posts)
    }
}
```

**`schema.graphql`:**
```graphql
# --- EXTENSIONS ---

extend type Query {
  """Fetches a user's timeline of posts from followed users."""
  timeline: [Post!]! @auth

  """Fetches a single post by its ID."""
  post(id: ID!): Post @auth
}
```

## 4. Next Steps

With the timeline in place, the next logical steps would be to introduce pagination to the `timeline` query (`first`, `after` arguments) and to implement the `media_items` resolver on the `Post` type to show images and videos.