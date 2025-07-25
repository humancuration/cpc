-- Create enums for governance system
CREATE TYPE proposal_status AS ENUM ('DRAFT', 'VOTING', 'PASSED', 'FAILED', 'EXECUTED', 'EXPIRED');
CREATE TYPE proposal_type AS ENUM ('FEATURE', 'CONTENT', 'POLICY', 'BUGFIX', 'TECHNICAL', 'COMMUNITY');

-- Create proposals table
CREATE TABLE IF NOT EXISTS proposals (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    cooperative_id UUID NOT NULL,
    proposer_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    status proposal_status NOT NULL DEFAULT 'DRAFT',
    proposal_type proposal_type NOT NULL,
    options TEXT[] NOT NULL DEFAULT '{}',
    change_type VARCHAR(255) NOT NULL,
    target_system VARCHAR(255) NOT NULL,
    change_description TEXT NOT NULL,
    implementation_notes TEXT,
    rollback_plan TEXT,
    impact_assessment TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    voting_deadline TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    quorum_threshold REAL NOT NULL DEFAULT 0.5,
    participation_count INTEGER NOT NULL DEFAULT 0,
    eligible_voter_count INTEGER NOT NULL DEFAULT 0
);

-- Create governance_votes table (different from forum votes)
CREATE TABLE IF NOT EXISTS governance_votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    voter_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    choices TEXT[] NOT NULL DEFAULT '{}',
    voting_weight REAL NOT NULL DEFAULT 1.0,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_anonymous BOOLEAN NOT NULL DEFAULT FALSE,
    UNIQUE(proposal_id, voter_id)
);

-- Create vote_tallies table for RCV results
CREATE TABLE IF NOT EXISTS vote_tallies (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    round_number INTEGER NOT NULL,
    round_results JSONB NOT NULL DEFAULT '{}',
    eliminated_options TEXT[] DEFAULT '{}',
    total_votes INTEGER NOT NULL DEFAULT 0,
    total_weight REAL NOT NULL DEFAULT 0.0,
    calculated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    is_final_round BOOLEAN NOT NULL DEFAULT FALSE,
    winner VARCHAR(255)
);

-- Create voting_results table
CREATE TABLE IF NOT EXISTS voting_results (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    proposal_id UUID NOT NULL REFERENCES proposals(id) ON DELETE CASCADE,
    winner VARCHAR(255),
    total_participants INTEGER NOT NULL DEFAULT 0,
    quorum_met BOOLEAN NOT NULL DEFAULT FALSE,
    final_status proposal_status NOT NULL,
    finalized_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(proposal_id)
);

-- Create governance_participation table
CREATE TABLE IF NOT EXISTS governance_participation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    cooperative_id UUID NOT NULL,
    proposals_created INTEGER NOT NULL DEFAULT 0,
    votes_cast INTEGER NOT NULL DEFAULT 0,
    proposals_participated UUID[] DEFAULT '{}',
    participation_score REAL NOT NULL DEFAULT 0.0,
    last_activity TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(user_id, cooperative_id)
);

-- Create indexes for performance
CREATE INDEX IF NOT EXISTS idx_proposals_cooperative_id ON proposals(cooperative_id);
CREATE INDEX IF NOT EXISTS idx_proposals_proposer_id ON proposals(proposer_id);
CREATE INDEX IF NOT EXISTS idx_proposals_status ON proposals(status);
CREATE INDEX IF NOT EXISTS idx_proposals_type ON proposals(proposal_type);
CREATE INDEX IF NOT EXISTS idx_proposals_created_at ON proposals(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_proposals_voting_deadline ON proposals(voting_deadline);

CREATE INDEX IF NOT EXISTS idx_governance_votes_proposal_id ON governance_votes(proposal_id);
CREATE INDEX IF NOT EXISTS idx_governance_votes_voter_id ON governance_votes(voter_id);
CREATE INDEX IF NOT EXISTS idx_governance_votes_created_at ON governance_votes(created_at DESC);

CREATE INDEX IF NOT EXISTS idx_vote_tallies_proposal_id ON vote_tallies(proposal_id);
CREATE INDEX IF NOT EXISTS idx_vote_tallies_round_number ON vote_tallies(proposal_id, round_number);

CREATE INDEX IF NOT EXISTS idx_voting_results_proposal_id ON voting_results(proposal_id);

CREATE INDEX IF NOT EXISTS idx_governance_participation_user_id ON governance_participation(user_id);
CREATE INDEX IF NOT EXISTS idx_governance_participation_cooperative_id ON governance_participation(cooperative_id);
CREATE INDEX IF NOT EXISTS idx_governance_participation_score ON governance_participation(participation_score DESC);