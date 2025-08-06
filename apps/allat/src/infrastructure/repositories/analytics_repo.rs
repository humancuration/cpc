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