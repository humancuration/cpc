//! Integration tests for the SiteRepository
//!
//! These tests require a PostgreSQL database to run.
//! Set the TEST_DATABASE_URL environment variable to point to a test database.

use std::sync::Arc;
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

use crate::infrastructure::repository::SiteRepository;
use crate::domain::models::{
    Site, SiteType, FullWebsiteData, LinkInBioData, FundraisingCampaignData, CampaignType, Template, TemplateType, TemplateStructure, TemplateLayout, TemplateSection
};
use crate::domain::value_objects::ColorHex;

/// Setup a test database connection
async fn setup_test_db() -> PgPool {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .unwrap_or_else(|_| "postgres://localhost/cpc_test".to_string());
    
    PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to test database")
}

/// Cleanup test data
async fn cleanup_test_data(pool: &PgPool) {
    // Clean up in reverse order of dependencies
    sqlx::query!("DELETE FROM site_analytics WHERE site_id IN (SELECT id FROM sites WHERE name LIKE 'test_%')")
        .execute(pool)
        .await
        .expect("Failed to cleanup site_analytics");
        
    sqlx::query!("DELETE FROM link_items WHERE site_id IN (SELECT id FROM sites WHERE name LIKE 'test_%')")
        .execute(pool)
        .await
        .expect("Failed to cleanup link_items");
        
    sqlx::query!("DELETE FROM pages WHERE site_id IN (SELECT id FROM sites WHERE name LIKE 'test_%')")
        .execute(pool)
        .await
        .expect("Failed to cleanup pages");
        
    sqlx::query!("DELETE FROM sites WHERE name LIKE 'test_%'")
        .execute(pool)
        .await
        .expect("Failed to cleanup sites");
        
    sqlx::query!("DELETE FROM templates WHERE name LIKE 'test_%'")
        .execute(pool)
        .await
        .expect("Failed to cleanup templates");
}

