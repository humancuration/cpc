-- +goose Up
-- SQL in section 'Up' is executed when this migration is applied

-- Create contacts table
CREATE TABLE crm_contacts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    contact_type VARCHAR(20) NOT NULL CHECK (contact_type IN ('platform_native', 'external')),
    platform_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    name VARCHAR(255) NOT NULL,
    primary_email VARCHAR(255),
    primary_phone VARCHAR(50),
    company VARCHAR(255),
    tags JSONB,
    consent_settings JSONB,
    external_data JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    last_interaction TIMESTAMP WITH TIME ZONE
);

-- Create indexes for contacts
CREATE INDEX idx_crm_contacts_user_id ON crm_contacts(user_id);
CREATE INDEX idx_crm_contacts_platform_user_id ON crm_contacts(platform_user_id);
CREATE INDEX idx_crm_contacts_email ON crm_contacts(primary_email);
CREATE INDEX idx_crm_contacts_company ON crm_contacts(company);

-- Create interactions table
CREATE TABLE crm_interactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL REFERENCES crm_contacts(id) ON DELETE CASCADE,
    interaction_type VARCHAR(20) NOT NULL CHECK (interaction_type IN ('call', 'email', 'meeting', 'message', 'platform_event')),
    platform_event_id UUID,
    summary TEXT NOT NULL,
    details TEXT,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    duration_seconds INTEGER,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_platform_native BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for interactions
CREATE INDEX idx_crm_interactions_contact_id ON crm_interactions(contact_id);
CREATE INDEX idx_crm_interactions_timestamp ON crm_interactions(timestamp);
CREATE INDEX idx_crm_interactions_created_by ON crm_interactions(created_by);

-- Create pipelines table
CREATE TABLE crm_pipelines (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_shared BOOLEAN NOT NULL DEFAULT false,
    shared_with JSONB,
    custom_fields JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for pipelines
CREATE INDEX idx_crm_pipelines_owner_id ON crm_pipelines(owner_id);

-- Create pipeline stages table
CREATE TABLE crm_pipeline_stages (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pipeline_id UUID NOT NULL REFERENCES crm_pipelines(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    probability INTEGER NOT NULL CHECK (probability >= 0 AND probability <= 100),
    estimated_value_cents BIGINT,
    estimated_value_currency VARCHAR(3),
    position INTEGER NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for pipeline stages
CREATE INDEX idx_crm_pipeline_stages_pipeline_id ON crm_pipeline_stages(pipeline_id);
CREATE INDEX idx_crm_pipeline_stages_position ON crm_pipeline_stages(position);

-- Create deals table
CREATE TABLE crm_deals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    contact_id UUID NOT NULL REFERENCES crm_contacts(id) ON DELETE CASCADE,
    pipeline_id UUID NOT NULL REFERENCES crm_pipelines(id) ON DELETE CASCADE,
    current_stage_id UUID NOT NULL REFERENCES crm_pipeline_stages(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    value_cents BIGINT NOT NULL,
    value_currency VARCHAR(3) NOT NULL,
    expected_close_date TIMESTAMP WITH TIME ZONE,
    is_platform_deal BOOLEAN NOT NULL DEFAULT false,
    owner_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    custom_fields JSONB,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for deals
CREATE INDEX idx_crm_deals_contact_id ON crm_deals(contact_id);
CREATE INDEX idx_crm_deals_pipeline_id ON crm_deals(pipeline_id);
CREATE INDEX idx_crm_deals_current_stage_id ON crm_deals(current_stage_id);
CREATE INDEX idx_crm_deals_owner_id ON crm_deals(owner_id);
CREATE INDEX idx_crm_deals_expected_close_date ON crm_deals(expected_close_date);

-- Create deal notes table
CREATE TABLE crm_deal_notes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    deal_id UUID NOT NULL REFERENCES crm_deals(id) ON DELETE CASCADE,
    content TEXT NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    is_shared_with_contact BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for deal notes
CREATE INDEX idx_crm_deal_notes_deal_id ON crm_deal_notes(deal_id);
CREATE INDEX idx_crm_deal_notes_created_by ON crm_deal_notes(created_by);

-- +goose Down
-- SQL section 'Down' is executed when this migration is rolled back

DROP TABLE IF EXISTS crm_deal_notes;
DROP TABLE IF EXISTS crm_deals;
DROP TABLE IF EXISTS crm_pipeline_stages;
DROP TABLE IF EXISTS crm_pipelines;
DROP TABLE IF EXISTS crm_interactions;
DROP TABLE IF EXISTS crm_contacts;