-- Volunteer Coordination: schema creation (UP)
-- Mirrors domain models in shared_packages/volunteer_coordination and SQLx repositories.
-- Notes:
-- - SMALLINT enums are documented via COMMENT ON COLUMN with numeric mapping.
-- - Timestamps use NOW() defaults; updated_at maintained by app code per repository pattern.
-- - UUID defaults are not set here; values provided by application layer.

BEGIN;

-- 1) volunteer_opportunities
CREATE TABLE IF NOT EXISTS volunteer_opportunities (
    id UUID PRIMARY KEY,
    org_id UUID NOT NULL,
    created_by UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    tags TEXT[] NULL,
    status SMALLINT NOT NULL,
    location TEXT NULL,
    starts_at TIMESTAMPTZ NULL,
    ends_at TIMESTAMPTZ NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_vol_opps_status CHECK (status IN (0, 1, 2, 3))
);

-- Enum mapping comment
COMMENT ON COLUMN volunteer_opportunities.status IS
    'OpportunityStatus mapping: 0=Draft, 1=Published, 2=Closed, 3=Archived';

-- Helpful comments for columns
COMMENT ON COLUMN volunteer_opportunities.tags IS 'Arbitrary tags/labels (domain field `tags`)';
COMMENT ON COLUMN volunteer_opportunities.starts_at IS 'Optional schedule start';
COMMENT ON COLUMN volunteer_opportunities.ends_at IS 'Optional schedule end';

-- Indexes
CREATE INDEX IF NOT EXISTS idx_vol_opps_org_id ON volunteer_opportunities (org_id);
CREATE INDEX IF NOT EXISTS idx_vol_opps_status ON volunteer_opportunities (status);

-- 2) volunteer_applications
CREATE TABLE IF NOT EXISTS volunteer_applications (
    id UUID PRIMARY KEY,
    opportunity_id UUID NOT NULL REFERENCES volunteer_opportunities(id) ON DELETE CASCADE,
    applicant_id UUID NOT NULL,
    motivation TEXT NULL,
    status SMALLINT NOT NULL,
    submitted_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    decided_at TIMESTAMPTZ NULL,
    reviewer_id UUID NULL,
    CONSTRAINT chk_vol_apps_status CHECK (status IN (0, 1, 2, 3, 4))
);

COMMENT ON COLUMN volunteer_applications.status IS
    'ApplicationStatus mapping: 0=Submitted, 1=UnderReview, 2=Accepted, 3=Rejected, 4=Withdrawn';

-- Indexes
CREATE INDEX IF NOT EXISTS idx_vol_apps_opp ON volunteer_applications (opportunity_id);
CREATE INDEX IF NOT EXISTS idx_vol_apps_status ON volunteer_applications (status);

-- 3) volunteer_contributions
CREATE TABLE IF NOT EXISTS volunteer_contributions (
    id UUID PRIMARY KEY,
    opportunity_id UUID NOT NULL REFERENCES volunteer_opportunities(id) ON DELETE CASCADE,
    contributor_id UUID NOT NULL,
    kind SMALLINT NOT NULL,
    amount REAL NOT NULL,
    notes TEXT NULL,
    occurred_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    verification_ref UUID NULL,
    CONSTRAINT chk_vol_contribs_kind CHECK (kind IN (0, 1, 2, 3))
    -- Optional strict rule tying amount/hours to kind was requested; we keep it documented instead of strict:
    -- For Hours kind (=0), amount represents hours. For other kinds, amount is a generic quantity/points.
);

COMMENT ON COLUMN volunteer_contributions.kind IS
    'ContributionKind mapping: 0=Hours, 1=Deliverable, 2=Donation, 3=Other';

COMMENT ON COLUMN volunteer_contributions.amount IS
    'For Hours (kind=0), represents hours (float). For others, generic amount/points.';

-- Indexes
CREATE INDEX IF NOT EXISTS idx_vol_contribs_opp ON volunteer_contributions (opportunity_id);
CREATE INDEX IF NOT EXISTS idx_vol_contribs_contributor ON volunteer_contributions (contributor_id);
CREATE INDEX IF NOT EXISTS idx_vol_contribs_kind ON volunteer_contributions (kind);

COMMIT;