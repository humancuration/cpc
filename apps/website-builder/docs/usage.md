# Website Builder Module Usage Guide

This guide explains how to use the website builder module in your application.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
cpc-website-builder = { path = "../apps/website-builder" }
```

## Basic Usage

### Creating a Site

```rust
use cpc_website_builder::domain::models::{Site, SiteType, LinkInBioData};
use cpc_website_builder::domain::errors::WebsiteBuilderError;
use uuid::Uuid;

fn create_site() -> Result<(), WebsiteBuilderError> {
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

    // Use the site...
    Ok(())
}
```

### Creating a Fundraising Campaign Site

```rust
use cpc_website_builder::domain::models::{Site, SiteType, FundraisingCampaignData, CampaignType};
use cpc_website_builder::domain::errors::WebsiteBuilderError;
use uuid::Uuid;

fn create_fundraising_campaign_site() -> Result<(), WebsiteBuilderError> {
    let site = Site {
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

    // Use the site...
    Ok(())
}
```

### Using Services

To use the application services, you'll need to set up the infrastructure components first:

```rust
use cpc_website_builder::application::site_service::SiteService;
use cpc_website_builder::infrastructure::repository::SiteRepository;
use std::sync::Arc;

// Initialize the repository
let repository = Arc::new(SiteRepository::new(db_pool));

// Initialize the service
let site_service = SiteService::new(repository);
```

## GraphQL API

The module exposes a GraphQL API that can be integrated with your main GraphQL schema. See the `web/graphql.rs` file for the full schema definition.

## Web Routes

The module also provides REST API routes that can be mounted in your Axum application:

```rust
use cpc_website_builder::web::routes::create_website_builder_router;

let router = create_website_builder_router(site_service, template_service, analytics_service);
```

## Database Migrations

The module requires the following database tables to be created:

- `sites`
- `pages`
- `link_items`
- `templates`
- `site_analytics`

Run the migration file `apps/backend/migrations/20250726000000_create_website_builder_tables.sql` to create these tables.

Run the migration file `apps/website-builder/migrations/20250803000000_add_campaign_fields.sql` to add the fundraising campaign fields.

## Testing

To run the tests for this module, use:

```bash
cargo test -p cpc-website-builder
```

## Examples

See the `examples/` directory for complete usage examples.