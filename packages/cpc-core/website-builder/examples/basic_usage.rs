//! Basic usage example for the website builder module

use cpc_website_builder::domain::models::{Site, SiteType, LinkInBioData};
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

    Ok(())
}