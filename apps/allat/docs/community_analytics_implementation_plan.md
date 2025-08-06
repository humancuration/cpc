# Community Analytics Dashboard Implementation Plan

## Overview

This document details the implementation plan for adding a community analytics dashboard to the Allat app. The implementation will follow hexagonal architecture principles and leverage existing visualization capabilities in the CPC ecosystem.

## Implementation Steps

### 1. Domain Layer Implementation

#### 1.1. Create Analytics Entities
Create `src/domain/analytics.rs`:

```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityGrowthMetrics {
    pub community_id: Uuid,
    pub member_count_history: Vec<TimeSeriesPoint>,
    pub post_count_history: Vec<TimeSeriesPoint>,
    pub comment_count_history: Vec<TimeSeriesPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContributionMetrics {
    pub user_id: Uuid,
    pub username: String,
    pub post_count: u32,
    pub comment_count: u32,
    pub karma: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementMetrics {
    pub post_id: Uuid,
    pub title: String,
    pub upvotes: u32,
    pub downvotes: u32,
    pub comment_count: u32,
    pub engagement_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityOverview {
    pub total_members: u32,
    pub total_posts: u32,
    pub total_comments: u32,
    pub average_karma: i32,
    pub most_active_day: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSeriesPoint {
    pub timestamp: DateTime<Utc>,
    pub value: i32,
}

impl TimeSeriesPoint {
    pub fn new(timestamp: DateTime<Utc>, value: i32) -> Self {
        Self { timestamp, value }
    }
}
```

#### 1.2. Update Domain Module
Update `src/domain/mod.rs`:

```rust
// Add after existing imports
pub mod analytics;

// Add to exports
pub use analytics::*;
```

### 2. Application Layer Implementation

#### 2.1. Create Analytics Service
Create `src/application/analytics_service.rs`:

```rust
use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::domain::analytics::{
    CommunityGrowthMetrics, UserContributionMetrics, 
    EngagementMetrics, CommunityOverview
};
use crate::infrastructure::repositories::post_repo::PostRepository;
use crate::infrastructure::repositories::community_repo::CommunityRepository;
use crate::application::error::ApplicationError;

#[async_trait]
pub trait AnalyticsService: Send + Sync {
    async fn get_community_growth_metrics(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<CommunityGrowthMetrics, ApplicationError>;
    
    async fn get_top_contributors(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<UserContributionMetrics>, ApplicationError>;
    
    async fn get_top_engaged_posts(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<EngagementMetrics>, ApplicationError>;
    
    async fn get_community_overview(
        &self, 
        community_id: Uuid
    ) -> Result<CommunityOverview, ApplicationError>;
}

pub struct AnalyticsServiceImpl {
    post_repo: Arc<dyn PostRepository>,
    community_repo: Arc<dyn CommunityRepository>,
    // We'll need to add user repository if we don't have it already
}

impl AnalyticsServiceImpl {
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
impl AnalyticsService for AnalyticsServiceImpl {
    async fn get_community_growth_metrics(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<CommunityGrowthMetrics, ApplicationError> {
        // This will be implemented in the repository layer
        todo!("Implement get_community_growth_metrics")
    }
    
    async fn get_top_contributors(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<UserContributionMetrics>, ApplicationError> {
        // This will be implemented in the repository layer
        todo!("Implement get_top_contributors")
    }
    
    async fn get_top_engaged_posts(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<EngagementMetrics>, ApplicationError> {
        // This will be implemented in the repository layer
        todo!("Implement get_top_engaged_posts")
    }
    
    async fn get_community_overview(
        &self, 
        community_id: Uuid
    ) -> Result<CommunityOverview, ApplicationError> {
        // This will be implemented in the repository layer
        todo!("Implement get_community_overview")
    }
}
```

#### 2.2. Update Application Module
Update `src/application/mod.rs`:

```rust
// Add after existing imports
pub mod analytics_service;

// Add to exports
pub use analytics_service::{AnalyticsService, AnalyticsServiceImpl};
```

### 3. Infrastructure Layer Implementation

#### 3.1. Create Analytics Repository
Create `src/infrastructure/repositories/analytics_repo.rs`:

