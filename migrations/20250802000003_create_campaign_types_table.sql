-- Create membership_requirements table (for cooperative_membership campaigns)
CREATE TABLE membership_requirements (
    campaign_id UUID PRIMARY KEY REFERENCES campaigns(id),
    max_participants INTEGER,
    required_actions JSONB NOT NULL
    -- Example: [
    --   {"type": "attend_meeting", "count": 1},
    --   {"type": "complete_training", "module": "governance"}
    -- ]
);

-- Create donation_campaigns table (for pure_donation/reg_cf/reg_a)
CREATE TABLE donation_campaigns (
    campaign_id UUID PRIMARY KEY REFERENCES campaigns(id),
    funding_goal NUMERIC(18,2),
    external_use_case TEXT NOT NULL,
    currency VARCHAR(3) DEFAULT 'USD'
);

-- Create indexes for better performance
CREATE INDEX idx_membership_requirements_campaign ON membership_requirements(campaign_id);
CREATE INDEX idx_donation_campaigns_campaign ON donation_campaigns(campaign_id);

-- Create constraint triggers for contribution validation
-- For donation campaigns: must have monetary data
CREATE CONSTRAINT TRIGGER donation_contributions_check
AFTER INSERT OR UPDATE ON contributions
FOR EACH ROW
WHEN (NEW.campaign_id IN (SELECT id FROM donation_campaigns))
EXECUTE FUNCTION validate_donation_contribution();

-- For membership campaigns: must have volunteer action
CREATE CONSTRAINT TRIGGER membership_contributions_check
AFTER INSERT OR UPDATE ON contributions
FOR EACH ROW
WHEN (NEW.campaign_id IN (SELECT id FROM membership_requirements))
EXECUTE FUNCTION validate_membership_contribution();