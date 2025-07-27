//! Analytics service for tracking site visits and link clicks

use std::sync::Arc;
use uuid::Uuid;
use tracing::instrument;

use crate::domain::models::{AnalyticsReport};
use crate::domain::errors::WebsiteBuilderError;
use crate::infrastructure::repository::SiteRepository;

pub struct AnalyticsService {
    site_repository: Arc<SiteRepository>,
}

impl AnalyticsService {
    pub fn new(site_repository: Arc<SiteRepository>) -> Self {
        Self { site_repository }
    }

    /// Tracks a link click
    #[instrument(skip(self))]
    pub async fn track_link_click(&self, link_id: Uuid) -> Result<(), WebsiteBuilderError> {
        // Increment the click counter for the link
        self.site_repository.increment_link_click_count(link_id).await?;
        
        // Also increment the overall click count for the site
        self.site_repository.increment_site_click_count(link_id).await?;
        
        Ok(())
    }

    /// Gets analytics data for a site
    #[instrument(skip(self))]
    pub async fn get_analytics_data(
        &self,
        site_id: Uuid,
        period_start: chrono::DateTime<chrono::Utc>,
        period_end: chrono::DateTime<chrono::Utc>,
    ) -> Result<AnalyticsReport, WebsiteBuilderError> {
        // Get the analytics data from the repository
        let report = self.site_repository.get_analytics_report(site_id, period_start, period_end).await?;
        
        Ok(report)
    }

    /// Tracks a page view
    #[instrument(skip(self))]
    pub async fn track_page_view(&self, page_id: Uuid) -> Result<(), WebsiteBuilderError> {
        // Increment the view counter for the page
        self.site_repository.increment_page_view_count(page_id).await?;
        
        Ok(())
    }
}