-- Create organization impact reports table
CREATE TABLE organization_impact_reports (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL,
    year INT NOT NULL,
    carbon_footprint FLOAT NOT NULL,
    community_investment FLOAT NOT NULL,
    gender_diversity FLOAT NOT NULL,
    ethnic_diversity FLOAT NOT NULL,
    supply_chain_score FLOAT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index for efficient lookups
CREATE INDEX idx_org_impact ON organization_impact_reports(organization_id, year);