```rust
use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::domain::analytics::{
    TimeSeriesPoint, UserContributionMetrics, 
    EngagementMetrics, CommunityOverview
};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AnalyticsRepositoryError {
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

#[async_trait]
pub trait AnalyticsRepository: Send + Sync {
    async fn get_member_growth_history(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsRepositoryError>;
    
    async fn get_post_activity_history(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsRepositoryError>;
    
    async fn get_comment_activity_history(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsRepositoryError>;
    
    async fn get_top_contributors(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<UserContributionMetrics>, AnalyticsRepositoryError>;
    
    async fn get_top_engaged_posts(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<EngagementMetrics>, AnalyticsRepositoryError>;
    
    async fn get_community_overview(
        &self, 
        community_id: Uuid
    ) -> Result<CommunityOverview, AnalyticsRepositoryError>;
}

pub struct PgAnalyticsRepository {
    pool: PgPool,
}

impl PgAnalyticsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AnalyticsRepository for PgAnalyticsRepository {
    async fn get_member_growth_history(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                DATE(created_at) as day,
                COUNT(*) as count
            FROM posts 
            WHERE community_id = $1 
                AND parent_id IS NULL
                AND created_at >= CURRENT_DATE - INTERVAL '$2 days'
            GROUP BY DATE(created_at)
            ORDER BY day
            "#,
            community_id,
            days as i32
        )
        .fetch_all(&self.pool)
        .await?;

        let points = rows
            .into_iter()
            .map(|row| {
                // We need to handle the optional date field
                TimeSeriesPoint::new(
                    row.day.unwrap(), // This assumes the date is always present
                    row.count.unwrap_or(0) as i32
                )
            })
            .collect();

        Ok(points)
    }
    
    async fn get_post_activity_history(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                DATE(created_at) as day,
                COUNT(*) as count
            FROM posts 
            WHERE community_id = $1 
                AND parent_id IS NULL
                AND created_at >= CURRENT_DATE - INTERVAL '$2 days'
            GROUP BY DATE(created_at)
            ORDER BY day
            "#,
            community_id,
            days as i32
        )
        .fetch_all(&self.pool)
        .await?;

        let points = rows
            .into_iter()
            .map(|row| {
                TimeSeriesPoint::new(
                    row.day.unwrap(),
                    row.count.unwrap_or(0) as i32
                )
            })
            .collect();

        Ok(points)
    }
    
    async fn get_comment_activity_history(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<Vec<TimeSeriesPoint>, AnalyticsRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                DATE(created_at) as day,
                COUNT(*) as count
            FROM posts 
            WHERE community_id = $1 
                AND parent_id IS NOT NULL
                AND created_at >= CURRENT_DATE - INTERVAL '$2 days'
            GROUP BY DATE(created_at)
            ORDER BY day
            "#,
            community_id,
            days as i32
        )
        .fetch_all(&self.pool)
        .await?;

        let points = rows
            .into_iter()
            .map(|row| {
                TimeSeriesPoint::new(
                    row.day.unwrap(),
                    row.count.unwrap_or(0) as i32
                )
            })
            .collect();

        Ok(points)
    }
    
    async fn get_top_contributors(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<UserContributionMetrics>, AnalyticsRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                u.id,
                u.username,
                COUNT(p.id) as post_count,
                COUNT(c.id) as comment_count,
                u.karma
            FROM users u
            LEFT JOIN posts p ON u.id = p.user_id AND p.community_id = $1 AND p.parent_id IS NULL
            LEFT JOIN posts c ON u.id = c.user_id AND c.community_id = $1 AND c.parent_id IS NOT NULL
            WHERE u.id IN (
                SELECT DISTINCT user_id 
                FROM posts 
                WHERE community_id = $1
            )
            GROUP BY u.id, u.username, u.karma
            ORDER BY (COUNT(p.id) + COUNT(c.id)) DESC
            LIMIT $2
            "#,
            community_id,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let contributors = rows
            .into_iter()
            .map(|row| UserContributionMetrics {
                user_id: row.id,
                username: row.username,
                post_count: row.post_count.unwrap_or(0) as u32,
                comment_count: row.comment_count.unwrap_or(0) as u32,
                karma: row.karma,
            })
            .collect();

        Ok(contributors)
    }
    
    async fn get_top_engaged_posts(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<EngagementMetrics>, AnalyticsRepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT 
                p.id,
                p.title,
                COUNT(v.id) FILTER (WHERE v.vote_type = 'Upvote') as upvotes,
                COUNT(v.id) FILTER (WHERE v.vote_type = 'Downvote') as downvotes,
                COUNT(c.id) as comment_count
            FROM posts p
            LEFT JOIN votes v ON p.id = v.post_id
            LEFT JOIN posts c ON p.id = c.parent_id AND c.parent_id IS NOT NULL
            WHERE p.community_id = $1 AND p.parent_id IS NULL
            GROUP BY p.id, p.title
            ORDER BY (COUNT(v.id) + COUNT(c.id)) DESC
            LIMIT $2
            "#,
            community_id,
            limit as i64
        )
        .fetch_all(&self.pool)
        .await?;

        let posts = rows
            .into_iter()
            .map(|row| {
                let upvotes = row.upvotes.unwrap_or(0) as u32;
                let downvotes = row.downvotes.unwrap_or(0) as u32;
                let comment_count = row.comment_count.unwrap_or(0) as u32;
                
                // Simple engagement score calculation
                let engagement_score = (upvotes + comment_count) as f64 / (upvotes + downvotes + 1) as f64;
                
                EngagementMetrics {
                    post_id: row.id,
                    title: row.title,
                    upvotes,
                    downvotes,
                    comment_count,
                    engagement_score,
                }
            })
            .collect();

        Ok(posts)
    }
    
    async fn get_community_overview(
        &self, 
        community_id: Uuid
    ) -> Result<CommunityOverview, AnalyticsRepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT 
                COUNT(DISTINCT p.user_id) as total_members,
                COUNT(p.id) FILTER (WHERE p.parent_id IS NULL) as total_posts,
                COUNT(c.id) as total_comments,
                COALESCE(AVG(u.karma), 0) as average_karma,
                (
                    SELECT DATE(created_at)
                    FROM posts
                    WHERE community_id = $1
                    GROUP BY DATE(created_at)
                    ORDER BY COUNT(*) DESC
                    LIMIT 1
                ) as most_active_day
            FROM posts p
            LEFT JOIN posts c ON p.id = c.parent_id AND c.parent_id IS NOT NULL
            LEFT JOIN users u ON p.user_id = u.id
            WHERE p.community_id = $1
            "#,
            community_id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(CommunityOverview {
            total_members: row.total_members.unwrap_or(0) as u32,
            total_posts: row.total_posts.unwrap_or(0) as u32,
            total_comments: row.total_comments.unwrap_or(0) as u32,
            average_karma: row.average_karma.unwrap_or(0.0) as i32,
            most_active_day: row.most_active_day.unwrap_or_else(|| Utc::now()),
        })
    }
}
```

