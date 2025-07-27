# Website Builder Module Architecture

This document outlines the architecture for the website-builder module, implementing both full website building and link-in-bio functionality as requested in planned_apps.md (lines 27-28). The design follows our hexagonal architecture principles as documented in planned_apps.md (lines 134-315).

## 1. Directory Structure

```
apps/website-builder/
├── Cargo.toml
└── src/
    ├── lib.rs                  # Main crate entry, exports the module
    ├── domain/                 # Core business models
    │   ├── models.rs           # Primary entities
    │   ├── value_objects.rs    # Domain-specific types
    │   └── errors.rs           # Custom error types
    ├── application/            # Business logic services
    │   ├── site_service.rs     # Core operations for sites
    │   ├── template_service.rs # Template management
    │   └── analytics_service.rs # Click tracking & analytics
    ├── infrastructure/         # External implementations
    │   ├── repository.rs       # Database access layer
    │   ├── p2p_store.rs        # p2panda integration for distributed storage
    │   └── media_processor.rs  # Image/video processing
    └── web/                    # Adapter layer
        ├── routes.rs           # Axum routes
        ├── graphql.rs          # GraphQL definitions
        ├── module.rs           # Module initialization & wiring
        └── types.rs            # GraphQL input/output types
```

## 2. Core Domain Models

### Primary Entities

```rust
// domain/models.rs

/// The central entity representing both full websites and link-in-bio sites
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
pub enum SiteType {
    FullWebsite(FullWebsiteData),
    LinkInBio(LinkInBioData),
}

/// Data specific to full websites
pub struct FullWebsiteData {
    pub template_id: Uuid,
    pub pages: Vec<Page>,
}

/// Data specific to link-in-bio sites
pub struct LinkInBioData {
    pub profile_image: Option<MediaAsset>,
    pub headline: String,
    pub description: String,
    pub links: Vec<LinkItem>,
    pub click_count: u64,
}

/// Page content for full websites
pub struct Page {
    pub id: Uuid,
    pub title: String,
    pub slug: String,
    pub content: PageContent,
    pub is_published: bool,
}

/// Link item for link-in-bio sites
pub struct LinkItem {
    pub id: Uuid,
    pub title: String,
    pub url: String,
    pub icon: Option<String>, // Emoji or SVG path
    pub position: u8,
    pub click_count: u64,
}

/// Media asset stored via p2panda
pub struct MediaAsset {
    pub cid: String, // Content ID for p2p storage
    pub filename: String,
    pub mime_type: String,
    pub size_bytes: u64,
}
```

### Value Objects

```rust
// domain/value_objects.rs

/// Valid color format with validation
pub struct ColorHex(String);

/// Valid URL with validation
pub struct ValidUrl(String);

/// Template identifier with validation
pub struct TemplateId(Uuid);
```

## 3. Key Application Services

### SiteService
- `create_site(owner_id, site_type) -> Result<Site>`: Creates new site with appropriate variant
- `update_site_settings(site_id, settings) -> Result<Site>`: Updates core site properties
- `publish_site(site_id) -> Result<()>`: Publishes site to p2p network
- `get_site_for_owner(site_id, owner_id) -> Result<Site>`: Access control enforced

### TemplateService
- `get_available_templates() -> Vec<Template>`: Returns templates for both site types
- `apply_template(site_id, template_id) -> Result<Site>`: Applies template to site
- `create_custom_template(owner_id, template_data) -> Result<Template>`

### AnalyticsService
- `track_link_click(link_id) -> Result<()>`: Increments click counter
- `get_analytics_data(site_id, period) -> AnalyticsReport`: Aggregates statistics

## 4. GraphQL API Integration

The module integrates with our existing GraphQL API by exposing:

### Queries
- `site(id: ID!): Site` - Get site details
- `sitesByOwner(ownerId: ID!): [Site!]!` - List all sites for a cooperative member
- `templates: [Template!]!` - List available templates
- `siteAnalytics(siteId: ID!, period: AnalyticsPeriod): AnalyticsReport` - Get analytics

