//! Template service for managing website templates

use std::sync::Arc;
use uuid::Uuid;
use tracing::instrument;

use crate::domain::models::{Template, TemplateType};
use crate::domain::errors::WebsiteBuilderError;
use crate::infrastructure::repository::SiteRepository;

pub struct TemplateService {
    site_repository: Arc<SiteRepository>,
}

impl TemplateService {
    pub fn new(site_repository: Arc<SiteRepository>) -> Self {
        Self { site_repository }
    }

    /// Gets all available templates
    #[instrument(skip(self))]
    pub async fn get_available_templates(&self) -> Result<Vec<Template>, WebsiteBuilderError> {
        let templates = self.site_repository.get_all_templates().await?;
        Ok(templates)
    }

    /// Gets templates by type
    #[instrument(skip(self))]
    pub async fn get_templates_by_type(
        &self,
        template_type: TemplateType,
    ) -> Result<Vec<Template>, WebsiteBuilderError> {
        let templates = self.site_repository.get_templates_by_type(template_type).await?;
        Ok(templates)
    }

    /// Gets a template by ID
    #[instrument(skip(self))]
    pub async fn get_template_by_id(&self, template_id: Uuid) -> Result<Template, WebsiteBuilderError> {
        let template = self.site_repository.get_template_by_id(template_id).await?;
        Ok(template)
    }

    /// Applies a template to a site
    #[instrument(skip(self))]
    pub async fn apply_template(
        &self,
        site_id: Uuid,
        template_id: Uuid,
        owner_id: Uuid,
    ) -> Result<(), WebsiteBuilderError> {
        // Get the site and template
        let site = self.site_repository.get_site_by_id(site_id).await?;
        let template = self.site_repository.get_template_by_id(template_id).await?;

        // Check ownership
        if site.owner_id != owner_id {
            return Err(WebsiteBuilderError::Unauthorized);
        }

        // Check if template is compatible with site type
        match (&site.site_type, &template.template_type) {
            (crate::domain::models::SiteType::FullWebsite(_), TemplateType::FullWebsite) => {}
            (crate::domain::models::SiteType::LinkInBio(_), TemplateType::LinkInBio) => {}
            _ => return Err(WebsiteBuilderError::TemplateIncompatible),
        }

        // Apply the template to the site
        self.site_repository.apply_template_to_site(site_id, template_id).await?;

        Ok(())
    }

    /// Creates a custom template
    #[instrument(skip(self))]
    pub async fn create_custom_template(
        &self,
        owner_id: Uuid,
        name: String,
        template_type: TemplateType,
        structure: crate::domain::models::TemplateStructure,
    ) -> Result<Template, WebsiteBuilderError> {
        // Create the template entity
        let template = Template {
            id: Uuid::new_v4(),
            name,
            description: None,
            template_type,
            preview_image_cid: "".to_string(), // Will be set later
            structure,
            is_default: false,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        // Save the template
        let saved_template = self.site_repository.create_template(template).await?;

        Ok(saved_template)
    }
}