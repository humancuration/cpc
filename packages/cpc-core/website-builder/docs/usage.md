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

## Testing

To run the tests for this module, use:

```bash
cargo test -p cpc-website-builder
```

## Examples

See the `examples/` directory for complete usage examples.