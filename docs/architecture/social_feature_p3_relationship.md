# Social Feature - Phase 3: Relationship Repository Implementation Plan

**Author:** Elias Petrova
**Date:** 2025-07-23
**Status:** Ready for Implementation

## 1. Overview

This document provides a detailed plan for implementing the user-to-user relationship component of the social feature. This phase focuses on creating a `RelationshipRepository` to manage follow/unfollow actions in the database, integrating this repository into the `SocialService`, and exposing the functionality through the GraphQL API.

This plan will guide the `ougcode` persona through the required steps.

## 2. Task Breakdown

### Task 3.1: Create the Relationship Repository

The first step is to create the repository trait and its `SQLite` implementation to handle all database interactions for user relationships.

**File to Create:** `packages/cpc-core/src/repositories/social/relationship_repository.rs`

**File to Modify:** `packages/cpc-core/src/repositories/social/mod.rs`

**Instructions:**

1.  Create the new file `relationship_repository.rs` at the path specified above.
2.  Add the following code, which defines the `RelationshipRepository` trait and the `SqliteRelationshipRepository` implementation, including the necessary SQL queries.
3.  Update the social repository `mod.rs` file to include the new module.

**`packages/cpc-core/src/repositories/social/mod.rs`:**
```rust
pub mod post_repository;
pub mod relationship_repository;
```

**`packages/cpc-core/src/repositories/social/relationship_repository.rs`:**
```rust
use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::social::relationship::Relationship;

#[async_trait]
pub trait RelationshipRepository: Send + Sync {
    async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<Relationship, sqlx::Error>;
    async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<u64, sqlx::Error>;
    async fn get_followed_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn get_follower_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error>;
    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool, sqlx::Error>;
}

pub struct SqliteRelationshipRepository {
    pool: SqlitePool,
}

impl SqliteRelationshipRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RelationshipRepository for SqliteRelationshipRepository {
    async fn follow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<Relationship, sqlx::Error> {
        let relationship = sqlx::query_as!(
            Relationship,
            r#"
            INSERT INTO relationships (id, follower_id, followed_id)
            VALUES ($1, $2, $3)
            RETURNING id, follower_id, followed_id, created_at
            "#,
            Uuid::new_v4(),
            follower_id,
            followed_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(relationship)
    }

    async fn unfollow_user(&self, follower_id: Uuid, followed_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM relationships WHERE follower_id = $1 AND followed_id = $2",
            follower_id,
            followed_id
        )
        .execute(&self.pool)
        .await?;
        Ok(result.rows_affected())
    }

    async fn get_followed_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = sqlx::query!(
            "SELECT followed_id FROM relationships WHERE follower_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|rec| rec.followed_id)
        .collect();
        Ok(ids)
    }

    async fn get_follower_user_ids(&self, user_id: Uuid) -> Result<Vec<Uuid>, sqlx::Error> {
        let ids = sqlx::query!(
            "SELECT follower_id FROM relationships WHERE followed_id = $1",
            user_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(|rec| rec.follower_id)
        .collect();
        Ok(ids)
    }

    async fn check_is_following(&self, follower_id: Uuid, followed_id: Uuid) -> Result<bool, sqlx::Error> {
        let count = sqlx::query!(
            "SELECT COUNT(*) as count FROM relationships WHERE follower_id = $1 AND followed_id = $2",
            follower_id,
            followed_id
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        Ok(count > 0)
    }
}
```

### Task 3.2: Integrate RelationshipRepository into SocialService

Next, refactor the `SocialService` to use the new `RelationshipRepository`, adding business logic for the follow/unfollow features.

**File to Modify:** `apps/backend/src/services/social_service.rs`

**Instructions:**

1.  Update the `SocialService` struct to hold an `Arc<dyn RelationshipRepository>`.
2.  Update the `SocialService::new` constructor to accept the new repository.
3.  Add the `follow_user` and `unfollow_user` methods, which include business logic and delegate to the repository.
4.  Add a new `UserCannotFollowSelf` variant to the `SocialServiceError` enum.

```rust
use cpc_core::models::social::post::{Post, Visibility};
use cpc_core::repositories::social::post_repository::{PostRepository, CreatePostData};
use cpc_core::repositories::social::relationship_repository::RelationshipRepository; // Add this
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub enum SocialServiceError {
    PostNotFound,
    DatabaseError(sqlx::Error),
    UserCannotFollowSelf, // Add this
    // Add other specific errors as needed
}

impl From<sqlx::Error> for SocialServiceError {
    fn from(e: sqlx::Error) -> Self {
        SocialServiceError::DatabaseError(e)
    }
}

pub type Result<T> = std::result::Result<T, SocialServiceError>;

pub struct SocialService {
    post_repo: Arc<dyn PostRepository>,
    relationship_repo: Arc<dyn RelationshipRepository>, // Add this
}

impl SocialService {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        relationship_repo: Arc<dyn RelationshipRepository>, // Add this
    ) -> Self {
        Self { post_repo, relationship_repo }
    }

    // ... existing create_post and get_post methods

    pub async fn follow_user(&self, current_user_id: Uuid, target_user_id: Uuid) -> Result<()> {
        if current_user_id == target_user_id {
            return Err(SocialServiceError::UserCannotFollowSelf);
        }
        
        // In a real app, you might also check if the users exist
        
        self.relationship_repo.follow_user(current_user_id, target_user_id).await?;
        Ok(())
    }

    pub async fn unfollow_user(&self, current_user_id: Uuid, target_user_id: Uuid) -> Result<()> {
        self.relationship_repo.unfollow_user(current_user_id, target_user_id).await?;
        Ok(())
    }
}
```

