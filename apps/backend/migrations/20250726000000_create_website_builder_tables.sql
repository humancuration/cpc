-- Create tables for the website builder module

-- Sites table
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

-- Pages table
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

-- Link items table
CREATE TABLE link_items (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    title VARCHAR(100) NOT NULL,
    url VARCHAR(2048) NOT NULL,
    icon VARCHAR(50),
    position SMALLINT NOT NULL,
    click_count BIGINT NOT NULL DEFAULT 0
);

-- Templates table
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

-- Site analytics table
CREATE TABLE site_analytics (
    id UUID PRIMARY KEY,
    site_id UUID NOT NULL REFERENCES sites(id) ON DELETE CASCADE,
    date DATE NOT NULL,
    total_views BIGINT NOT NULL DEFAULT 0,
    UNIQUE(site_id, date)
);

-- Indexes for performance
CREATE INDEX idx_sites_owner_id ON sites(owner_id);
CREATE INDEX idx_sites_site_type ON sites(site_type);
CREATE INDEX idx_pages_site_id ON pages(site_id);
CREATE INDEX idx_pages_slug ON pages(slug);
CREATE INDEX idx_link_items_site_id ON link_items(site_id);
CREATE INDEX idx_link_items_position ON link_items(position);
CREATE INDEX idx_templates_template_type ON templates(template_type);
CREATE INDEX idx_site_analytics_site_id ON site_analytics(site_id);
CREATE INDEX idx_site_analytics_date ON site_analytics(date);