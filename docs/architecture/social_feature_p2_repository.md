# Social Feature - Phase 2: Repository Layer Implementation Plan

**Author:** Elias Petrova
**Date:** 2025-07-23
**Status:** Ready for Implementation

## 1. Overview

This document provides a detailed plan for implementing the repository layer for the social feature. This phase focuses on building a robust data access layer that replaces the placeholder logic in the `SocialService`. It involves expanding the data models to their target state, creating a `PostRepository` for database interactions, and integrating it into the existing service.

This plan will guide the `ougcode` persona through the required steps.

## 2. Task Breakdown

### Task 2.1: Expand Core Data Models

The first step is to update the `Post` model in `packages/cpc-core/src/models/social/post.rs` to match the full architecture specification.

**File to Modify:** `packages/cpc-core/src/models/social/post.rs`

**Instructions:**

1.  Replace the current content of the file with the code below.
2.  This change introduces the `MediaItem`, `MediaType`, and `Visibility` types.
3.  It updates the `Post` struct to include `media_items` and `cooperative_id`, and changes `visibility` from `String` to the `Visibility` enum.
4.  Note the addition of `sqlx` attributes, which will be used by the repository.

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// The fully-featured Post model for Phase 2
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
#[graphql(complex)]
pub struct Post {
    pub id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    #[graphql(skip)]
    #[sqlx(default)] 
    pub media_items: Vec<MediaItem>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, async_graphql::SimpleObject, sqlx::FromRow)]
pub struct MediaItem {
    pub id: Uuid,
    #[graphql(skip)]
    pub post_id: Uuid,
    pub url: String,
    pub media_type: MediaType,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "visibility", rename_all = "UPPERCASE")]
pub enum Visibility {
    Public,
    Cooperative,
    Private,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, async_graphql::Enum, sqlx::Type)]
#[sqlx(type_name = "media_type", rename_all = "UPPERCASE")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Unknown,
}
```

### Task 2.2: Create the Post Repository

Next, create the repository trait and its `SQLite` implementation. This will handle all direct database communication for posts.

**File to Create:** `packages/cpc-core/src/repositories/social/post_repository.rs`

**Instructions:**

1.  Create the new file at the specified path.
2.  Add the following code, which defines the `PostRepository` trait and a skeleton for the `SqlitePostRepository` implementation.
3.  You will also need to create the parent modules. Create `packages/cpc-core/src/repositories/mod.rs` and `packages/cpc-core/src/repositories/social/mod.rs`.

**`packages/cpc-core/src/repositories/mod.rs`:**
```rust
pub mod social;
```

**`packages/cpc-core/src/repositories/social/mod.rs`:**
```rust
pub mod post_repository;
```

**`packages/cpc-core/src/repositories/social/post_repository.rs`:**
```rust
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::social::post::{Post, Visibility};

// Using a struct for creation data promotes clarity and type safety
pub struct CreatePostData {
    pub author_id: Uuid,
    pub content: String,
    pub visibility: Visibility,
    pub cooperative_id: Option<Uuid>,
    // In the future, this could include media item data
}

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn create_post(&self, data: CreatePostData) -> Result<Post, sqlx::Error>;
    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error>;
}

pub struct SqlitePostRepository {
    pool: SqlitePool,
}

