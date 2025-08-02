//! GraphQL implementation for the website builder module

use async_graphql::{
    Context, Enum, InputObject, Object, Result, SimpleObject, Subscription,
};
use futures_util::stream::Stream;
use std::collections::HashMap;
use uuid::Uuid;

use crate::application::site_service::SiteService;
use crate::application::template_service::TemplateService;
use crate::application::analytics_service::AnalyticsService;
use crate::domain::errors::WebsiteBuilderError;
use crate::web::types::*;
use crate::domain::models::{CampaignType, FundraisingCampaignData};

// GraphQL Query Root
#[derive(Default)]
pub struct WebsiteBuilderQuery;

#[Object]
impl WebsiteBuilderQuery {
    async fn site(&self, ctx: &Context<'_>, id: Uuid) -> Result<SiteOutput> {
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        let site = site_service.get_site_for_owner(id, *user_id).await?;
        
        // Convert domain model to GraphQL output type
        let site_output = convert_site_to_output(site);
        Ok(site_output)
    }

    async fn sites_by_owner(&self, ctx: &Context<'_>, owner_id: Uuid) -> Result<Vec<SiteOutput>> {
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        // Check if user is requesting their own sites or has admin privileges
        if *user_id != owner_id {
            return Err(async_graphql::Error::new("Unauthorized"));
        }

        let sites = site_service.get_sites_for_owner(owner_id).await?;
        
        // Convert domain models to GraphQL output types
        let sites_output: Vec<SiteOutput> = sites.into_iter().map(convert_site_to_output).collect();
        Ok(sites_output)
    }

    async fn templates(&self, ctx: &Context<'_>) -> Result<Vec<TemplateOutput>> {
        let template_service = ctx.data_unchecked::<TemplateService>();
        
        let templates = template_service.get_available_templates().await?;
        
        // Convert domain models to GraphQL output types
        let templates_output: Vec<TemplateOutput> = templates.into_iter().map(convert_template_to_output).collect();
        Ok(templates_output)
    }

    async fn site_analytics(
        &self,
        ctx: &Context<'_>,
        site_id: Uuid,
        period: AnalyticsPeriodInput,
    ) -> Result<AnalyticsReportOutput> {
        let analytics_service = ctx.data_unchecked::<AnalyticsService>();
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        // Check if user owns the site
        let site = site_service.get_site_for_owner(site_id, *user_id).await?;
        
        let report = analytics_service
            .get_analytics_data(site_id, period.start_date, period.end_date)
            .await?;
        
        // Convert domain model to GraphQL output type
        let report_output = convert_analytics_report_to_output(report);
        Ok(report_output)
    }
}

// GraphQL Mutation Root
#[derive(Default)]
pub struct WebsiteBuilderMutation;

#[Object]
impl WebsiteBuilderMutation {
    async fn create_site(&self, ctx: &Context<'_>, input: CreateSiteInput) -> Result<SiteOutput> {
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        // Convert input to domain types
        let site_type = convert_site_type_input_to_domain(input.site_type);

        let site = site_service
            .create_site(*user_id, site_type, input.name)
            .await?;
        
        // Convert domain model to GraphQL output type
        let site_output = convert_site_to_output(site);
        Ok(site_output)
    }
#[derive(InputObject)]
pub struct CreateFundraisingCampaignInput {
    pub site_name: String,
    pub title: String,
    pub description: String,
    pub campaign_type: CampaignTypeInput,
    pub goal_amount: i64,
    pub currency: String,
    pub start_date: String,
    pub end_date: Option<String>,
}

    async fn update_site_settings(
        &self,
        ctx: &Context<'_>,
        input: UpdateSiteSettingsInput,
    ) -> Result<SiteOutput> {
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        let site = site_service
            .update_site_settings(
                input.site_id,
                *user_id,
                input.name,
                input.custom_domain,
                input.primary_color,
                input.secondary_color,
                input.font_family,
            )
            .await?;
        
        // Convert domain model to GraphQL output type
        let site_output = convert_site_to_output(site);
        Ok(site_output)
    }

    async fn update_site_content(
        &self,
        ctx: &Context<'_>,
        input: UpdateSiteContentInput,
    ) -> Result<SiteOutput> {
        // TODO: Implement content updates
        Err(async_graphql::Error::new("Not implemented"))
    }