#### 3.2. Update Infrastructure Module
Update `src/infrastructure/repositories/mod.rs`:

```rust
// Add after existing imports
pub mod analytics_repo;

// Add to exports
pub use analytics_repo::{AnalyticsRepository, PgAnalyticsRepository};
```

#### 3.3. Implement Analytics Service Methods
Update `src/application/analytics_service.rs` to implement the actual methods:

```rust
// Replace the todo! implementations with actual code
#[async_trait]
impl AnalyticsService for AnalyticsServiceImpl {
    async fn get_community_growth_metrics(
        &self, 
        community_id: Uuid, 
        days: u32
    ) -> Result<CommunityGrowthMetrics, ApplicationError> {
        // We'll need to create or inject an AnalyticsRepository
        // For now, we'll assume it's available through the service
        todo!("Implement with AnalyticsRepository")
    }
    
    async fn get_top_contributors(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<UserContributionMetrics>, ApplicationError> {
        todo!("Implement with AnalyticsRepository")
    }
    
    async fn get_top_engaged_posts(
        &self, 
        community_id: Uuid, 
        limit: u32
    ) -> Result<Vec<EngagementMetrics>, ApplicationError> {
        todo!("Implement with AnalyticsRepository")
    }
    
    async fn get_community_overview(
        &self, 
        community_id: Uuid
    ) -> Result<CommunityOverview, ApplicationError> {
        todo!("Implement with AnalyticsRepository")
    }
}
```

### 4. API Layer Implementation

#### 4.1. Create GraphQL Objects for Analytics
Create `src/api/objects/analytics.rs`:

