-- Add fields for fundraising campaign sites

-- Add campaign-related columns to sites table
ALTER TABLE sites 
ADD COLUMN IF NOT EXISTS campaign_id UUID,
ADD COLUMN IF NOT EXISTS campaign_title VARCHAR(255),
ADD COLUMN IF NOT EXISTS campaign_description TEXT,
ADD COLUMN IF NOT EXISTS campaign_type VARCHAR(50),
ADD COLUMN IF NOT EXISTS goal_amount BIGINT,
ADD COLUMN IF NOT EXISTS current_amount BIGINT DEFAULT 0,
ADD COLUMN IF NOT EXISTS campaign_start_date TIMESTAMPTZ,
ADD COLUMN IF NOT EXISTS campaign_end_date TIMESTAMPTZ;

-- Add index for campaign_id for performance
CREATE INDEX IF NOT EXISTS idx_sites_campaign_id ON sites(campaign_id);

-- Add constraint to ensure campaign data consistency
-- This ensures that if campaign_id is set, campaign_title must also be set
ALTER TABLE sites 
ADD CONSTRAINT chk_campaign_data CHECK (
    (campaign_id IS NULL) = (campaign_title IS NULL)
);