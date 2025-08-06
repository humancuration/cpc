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