impl SqlitePostRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PostRepository for SqlitePostRepository {
    async fn create_post(&self, data: CreatePostData) -> Result<Post, sqlx::Error> {
        // The 'RETURNING *' clause is specific to PostgreSQL and SQLite,
        // making it easy to get the created record back.
        let post = sqlx::query_as!(
            Post,
            r#"
            INSERT INTO posts (id, author_id, content, visibility, cooperative_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, author_id, content, visibility as "visibility: _", cooperative_id, created_at, updated_at
            "#,
            Uuid::new_v4(),
            data.author_id,
            data.content,
            data.visibility,
            data.cooperative_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(post)
    }

    async fn find_post_by_id(&self, id: Uuid) -> Result<Option<Post>, sqlx::Error> {
        let post = sqlx::query_as!(
            Post,
            r#"
            SELECT id, author_id, content, visibility as "visibility: _", cooperative_id, created_at, updated_at
            FROM posts
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(post)
    }
}
```

### Task 2.3: Update SocialService to Use the Repository

Finally, refactor the `SocialService` to use the new `PostRepository`. This decouples the business logic from the data access details.

**File to Modify:** `apps/backend/src/services/social_service.rs`

**Instructions:**

1.  Modify the `SocialService` struct to hold a `PostRepository`.
2.  Update the `new` method to accept the repository.
3.  Rewrite `create_post` and `get_post` to delegate their calls to the repository.

```rust
use cpc_core::models::social::post::{Post, Visibility};
use cpc_core::repositories::social::post_repository::{PostRepository, CreatePostData};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum SocialServiceError {
    PostNotFound,
    DatabaseError(sqlx::Error),
    // Add other specific errors as needed
}

// Allow converting sqlx::Error into our custom error type
impl From<sqlx::Error> for SocialServiceError {
    fn from(e: sqlx::Error) -> Self {
        SocialServiceError::DatabaseError(e)
    }
}

pub type Result<T> = std::result::Result<T, SocialServiceError>;

pub struct SocialService {
    post_repo: Arc<dyn PostRepository>,
}

impl SocialService {
    // The service now depends on the repository trait, not a concrete implementation
    pub fn new(post_repo: Arc<dyn PostRepository>) -> Self {
        Self { post_repo }
    }

    pub async fn create_post(
        &self,
        author_id: Uuid,
        content: String,
        visibility: Visibility,
    ) -> Result<Post> {
        let create_data = CreatePostData {
            author_id,
            content,
            visibility,
            cooperative_id: None, // Or handle this based on visibility
        };

        let post = self.post_repo.create_post(create_data).await?;
        Ok(post)
    }

    pub async fn get_post(&self, post_id: Uuid) -> Result<Post> {
        self.post_repo
            .find_post_by_id(post_id)
            .await?
            .ok_or(SocialServiceError::PostNotFound)
    }
}
```

### Task 2.4: Update Backend Dependencies and Initialization

The main backend application needs to be updated to construct and inject the new repository.

**File to Modify:** `apps/backend/src/main.rs`
**File to Modify:** `apps/backend/src/graphql/schema.rs`
**File to Modify:** `apps/backend/Cargo.toml`

**Instructions:**

1.  **`apps/backend/Cargo.toml`**: Add `sqlx` with required features.
    ```toml
    [dependencies]
    # ... other dependencies
    sqlx = { version = "0.7", features = [ "runtime-tokio-rustls", "sqlite", "macros", "uuid", "chrono" ] }
    ```

2.  **`apps/backend/src/main.rs`**: Update `main` to set up the database pool.
    ```rust
    // ... other imports
    use cpc_core::repositories::social::post_repository::{SqlitePostRepository};
    use sqlx::SqlitePool;
    use std::sync::Arc;

    #[tokio::main]
    async fn main() {
        // 1. Set up the database connection pool
        // Using ":memory:" for now, but this could be a file path.
        let db_pool = SqlitePool::connect(":memory:").await.expect("Failed to create pool.");

        // 2. Run database migrations (we will need to create these)
        // sqlx::migrate!("./migrations").run(&db_pool).await.expect("Failed to run migrations.");

        // 3. Create repository instance
        let post_repo = Arc::new(SqlitePostRepository::new(db_pool.clone()));

        // 4. Create the schema, passing in dependencies
        let schema = crate::graphql::schema::create_schema(post_repo);
        
        // ... rest of the main function for setting up Axum
    }
    ```

3.  **`apps/backend/src/graphql/schema.rs`**: Update `create_schema` to accept and inject the repository.
    ```rust
    use async_graphql::{EmptySubscription, MergedObject, Schema};
    use crate::services::social_service::SocialService;
    use crate::graphql::social::{SocialQuery, SocialMutation};
    use cpc_core::repositories::social::post_repository::PostRepository;
    use std::sync::Arc;

    // ... Query and Mutation structs

    pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

    pub fn create_schema(post_repo: Arc<dyn PostRepository>) -> AppSchema {
        let social_service = SocialService::new(post_repo);
        
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(social_service)
            .finish()
    }
    ```

4. **`apps/backend/src/graphql/social.rs`**: Update `create_post` mutation to use the new `Visibility` enum.
    ```rust
    // In SocialMutation impl
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        author_id: Uuid,
        content: String,
        visibility: Visibility, // Changed from String
    ) -> Result<Post> {
        // ... service lookup logic is the same
        let post = service.create_post(author_id, content, visibility).await
            .map_err(|e| async_graphql::Error::new(format!("Failed to create post: {:?}", e)))?;
        
        Ok(post)
    }
    ```

## 3. Next Steps

Once this plan is implemented, the backend will have a proper, extensible data access layer for the social features. The next phase would involve:

-   Creating SQL database migrations for the `posts` table.
-   Implementing the `media_items` functionality, including adding them in `create_post` and fetching them in `find_post_by_id`.
-   Building out the `RelationshipRepository` and integrating it.
-   Implementing the timeline functionality.