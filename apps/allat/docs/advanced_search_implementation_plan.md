# Advanced Search Implementation Plan

## Overview

This document details the implementation plan for adding advanced search functionality to the Allat app. The implementation will follow hexagonal architecture principles and integrate with PostgreSQL's full-text search capabilities.

## Implementation Steps

### 1. Database Schema Enhancement

#### 1.1. Create Migration Script
Create a new migration file `migrations/0002_search_enhancement.up.sql`:

```sql
-- Add search vector columns
ALTER TABLE posts ADD COLUMN IF NOT EXISTS search_vector tsvector;
ALTER TABLE communities ADD COLUMN IF NOT EXISTS search_vector tsvector;

-- Populate initial search vectors
UPDATE posts SET search_vector = 
    setweight(to_tsvector('english', title), 'A') || 
    setweight(to_tsvector('english', content), 'B');

UPDATE communities SET search_vector = 
    setweight(to_tsvector('english', name), 'A') || 
    setweight(to_tsvector('english', description), 'B');

-- Create GIN indexes for efficient searching
CREATE INDEX IF NOT EXISTS idx_posts_search ON posts USING GIN(search_vector);
CREATE INDEX IF NOT EXISTS idx_communities_search ON communities USING GIN(search_vector);

-- Create functions to update search vectors
CREATE OR REPLACE FUNCTION update_post_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A') || 
        setweight(to_tsvector('english', COALESCE(NEW.content, '')), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_community_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') || 
        setweight(to_tsvector('english', COALESCE(NEW.description, '')), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

-- Create triggers to automatically update search vectors
CREATE TRIGGER IF NOT EXISTS update_post_search_vector_trigger
    BEFORE INSERT OR UPDATE ON posts
    FOR EACH ROW EXECUTE FUNCTION update_post_search_vector();

CREATE TRIGGER IF NOT EXISTS update_community_search_vector_trigger
    BEFORE INSERT OR UPDATE ON communities
    FOR EACH ROW EXECUTE FUNCTION update_community_search_vector();
```

#### 1.2. Create Down Migration
Create `migrations/0002_search_enhancement.down.sql`:

```sql
-- Drop triggers
DROP TRIGGER IF EXISTS update_post_search_vector_trigger ON posts;
DROP TRIGGER IF EXISTS update_community_search_vector_trigger ON communities;

-- Drop functions
DROP FUNCTION IF EXISTS update_post_search_vector();
DROP FUNCTION IF EXISTS update_community_search_vector();

-- Drop indexes
DROP INDEX IF EXISTS idx_posts_search;
DROP INDEX IF EXISTS idx_communities_search;

-- Drop columns
ALTER TABLE posts DROP COLUMN IF EXISTS search_vector;
ALTER TABLE communities DROP COLUMN IF EXISTS search_vector;
```

### 2. Domain Layer Implementation

#### 2.1. Add Search Helper Methods to Entities
Modify `src/domain/post.rs`:

```rust
// Add after existing imports
use std::fmt::Write;

impl Post {
    // ... existing methods
    
    pub fn get_searchable_text(&self) -> String {
        let mut text = String::new();
        write!(&mut text, "{} {}", self.title, self.content).unwrap();
        text
    }
}
```

Modify `src/domain/community.rs`:

```rust
// Add after existing imports
use std::fmt::Write;

impl Community {
    // ... existing methods
    
    pub fn get_searchable_text(&self) -> String {
        let mut text = String::new();
        write!(&mut text, "{} {}", self.name, self.description).unwrap();
        text
    }
}
```

### 3. Application Layer Implementation

#### 3.1. Create Search Service
Create `src/application/search_service.rs`:

```rust
use async_trait::async_trait;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use crate::domain::post::Post;
use crate::domain::community::Community;
use crate::infrastructure::repositories::post_repo::PostRepository;
use crate::infrastructure::repositories::community_repo::CommunityRepository;
use crate::application::error::ApplicationError;

#[derive(Debug, Clone)]
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

impl SearchServiceImpl {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        community_repo: Arc<dyn CommunityRepository>,
    ) -> Self {
        Self {
            post_repo,
            community_repo,
        }
    }
}

#[async_trait]
impl SearchService for SearchServiceImpl {
    async fn search_posts(&self, criteria: SearchCriteria) -> Result<Vec<Post>, ApplicationError> {
        // This will be implemented in the repository layer
        self.post_repo.search(criteria).await.map_err(ApplicationError::from)
    }
    
    async fn search_communities(&self, query: String) -> Result<Vec<Community>, ApplicationError> {
        // This will be implemented in the repository layer
        self.community_repo.search(&query).await.map_err(ApplicationError::from)
    }
}
```

#### 3.2. Update Application Module
Update `src/application/mod.rs`:

```rust
// Add after existing imports
pub mod search_service;

// Add to exports
pub use search_service::{SearchService, SearchServiceImpl, SearchCriteria};
```

### 4. Infrastructure Layer Implementation

#### 4.1. Extend Repository Traits
Update `src/infrastructure/repositories/post_repo.rs`:

```rust
// Add to imports
use crate::application::search_service::SearchCriteria;

// Add to PostRepository trait
#[async_trait]
pub trait PostRepository: Send + Sync {
    // ... existing methods
    async fn search(&self, criteria: SearchCriteria) -> Result<Vec<Post>, PostRepositoryError>;
}

// Add to PgPostRepository implementation
impl PgPostRepository {
    // ... existing methods
    
    async fn search(&self, criteria: SearchCriteria) -> Result<Vec<Post>, PostRepositoryError> {
        // Base query with full-text search
        let mut query = "SELECT id, community_id, user_id, title, content, created_at, updated_at, parent_id
                        FROM posts 
                        WHERE search_vector @@ websearch_to_tsquery('english', $1)".to_string();
        
        let mut params: Vec<Box<dyn postgres_types::ToSql + Sync>> = vec![
            Box::new(criteria.query)
        ];
        let mut param_index = 2;
        
        // Add community filter if provided
        if let Some(community_id) = criteria.community_id {
            query.push_str(&format!(" AND community_id = ${}", param_index));
            params.push(Box::new(community_id));
            param_index += 1;
        }
        
        // Add author filter if provided
        if let Some(author_id) = criteria.author_id {
            query.push_str(&format!(" AND user_id = ${}", param_index));
            params.push(Box::new(author_id));
            param_index += 1;
        }
        
        // Add date range filters if provided
        if let Some(date_from) = criteria.date_from {
            query.push_str(&format!(" AND created_at >= ${}", param_index));
            params.push(Box::new(date_from));
            param_index += 1;
        }
        
        if let Some(date_to) = criteria.date_to {
            query.push_str(&format!(" AND created_at <= ${}", param_index));
            params.push(Box::new(date_to));
            param_index += 1;
        }
        
        // Add ordering by relevance and date
        query.push_str(" ORDER BY ts_rank(search_vector, websearch_to_tsquery('english', $1)) DESC, created_at DESC");
        
        // Add limit and offset if provided
        if let Some(limit) = criteria.limit {
            query.push_str(&format!(" LIMIT ${}", param_index));
            params.push(Box::new(limit as i64));
            param_index += 1;
        }
        
        if let Some(offset) = criteria.offset {
            query.push_str(&format!(" OFFSET ${}", param_index));
            params.push(Box::new(offset as i64));
        }
        
        // Execute query
        // Note: This is a simplified approach. In practice, we'd need to use a more complex
        // parameter binding mechanism or a query builder.
        todo!("Implement parameter binding for dynamic query")
    }
}
```

Update `src/infrastructure/repositories/community_repo.rs`:

```rust
// Add to CommunityRepository trait
#[async_trait]
pub trait CommunityRepository: Send + Sync {
    // ... existing methods
    async fn search(&self, query: &str) -> Result<Vec<Community>, CommunityRepositoryError>;
}

// Add to PgCommunityRepository implementation
impl PgCommunityRepository {
    // ... existing methods
    
    async fn search(&self, query: &str) -> Result<Vec<Community>, CommunityRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, description, rules, created_at
            FROM communities
            WHERE search_vector @@ websearch_to_tsquery('english', $1)
            ORDER BY ts_rank(search_vector, websearch_to_tsquery('english', $1)) DESC, created_at DESC
            "#,
            query
        )
        .fetch_all(&self.pool)
        .await?;

        let communities = rows
            .into_iter()
            .map(|row| Community {
                id: row.id,
                name: row.name,
                description: row.description,
                rules: row.rules,
                created_at: row.created_at,
            })
            .collect();

        Ok(communities)
    }
}
```

