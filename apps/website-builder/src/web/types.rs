//! GraphQL input/output types for the website builder module

use async_graphql::{InputObject, SimpleObject};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

// Input types

#[derive(InputObject)]
pub struct CreateSiteInput {
    pub name: String,
    pub site_type: SiteTypeInput,
}

#[derive(InputObject)]
pub struct UpdateSiteSettingsInput {
    pub site_id: Uuid,
    pub name: Option<String>,
    pub custom_domain: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub font_family: Option<String>,
}

#[derive(InputObject)]
pub struct UpdateSiteContentInput {
    pub site_id: Uuid,
    pub content: SiteContentInput,
}

#[derive(InputObject)]
pub struct SiteContentInput {
    pub pages: Option<Vec<PageInput>>,
    pub links: Option<Vec<LinkItemInput>>,
    pub headline: Option<String>,
    pub description: Option<String>,
}

#[derive(InputObject)]
pub struct PageInput {
    pub id: Option<Uuid>,
    pub title: String,
    pub slug: String,
    pub content: PageContentInput,
    pub is_published: bool,
}

#[derive(InputObject)]
pub struct PageContentInput {
    pub sections: Vec<ContentSectionInput>,
}

#[derive(InputObject)]
pub struct ContentSectionInput {
    pub id: Option<Uuid>,
    pub section_type: SectionTypeInput,
    pub content: SectionContentInput,
}

#[derive(InputObject)]
pub struct SectionContentInput {
    pub text_content: Option<String>,
    pub image_asset: Option<MediaAssetInput>,
    pub video_asset: Option<MediaAssetInput>,
    pub gallery_assets: Option<Vec<MediaAssetInput>>,
    pub form_fields: Option<Vec<FormFieldInput>>,
}

#[derive(InputObject)]
pub struct FormFieldInput {
    pub id: Option<Uuid>,
    pub field_type: FormFieldTypeInput,
    pub label: String,
    pub required: bool,
    pub placeholder: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(InputObject)]
pub struct LinkItemInput {
    pub id: Option<Uuid>,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub position: u8,
}

#[derive(InputObject)]
pub struct MediaAssetInput {
    pub cid: String,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
}

#[derive(InputObject)]
pub struct CreateCustomTemplateInput {
    pub name: String,
    pub template_type: TemplateTypeInput,
    pub structure: TemplateStructureInput,
}

#[derive(InputObject)]
pub struct TemplateStructureInput {
    pub layout: TemplateLayoutInput,
    pub default_content: String, // JSON string
}

#[derive(InputObject)]
pub struct TemplateLayoutInput {
    pub sections: Vec<TemplateSectionInput>,
}

#[derive(InputObject)]
pub struct TemplateSectionInput {
    pub id: String,
    pub section_type: SectionTypeInput,
    pub label: String,
    pub is_editable: bool,
    pub is_required: bool,
}

#[derive(InputObject)]
pub struct AnalyticsPeriodInput {
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

// Enum input types

#[derive(InputObject)]
pub struct SiteTypeInput {
    pub full_website: Option<FullWebsiteDataInput>,
    pub link_in_bio: Option<LinkInBioDataInput>,
}

#[derive(InputObject)]
pub struct FullWebsiteDataInput {
    pub template_id: Uuid,
}

#[derive(InputObject)]
pub struct LinkInBioDataInput {
    pub headline: String,
    pub description: String,
}

// Fundraising campaign input is now handled by CreateFundraisingCampaignInput in graphql.rs

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SectionTypeInput {
    Text,
    Image,
    Video,
    Gallery,
    Form,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum FormFieldTypeInput {
    Text,
    Email,
    Number,
    TextArea,
    Select,
    Checkbox,
    Radio,
    Date,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum TemplateTypeInput {
    FullWebsite,
    LinkInBio,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum CampaignTypeInput {
    CooperativeMembership,
    PureDonation,
    RegCF,
    RegA,
    RegD,
}

// Output types

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SiteOutput {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub site_type: SiteTypeOutput,
    pub name: String,
    pub custom_domain: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: String,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SiteTypeOutput {
    pub full_website: Option<FullWebsiteDataOutput>,
    pub link_in_bio: Option<LinkInBioDataOutput>,
    pub fundraising_campaign: Option<FundraisingCampaignDataOutput>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct FullWebsiteDataOutput {
    pub template_id: Uuid,
    pub pages: Vec<PageOutput>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct LinkInBioDataOutput {
    pub profile_image: Option<MediaAssetOutput>,
    pub headline: String,
    pub description: String,
    pub links: Vec<LinkItemOutput>,
    pub click_count: u64,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct FundraisingCampaignDataOutput {
    pub campaign_id: Uuid,
    pub campaign_title: String,
    pub campaign_description: String,
    pub campaign_type: CampaignTypeOutput,
    pub goal_amount: Option<u64>,
    pub current_amount: u64,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct PageOutput {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: PageContentOutput,
    pub is_published: bool,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct PageContentOutput {
    pub sections: Vec<ContentSectionOutput>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct ContentSectionOutput {
    pub id: Uuid,
    pub section_type: SectionTypeOutput,
    pub content: SectionContentOutput,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum SectionTypeOutput {
    Text,
    Image,
    Video,
    Gallery,
    Form,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct SectionContentOutput {
    pub text_content: Option<String>,
    pub image_asset: Option<MediaAssetOutput>,
    pub video_asset: Option<MediaAssetOutput>,
    pub gallery_assets: Option<Vec<MediaAssetOutput>>,
    pub form_fields: Option<Vec<FormFieldOutput>>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct FormFieldOutput {
    pub id: Uuid,
    pub field_type: FormFieldTypeOutput,
    pub label: String,
    pub required: bool,
    pub placeholder: Option<String>,
    pub options: Option<Vec<String>>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum FormFieldTypeOutput {
    Text,
    Email,
    Number,
    TextArea,
    Select,
    Checkbox,
    Radio,
    Date,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct LinkItemOutput {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub position: u8,
    pub click_count: u64,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct MediaAssetOutput {
    pub cid: String,
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct TemplateOutput {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: TemplateTypeOutput,
    pub preview_image_cid: String,
    pub structure: TemplateStructureOutput,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum TemplateTypeOutput {
    FullWebsite,
    LinkInBio,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum CampaignTypeOutput {
    CooperativeMembership,
    PureDonation,
    RegCF,
    RegA,
    RegD,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct TemplateStructureOutput {
    pub layout: TemplateLayoutOutput,
    pub default_content: String, // JSON string
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct TemplateLayoutOutput {
    pub sections: Vec<TemplateSectionOutput>,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct TemplateSectionOutput {
    pub id: String,
    pub section_type: SectionTypeOutput,
    pub label: String,
    pub is_editable: bool,
    pub is_required: bool,
}

#[derive(SimpleObject, Serialize, Deserialize)]
pub struct AnalyticsReportOutput {
    pub site_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_views: u64,
    pub unique_visitors: u64,
    pub link_clicks: HashMap<Uuid, u64>,
    pub page_views: HashMap<Uuid, u64>,
}

#[derive(SimpleObject)]
pub struct SitePublishedEvent {
    pub site_id: Uuid,
    pub content_address: String,
}

#[derive(SimpleObject)]
pub struct LinkClickedEvent {
    pub link_id: Uuid,
    pub site_id: Uuid,
}