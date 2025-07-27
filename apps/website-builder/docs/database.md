# Website Builder Module Database Schema

This document describes the database schema used by the website builder module.

## Overview

The module uses the following tables to store its data:

- `sites` - Main site entities
- `pages` - Pages for full websites
- `link_items` - Links for link-in-bio sites
- `templates` - Site templates
- `site_analytics` - Analytics data

## Tables

### sites

Stores the main site entities, which can be either full websites or link-in-bio sites.

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
    click_count BIGINT NOT NULL DEFAULT 0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Columns:
- `id` - Unique identifier for the site
- `owner_id` - Reference to the cooperative member who owns the site
- `site_type` - Type of site ('full_website' or 'link_in_bio')
- `name` - Name of the site
- `custom_domain` - Custom domain for the site (optional)
- `primary_color` - Primary color for the site's theme
- `secondary_color` - Secondary color for the site's theme
- `font_family` - Font family for the site's text
- `is_published` - Whether the site is published
- `click_count` - Total click count for link-in-bio sites
- `created_at` - Timestamp when the site was created
- `updated_at` - Timestamp when the site was last updated

### pages

Stores pages for full websites.

```sql
CREATE TABLE pages (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    slug VARCHAR(100) NOT NULL,
    content JSONB NOT NULL,
    is_published BOOLEAN NOT NULL DEFAULT false,
    view_count BIGINT,
    position INTEGER NOT NULL DEFAULT 0
);
```

Columns:
- `id` - Unique identifier for the page
- `site_id` - Reference to the site this page belongs to
- `title` - Title of the page
- `slug` - URL slug for the page
- `content` - JSONB content of the page
- `is_published` - Whether the page is published
- `view_count` - Number of views for the page
- `position` - Position of the page in the site's navigation

### link_items

Stores links for link-in-bio sites.

```sql
CREATE TABLE link_items (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    url VARCHAR(2048) NOT NULL,
    icon VARCHAR(50),
    position SMALLINT NOT NULL,
    click_count BIGINT NOT NULL DEFAULT 0
);
```

Columns:
- `id` - Unique identifier for the link
- `site_id` - Reference to the site this link belongs to
- `title` - Title of the link
- `url` - URL the link points to
- `icon` - Icon for the link (optional)
- `position` - Position of the link in the site's layout
- `click_count` - Number of clicks on the link

### templates

Stores site templates.

```sql
CREATE TABLE templates (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    template_type VARCHAR(20) NOT NULL, -- 'full_website' or 'link_in_bio'
    preview_image_cid VARCHAR(100) NOT NULL,
    structure JSONB NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
```

Columns:
- `id` - Unique identifier for the template
- `name` - Name of the template
- `description` - Description of the template
- `template_type` - Type of template ('full_website' or 'link_in_bio')
- `preview_image_cid` - Content ID of the preview image
- `structure` - JSONB structure of the template
- `is_default` - Whether this is a default template
- `created_at` - Timestamp when the template was created
- `updated_at` - Timestamp when the template was last updated

### site_analytics

Stores analytics data for sites.

```sql
CREATE TABLE site_analytics (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    total_views BIGINT NOT NULL DEFAULT 0,
    UNIQUE(site_id, date)
);
```

Columns:
- `id` - Unique identifier for the analytics record
- `site_id` - Reference to the site this record belongs to
- `date` - Date of the analytics data
- `total_views` - Total views for the site on this date

## Indexes

The following indexes are created for performance:

```sql
CREATE INDEX idx_sites_owner_id ON sites(owner_id);
CREATE INDEX idx_sites_site_type ON sites(site_type);
CREATE INDEX idx_pages_site_id ON pages(site_id);
CREATE INDEX idx_pages_slug ON pages(slug);
CREATE INDEX idx_link_items_site_id ON link_items(site_id);
CREATE INDEX idx_link_items_position ON link_items(position);
CREATE INDEX idx_templates_template_type ON templates(template_type);
CREATE INDEX idx_site_analytics_site_id ON site_analytics(site_id);
CREATE INDEX idx_site_analytics_date ON site_analytics(date);
```

## Relationships

- Each site belongs to one cooperative member (owner_id references cooperative_members.id)
- Each page belongs to one site (site_id references sites.id)
- Each link item belongs to one site (site_id references sites.id)
- Each analytics record belongs to one site (site_id references sites.id)
- When a site is deleted, all its pages and link items are also deleted (CASCADE)

## JSONB Columns

The `pages.content` and `templates.structure` columns use the JSONB data type to store flexible, structured data. This allows for complex page content and template structures while still maintaining the benefits of a relational database.

## Migration

The database schema is created by running the migration file `apps/backend/migrations/20250726000000_create_website_builder_tables.sql`.