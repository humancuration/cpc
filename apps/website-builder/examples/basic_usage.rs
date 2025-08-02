//! Basic usage example for the website builder module

use cpc_website_builder::domain::models::{Site, SiteType, LinkInBioData, FundraisingCampaignData, CampaignType};
use cpc_website_builder::domain::errors::WebsiteBuilderError;
use uuid::Uuid;

fn main() -> Result<(), WebsiteBuilderError> {
    // Create a new link-in-bio site
    let site = Site {
        id: Uuid::new_v4(),
        owner_id: Uuid::new_v4(),
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "My Social Links".to_string(),
            description: "Check out my social media profiles".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "My Social Links".to_string(),
        custom_domain: None,
        primary_color: "#000000".to_string(),
        secondary_color: "#FFFFFF".to_string(),
        font_family: "Arial, sans-serif".to_string(),
        is_published: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    println!("Created site: {}", site.name);
    println!("Site type: {:?}", site.site_type);

    // Create a new fundraising campaign site
    let campaign_site = Site {
        id: Uuid::new_v4(),
        owner_id: Uuid::new_v4(),
        site_type: SiteType::FundraisingCampaign(FundraisingCampaignData {
            campaign_id: Uuid::new_v4(),
            campaign_title: "Community Garden Project".to_string(),
            campaign_description: "Help us build a community garden for local families".to_string(),
            campaign_type: CampaignType::PureDonation,
            goal_amount: Some(50000), // $500 goal
            current_amount: 0,
            start_date: chrono::Utc::now(),
            end_date: Some(chrono::Utc::now() + chrono::Duration::days(60)), // 60-day campaign
        }),
        name: "Community Garden Fundraiser".to_string(),
        custom_domain: None,
        primary_color: "#228B22".to_string(), // Forest green
        secondary_color: "#32CD32".to_string(), // Lime green
        font_family: "Georgia, serif".to_string(),
        is_published: false,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    println!("Created campaign site: {}", campaign_site.name);
    println!("Campaign title: {:?}",
        match &campaign_site.site_type {
            SiteType::FundraisingCampaign(data) => &data.campaign_title,
            _ => "Not a campaign site",
        }
    );

    Ok(())
}