//! Domain models for the website builder module

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// The central entity representing both full websites and link-in-bio sites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: Uuid,
    pub owner_id: Uuid, // References cooperative member
    pub site_type: SiteType,
    pub name: String,
    pub custom_domain: Option<String>,
    pub primary_color: String,
    pub secondary_color: String,
    pub font_family: String,
    pub is_published: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Discriminator for the two site variants
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SiteType {
    FullWebsite(FullWebsiteData),
    LinkInBio(LinkInBioData),
    FundraisingCampaign(FundraisingCampaignData),
}

/// Data specific to full websites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullWebsiteData {
    pub template_id: Uuid,
    pub pages: Vec<Page>,
}

/// Data specific to link-in-bio sites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkInBioData {
    pub profile_image: Option<MediaAsset>,
    pub headline: String,
    pub description: String,
    pub links: Vec<LinkItem>,
    pub click_count: u64,
}

/// Page content for full websites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: PageContent,
    pub is_published: bool,
}

/// Content structure for pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageContent {
    pub sections: Vec<ContentSection>,
}

/// A section within a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentSection {
    pub id: Uuid,
    pub section_type: SectionType,
    pub content: SectionContent,
}

/// Types of sections that can be added to a page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionType {
    Text,
    Image,
    Video,
    Gallery,
    Form,
}

/// Content for different section types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SectionContent {
    Text { content: String },
    Image { asset: MediaAsset, alt_text: Option<String> },
    Video { asset: MediaAsset, thumbnail: Option<MediaAsset> },
    Gallery { assets: Vec<MediaAsset> },
    Form { fields: Vec<FormField> },
}

/// A form field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormField {
    pub id: Uuid,
    pub field_type: FormFieldType,
    pub label: String,
    pub required: bool,
    pub placeholder: Option<String>,
    pub options: Option<Vec<String>>, // For select, checkbox, radio fields
}

/// Types of form fields
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FormFieldType {
    Text,
    Email,
    Number,
    TextArea,
    Select,
    Checkbox,
    Radio,
    Date,
}

/// Link item for link-in-bio sites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkItem {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub icon: Option<String>, // Emoji or SVG path
    pub position: u8,
    pub click_count: u64,
}

/// Media asset stored via p2panda
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAsset {
    pub cid: String, // Content ID for p2p storage
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
}

/// Template for sites
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub template_type: TemplateType,
    pub preview_image_cid: String,
    pub structure: TemplateStructure,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Types of templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TemplateType {
    FullWebsite,
    LinkInBio,
}

/// Structure of a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateStructure {
    pub layout: TemplateLayout,
    pub default_content: HashMap<String, serde_json::Value>,
}

/// Layout definition for templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateLayout {
    pub sections: Vec<TemplateSection>,
}

/// A section in a template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateSection {
    pub id: String,
    pub section_type: SectionType,
    pub label: String,
    pub is_editable: bool,
    pub is_required: bool,
}

/// Analytics data for a site
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyticsReport {
    pub site_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_views: u64,
    pub unique_visitors: u64,
    pub link_clicks: HashMap<Uuid, u64>, // link_id -> click_count
    pub page_views: HashMap<Uuid, u64>, // page_id -> view_count
}