#[tokio::test]
async fn test_create_and_get_site() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site".to_string(),
        custom_domain: Some("test.example.com".to_string()),
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Test create
    let created_site = repository.create_site(site.clone()).await.unwrap();
    assert_eq!(created_site.id, site.id);
    assert_eq!(created_site.owner_id, site.owner_id);
    assert_eq!(created_site.name, site.name);
    
    // Test get by ID
    let retrieved_site = repository.get_site_by_id(site.id).await.unwrap();
    assert_eq!(retrieved_site.id, site.id);
    assert_eq!(retrieved_site.owner_id, site.owner_id);
    assert_eq!(retrieved_site.name, site.name);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_update_site() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_update".to_string(),
        custom_domain: Some("test.example.com".to_string()),
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_site = repository.create_site(site.clone()).await.unwrap();
    
    // Update the site
    let updated_site = Site {
        name: "updated_test_site".to_string(),
        custom_domain: Some("updated.example.com".to_string()),
        primary_color: "#0000FF".to_string(),
        secondary_color: "#FFFF00".to_string(),
        font_family: "Times New Roman".to_string(),
        ..created_site
    };
    
    let result = repository.update_site(updated_site.clone()).await.unwrap();
    assert_eq!(result.name, "updated_test_site");
    assert_eq!(result.custom_domain, Some("updated.example.com".to_string()));
    assert_eq!(result.primary_color, "#0000FF");
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_sites_by_owner() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    let owner_id = Uuid::new_v4();
    
    // Create multiple sites for the same owner
    let site1 = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline 1".to_string(),
            description: "Test Description 1".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_1".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let site2 = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline 2".to_string(),
            description: "Test Description 2".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_2".to_string(),
        custom_domain: None,
        primary_color: "#0000FF".to_string(),
        secondary_color: "#FFFF00".to_string(),
        font_family: "Times New Roman".to_string(),
        is_published: true,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    repository.create_site(site1).await.unwrap();
    repository.create_site(site2).await.unwrap();
    
    // Get sites by owner
    let sites = repository.get_sites_by_owner(owner_id).await.unwrap();
    assert_eq!(sites.len(), 2);
    
    // Verify both sites are returned
    let site_names: Vec<&String> = sites.iter().map(|s| &s.name).collect();
    assert!(site_names.contains(&&"test_site_1".to_string()));
    assert!(site_names.contains(&&"test_site_2".to_string()));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_has_link_in_bio_site() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    let owner_id = Uuid::new_v4();
    
    // Initially, user should not have a link-in-bio site
    let has_site = repository.has_link_in_bio_site(owner_id).await.unwrap();
    assert!(!has_site);
    
    // Create a link-in-bio site
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_link_in_bio".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    repository.create_site(site).await.unwrap();
    
    // Now user should have a link-in-bio site
    let has_site = repository.has_link_in_bio_site(owner_id).await.unwrap();
    assert!(has_site);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_get_template() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test template
    let template = Template {
        id: Uuid::new_v4(),
        name: "test_template".to_string(),
        description: Some("Test template description".to_string()),
        template_type: TemplateType::LinkInBio,
        preview_image_cid: "test_cid".to_string(),
        structure: TemplateStructure {
            layout: TemplateLayout {
                sections: vec![
                    TemplateSection {
                        id: "header".to_string(),
                        section_type: crate::domain::models::SectionType::Text,
                        label: "Header".to_string(),
                        is_editable: true,
                        is_required: true,
                    }
                ]
            },
            default_content: std::collections::HashMap::new(),
        },
        is_default: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Test create template
    let created_template = repository.create_template(template.clone()).await.unwrap();
    assert_eq!(created_template.id, template.id);
    assert_eq!(created_template.name, template.name);
    
    // Test get template by ID
    let retrieved_template = repository.get_template_by_id(template.id).await.unwrap();
    assert_eq!(retrieved_template.id, template.id);
    assert_eq!(retrieved_template.name, template.name);
    
    // Test get all templates
    let all_templates = repository.get_all_templates().await.unwrap();
    assert!(!all_templates.is_empty());
    assert!(all_templates.iter().any(|t| t.id == template.id));
    
    // Test get templates by type
    let link_in_bio_templates = repository.get_templates_by_type(TemplateType::LinkInBio).await.unwrap();
    assert!(!link_in_bio_templates.is_empty());
    assert!(link_in_bio_templates.iter().any(|t| t.id == template.id));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_mark_site_as_published() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_publish".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_site = repository.create_site(site).await.unwrap();
    assert!(!created_site.is_published);
    
    // Mark site as published
    repository.mark_site_as_published(created_site.id).await.unwrap();
    
    // Verify site is now published
    let updated_site = repository.get_site_by_id(created_site.id).await.unwrap();
    assert!(updated_site.is_published);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_apply_template_to_site() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_template".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_site = repository.create_site(site).await.unwrap();
    
    // Create a test template
    let template = Template {
        id: Uuid::new_v4(),
        name: "test_template_apply".to_string(),
        description: Some("Test template for applying".to_string()),
        template_type: TemplateType::LinkInBio,
        preview_image_cid: "test_cid".to_string(),
        structure: TemplateStructure {
            layout: TemplateLayout {
                sections: vec![]
            },
            default_content: std::collections::HashMap::new(),
        },
        is_default: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_template = repository.create_template(template).await.unwrap();
    
    // Apply template to site
    let result = repository.apply_template_to_site(created_site.id, created_template.id).await;
    assert!(result.is_ok());
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_increment_link_click_count() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_click".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_site = repository.create_site(site).await.unwrap();
    
    // Create a link item
    let link_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO link_items (id, site_id, title, url, position, click_count)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        link_id,
        created_site.id,
        "Test Link",
        "https://example.com",
        1i16,
        0i64
    )
    .execute(&*pool)
    .await
    .unwrap();
    
    // Increment click count
    repository.increment_link_click_count(link_id).await.unwrap();
    
    // Verify click count was incremented
    let updated_link = sqlx::query!(
        r#"
        SELECT click_count
        FROM link_items
        WHERE id = $1
        "#,
        link_id
    )
    .fetch_one(&*pool)
    .await
    .unwrap();
    
    assert_eq!(updated_link.click_count, 1);
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_increment_page_view_count() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::FullWebsite(FullWebsiteData {
            template_id: Uuid::nil(),
            pages: vec![],
        }),
        name: "test_site_page_view".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_site = repository.create_site(site).await.unwrap();
    
    // Create a page
    let page_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO pages (id, site_id, title, slug, content, position)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        page_id,
        created_site.id,
        "Test Page",
        "test-page",
        serde_json::json!({"sections": []}),
        1i32
    )
    .execute(&*pool)
    .await
    .unwrap();
    
    // Increment page view count
    repository.increment_page_view_count(page_id).await.unwrap();
    
    // Verify view count was incremented
    let updated_page = sqlx::query!(
        r#"
        SELECT view_count
        FROM pages
        WHERE id = $1
        "#,
        page_id
    )
    .fetch_one(&*pool)
    .await
    .unwrap();
    
    assert_eq!(updated_page.view_count, Some(1));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_get_analytics_report() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test site
    let owner_id = Uuid::new_v4();
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::LinkInBio(LinkInBioData {
            profile_image: None,
            headline: "Test Headline".to_string(),
            description: "Test Description".to_string(),
            links: vec![],
            click_count: 0,
        }),
        name: "test_site_analytics".to_string(),
        custom_domain: None,
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    let created_site = repository.create_site(site).await.unwrap();
    
    // Create some analytics data
    let today = Utc::now().date_naive();
    sqlx::query!(
        r#"
        INSERT INTO site_analytics (id, site_id, date, total_views)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        created_site.id,
        today,
        100i64
    )
    .execute(&*pool)
    .await
    .unwrap();
    
    // Create a link item with clicks
    let link_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO link_items (id, site_id, title, url, position, click_count)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#,
        link_id,
        created_site.id,
        "Test Link",
        "https://example.com",
        1i16,
        25i64
    )
    .execute(&*pool)
    .await
    .unwrap();
    
    // Create a page with views
    let page_id = Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO pages (id, site_id, title, slug, content, view_count, position)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
        page_id,
        created_site.id,
        "Test Page",
        "test-page",
        serde_json::json!({"sections": []}),
        50i64,
        1i32
    )
    .execute(&*pool)
    .await
    .unwrap();
    
    // Get analytics report
    let start_date = Utc::now() - chrono::Duration::days(1);
    let end_date = Utc::now() + chrono::Duration::days(1);
    let report = repository.get_analytics_report(created_site.id, start_date, end_date).await.unwrap();
    
    assert_eq!(report.site_id, created_site.id);
    assert_eq!(report.total_views, 100);
    assert_eq!(report.link_clicks.get(&link_id), Some(&25));
    assert_eq!(report.page_views.get(&page_id), Some(&50));
    
    cleanup_test_data(&pool).await;
}

