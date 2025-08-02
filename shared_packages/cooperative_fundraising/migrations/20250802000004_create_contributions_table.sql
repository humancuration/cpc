-- Create contributions table
CREATE TABLE contributions (
    id UUID PRIMARY KEY,
    campaign_id UUID NOT NULL REFERENCES campaigns(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    cpay_transaction_id UUID,
    amount NUMERIC,
    currency VARCHAR(3) NOT NULL,
    opportunity_id UUID,
    hours INTEGER,
    verification_status TEXT CHECK (verification_status IN (
        'pending', 
        'verified', 
        'disputed', 
        'rejected'
    ))
);

-- Indexes for common queries
CREATE INDEX idx_contributions_campaign ON contributions(campaign_id);
CREATE INDEX idx_contributions_user ON contributions(user_id);
CREATE INDEX idx_contributions_status ON contributions(verification_status);