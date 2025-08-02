-- Create donation-specific campaign details
CREATE TABLE donation_campaigns (
    campaign_id UUID PRIMARY KEY REFERENCES campaigns(id) ON DELETE CASCADE,
    funding_goal NUMERIC,
    external_use_case TEXT NOT NULL,
    currency VARCHAR(3) NOT NULL
);