### Task 3.3: Update Backend for Integration

The main backend application needs to be updated to construct the `RelationshipRepository` and inject it into the service and GraphQL layers.

**File to Modify:** `apps/backend/src/main.rs`
**File to Modify:** `apps/backend/src/graphql/schema.rs`
**File to Modify:** `apps/backend/src/graphql/social.rs`

**Instructions:**

1.  **`apps/backend/src/main.rs`**: In the `main` function, instantiate `SqliteRelationshipRepository` and pass it to the schema creation function.
    ```rust
    // ... other imports
    use cpc_core::repositories::social::post_repository::SqlitePostRepository;
    use cpc_core::repositories::social::relationship_repository::SqliteRelationshipRepository; // Add this
    use sqlx::SqlitePool;
    use std::sync::Arc;

    #[tokio::main]
    async fn main() {
        // ... database pool setup
        let db_pool = SqlitePool::connect(":memory:").await.expect("Failed to create pool.");

        // ... migrations (will be added later)

        // Create repository instances
        let post_repo = Arc::new(SqlitePostRepository::new(db_pool.clone()));
        let relationship_repo = Arc::new(SqliteRelationshipRepository::new(db_pool.clone())); // Add this

        // Create the schema, passing in dependencies
        let schema = crate::graphql::schema::create_schema(post_repo, relationship_repo); // Modify this
        
        // ... rest of the main function for setting up Axum
    }
    ```

2.  **`apps/backend/src/graphql/schema.rs`**: Update `create_schema` to accept the `RelationshipRepository` and pass it to the `SocialService` constructor.
    ```rust
    use async_graphql::{EmptySubscription, MergedObject, Schema};
    use crate::services::social_service::SocialService;
    use crate::graphql::social::{SocialQuery, SocialMutation};
    use cpc_core::repositories::social::post_repository::PostRepository;
    use cpc_core::repositories::social::relationship_repository::RelationshipRepository; // Add this
    use std::sync::Arc;

    // ... Query and Mutation structs

    pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

    pub fn create_schema(
        post_repo: Arc<dyn PostRepository>,
        relationship_repo: Arc<dyn RelationshipRepository>, // Add this
    ) -> AppSchema {
        let social_service = Arc::new(SocialService::new(post_repo, relationship_repo)); // Modify this
        
        Schema::build(Query::default(), Mutation::default(), EmptySubscription)
            .data(social_service)
            .finish()
    }
    ```

3.  **`apps/backend/src/graphql/social.rs`**: Implement the `followUser` and `unfollowUser` mutations.
    ```rust
    // ... other imports
    use async_graphql::{Context, Object, Result};
    use crate::services::social_service::SocialService;
    use uuid::Uuid;
    
    // ... SocialQuery impl
    
    #[derive(Default)]
    pub struct SocialMutation;

    #[Object]
    impl SocialMutation {
        // ... existing create_post mutation

        async fn follow_user(
            &self,
            ctx: &Context<'_>,
            user_id: Uuid,
        ) -> Result<bool> {
            let service = ctx.data_unchecked::<Arc<SocialService>>();
            // In a real app, you'd get the current user's ID from the context (e.g., JWT)
            let current_user_id = Uuid::new_v4(); // Placeholder
            
            service.follow_user(current_user_id, user_id).await
                .map_err(|e| async_graphql::Error::new(format!("Failed to follow user: {:?}", e)))?;
            
            Ok(true)
        }

        async fn unfollow_user(
            &self,
            ctx: &Context<'_>,
            user_id: Uuid,
        ) -> Result<bool> {
            let service = ctx.data_unchecked::<Arc<SocialService>>();
            // Placeholder for current user ID
            let current_user_id = Uuid::new_v4();
            
            service.unfollow_user(current_user_id, user_id).await
                .map_err(|e| async_graphql::Error::new(format!("Failed to unfollow user: {:?}", e)))?;

            Ok(true)
        }
    }
    ```

### Task 3.4: Create Database Migration

A database migration is required to create the `relationships` table. This SQL should be added to a new migration file (e.g., `migrations/YYYYMMDDHHMMSS_create_relationships.sql`).

**SQL for Migration:**
```sql
-- migrations/YYYYMMDDHHMMSS_create_relationships.sql

CREATE TABLE relationships (
    id UUID PRIMARY KEY NOT NULL,
    follower_id UUID NOT NULL,
    followed_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT (strftime('%Y-%m-%d %H:%M:%f', 'now')),
    FOREIGN KEY(follower_id) REFERENCES users(id),
    FOREIGN KEY(followed_id) REFERENCES users(id),
    UNIQUE(follower_id, followed_id)
);
```

## 4. Next Steps

Once this plan is implemented, the backend will have a fully functional relationship system. Future phases will build upon this to create features like user timelines, which will use the `get_followed_user_ids` method to fetch relevant posts.