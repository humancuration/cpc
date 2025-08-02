-- Create core campaigns table
CREATE TABLE campaigns (
    id UUID PRIMARY KEY,
    type TEXT NOT NULL CHECK (type IN (
        'cooperative_membership', 
        'pure_donation', 
        'reg_cf', 
        'reg_a', 
        'reg_d'
    )),
    title TEXT NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL,
    owner_user_id UUID NOT NULL,
    status TEXT NOT NULL CHECK (status IN (
        'draft', 
        'active', 
        'completed', 
        'failed', 
        'cancelled'
    ))
);

-- Index for common queries
CREATE INDEX idx_campaigns_type ON campaigns(type);
CREATE INDEX idx_campaigns_status ON campaigns(status);
CREATE INDEX idx_campaigns_owner ON campaigns(owner_user_id);