//! Repository for accessing website builder data

use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use tracing::instrument;

use crate::domain::models::{
    Site, SiteType, FullWebsiteData, LinkInBioData, Page, LinkItem, MediaAsset,
    Template, TemplateType, TemplateStructure, AnalyticsReport
};
use crate::domain::errors::WebsiteBuilderError;

pub struct SiteRepository {
    db_pool: Arc<PgPool>,
}

impl SiteRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }

    /// Creates a new site
    #[instrument(skip(self))]
    pub async fn create_site(&self, site: Site) -> Result<Site, WebsiteBuilderError> {
        let site_type_str = match &site.site_type {
            SiteType::FullWebsite(_) => "full_website",
            SiteType::LinkInBio(_) => "link_in_bio",
        };

        // Handle fundraising campaign data
        let (campaign_id, campaign_title, campaign_description, campaign_type, goal_amount, current_amount, campaign_start_date, campaign_end_date) =
            if let SiteType::FundraisingCampaign(data) = &site.site_type {
                (
                    Some(data.campaign_id),
                    Some(data.campaign_title.clone()),
                    Some(data.campaign_description.clone()),
                    Some(match data.campaign_type {
                        crate::domain::models::CampaignType::CooperativeMembership => "cooperative_membership".to_string(),
                        crate::domain::models::CampaignType::PureDonation => "pure_donation".to_string(),
                        crate::domain::models::CampaignType::RegCF => "reg_cf".to_string(),
                        crate::domain::models::CampaignType::RegA => "reg_a".to_string(),
                        crate::domain::models::CampaignType::RegD => "reg_d".to_string(),
                    }),
                    data.goal_amount,
                    Some(data.current_amount as i64),
                    Some(data.start_date),
                    data.end_date,
                )
            } else {
                (None, None, None, None, None, None, None, None)
            };

        let created_site = sqlx::query_as!(
            Site,
            r#"
            INSERT INTO sites (
                id, owner_id, site_type, name, custom_domain,
                primary_color, secondary_color, font_family, is_published,
                created_at, updated_at,
                campaign_id, campaign_title, campaign_description, campaign_type,
                goal_amount, current_amount, campaign_start_date, campaign_end_date
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
            RETURNING *
            "#,
            site.id,
            site.owner_id,
            site_type_str,
            site.name,
            site.custom_domain,
            site.primary_color,
            site.secondary_color,
            site.font_family,
            site.is_published,
            site.created_at,
            site.updated_at,
            campaign_id,
            campaign_title,
            campaign_description,
            campaign_type,
            goal_amount.map(|v| v as i64),
            current_amount,
            campaign_start_date,
            campaign_end_date
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(created_site)
    }

    /// Gets a site by ID
    #[instrument(skip(self))]
    pub async fn get_site_by_id(&self, site_id: Uuid) -> Result<Site, WebsiteBuilderError> {
        let site_row = sqlx::query!(
            r#"
            SELECT id, owner_id, site_type, name, custom_domain,
                   primary_color, secondary_color, font_family, is_published,
                   created_at, updated_at,
                   campaign_id, campaign_title, campaign_description, campaign_type,
                   goal_amount, current_amount, campaign_start_date, campaign_end_date
            FROM sites
            WHERE id = $1
            "#,
            site_id
        )
        .fetch_optional(&*self.db_pool)
        .await?;

        let site_row = site_row.ok_or_else(|| WebsiteBuilderError::SiteNotFound(site_id.to_string()))?;

        // Convert to Site entity
        let site_type = match site_row.site_type.as_str() {
            "full_website" => SiteType::FullWebsite(FullWebsiteData {
                template_id: Uuid::nil(), // Will be populated later
                pages: vec![], // Will be populated later
            }),
            "link_in_bio" => SiteType::LinkInBio(LinkInBioData {
                profile_image: None, // Will be populated later
                headline: "".to_string(), // Will be populated later
                description: "".to_string(), // Will be populated later
                links: vec![], // Will be populated later
                click_count: 0,
            }),
            "fundraising_campaign" => {
                // Handle fundraising campaign
                let campaign_type = match site_row.campaign_type.as_deref() {
                    Some("cooperative_membership") => crate::domain::models::CampaignType::CooperativeMembership,
                    Some("pure_donation") => crate::domain::models::CampaignType::PureDonation,
                    Some("reg_cf") => crate::domain::models::CampaignType::RegCF,
                    Some("reg_a") => crate::domain::models::CampaignType::RegA,
                    Some("reg_d") => crate::domain::models::CampaignType::RegD,
                    _ => crate::domain::models::CampaignType::PureDonation, // Default
                };
                
                SiteType::FundraisingCampaign(FundraisingCampaignData {
                    campaign_id: site_row.campaign_id.unwrap_or_else(Uuid::nil),
                    campaign_title: site_row.campaign_title.unwrap_or_default(),
                    campaign_description: site_row.campaign_description.unwrap_or_default(),
                    campaign_type,
                    goal_amount: site_row.goal_amount.map(|v| v as u64),
                    current_amount: site_row.current_amount.unwrap_or(0) as u64,
                    start_date: site_row.campaign_start_date.unwrap_or_else(chrono::Utc::now),
                    end_date: site_row.campaign_end_date,
                })
            },
            _ => return Err(WebsiteBuilderError::InvalidSiteType(site_row.site_type)),
        };

        let site = Site {
            id: site_row.id,
            owner_id: site_row.owner_id,
            site_type,
            name: site_row.name,
            custom_domain: site_row.custom_domain,
            primary_color: site_row.primary_color,
            secondary_color: site_row.secondary_color,
            font_family: site_row.font_family,
            is_published: site_row.is_published,
            created_at: site_row.created_at,
            updated_at: site_row.updated_at,
        };

        Ok(site)
    }

    /// Updates a site
    #[instrument(skip(self))]
    pub async fn update_site(&self, site: Site) -> Result<Site, WebsiteBuilderError> {
        let site_type_str = match &site.site_type {
            SiteType::FullWebsite(_) => "full_website",
            SiteType::LinkInBio(_) => "link_in_bio",
            SiteType::FundraisingCampaign(_) => "fundraising_campaign",
        };

        // Handle fundraising campaign data
        let (campaign_id, campaign_title, campaign_description, campaign_type, goal_amount, current_amount, campaign_start_date, campaign_end_date) =
            if let SiteType::FundraisingCampaign(data) = &site.site_type {
                (
                    Some(data.campaign_id),
                    Some(data.campaign_title.clone()),
                    Some(data.campaign_description.clone()),
                    Some(match data.campaign_type {
                        crate::domain::models::CampaignType::CooperativeMembership => "cooperative_membership".to_string(),
                        crate::domain::models::CampaignType::PureDonation => "pure_donation".to_string(),
                        crate::domain::models::CampaignType::RegCF => "reg_cf".to_string(),
                        crate::domain::models::CampaignType::RegA => "reg_a".to_string(),
                        crate::domain::models::CampaignType::RegD => "reg_d".to_string(),
                    }),
                    data.goal_amount,
                    Some(data.current_amount as i64),
                    Some(data.start_date),
                    data.end_date,
                )
            } else {
                (None, None, None, None, None, None, None, None)
            };

        let updated_site = sqlx::query_as!(
            Site,
            r#"
            UPDATE sites
            SET owner_id = $2, site_type = $3, name = $4, custom_domain = $5,
                primary_color = $6, secondary_color = $7, font_family = $8,
                is_published = $9, updated_at = $10,
                campaign_id = $11, campaign_title = $12, campaign_description = $13, campaign_type = $14,
                goal_amount = $15, current_amount = $16, campaign_start_date = $17, campaign_end_date = $18
            WHERE id = $1
            RETURNING *
            "#,
            site.id,
            site.owner_id,
            site_type_str,
            site.name,
            site.custom_domain,
            site.primary_color,
            site.secondary_color,
            site.font_family,
            site.is_published,
            site.updated_at,
            campaign_id,
            campaign_title,
            campaign_description,
            campaign_type,
            goal_amount.map(|v| v as i64),
            current_amount,
            campaign_start_date,
            campaign_end_date
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(updated_site)
    }

    /// Marks a site as published
    #[instrument(skip(self))]
    pub async fn mark_site_as_published(&self, site_id: Uuid) -> Result<(), WebsiteBuilderError> {
        sqlx::query!(
            r#"
            UPDATE sites
            SET is_published = true, updated_at = NOW()
            WHERE id = $1
            "#,
            site_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    /// Gets all sites for an owner
    #[instrument(skip(self))]
    pub async fn get_sites_by_owner(&self, owner_id: Uuid) -> Result<Vec<Site>, WebsiteBuilderError> {
        let site_rows = sqlx::query!(
            r#"
            SELECT id, owner_id, site_type, name, custom_domain,
                   primary_color, secondary_color, font_family, is_published,
                   created_at, updated_at,
                   campaign_id, campaign_title, campaign_description, campaign_type,
                   goal_amount, current_amount, campaign_start_date, campaign_end_date
            FROM sites
            WHERE owner_id = $1
            ORDER BY created_at DESC
            "#,
            owner_id
        )
        .fetch_all(&*self.db_pool)
        .await?;

        let mut sites = Vec::new();
        for site_row in site_rows {
            // Convert to Site entity
            let site_type = match site_row.site_type.as_str() {
                "full_website" => SiteType::FullWebsite(FullWebsiteData {
                    template_id: Uuid::nil(), // Will be populated later
                    pages: vec![], // Will be populated later
                }),
                "link_in_bio" => SiteType::LinkInBio(LinkInBioData {
                    profile_image: None, // Will be populated later
                    headline: "".to_string(), // Will be populated later
                    description: "".to_string(), // Will be populated later
                    links: vec![], // Will be populated later
                    click_count: 0,
                }),
                "fundraising_campaign" => {
                    // Handle fundraising campaign
                    let campaign_type = match site_row.campaign_type.as_deref() {
                        Some("cooperative_membership") => crate::domain::models::CampaignType::CooperativeMembership,
                        Some("pure_donation") => crate::domain::models::CampaignType::PureDonation,
                        Some("reg_cf") => crate::domain::models::CampaignType::RegCF,
                        Some("reg_a") => crate::domain::models::CampaignType::RegA,
                        Some("reg_d") => crate::domain::models::CampaignType::RegD,
                        _ => crate::domain::models::CampaignType::PureDonation, // Default
                    };
                    
                    SiteType::FundraisingCampaign(FundraisingCampaignData {
                        campaign_id: site_row.campaign_id.unwrap_or_else(Uuid::nil),
                        campaign_title: site_row.campaign_title.unwrap_or_default(),
                        campaign_description: site_row.campaign_description.unwrap_or_default(),
                        campaign_type,
                        goal_amount: site_row.goal_amount.map(|v| v as u64),
                        current_amount: site_row.current_amount.unwrap_or(0) as u64,
                        start_date: site_row.campaign_start_date.unwrap_or_else(chrono::Utc::now),
                        end_date: site_row.campaign_end_date,
                    })
                },
                _ => return Err(WebsiteBuilderError::InvalidSiteType(site_row.site_type)),
            };

            let site = Site {
                id: site_row.id,
                owner_id: site_row.owner_id,
                site_type,
                name: site_row.name,
                custom_domain: site_row.custom_domain,
                primary_color: site_row.primary_color,
                secondary_color: site_row.secondary_color,
                font_family: site_row.font_family,
                is_published: site_row.is_published,
                created_at: site_row.created_at,
                updated_at: site_row.updated_at,
            };

            sites.push(site);
        }

        Ok(sites)
    }

    /// Checks if a user already has a link-in-bio site
    #[instrument(skip(self))]
    pub async fn has_link_in_bio_site(&self, owner_id: Uuid) -> Result<bool, WebsiteBuilderError> {
        let count = sqlx::query_scalar!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM sites
            WHERE owner_id = $1 AND site_type = 'link_in_bio'
            "#,
            owner_id
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(count > 0)
    }

    /// Gets all templates
    #[instrument(skip(self))]
    pub async fn get_all_templates(&self) -> Result<Vec<Template>, WebsiteBuilderError> {
        let template_rows = sqlx::query!(
            r#"
            SELECT id, name, description, template_type, preview_image_cid,
                   structure, is_default, created_at, updated_at
            FROM templates
            ORDER BY name
            "#
        )
        .fetch_all(&*self.db_pool)
        .await?;

        let mut templates = Vec::new();
        for row in template_rows {
            let template_type = match row.template_type.as_str() {
                "full_website" => TemplateType::FullWebsite,
                "link_in_bio" => TemplateType::LinkInBio,
                _ => return Err(WebsiteBuilderError::InvalidTemplateStructure(row.template_type)),
            };

            // Parse the structure JSON
            let structure: TemplateStructure = serde_json::from_value(row.structure)
                .map_err(|e| WebsiteBuilderError::InvalidTemplateStructure(e.to_string()))?;

            let template = Template {
                id: row.id,
                name: row.name,
                description: row.description,
                template_type,
                preview_image_cid: row.preview_image_cid,
                structure,
                is_default: row.is_default,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            templates.push(template);
        }

        Ok(templates)
    }

    /// Gets templates by type
    #[instrument(skip(self))]
    pub async fn get_templates_by_type(&self, template_type: TemplateType) -> Result<Vec<Template>, WebsiteBuilderError> {
        let template_type_str = match template_type {
            TemplateType::FullWebsite => "full_website",
            TemplateType::LinkInBio => "link_in_bio",
        };

        let template_rows = sqlx::query!(
            r#"
            SELECT id, name, description, template_type, preview_image_cid,
                   structure, is_default, created_at, updated_at
            FROM templates
            WHERE template_type = $1
            ORDER BY name
            "#,
            template_type_str
        )
        .fetch_all(&*self.db_pool)
        .await?;

        let mut templates = Vec::new();
        for row in template_rows {
            let template_type = match row.template_type.as_str() {
                "full_website" => TemplateType::FullWebsite,
                "link_in_bio" => TemplateType::LinkInBio,
                _ => return Err(WebsiteBuilderError::InvalidTemplateStructure(row.template_type)),
            };

            // Parse the structure JSON
            let structure: TemplateStructure = serde_json::from_value(row.structure)
                .map_err(|e| WebsiteBuilderError::InvalidTemplateStructure(e.to_string()))?;

            let template = Template {
                id: row.id,
                name: row.name,
                description: row.description,
                template_type,
                preview_image_cid: row.preview_image_cid,
                structure,
                is_default: row.is_default,
                created_at: row.created_at,
                updated_at: row.updated_at,
            };

            templates.push(template);
        }

        Ok(templates)
    }

    /// Gets a template by ID
    #[instrument(skip(self))]
    pub async fn get_template_by_id(&self, template_id: Uuid) -> Result<Template, WebsiteBuilderError> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, description, template_type, preview_image_cid,
                   structure, is_default, created_at, updated_at
            FROM templates
            WHERE id = $1
            "#,
            template_id
        )
        .fetch_optional(&*self.db_pool)
        .await?;

        let row = row.ok_or_else(|| WebsiteBuilderError::TemplateNotFound(template_id.to_string()))?;

        let template_type = match row.template_type.as_str() {
            "full_website" => TemplateType::FullWebsite,
            "link_in_bio" => TemplateType::LinkInBio,
            _ => return Err(WebsiteBuilderError::InvalidTemplateStructure(row.template_type)),
        };

        // Parse the structure JSON
        let structure: TemplateStructure = serde_json::from_value(row.structure)
            .map_err(|e| WebsiteBuilderError::InvalidTemplateStructure(e.to_string()))?;

        let template = Template {
            id: row.id,
            name: row.name,
            description: row.description,
            template_type,
            preview_image_cid: row.preview_image_cid,
            structure,
            is_default: row.is_default,
            created_at: row.created_at,
            updated_at: row.updated_at,
        };

        Ok(template)
    }

    /// Creates a template
    #[instrument(skip(self))]
    pub async fn create_template(&self, template: Template) -> Result<Template, WebsiteBuilderError> {
        let template_type_str = match template.template_type {
            TemplateType::FullWebsite => "full_website",
            TemplateType::LinkInBio => "link_in_bio",
        };

        // Convert structure to JSON
        let structure_json = serde_json::to_value(&template.structure)
            .map_err(|e| WebsiteBuilderError::InvalidTemplateStructure(e.to_string()))?;

        let created_template = sqlx::query_as!(
            Template,
            r#"
            INSERT INTO templates (
                id, name, description, template_type, preview_image_cid,
                structure, is_default, created_at, updated_at
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING *
            "#,
            template.id,
            template.name,
            template.description,
            template_type_str,
            template.preview_image_cid,
            structure_json,
            template.is_default,
            template.created_at,
            template.updated_at
        )
        .fetch_one(&*self.db_pool)
        .await?;

        Ok(created_template)
    }

    /// Applies a template to a site
    #[instrument(skip(self))]
    pub async fn apply_template_to_site(&self, site_id: Uuid, template_id: Uuid) -> Result<(), WebsiteBuilderError> {
        // For now, we'll just update the site with the template ID
        // In a more complete implementation, we would also update the site's content
        // based on the template structure
        sqlx::query!(
            r#"
            UPDATE sites
            SET updated_at = NOW()
            WHERE id = $1
            "#,
            site_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    /// Increments link click count
    #[instrument(skip(self))]
    pub async fn increment_link_click_count(&self, link_id: Uuid) -> Result<(), WebsiteBuilderError> {
        sqlx::query!(
            r#"
            UPDATE link_items
            SET click_count = click_count + 1
            WHERE id = $1
            "#,
            link_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    /// Increments site click count
    #[instrument(skip(self))]
    pub async fn increment_site_click_count(&self, link_id: Uuid) -> Result<(), WebsiteBuilderError> {
        sqlx::query!(
            r#"
            UPDATE sites
            SET click_count = COALESCE(click_count, 0) + 1
            WHERE id = (
                SELECT site_id
                FROM link_items
                WHERE id = $1
            )
            "#,
            link_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    /// Increments page view count
    #[instrument(skip(self))]
    pub async fn increment_page_view_count(&self, page_id: Uuid) -> Result<(), WebsiteBuilderError> {
        sqlx::query!(
            r#"
            UPDATE pages
            SET view_count = COALESCE(view_count, 0) + 1
            WHERE id = $1
            "#,
            page_id
        )
        .execute(&*self.db_pool)
        .await?;

        Ok(())
    }

    /// Gets analytics report
    #[instrument(skip(self))]
    pub async fn get_analytics_report(
        &self,
        site_id: Uuid,
        period_start: chrono::DateTime<chrono::Utc>,
        period_end: chrono::DateTime<chrono::Utc>,
    ) -> Result<AnalyticsReport, WebsiteBuilderError> {
        // Get total views for the site
        let total_views: i64 = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(total_views), 0) as "sum!"
            FROM site_analytics
            WHERE site_id = $1 AND date >= $2 AND date <= $3
            "#,
            site_id,
            period_start.date_naive(),
            period_end.date_naive()
        )
        .fetch_one(&*self.db_pool)
        .await?;

        // Get link clicks
        let link_clicks = sqlx::query!(
            r#"
            SELECT id, click_count
            FROM link_items
            WHERE site_id = $1
            "#,
            site_id
        )
        .fetch_all(&*self.db_pool)
        .await?;

        let mut link_click_map = std::collections::HashMap::new();
        for row in link_clicks {
            link_click_map.insert(row.id, row.click_count as u64);
        }

        // Get page views
        let page_views = sqlx::query!(
            r#"
            SELECT id, view_count
            FROM pages
            WHERE site_id = $1
            "#,
            site_id
        )
        .fetch_all(&*self.db_pool)
        .await?;

        let mut page_view_map = std::collections::HashMap::new();
        for row in page_views {
            page_view_map.insert(row.id, row.view_count.unwrap_or(0) as u64);
        }

        let report = AnalyticsReport {
            site_id,
            period_start,
            period_end,
            total_views: total_views as u64,
            unique_visitors: 0, // Would need additional tracking to implement this
            link_clicks: link_click_map,
            page_views: page_view_map,
        };

        Ok(report)
    }
}