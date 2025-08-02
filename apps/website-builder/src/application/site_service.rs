//! Site service for managing websites and link-in-bio sites

use std::sync::Arc;
use uuid::Uuid;
use tracing::instrument;

use crate::domain::models::{Site, SiteType};
use crate::domain::errors::WebsiteBuilderError;
use crate::infrastructure::repository::SiteRepository;
use crate::application::template_service::TemplateService;
use crate::infrastructure::grpc::{FundraisingClient, CreateCampaignRequest};
use crate::domain::models::{SiteType, FundraisingCampaignData};

pub struct SiteService {
    site_repository: Arc<SiteRepository>,
    template_service: Arc<TemplateService>,
    fundraising_client: Arc<FundraisingClient>,
}

impl SiteService {
    pub fn new(
        site_repository: Arc<SiteRepository>,
        template_service: Arc<TemplateService>,
        fundraising_client: Arc<FundraisingClient>,
    ) -> Self {
        Self {
            site_repository,
            template_service,
            fundraising_client,
        }
    }

    /// Creates a new site with the specified type
    #[instrument(skip(self))]
    pub async fn create_site(
        &self,
        owner_id: Uuid,
        site_type: SiteType,
        name: String,
    ) -> Result<Site, WebsiteBuilderError> {
        // Validate that the name is provided
        if name.is_empty() {
            return Err(WebsiteBuilderError::SiteNameRequired);
        }

        // Handle fundraising campaigns specially
        if let SiteType::FundraisingCampaign(campaign_data) = site_type {
            return self.create_fundraising_campaign(owner_id, campaign_data, name).await;
        }

        // For link-in-bio sites, check if user already has one
        if matches!(site_type, SiteType::LinkInBio(_)) {
            if self.site_repository.has_link_in_bio_site(owner_id).await? {
                return Err(WebsiteBuilderError::LinkInBioSiteExists);
            }
        }

        // Create the site entity
        let site = Site {
            id: Uuid::new_v4(),
            owner_id,
            site_type,
            name,
            custom_domain: None,
            primary_color: "#000000".to_string(),
            secondary_color: "#FFFFFF".to_string(),
            font_family: "Arial, sans-serif".to_string(),
            is_published: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Save the site
        let saved_site = self.site_repository.create_site(site).await?;

        Ok(saved_site)
    }
    
    /// Creates a new fundraising campaign site
    #[instrument(skip(self))]
    pub async fn create_fundraising_campaign(
        &self,
        owner_id: Uuid,
        campaign_data: FundraisingCampaignData,
        name: String,
    ) -> Result<Site, WebsiteBuilderError> {
        // Validate that the name is provided
        if name.is_empty() {
            return Err(WebsiteBuilderError::SiteNameRequired);
        }
        
        // Create the fundraising campaign in the cooperative fundraising service
        let request = CreateCampaignRequest {
            title: campaign_data.campaign_title.clone(),
            description: campaign_data.campaign_description.clone(),
            campaign_type: campaign_data.campaign_type.clone(),
            owner_user_id: owner_id,
            goal_amount: campaign_data.goal_amount,
            currency: Some("USD".to_string()), // Default currency for now
            start_date: campaign_data.start_date,
            end_date: campaign_data.end_date,
        };
        
        let response = self.fundraising_client.create_campaign(request).await?;
        
        // Update the campaign data with the response
        let updated_campaign_data = FundraisingCampaignData {
            campaign_id: response.campaign_id,
            campaign_title: response.title,
            campaign_description: response.description,
            campaign_type: response.campaign_type.into(),
            goal_amount: response.goal_amount,
            current_amount: response.current_amount,
            start_date: response.start_date,
            end_date: response.end_date,
        };
        
        // Create the site entity with the fundraising campaign data
        let site = Site {
            id: Uuid::new_v4(),
            owner_id,
            site_type: SiteType::FundraisingCampaign(updated_campaign_data),
            name,
            custom_domain: None,
            primary_color: "#000000".to_string(),
            secondary_color: "#FFFFFF".to_string(),
            font_family: "Arial, sans-serif".to_string(),
            is_published: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Save the site
        let saved_site = self.site_repository.create_site(site).await?;

        Ok(saved_site)
    }

    /// Updates site settings
    #[instrument(skip(self))]
    pub async fn update_site_settings(
        &self,
        site_id: Uuid,
        owner_id: Uuid,
        name: Option<String>,
        custom_domain: Option<String>,
        primary_color: Option<String>,
        secondary_color: Option<String>,
        font_family: Option<String>,
    ) -> Result<Site, WebsiteBuilderError> {
        let mut site = self.site_repository.get_site_by_id(site_id).await?;

        // Check ownership
        if site.owner_id != owner_id {
            return Err(WebsiteBuilderError::Unauthorized);
        }

        // Update fields if provided
        if let Some(name) = name {
            if !name.is_empty() {
                site.name = name;
            }
        }

        if let Some(custom_domain) = custom_domain {
            site.custom_domain = Some(custom_domain);
        }

        if let Some(primary_color) = primary_color {
            site.primary_color = primary_color;
        }

        if let Some(secondary_color) = secondary_color {
            site.secondary_color = secondary_color;
        }

        if let Some(font_family) = font_family {
            site.font_family = font_family;
        }

        site.updated_at = chrono::Utc::now();

        let updated_site = self.site_repository.update_site(site).await?;
        Ok(updated_site)
    }

    /// Publishes a site to the p2p network
    #[instrument(skip(self))]
    pub async fn publish_site(
        &self,
        site_id: Uuid,
        owner_id: Uuid,
    ) -> Result<(), WebsiteBuilderError> {
        let site = self.site_repository.get_site_by_id(site_id).await?;

        // Check ownership
        if site.owner_id != owner_id {
            return Err(WebsiteBuilderError::Unauthorized);
        }

        // TODO: Generate static assets
        // TODO: Store via p2panda (use cpc-net)
        // TODO: Update site status with content address

        // For now, just mark as published
        self.site_repository.mark_site_as_published(site_id).await?;

        Ok(())
    }

    /// Gets a site for a specific owner
    #[instrument(skip(self))]
    pub async fn get_site_for_owner(
        &self,
        site_id: Uuid,
        owner_id: Uuid,
    ) -> Result<Site, WebsiteBuilderError> {
        let site = self.site_repository.get_site_by_id(site_id).await?;

        // Check ownership
        if site.owner_id != owner_id {
            return Err(WebsiteBuilderError::Unauthorized);
        }

        Ok(site)
    }

    /// Gets all sites for an owner
    #[instrument(skip(self))]
    pub async fn get_sites_for_owner(
        &self,
        owner_id: Uuid,
    ) -> Result<Vec<Site>, WebsiteBuilderError> {
        let sites = self.site_repository.get_sites_by_owner(owner_id).await?;
        Ok(sites)
    }
}