#[tokio::test]
async fn test_create_and_get_fundraising_campaign_site() {
    let pool = setup_test_db().await;
    let repository = SiteRepository::new(Arc::new(pool.clone()));
    
    // Create a test fundraising campaign site
    let owner_id = Uuid::new_v4();
    let campaign_id = Uuid::new_v4();
    let start_date = Utc::now();
    let end_date = start_date + chrono::Duration::days(30);
    
    let site = Site {
        id: Uuid::new_v4(),
        owner_id,
        site_type: SiteType::FundraisingCampaign(FundraisingCampaignData {
            campaign_id,
            campaign_title: "Test Fundraising Campaign".to_string(),
            campaign_description: "Test Description".to_string(),
            campaign_type: CampaignType::PureDonation,
            goal_amount: Some(10000),
            current_amount: 2500,
            start_date,
            end_date: Some(end_date),
        }),
        name: "test_fundraising_campaign".to_string(),
        custom_domain: Some("campaign.example.com".to_string()),
        primary_color: "#FF0000".to_string(),
        secondary_color: "#00FF00".to_string(),
        font_family: "Arial".to_string(),
        is_published: false,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    // Test create
    let created_site = repository.create_site(site.clone()).await.unwrap();
    assert_eq!(created_site.id, site.id);
    assert_eq!(created_site.owner_id, site.owner_id);
    assert_eq!(created_site.name, site.name);
    
    // Test get by ID
    let retrieved_site = repository.get_site_by_id(site.id).await.unwrap();
    assert_eq!(retrieved_site.id, site.id);
    assert_eq!(retrieved_site.owner_id, site.owner_id);
    assert_eq!(retrieved_site.name, site.name);
    
    // Verify the site type is correctly retrieved
    match retrieved_site.site_type {
        SiteType::FundraisingCampaign(data) => {
            assert_eq!(data.campaign_id, campaign_id);
            assert_eq!(data.campaign_title, "Test Fundraising Campaign");
            assert_eq!(data.campaign_description, "Test Description");
            assert_eq!(data.campaign_type, CampaignType::PureDonation);
            assert_eq!(data.goal_amount, Some(10000));
            assert_eq!(data.current_amount, 2500);
            assert_eq!(data.start_date, start_date);
            assert_eq!(data.end_date, Some(end_date));
        }
        _ => panic!("Expected FundraisingCampaign site type"),
    }
    
    cleanup_test_data(&pool).await;
}