### 5. API Layer Implementation

#### 5.1. Update GraphQL Schema
Update `src/api/schema.rs` or create new objects in `src/api/objects/input.rs`:

```rust
// Add to src/api/objects/input.rs
use async_graphql::InputObject;

#[derive(InputObject, Debug)]
pub struct SearchCriteriaInput {
    pub query: String,
    pub community_id: Option<uuid::Uuid>,
    pub author_id: Option<uuid::Uuid>,
    pub date_from: Option<chrono::DateTime<chrono::Utc>>,
    pub date_to: Option<chrono::DateTime<chrono::Utc>>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}
```

Update `src/api/queries.rs`:

```rust
// Add to imports
use crate::api::objects::input::SearchCriteriaInput;
use crate::application::search_service::SearchCriteria;

// Add to QueryRoot impl
async fn search_posts(&self, ctx: &Context<'_>, criteria: SearchCriteriaInput) -> Result<Vec<PostObject>> {
    let service = ctx.data::<std::sync::Arc<dyn SearchService>>()?;
    
    let search_criteria = SearchCriteria {
        query: criteria.query,
        community_id: criteria.community_id,
        author_id: criteria.author_id,
        date_from: criteria.date_from,
        date_to: criteria.date_to,
        limit: criteria.limit.map(|l| l as u32),
        offset: criteria.offset.map(|o| o as u32),
    };
    
    let posts = service.search_posts(search_criteria).await?;
    Ok(posts.into_iter().map(PostObject::from).collect())
}

async fn search_communities(&self, ctx: &Context<'_>, query: String) -> Result<Vec<CommunityObject>> {
    let service = ctx.data::<std::sync::Arc<dyn SearchService>>()?;
    let communities = service.search_communities(query).await?;
    Ok(communities.into_iter().map(CommunityObject::from).collect())
}
```

Update GraphQL schema definition:

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

### 6. Dependency Injection and Service Registration

Update `src/main.rs` to register the new service:

```rust
// Add to imports
use crate::application::search_service::{SearchServiceImpl, SearchService};

// In main function, after other service initializations
let search_service: Arc<dyn SearchService> = Arc::new(SearchServiceImpl::new(
    post_repo.clone(),
    community_repo.clone(),
));
```

Update GraphQL schema registration to include the new queries.

### 7. Testing

#### 7.1. Unit Tests for Search Service
Create `tests/search_service_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::search_service::{SearchServiceImpl, SearchCriteria};
    use std::sync::Arc;
    
    // Mock repositories would be defined here
    
    #[tokio::test]
    async fn test_search_posts() {
        // Implementation for testing search posts functionality
    }
    
    #[tokio::test]
    async fn test_search_communities() {
        // Implementation for testing search communities functionality
    }
}
```

#### 7.2. Integration Tests
Update `tests/integration_test.rs` to include search functionality tests.

### 8. Documentation Updates

Update `docs/allat_architecture.md` to reflect the new search functionality:

1. Add search to the Core Components section
2. Update the TODO list to mark search as complete
3. Add details about the search implementation to the Infrastructure Layer section

## Dependencies

This implementation depends on:
1. PostgreSQL with full-text search capabilities
2. The existing repository implementations
3. The GraphQL API layer

## Timeline

Estimated implementation time: 3-5 days

1. Day 1: Database schema changes and domain layer updates
2. Day 2: Application layer implementation
3. Day 3: Infrastructure layer implementation
4. Day 4: API layer integration and testing
5. Day 5: Documentation and final testing

## Rollback Plan

If issues are encountered:
1. Revert the database migration
2. Remove the new service registration
3. Revert code changes to repository and service layers
4. Update documentation to reflect rollback