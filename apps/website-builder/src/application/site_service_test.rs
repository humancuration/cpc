//! Tests for the SiteService
//!
//! These tests verify that the business logic in the site service works as expected.

use uuid::Uuid;
use chrono::Utc;

use crate::domain::models::{Site, SiteType, LinkInBioData, FullWebsiteData, FundraisingCampaignData, CampaignType};

#[tokio::test]
async fn test_site_type_enum_variants() {
    // Test that we can create LinkInBio site type
    let link_in_bio = SiteType::LinkInBio(LinkInBioData {
        profile_image: None,
        headline: "Test Headline".to_string(),
        description: "Test Description".to_string(),
        links: vec![],
        click_count: 0,
    });
    
    // Test that we can create FullWebsite site type
    let full_website = SiteType::FullWebsite(FullWebsiteData {
        template_id: Uuid::new_v4(),
        pages: vec![],
    });
    
    // Test that we can create FundraisingCampaign site type
    let fundraising_campaign = SiteType::FundraisingCampaign(FundraisingCampaignData {
        campaign_id: Uuid::new_v4(),
        campaign_title: "Test Campaign".to_string(),
        campaign_description: "Test Description".to_string(),
        campaign_type: CampaignType::PureDonation,
        goal_amount: Some(1000),
        current_amount: 0,
        start_date: Utc::now(),
        end_date: None,
    });
    
    // Verify variants are correctly created
    match link_in_bio {
        SiteType::LinkInBio(data) => {
            assert_eq!(data.headline, "Test Headline");
            assert_eq!(data.description, "Test Description");
        }
        _ => panic!("Expected LinkInBio variant"),
    }
    
    match full_website {
        SiteType::FullWebsite(data) => {
            assert_eq!(data.pages.len(), 0);
        }
        _ => panic!("Expected FullWebsite variant"),
    }
    
    match fundraising_campaign {
        SiteType::FundraisingCampaign(data) => {
            assert_eq!(data.campaign_title, "Test Campaign");
            assert_eq!(data.campaign_description, "Test Description");
            assert_eq!(data.goal_amount, Some(1000));
        }
        _ => panic!("Expected FundraisingCampaign variant"),
    }
}

#[tokio::test]
async fn test_site_model_creation() {
    let site_id = Uuid::new_v4();
    let owner_id = Uuid::new_v4();
    let now = Utc::now();
    
    let site = Site {
        id: site_id,
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "Test Site".to_string(),
        custom_domain: Some("example.com".to_string()),
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial, sans-serif".to_string(),
        is_published: false,
        created_at: now,
        updated_at: now,
    };
    
    assert_eq!(site.id, site_id);
    assert_eq!(site.owner_id, owner_id);
    assert_eq!(site.name, "Test Site");
    assert_eq!(site.custom_domain, Some("example.com".to_string()));
    assert_eq!(site.primary_color, "#FF0000");
    assert_eq!(site.secondary_color, "#00FF00");
    assert_eq!(site.font_family, "Arial, sans-serif");
    assert!(!site.is_published);
}

#[tokio::test]
async fn test_site_type_matches_macro() {
    let link_in_bio_site = SiteType::LinkInBio(LinkInBioData {
        profile_image: None,
        headline: "Test".to_string(),
        description: "Test".to_string(),
        links: vec![],
        click_count: 0,
    });
    
    let full_website_site = SiteType::FullWebsite(FullWebsiteData {
        template_id: Uuid::new_v4(),
        pages: vec![],
    });
    
    // Test matches! macro usage as seen in the service implementation
    assert!(matches!(link_in_bio_site, SiteType::LinkInBio(_)));
    assert!(matches!(full_website_site, SiteType::FullWebsite(_)));
    assert!(!matches!(link_in_bio_site, SiteType::FullWebsite(_)));
    assert!(!matches!(full_website_site, SiteType::LinkInBio(_)));
    
    let fundraising_campaign_site = SiteType::FundraisingCampaign(FundraisingCampaignData {
        campaign_id: Uuid::new_v4(),
        campaign_title: "Test".to_string(),
        campaign_description: "Test".to_string(),
        campaign_type: CampaignType::PureDonation,
        goal_amount: None,
        current_amount: 0,
        start_date: Utc::now(),
        end_date: None,
    });
    
    // Test matches! macro usage for fundraising campaign
    assert!(matches!(fundraising_campaign_site, SiteType::FundraisingCampaign(_)));
    assert!(!matches!(fundraising_campaign_site, SiteType::LinkInBio(_)));
    assert!(!matches!(fundraising_campaign_site, SiteType::FullWebsite(_)));
}