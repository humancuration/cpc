-- Create membership requirements table
CREATE TABLE membership_requirements (
    campaign_id UUID PRIMARY KEY REFERENCES campaigns(id) ON DELETE CASCADE,
    max_participants INTEGER,
    required_actions JSONB NOT NULL
);