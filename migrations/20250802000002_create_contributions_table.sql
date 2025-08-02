-- Create contributions table
CREATE TABLE contributions (
    id UUID PRIMARY KEY,
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ DEFAULT NOW(),
    
    -- Monetary contributions (for donation campaigns only)
    cpay_transaction_id UUID, -- Links to cpay system
    amount NUMERIC(18,2),
    currency VARCHAR(3),
    
    -- Volunteer actions (for ALL campaign types)
    opportunity_id UUID, -- Links to skill_volunteering opportunities
    hours INTEGER,
    verification_status verification_status NOT NULL DEFAULT 'pending'
);

CREATE TYPE verification_status AS ENUM (
    'pending', 'verified', 'disputed', 'rejected'
);

-- Create indexes for better performance
CREATE INDEX idx_contributions_campaign ON contributions(campaign_id);
CREATE INDEX idx_contributions_user ON contributions(user_id);
CREATE INDEX idx_contributions_opportunity ON contributions(opportunity_id);
CREATE INDEX idx_contributions_verification ON contributions(verification_status);

-- Foreign key to skill_volunteering opportunities
ALTER TABLE contributions 
ADD CONSTRAINT fk_opportunity
FOREIGN KEY (opportunity_id) 
REFERENCES skill_volunteering.opportunities(id)
ON DELETE CASCADE;

-- Foreign key to cpay transactions (hypothetical structure)
-- ALTER TABLE contributions 
-- ADD CONSTRAINT fk_cpay
-- FOREIGN KEY (cpay_transaction_id) 
-- REFERENCES cpay.transactions(id);