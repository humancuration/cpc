use async_trait::async_trait;
use uuid::Uuid;
use std::sync::Arc;
use crate::domain::analytics::{
    CommunityGrowthMetrics, UserContributionMetrics, 
    EngagementMetrics, CommunityOverview
};
use crate::infrastructure::repositories::post_repo::PostRepository;
use crate::infrastructure::repositories::community_repo::CommunityRepository;
use crate::infrastructure::repositories::analytics_repo::AnalyticsRepository;
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
    analytics_repo: Arc<dyn AnalyticsRepository>,
    // We'll need to add user repository if we don't have it already
}

impl AnalyticsServiceImpl {
    pub fn new(
        post_repo: Arc<dyn PostRepository>,
        community_repo: Arc<dyn CommunityRepository>,
        analytics_repo: Arc<dyn AnalyticsRepository>,
    ) -> Self {
        Self {
            post_repo,
            community_repo,
            analytics_repo,
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
        let member_count_history = self.analytics_repo.get_member_growth_history(community_id, days).await?;
        let post_count_history = self.analytics_repo.get_post_activity_history(community_id, days).await?;
        let comment_count_history = self.analytics_repo.get_comment_activity_history(community_id, days).await?;
        
        Ok(CommunityGrowthMetrics {
            community_id,
            member_count_history,
            post_count_history,
            comment_count_history,
        })
    }
    
    async fn get_top_contributors(
        &self,
        community_id: Uuid,
        limit: u32
    ) -> Result<Vec<UserContributionMetrics>, ApplicationError> {
        self.analytics_repo.get_top_contributors(community_id, limit).await
            .map_err(ApplicationError::from)
    }
    
    async fn get_top_engaged_posts(
        &self,
        community_id: Uuid,
        limit: u32
    ) -> Result<Vec<EngagementMetrics>, ApplicationError> {
        self.analytics_repo.get_top_engaged_posts(community_id, limit).await
            .map_err(ApplicationError::from)
    }
    
    async fn get_community_overview(
        &self,
        community_id: Uuid
    ) -> Result<CommunityOverview, ApplicationError> {
        self.analytics_repo.get_community_overview(community_id).await
            .map_err(ApplicationError::from)
    }
}