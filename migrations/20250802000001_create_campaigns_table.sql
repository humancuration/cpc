-- Create campaigns table
CREATE TABLE campaigns (
    id UUID PRIMARY KEY,
    type campaign_type NOT NULL,
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    owner_user_id UUID NOT NULL, -- Cooperative admin
    status campaign_status NOT NULL DEFAULT 'draft'
);

-- Campaign types reflect community vs regulatory needs
CREATE TYPE campaign_type AS ENUM (
    'cooperative_membership', -- Pure participation-based
    'pure_donation',          -- GoFundMe style
    'reg_cf',                 -- SEC Regulation Crowdfunding
    'reg_a',                  -- SEC Regulation A+
    'reg_d'                   -- SEC Regulation D
);

CREATE TYPE campaign_status AS ENUM (
    'draft', 'active', 'completed', 'failed', 'cancelled'
);

-- Create indexes for better performance
CREATE INDEX idx_campaigns_type ON campaigns(type);
CREATE INDEX idx_campaigns_status ON campaigns(status);
CREATE INDEX idx_campaigns_owner ON campaigns(owner_user_id);