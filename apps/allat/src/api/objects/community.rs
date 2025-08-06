use async_graphql::{SimpleObject, ID, Context, Object};
use chrono::{DateTime, Utc};
use crate::domain::community::Community;
use crate::application::analytics_service::AnalyticsService;
use crate::api::objects::analytics::{CommunityGrowthMetricsObject, UserContributionMetricsObject, EngagementMetricsObject, CommunityOverviewObject};

#[derive(Clone)]
pub struct CommunityObject {
    pub inner: Community,
    pub analytics_service: Option<std::sync::Arc<dyn AnalyticsService>>,
}

impl From<Community> for CommunityObject {
    fn from(community: Community) -> Self {
        Self {
            inner: community,
            analytics_service: None,
        }
    }
}

#[Object]
impl CommunityObject {
    async fn id(&self) -> ID {
        ID::from(self.inner.id.to_string())
    }
    
    async fn name(&self) -> &str {
        &self.inner.name
    }
    
    async fn description(&self) -> &str {
        &self.inner.description
    }
    
    async fn rules(&self) -> &Vec<String> {
        &self.inner.rules
    }
    
    async fn created_at(&self) -> DateTime<Utc> {
        self.inner.created_at
    }
    
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

impl From<Community> for CommunityObject {
    fn from(community: Community) -> Self {
        Self {
            id: ID::from(community.id.to_string()),
            name: community.name,
            description: community.description,
            rules: community.rules,
            created_at: community.created_at,
        }
    }
}