    async fn publish_site(&self, ctx: &Context<'_>, site_id: Uuid) -> Result<bool> {
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>(); // Get from auth middleware

        site_service.publish_site(site_id, *user_id).await?;
        Ok(true)
    }

    async fn track_link_click(&self, ctx: &Context<'_>, link_id: Uuid) -> Result<bool> {
        let analytics_service = ctx.data_unchecked::<AnalyticsService>();
        
        analytics_service.track_link_click(link_id).await?;
        Ok(true)
    }
    
    async fn create_fundraising_campaign(&self, ctx: &Context<'_>, input: CreateFundraisingCampaignInput) -> Result<SiteOutput> {
        let site_service = ctx.data_unchecked::<SiteService>();
        let user_id = ctx.data_unchecked::<Uuid>();

        let campaign_data = FundraisingCampaignData {
            campaign_id: Uuid::nil(), // To be set by service
            campaign_title: input.title,
            campaign_description: input.description,
            campaign_type: convert_campaign_type_input_to_domain(input.campaign_type),
            goal_amount: input.goal_amount,
            current_amount: 0,
            start_date: input.start_date,
            end_date: input.end_date,
        };

        let site = site_service
            .create_fundraising_campaign(*user_id, campaign_data, input.site_name)
            .await?;
        
        Ok(convert_site_to_output(site))
    }
}

// GraphQL Subscription Root
#[derive(Default)]
pub struct WebsiteBuilderSubscription;

#[Subscription]
impl WebsiteBuilderSubscription {
    async fn site_published(
        &self,
        ctx: &Context<'_>,
        site_id: Uuid,
    ) -> impl Stream<Item = SitePublishedEvent> {
        // TODO: Implement real-time publishing updates
        // This would use a pub/sub system like Redis or a broadcast channel
        futures_util::stream::empty()
    }

    async fn link_clicked(
        &self,
        ctx: &Context<'_>,
        site_id: Uuid,
    ) -> impl Stream<Item = LinkClickedEvent> {
        // TODO: Implement real-time analytics
        // This would use a pub/sub system like Redis or a broadcast channel
        futures_util::stream::empty()
    }
}

// Conversion functions

fn convert_site_to_output(site: crate::domain::models::Site) -> SiteOutput {
    SiteOutput {
        id: site.id,
        owner_id: site.owner_id,
        site_type: convert_site_type_to_output(site.site_type),
        name: site.name,
        custom_domain: site.custom_domain,
        primary_color: site.primary_color,
        secondary_color: site.secondary_color,
        font_family: site.font_family,
        is_published: site.is_published,
        created_at: site.created_at,
        updated_at: site.updated_at,
    }
}

fn convert_site_type_to_output(site_type: crate::domain::models::SiteType) -> SiteTypeOutput {
    match site_type {
        crate::domain::models::SiteType::FullWebsite(data) => SiteTypeOutput {
            full_website: Some(FullWebsiteDataOutput {
                template_id: data.template_id,
                pages: data.pages.into_iter().map(convert_page_to_output).collect(),
            }),
            link_in_bio: None,
        },
        crate::domain::models::SiteType::LinkInBio(data) => SiteTypeOutput {
            full_website: None,
            link_in_bio: Some(LinkInBioDataOutput {
                profile_image: data.profile_image.map(convert_media_asset_to_output),
                headline: data.headline,
                description: data.description,
                links: data.links.into_iter().map(convert_link_item_to_output).collect(),
                click_count: data.click_count,
            }),
        },
        crate::domain::models::SiteType::FundraisingCampaign(data) => SiteTypeOutput {
            full_website: None,
            link_in_bio: None,
            fundraising_campaign: Some(FundraisingCampaignDataOutput {
                campaign_id: data.campaign_id,
                campaign_title: data.campaign_title,
                campaign_description: data.campaign_description,
                campaign_type: convert_campaign_type_to_output(data.campaign_type),
                goal_amount: data.goal_amount,
                current_amount: data.current_amount,
                start_date: data.start_date,
                end_date: data.end_date,
            }),
        },
    }
}

fn convert_page_to_output(page: crate::domain::models::Page) -> PageOutput {
    PageOutput {
        id: page.id,
        title: page.title,
        slug: page.slug,
        content: convert_page_content_to_output(page.content),
        is_published: page.is_published,
    }
}

