# Cross-Posting to Yapper Implementation Plan

## Overview

This document details the implementation plan for adding cross-posting functionality between Allat and Yapper. The implementation will follow hexagonal architecture principles and integrate with the existing `social_integration` shared package.

## Implementation Steps

### 1. Domain Layer Implementation

#### 1.1. Create Cross-Post Entities
Create `src/domain/cross_post.rs`:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossPost {
    pub id: Uuid,
    pub source_app: SocialApp,
    pub source_id: Uuid,
    pub target_app: SocialApp,
    pub target_id: Uuid,
    pub user_id: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SocialApp {
    Allat,
    Yapper,
}

impl ToString for SocialApp {
    fn to_string(&self) -> String {
        match self {
            SocialApp::Allat => "allat".to_string(),
            SocialApp::Yapper => "yapper".to_string(),
        }
    }
}

impl From<String> for SocialApp {
    fn from(s: String) -> Self {
        match s.as_str() {
            "allat" => SocialApp::Allat,
            "yapper" => SocialApp::Yapper,
            _ => panic!("Invalid social app: {}", s),
        }
    }
}
```

#### 1.2. Update Domain Module
Update `src/domain/mod.rs`:

```rust
// Add after existing imports
pub mod cross_post;

// Add to exports
pub use cross_post::*;
```

### 2. Application Layer Implementation

#### 2.1. Create Cross-Post Service
Create `src/application/cross_post_service.rs`:

```rust
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::domain::cross_post::{CrossPost, SocialApp};
use crate::infrastructure::repositories::post_repo::PostRepository;
use crate::infrastructure::repositories::cross_post_repo::CrossPostRepository;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
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

impl CrossPostServiceImpl {
    pub fn new(
        social_integration_service: Arc<dyn SocialIntegrationService>,
        post_repo: Arc<dyn PostRepository>,
        cross_post_repo: Arc<dyn CrossPostRepository>,
    ) -> Self {
        Self {
            social_integration_service,
            post_repo,
            cross_post_repo,
        }
    }
}

#[async_trait]
impl CrossPostService for CrossPostServiceImpl {
    async fn cross_post_to_yapper(&self, input: CrossPostInput) -> Result<Uuid, ApplicationError> {
        // Fetch the Allat post
        let post = self.post_repo.find_by_id(input.source_post_id).await?
            .ok_or(ApplicationError::NotFound)?;
        
        // Transform the post for Yapper
        let yapper_post_content = format!("{}\n\nCross-posted from Allat community: {}", 
                                          post.content, 
                                          "Community Name"); // We'd need to fetch the actual community name
        
        // Create cross-post in Yapper through social integration service
        let yapper_post_id = self.social_integration_service
            .cross_post_to_yapper(input.source_post_id, input.user_id)
            .await?;
        
        // Record the cross-post relationship
        let cross_post = CrossPost {
            id: Uuid::new_v4(),
            source_app: SocialApp::Allat,
            source_id: input.source_post_id,
            target_app: SocialApp::Yapper,
            target_id: yapper_post_id,
            user_id: input.user_id,
            created_at: chrono::Utc::now(),
        };
        
        self.cross_post_repo.create(&cross_post).await?;
        
        Ok(yapper_post_id)
    }
    
    async fn cross_post_to_allat(&self, input: CrossPostInput) -> Result<Uuid, ApplicationError> {
        // For cross-posting from Yapper to Allat, we'd need the target community ID
        let target_community_id = input.target_community_id
            .ok_or(ApplicationError::InvalidInput("Target community ID is required for cross-posting to Allat".to_string()))?;
        
        // Create post in Allat
        let post = crate::domain::post::Post::new(
            target_community_id,
            input.user_id,
            "Cross-posted from Yapper".to_string(), // We'd need to extract the actual title
            "Content from Yapper post".to_string(), // We'd need to extract the actual content
            None,
            vec![], // We'd need to handle media assets
        );
        
        self.post_repo.create(&post).await?;
        
        // Record the cross-post relationship
        let cross_post = CrossPost {
            id: Uuid::new_v4(),
            source_app: SocialApp::Yapper,
            source_id: input.source_post_id,
            target_app: SocialApp::Allat,
            target_id: post.id,
            user_id: input.user_id,
            created_at: chrono::Utc::now(),
        };
        
        self.cross_post_repo.create(&cross_post).await?;
        
        Ok(post.id)
    }
    
    async fn get_cross_posts(&self, source_id: Uuid, source_app: SocialApp) -> Result<Vec<CrossPost>, ApplicationError> {
        self.cross_post_repo.find_by_source(source_id, source_app).await
            .map_err(ApplicationError::from)
    }
}

// We'll need to define the trait for the social integration service
// This should match the interface from social_integration
#[async_trait]
pub trait SocialIntegrationService: Send + Sync {
    async fn cross_post_to_yapper(&self, allat_post_id: Uuid, user_id: Uuid) -> Result<Uuid, ApplicationError>;
    async fn cross_post_to_allat(&self, yapper_post_id: Uuid, community_id: Uuid, user_id: Uuid) -> Result<Uuid, ApplicationError>;
}
```

#### 2.2. Update Application Module
Update `src/application/mod.rs`:

```rust
// Add after existing imports
pub mod cross_post_service;

// Add to exports
pub use cross_post_service::{CrossPostService, CrossPostServiceImpl, CrossPostInput};
```

### 3. Infrastructure Layer Implementation

#### 3.1. Create Cross-Post Repository
Create `src/infrastructure/repositories/cross_post_repo.rs`:

```rust
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::cross_post::{CrossPost, SocialApp};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CrossPostRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[async_trait]
pub trait CrossPostRepository: Send + Sync {
    async fn create(&self, cross_post: &CrossPost) -> Result<(), CrossPostRepositoryError>;
    async fn find_by_source(&self, source_id: Uuid, source_app: SocialApp) -> Result<Vec<CrossPost>, CrossPostRepositoryError>;
    async fn find_by_target(&self, target_id: Uuid, target_app: SocialApp) -> Result<Vec<CrossPost>, CrossPostRepositoryError>;
}

pub struct PgCrossPostRepository {
    pool: PgPool,
}

impl PgCrossPostRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CrossPostRepository for PgCrossPostRepository {
    async fn create(&self, cross_post: &CrossPost) -> Result<(), CrossPostRepositoryError> {
        sqlx::query!(
            r#"
            INSERT INTO cross_posts (id, source_app, source_id, target_app, target_id, user_id, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            cross_post.id,
            cross_post.source_app.to_string(),
            cross_post.source_id,
            cross_post.target_app.to_string(),
            cross_post.target_id,
            cross_post.user_id,
            cross_post.created_at
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    async fn find_by_source(&self, source_id: Uuid, source_app: SocialApp) -> Result<Vec<CrossPost>, CrossPostRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, source_app, source_id, target_app, target_id, user_id, created_at
            FROM cross_posts
            WHERE source_id = $1 AND source_app = $2
            ORDER BY created_at DESC
            "#,
            source_id,
            source_app.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let cross_posts = rows
            .into_iter()
            .map(|row| CrossPost {
                id: row.id,
                source_app: SocialApp::from(row.source_app),
                source_id: row.source_id,
                target_app: SocialApp::from(row.target_app),
                target_id: row.target_id,
                user_id: row.user_id,
                created_at: row.created_at,
            })
            .collect();

        Ok(cross_posts)
    }
    
    async fn find_by_target(&self, target_id: Uuid, target_app: SocialApp) -> Result<Vec<CrossPost>, CrossPostRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, source_app, source_id, target_app, target_id, user_id, created_at
            FROM cross_posts
            WHERE target_id = $1 AND target_app = $2
            ORDER BY created_at DESC
            "#,
            target_id,
            target_app.to_string()
        )
        .fetch_all(&self.pool)
        .await?;

        let cross_posts = rows
            .into_iter()
            .map(|row| CrossPost {
                id: row.id,
                source_app: SocialApp::from(row.source_app),
                source_id: row.source_id,
                target_app: SocialApp::from(row.target_app),
                target_id: row.target_id,
                user_id: row.user_id,
                created_at: row.created_at,
            })
            .collect();

        Ok(cross_posts)
    }
}
```

#### 3.2. Create Social Integration Adapter
Create `src/infrastructure/social_integration_adapter.rs`:

```rust
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use social_integration::CrossPostingService;
use crate::application::cross_post_service::{SocialIntegrationService, ApplicationError};

pub struct SocialIntegrationAdapter {
    cross_posting_service: CrossPostingService,
}

impl SocialIntegrationAdapter {
    pub fn new() -> Self {
        Self {
            cross_posting_service: CrossPostingService::new(),
        }
    }
}

#[async_trait]
impl SocialIntegrationService for SocialIntegrationAdapter {
    async fn cross_post_to_yapper(&self, allat_post_id: Uuid, user_id: Uuid) -> Result<Uuid, ApplicationError> {
        // In a real implementation, this would:
        // 1. Call the social_integration package to cross-post to Yapper
        // 2. Return the Yapper post ID
        
        // For now, we'll simulate the process
        match self.cross_posting_service.cross_post_to_yapper(allat_post_id, user_id) {
            Ok(_) => {
                // In a real implementation, we would get the actual Yapper post ID
                // For now, we'll return a placeholder
                Ok(Uuid::new_v4())
            },
            Err(e) => Err(ApplicationError::ServiceError(format!("Cross-posting failed: {}", e))),
        }
    }
    
    async fn cross_post_to_allat(&self, yapper_post_id: Uuid, community_id: Uuid, user_id: Uuid) -> Result<Uuid, ApplicationError> {
        // In a real implementation, this would:
        // 1. Call the social_integration package to cross-post to Allat
        // 2. Return the Allat post ID
        
        // For now, we'll simulate the process
        match self.cross_posting_service.cross_post_to_allat(yapper_post_id, community_id, user_id) {
            Ok(_) => {
                // In a real implementation, we would get the actual Allat post ID
                // For now, we'll return a placeholder
                Ok(Uuid::new_v4())
            },
            Err(e) => Err(ApplicationError::ServiceError(format!("Cross-posting failed: {}", e))),
        }
    }
}
```

#### 3.3. Update Infrastructure Module
Update `src/infrastructure/repositories/mod.rs`:

```rust
// Add after existing imports
pub mod cross_post_repo;

// Add to exports
pub use cross_post_repo::{CrossPostRepository, PgCrossPostRepository};
```

### 4. Database Migration

#### 4.1. Create Migration Script
Create `migrations/0003_cross_posting.up.sql`:

```sql
-- Create table to track cross-post relationships
CREATE TABLE IF NOT EXISTS cross_posts (
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

-- Create indexes for efficient querying
CREATE INDEX IF NOT EXISTS idx_cross_posts_source ON cross_posts(source_app, source_id);
CREATE INDEX IF NOT EXISTS idx_cross_posts_target ON cross_posts(target_app, target_id);
CREATE INDEX IF NOT EXISTS idx_cross_posts_user ON cross_posts(user_id);
```

#### 4.2. Create Down Migration
Create `migrations/0003_cross_posting.down.sql`:

```sql
-- Drop indexes
DROP INDEX IF EXISTS idx_cross_posts_source;
DROP INDEX IF EXISTS idx_cross_posts_target;
DROP INDEX IF EXISTS idx_cross_posts_user;

-- Drop table
DROP TABLE IF EXISTS cross_posts;
```

### 5. API Layer Implementation

#### 5.1. Create GraphQL Objects for Cross-Posting
Create `src/api/objects/cross_post.rs`:

```rust
use async_graphql::{Object, SimpleObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::cross_post::{CrossPost, SocialApp};

pub struct CrossPostObject(CrossPost);

impl From<CrossPost> for CrossPostObject {
    fn from(cross_post: CrossPost) -> Self {
        Self(cross_post)
    }
}

#[Object]
impl CrossPostObject {
    async fn id(&self) -> Uuid {
        self.0.id
    }
    
    async fn source_app(&self) -> String {
        self.0.source_app.to_string()
    }
    
    async fn source_id(&self) -> Uuid {
        self.0.source_id
    }
    
    async fn target_app(&self) -> String {
        self.0.target_app.to_string()
    }
    
    async fn target_id(&self) -> Uuid {
        self.0.target_id
    }
    
    async fn user_id(&self) -> Uuid {
        self.0.user_id
    }
    
    async fn created_at(&self) -> DateTime<Utc> {
        self.0.created_at
    }
}

#[derive(SimpleObject)]
pub struct CrossPostConnection {
    allat_post_id: Option<Uuid>,
    yapper_post_id: Option<Uuid>,
    original_platform: String,
}

impl CrossPostConnection {
    pub fn new(allat_post_id: Option<Uuid>, yapper_post_id: Option<Uuid>, original_platform: String) -> Self {
        Self {
            allat_post_id,
            yapper_post_id,
            original_platform,
        }
    }
}
```

#### 5.2. Update API Objects Module
Update `src/api/objects/mod.rs`:

```rust
// Add after existing imports
pub mod cross_post;

// Add to exports if needed
// pub use cross_post::*;
```

#### 5.3. Extend Post Object with Cross-Post Information
Update `src/api/objects/post.rs`:

```rust
// Add to imports
use crate::application::cross_post_service::CrossPostService;
use crate::domain::cross_post::SocialApp;

// Add to PostObject struct
#[derive(Clone)]
pub struct PostObject {
    pub inner: Post,
    pub cross_post_service: Option<std::sync::Arc<dyn CrossPostService>>,
}

// Update From implementation
impl From<Post> for PostObject {
    fn from(post: Post) -> Self {
        Self {
            inner: post,
            cross_post_service: None,
        }
    }
}

// Add cross_posts field to PostObject
#[Object]
impl PostObject {
    // ... existing methods
    
    async fn cross_posts(&self, ctx: &Context<'_>) -> Result<Vec<CrossPostObject>> {
        let service = ctx.data::<std::sync::Arc<dyn CrossPostService>>()
            .map_err(|_| "Cross-post service not available")?;
            
        let cross_posts = service.get_cross_posts(self.inner.id, SocialApp::Allat).await?;
        Ok(cross_posts.into_iter().map(|cp| cp.into()).collect())
    }
}
```

#### 5.4. Add Cross-Posting Mutations
Update `src/api/mutations.rs`:

```rust
// Add to imports
use crate::application::cross_post_service::{CrossPostService, CrossPostInput};

// Add to MutationRoot impl
async fn cross_post_to_yapper(&self, ctx: &Context<'_>, allat_post_id: Uuid) -> Result<Uuid> {
    let service = ctx.data::<std::sync::Arc<dyn CrossPostService>>()
        .map_err(|_| "Cross-post service not available")?;
    
    // In a real implementation, we would get the current user ID from the context
    let user_id = Uuid::new_v4(); // Placeholder
    
    let input = CrossPostInput {
        source_post_id: allat_post_id,
        target_community_id: None,
        user_id,
    };
    
    service.cross_post_to_yapper(input).await
        .map_err(|e| format!("Failed to cross-post to Yapper: {}", e).into())
}

async fn cross_post_to_allat(&self, ctx: &Context<'_>, yapper_post_id: Uuid, community_id: Uuid) -> Result<Uuid> {
    let service = ctx.data::<std::sync::Arc<dyn CrossPostService>>()
        .map_err(|_| "Cross-post service not available")?;
    
    // In a real implementation, we would get the current user ID from the context
    let user_id = Uuid::new_v4(); // Placeholder
    
    let input = CrossPostInput {
        source_post_id: yapper_post_id,
        target_community_id: Some(community_id),
        user_id,
    };
    
    service.cross_post_to_allat(input).await
        .map_err(|e| format!("Failed to cross-post to Allat: {}", e).into())
}
```

#### 5.5. Update GraphQL Schema
Update the GraphQL schema definition:

```graphql
extend type Post {
    crossPosts: [CrossPost!]!
}

extend type Mutation {
    crossPostToYapper(allatPostId: ID!): ID!
    crossPostToAllat(yapperPostId: ID!, communityId: ID!): ID!
}

type CrossPost {
    id: ID!
    sourceApp: String!
    sourceId: ID!
    targetApp: String!
    targetId: ID!
    userId: ID!
    createdAt: DateTime!
}
```

### 6. Update Main Application to Register Services

Update `src/main.rs`:

```rust
// Add to imports
use crate::application::cross_post_service::{CrossPostServiceImpl, CrossPostService};
use crate::infrastructure::repositories::cross_post_repo::{PgCrossPostRepository, CrossPostRepository};
use crate::infrastructure::social_integration_adapter::SocialIntegrationAdapter;

// In main function, after other service initializations
let cross_post_repo: Arc<dyn CrossPostRepository> = Arc::new(PgCrossPostRepository::new(pool.clone()));
let social_integration_adapter: Arc<dyn crate::application::cross_post_service::SocialIntegrationService> = 
    Arc::new(SocialIntegrationAdapter::new());

let cross_post_service: Arc<dyn CrossPostService> = Arc::new(CrossPostServiceImpl::new(
    social_integration_adapter,
    post_repo.clone(),
    cross_post_repo,
));

// Update GraphQL schema registration to include the new mutations and types
```

### 7. Testing

#### 7.1. Unit Tests for Cross-Post Service
Create `tests/cross_post_service_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::cross_post_service::{CrossPostServiceImpl, CrossPostInput, SocialApp};
    use std::sync::Arc;
    use uuid::Uuid;
    
    // Mock services would be defined here
    
    #[tokio::test]
    async fn test_cross_post_to_yapper() {
        // Implementation for testing cross-post to Yapper functionality
    }
    
    #[tokio::test]
    async fn test_cross_post_to_allat() {
        // Implementation for testing cross-post to Allat functionality
    }
    
    #[tokio::test]
    async fn test_get_cross_posts() {
        // Implementation for testing get cross-posts functionality
    }
}
```

#### 7.2. Repository Tests
Create `tests/cross_post_repo_test.rs` to test the database queries.

#### 7.3. Integration Tests
Update `tests/integration_test.rs` to include cross-posting functionality tests.

### 8. Documentation Updates

Update `docs/allat_architecture.md` to reflect the new cross-posting functionality:

1. Add cross-posting to the Integration Points section
2. Update the TODO list to mark cross-posting as complete
3. Add details about the cross-posting implementation to the Infrastructure Layer section
4. Update the GraphQL schema documentation

## Dependencies

This implementation depends on:
1. The existing `social_integration` shared package
2. PostgreSQL with UUID support
3. The existing repository implementations
4. The GraphQL API layer

## Timeline

Estimated implementation time: 4-5 days

1. Day 1: Domain layer and basic infrastructure
2. Day 2: Repository implementation and database migration
3. Day 3: Application service implementation
4. Day 4: API layer integration and social integration adapter
5. Day 5: Testing and documentation

## Rollback Plan

If issues are encountered:
1. Revert the database migration
2. Remove the new service registrations
3. Revert code changes to repository and service layers
4. Update documentation to reflect rollback

## Security Considerations

1. Ensure that only authorized users can trigger cross-posting
2. Validate all cross-posting inputs to prevent injection attacks
3. Implement rate limiting to prevent abuse of cross-posting functionality
4. Respect user privacy settings when cross-posting content
5. Ensure proper authentication when communicating with Yapper