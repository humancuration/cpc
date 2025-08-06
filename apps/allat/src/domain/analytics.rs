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