### Mutations
- `createSite(input: CreateSiteInput!): Site!`
- `updateSiteSettings(input: UpdateSiteSettingsInput!): Site!`
- `updateSiteContent(input: UpdateSiteContentInput!): Site!`
- `publishSite(siteId: ID!): Boolean!`
- `trackLinkClick(linkId: ID!): Boolean!`

### Subscriptions
- `sitePublished(siteId: ID!): SitePublishedEvent` - For real-time publishing updates
- `linkClicked(siteId: ID!): LinkClickedEvent` - For real-time analytics

The implementation follows the same wiring pattern as the invoicing module (planned_apps.md lines 188-211), with a `WebsiteBuilderModule` struct that exposes:
```rust
pub struct WebsiteBuilderModule {
    pub router: Router,
    pub query: WebsiteBuilderQuery,
    pub mutation: WebsiteBuilderMutation,
    pub subscription: WebsiteBuilderSubscription,
}
```

## 5. Unified System for Both Site Types

The architecture handles both functionality variants through:

1. **Polymorphic Site Entity**: The `SiteType` enum allows storing both variants in the same table with type-specific data
2. **Shared Core Properties**: Common properties (colors, fonts, domain) apply to both variants
3. **Contextual Services**: Application services check the site type and apply appropriate logic
4. **Template System**: Templates define whether they're for full websites or link-in-bio
5. **Unified Publishing**: Both types use the same publishing workflow to p2p storage

Example workflow for link-in-bio creation:
1. User selects "Link-in-Bio" site type
2. System creates Site with LinkInBio variant
3. Default template with single-column layout is applied
4. User adds links through the same content update mutation used for pages
5. Analytics service tracks clicks on published site

## 6. Dependencies on Other Modules

| Dependency | Purpose |
|------------|---------|
| `cpc-core` | Access to cooperative member models and authentication |
| `cpc-net` | p2panda integration for distributed site storage |
| `cpc-protos` | Shared gRPC definitions for worker communication |
| `sqlx` | Database access (already in backend dependencies) |
| `tracing` | Structured logging |

The module will NOT depend on frontend-specific code (Yew, Tauri) as it's a backend service. The frontend will interact via the GraphQL API.

## 7. Database Schema Considerations

### Primary Tables

```sql
CREATE TABLE sites (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES cooperative_members(id),
    site_type VARCHAR(20) NOT NULL, -- 'full_website' or 'link_in_bio'
    name VARCHAR(100) NOT NULL,
    custom_domain VARCHAR(255),
    primary_color VARCHAR(7) NOT NULL DEFAULT '#000000',
    secondary_color VARCHAR(7) NOT NULL DEFAULT '#FFFFFF',
    font_family VARCHAR(50) NOT NULL DEFAULT 'Arial, sans-serif',
    is_published BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE pages (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id),
    title VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    content JSONB NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT false,
    position INTEGER NOT NULL
);

CREATE TABLE link_items (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id),
    title VARCHAR(100) NOT NULL,
    url VARCHAR(2048) NOT NULL,
    icon VARCHAR(50),
    position TINYINT NOT NULL,
    click_count BIGINT NOT NULL DEFAULT 0
);

CREATE TABLE templates (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    template_type VARCHAR(20) NOT NULL, -- 'full_website' or 'link_in_bio'
    preview_image_cid VARCHAR(100) NOT NULL,
    structure JSONB NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false
);

CREATE TABLE site_analytics (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id),
    date DATE NOT NULL,
    total_views BIGINT NOT NULL DEFAULT 0,
    UNIQUE(site_id, date)
);
```

### Key Implementation Notes

1. **Mobile-First Design**: All templates include responsive configurations by default
2. **p2p Storage**: Published sites are stored via p2panda with content addressing
3. **Authentication**: All operations validate cooperative membership through `cpc-core`
4. **Analytics**: Click tracking uses atomic increments to handle high concurrency
5. **Template System**: Templates are stored as JSON structures that define both layout and content constraints

This architecture provides a clean separation of concerns while supporting both website variants through a unified domain model. The hexagonal structure ensures the core business logic remains independent of implementation details, allowing for flexible evolution of the system.