```rust
use async_graphql::{Object, SimpleObject, ComplexObject};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use crate::domain::analytics::*;

pub struct CommunityGrowthMetricsObject(CommunityGrowthMetrics);

impl From<CommunityGrowthMetrics> for CommunityGrowthMetricsObject {
    fn from(metrics: CommunityGrowthMetrics) -> Self {
        Self(metrics)
    }
}

#[Object]
impl CommunityGrowthMetricsObject {
    async fn community_id(&self) -> Uuid {
        self.0.community_id
    }
    
    async fn member_count_history(&self) -> Vec<TimeSeriesPointObject> {
        self.0.member_count_history.iter().map(|p| p.clone().into()).collect()
    }
    
    async fn post_count_history(&self) -> Vec<TimeSeriesPointObject> {
        self.0.post_count_history.iter().map(|p| p.clone().into()).collect()
    }
    
    async fn comment_count_history(&self) -> Vec<TimeSeriesPointObject> {
        self.0.comment_count_history.iter().map(|p| p.clone().into()).collect()
    }
}

pub struct UserContributionMetricsObject(UserContributionMetrics);

impl From<UserContributionMetrics> for UserContributionMetricsObject {
    fn from(metrics: UserContributionMetrics) -> Self {
        Self(metrics)
    }
}

#[Object]
impl UserContributionMetricsObject {
    async fn user_id(&self) -> Uuid {
        self.0.user_id
    }
    
    async fn username(&self) -> &str {
        &self.0.username
    }
    
    async fn post_count(&self) -> u32 {
        self.0.post_count
    }
    
    async fn comment_count(&self) -> u32 {
        self.0.comment_count
    }
    
    async fn karma(&self) -> i32 {
        self.0.karma
    }
}

pub struct EngagementMetricsObject(EngagementMetrics);

impl From<EngagementMetrics> for EngagementMetricsObject {
    fn from(metrics: EngagementMetrics) -> Self {
        Self(metrics)
    }
}

#[Object]
impl EngagementMetricsObject {
    async fn post_id(&self) -> Uuid {
        self.0.post_id
    }
    
    async fn title(&self) -> &str {
        &self.0.title
    }
    
    async fn upvotes(&self) -> u32 {
        self.0.upvotes
    }
    
    async fn downvotes(&self) -> u32 {
        self.0.downvotes
    }
    
    async fn comment_count(&self) -> u32 {
        self.0.comment_count
    }
    
    async fn engagement_score(&self) -> f64 {
        self.0.engagement_score
    }
}

pub struct CommunityOverviewObject(CommunityOverview);

impl From<CommunityOverview> for CommunityOverviewObject {
    fn from(overview: CommunityOverview) -> Self {
        Self(overview)
    }
}

#[Object]
impl CommunityOverviewObject {
    async fn total_members(&self) -> u32 {
        self.0.total_members
    }
    
    async fn total_posts(&self) -> u32 {
        self.0.total_posts
    }
    
    async fn total_comments(&self) -> u32 {
        self.0.total_comments
    }
    
    async fn average_karma(&self) -> i32 {
        self.0.average_karma
    }
    
    async fn most_active_day(&self) -> DateTime<Utc> {
        self.0.most_active_day
    }
}

#[derive(SimpleObject)]
pub struct TimeSeriesPointObject {
    timestamp: DateTime<Utc>,
    value: i32,
}

impl From<TimeSeriesPoint> for TimeSeriesPointObject {
    fn from(point: TimeSeriesPoint) -> Self {
        Self {
            timestamp: point.timestamp,
            value: point.value,
        }
    }
}
```

#### 4.2. Update API Objects Module
Update `src/api/objects/mod.rs`:

```rust
// Add after existing imports
pub mod analytics;

// Add to exports if needed
// pub use analytics::*;
```

#### 4.3. Extend Community Object with Analytics
Update `src/api/objects/community.rs`:

```rust
// Add to imports
use crate::application::analytics_service::AnalyticsService;

// Add to CommunityObject struct
#[derive(Clone)]
pub struct CommunityObject {
    pub inner: Community,
    pub analytics_service: Option<std::sync::Arc<dyn AnalyticsService>>,
}

// Update From implementation
impl From<Community> for CommunityObject {
    fn from(community: Community) -> Self {
        Self {
            inner: community,
            analytics_service: None,
        }
    }
}

// Add analytics field to CommunityObject
#[Object]
impl CommunityObject {
    // ... existing methods
    
    async fn analytics(&self, ctx: &Context<'_>, days: Option<i32>) -> Result<CommunityAnalyticsObject> {
        let service = ctx.data::<std::sync::Arc<dyn AnalyticsService>>()
            .map_err(|_| "Analytics service not available")?;
            
        let days = days.unwrap_or(30) as u32;
        
        let growth_metrics = service.get_community_growth_metrics(self.inner.id, days).await?;
        let top_contributors = service.get_top_contributors(self.inner.id, 10).await?;
        let top_engaged_posts = service.get_top_engaged_posts(self.inner.id, 10).await?;
        let overview = service.get_community_overview(self.inner.id).await?;
        
        Ok(CommunityAnalyticsObject {
            growth_metrics: growth_metrics.into(),
            top_contributors: top_contributors.into_iter().map(|c| c.into()).collect(),
            top_engaged_posts: top_engaged_posts.into_iter().map(|p| p.into()).collect(),
            overview: overview.into(),
        })
    }
}

// Create CommunityAnalyticsObject
pub struct CommunityAnalyticsObject {
    growth_metrics: CommunityGrowthMetricsObject,
    top_contributors: Vec<UserContributionMetricsObject>,
    top_engaged_posts: Vec<EngagementMetricsObject>,
    overview: CommunityOverviewObject,
}

#[Object]
impl CommunityAnalyticsObject {
    async fn growth_metrics(&self) -> &CommunityGrowthMetricsObject {
        &self.growth_metrics
    }
    
    async fn top_contributors(&self) -> &Vec<UserContributionMetricsObject> {
        &self.top_contributors
    }
    
    async fn top_engaged_posts(&self) -> &Vec<EngagementMetricsObject> {
        &self.top_engaged_posts
    }
    
    async fn overview(&self) -> &CommunityOverviewObject {
        &self.overview
    }
}
```