fn convert_page_content_to_output(
    content: crate::domain::models::PageContent,
) -> PageContentOutput {
    PageContentOutput {
        sections: content
            .sections
            .into_iter()
            .map(convert_content_section_to_output)
            .collect(),
    }
}

fn convert_content_section_to_output(
    section: crate::domain::models::ContentSection,
) -> ContentSectionOutput {
    ContentSectionOutput {
        id: section.id,
        section_type: convert_section_type_to_output(section.section_type),
        content: convert_section_content_to_output(section.content),
    }
}

fn convert_section_type_to_output(
    section_type: crate::domain::models::SectionType,
) -> SectionTypeOutput {
    match section_type {
        crate::domain::models::SectionType::Text => SectionTypeOutput::Text,
        crate::domain::models::SectionType::Image => SectionTypeOutput::Image,
        crate::domain::models::SectionType::Video => SectionTypeOutput::Video,
        crate::domain::models::SectionType::Gallery => SectionTypeOutput::Gallery,
        crate::domain::models::SectionType::Form => SectionTypeOutput::Form,
    }
}

fn convert_section_content_to_output(
    content: crate::domain::models::SectionContent,
) -> SectionContentOutput {
    match content {
        crate::domain::models::SectionContent::Text { content } => SectionContentOutput {
            text_content: Some(content),
            image_asset: None,
            video_asset: None,
            gallery_assets: None,
            form_fields: None,
        },
        crate::domain::models::SectionContent::Image { asset, alt_text } => {
            SectionContentOutput {
                text_content: None,
                image_asset: Some(convert_media_asset_to_output(asset)),
                video_asset: None,
                gallery_assets: None,
                form_fields: None,
            }
        }
        crate::domain::models::SectionContent::Video { asset, thumbnail } => {
            SectionContentOutput {
                text_content: None,
                image_asset: None,
                video_asset: Some(convert_media_asset_to_output(asset)),
                gallery_assets: None,
                form_fields: None,
            }
        }
        crate::domain::models::SectionContent::Gallery { assets } => SectionContentOutput {
            text_content: None,
            image_asset: None,
            video_asset: None,
            gallery_assets: Some(
                assets
                    .into_iter()
                    .map(convert_media_asset_to_output)
                    .collect(),
            ),
            form_fields: None,
        },
        crate::domain::models::SectionContent::Form { fields } => SectionContentOutput {
            text_content: None,
            image_asset: None,
            video_asset: None,
            gallery_assets: None,
            form_fields: Some(
                fields
                    .into_iter()
                    .map(convert_form_field_to_output)
                    .collect(),
            ),
        },
    }
}

fn convert_media_asset_to_output(asset: crate::domain::models::MediaAsset) -> MediaAssetOutput {
    MediaAssetOutput {
        cid: asset.cid,
        filename: asset.filename,
        mime_type: asset.mime_type,
        size_bytes: asset.size_bytes,
    }
}

fn convert_link_item_to_output(item: crate::domain::models::LinkItem) -> LinkItemOutput {
    LinkItemOutput {
        id: item.id,
        title: item.title,
        url: item.url,
        icon: item.icon,
        position: item.position,
        click_count: item.click_count,
    }
}

fn convert_form_field_to_output(field: crate::domain::models::FormField) -> FormFieldOutput {
    FormFieldOutput {
        id: field.id,
        field_type: convert_form_field_type_to_output(field.field_type),
        label: field.label,
        required: field.required,
        placeholder: field.placeholder,
        options: field.options,
    }
}

fn convert_form_field_type_to_output(
    field_type: crate::domain::models::FormFieldType,
) -> FormFieldTypeOutput {
    match field_type {
        crate::domain::models::FormFieldType::Text => FormFieldTypeOutput::Text,
        crate::domain::models::FormFieldType::Email => FormFieldTypeOutput::Email,
        crate::domain::models::FormFieldType::Number => FormFieldTypeOutput::Number,
        crate::domain::models::FormFieldType::TextArea => FormFieldTypeOutput::TextArea,
        crate::domain::models::FormFieldType::Select => FormFieldTypeOutput::Select,
        crate::domain::models::FormFieldType::Checkbox => FormFieldTypeOutput::Checkbox,
        crate::domain::models::FormFieldType::Radio => FormFieldTypeOutput::Radio,
        crate::domain::models::FormFieldType::Date => FormFieldTypeOutput::Date,
    }
}