### 5. Dependency Injection and Service Registration

Update `src/main.rs` to register the new services:

```rust
// Add to imports
use crate::application::analytics_service::{AnalyticsServiceImpl, AnalyticsService};
use crate::infrastructure::repositories::analytics_repo::{PgAnalyticsRepository, AnalyticsRepository};

// In main function, after other service initializations
let analytics_repo: Arc<dyn AnalyticsRepository> = Arc::new(PgAnalyticsRepository::new(pool.clone()));
let analytics_service: Arc<dyn AnalyticsService> = Arc::new(AnalyticsServiceImpl::new(
    post_repo.clone(),
    community_repo.clone(),
));
```

Update GraphQL schema registration to include the new analytics objects.

### 6. Integration with Visualization Packages

#### 6.1. Add Dependencies
Update `Cargo.toml` to include visualization dependencies:

```toml
[dependencies]
# ... existing dependencies
plotters = { workspace = true }
bi_visualization = { path = "../../shared_packages/bi_visualization" }
```

#### 6.2. Create Chart Generation Service
Create `src/application/chart_service.rs`:

```rust
use plotters::prelude::*;
use crate::domain::analytics::TimeSeriesPoint;
use std::io::Cursor;

pub struct ChartService;

impl ChartService {
    pub fn new() -> Self {
        Self
    }
    
    pub fn generate_time_series_chart(
        &self,
        data: &[TimeSeriesPoint],
        title: &str,
        width: u32,
        height: u32,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut buffer = vec![0; (width * height * 3) as usize];
        let root = BitMapBackend::with_buffer(&mut buffer, (width, height)).into_drawing_area();
        
        root.fill(&WHITE)?;
        
        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 20))
            .margin(10)
            .x_label_area_size(30)
            .y_label_area_size(30)
            .build_cartesian_2d(
                data.iter().map(|p| p.timestamp).min().unwrap()..data.iter().map(|p| p.timestamp).max().unwrap(),
                0..data.iter().map(|p| p.value).max().unwrap_or(10),
            )?;
        
        chart.configure_mesh().draw()?;
        
        chart.draw_series(LineSeries::new(
            data.iter().map(|p| (p.timestamp, p.value)),
            &RED,
        ))?;
        
        root.present()?;
        
        Ok(buffer)
    }
}
```

### 7. Testing

#### 7.1. Unit Tests for Analytics Service
Create `tests/analytics_service_test.rs`:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::analytics_service::AnalyticsServiceImpl;
    use std::sync::Arc;
    
    // Mock repositories would be defined here
    
    #[tokio::test]
    async fn test_get_community_overview() {
        // Implementation for testing community overview functionality
    }
    
    #[tokio::test]
    async fn test_get_top_contributors() {
        // Implementation for testing top contributors functionality
    }
}
```

#### 7.2. Repository Tests
Create `tests/analytics_repo_test.rs` to test the database queries.

### 8. Documentation Updates

Update `docs/allat_architecture.md` to reflect the new analytics functionality:

1. Add analytics to the Core Components section
2. Update the TODO list to mark analytics as complete
3. Add details about the analytics implementation to the Infrastructure Layer section
4. Add information about visualization integration

## Dependencies

This implementation depends on:
1. PostgreSQL with date/time functions
2. The existing repository implementations
3. The GraphQL API layer
4. `plotters` crate for chart generation
5. `bi_visualization` shared package for advanced visualizations

## Timeline

Estimated implementation time: 4-6 days

1. Day 1: Domain layer and basic infrastructure
2. Day 2: Repository implementation and database queries
3. Day 3: Application service implementation
4. Day 4: API layer integration
5. Day 5: Visualization integration and chart generation
6. Day 6: Testing and documentation

## Rollback Plan

If issues are encountered:
1. Remove the new service registrations
2. Remove the new repository implementations
3. Revert code changes to domain and application layers
4. Update documentation to reflect rollback