fn convert_template_to_output(template: crate::domain::models::Template) -> TemplateOutput {
    TemplateOutput {
        id: template.id,
        name: template.name,
        description: template.description,
        template_type: convert_template_type_to_output(template.template_type),
        preview_image_cid: template.preview_image_cid,
        structure: convert_template_structure_to_output(template.structure),
        is_default: template.is_default,
        created_at: template.created_at,
        updated_at: template.updated_at,
    }
}

fn convert_template_type_to_output(
    template_type: crate::domain::models::TemplateType,
) -> TemplateTypeOutput {
    match template_type {
        crate::domain::models::TemplateType::FullWebsite => TemplateTypeOutput::FullWebsite,
        crate::domain::models::TemplateType::LinkInBio => TemplateTypeOutput::LinkInBio,
    }
}

fn convert_template_structure_to_output(
    structure: crate::domain::models::TemplateStructure,
) -> TemplateStructureOutput {
    TemplateStructureOutput {
        layout: convert_template_layout_to_output(structure.layout),
        default_content: serde_json::to_string(&structure.default_content)
            .unwrap_or_else(|_| "{}".to_string()),
    }
}

fn convert_template_layout_to_output(
    layout: crate::domain::models::TemplateLayout,
) -> TemplateLayoutOutput {
    TemplateLayoutOutput {
        sections: layout
            .sections
            .into_iter()
            .map(convert_template_section_to_output)
            .collect(),
    }
}

fn convert_template_section_to_output(
    section: crate::domain::models::TemplateSection,
) -> TemplateSectionOutput {
    TemplateSectionOutput {
        id: section.id,
        section_type: convert_section_type_to_output(section.section_type),
        label: section.label,
        is_editable: section.is_editable,
        is_required: section.is_required,
    }
}

fn convert_analytics_report_to_output(
    report: crate::domain::models::AnalyticsReport,
) -> AnalyticsReportOutput {
    AnalyticsReportOutput {
        site_id: report.site_id,
        period_start: report.period_start,
        period_end: report.period_end,
        total_views: report.total_views,
        unique_visitors: report.unique_visitors,
        link_clicks: report.link_clicks,
        page_views: report.page_views,
    }
}

fn convert_site_type_input_to_domain(site_type: SiteTypeInput) -> crate::domain::models::SiteType {
    if let Some(full_website) = site_type.full_website {
        crate::domain::models::SiteType::FullWebsite(crate::domain::models::FullWebsiteData {
            template_id: full_website.template_id,
            pages: vec![], // Will be populated later
        })
    } else if let Some(link_in_bio) = site_type.link_in_bio {
        crate::domain::models::SiteType::LinkInBio(crate::domain::models::LinkInBioData {
            profile_image: None, // Will be populated later
            headline: link_in_bio.headline,
            description: link_in_bio.description,
            links: vec![], // Will be populated later
            click_count: 0,
        })
    } else {
        // Default to link-in-bio if neither variant is specified
        crate::domain::models::SiteType::LinkInBio(crate::domain::models::LinkInBioData {
            profile_image: None,
            headline: "".to_string(),
            description: "".to_string(),
            links: vec![],
            click_count: 0,
        })
    }
}

fn convert_campaign_type_input_to_domain(campaign_type: CampaignTypeInput) -> crate::domain::models::CampaignType {
    match campaign_type {
        CampaignTypeInput::CooperativeMembership => crate::domain::models::CampaignType::CooperativeMembership,
        CampaignTypeInput::PureDonation => crate::domain::models::CampaignType::PureDonation,
        CampaignTypeInput::RegCF => crate::domain::models::CampaignType::RegCF,
        CampaignTypeInput::RegA => crate::domain::models::CampaignType::RegA,
        CampaignTypeInput::RegD => crate::domain::models::CampaignType::RegD,
    }
}

fn convert_campaign_type_to_output(campaign_type: crate::domain::models::CampaignType) -> CampaignTypeOutput {
    match campaign_type {
        crate::domain::models::CampaignType::CooperativeMembership => CampaignTypeOutput::CooperativeMembership,
        crate::domain::models::CampaignType::PureDonation => CampaignTypeOutput::PureDonation,
        crate::domain::models::CampaignType::RegCF => CampaignTypeOutput::RegCF,
        crate::domain::models::CampaignType::RegA => CampaignTypeOutput::RegA,
        crate::domain::models::CampaignType::RegD => CampaignTypeOutput::RegD,
    }
}