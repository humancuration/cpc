-- Add tables for skill exchange functionality

-- Create skill_listings table
CREATE TABLE skill_listings (
    id UUID PRIMARY KEY,
    provider_id UUID NOT NULL,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    category TEXT NOT NULL,
    estimated_time DECIMAL(10,2),
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create skill_claims table
CREATE TABLE skill_claims (
    id UUID PRIMARY KEY,
    listing_id UUID NOT NULL REFERENCES skill_listings(id) ON DELETE CASCADE,
    claimant_id UUID NOT NULL,
    status TEXT CHECK(status IN ('pending', 'accepted', 'rejected', 'completed')) NOT NULL DEFAULT 'pending',
    message TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create skill_exchange_completions table
CREATE TABLE skill_exchange_completions (
    id UUID PRIMARY KEY,
    listing_id UUID NOT NULL REFERENCES skill_listings(id) ON DELETE CASCADE,
    claim_id UUID NOT NULL REFERENCES skill_claims(id) ON DELETE CASCADE,
    provider_id UUID NOT NULL,
    claimant_id UUID NOT NULL,
    rating_value INTEGER CHECK(rating_value >= 1 AND rating_value <= 5),
    rating_comment TEXT,
    payment_amount DECIMAL(10,2),
    transaction_id UUID,
    completed_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_skill_listings_provider_id ON skill_listings(provider_id);
CREATE INDEX idx_skill_listings_category ON skill_listings(category);
CREATE INDEX idx_skill_listings_active ON skill_listings(is_active);
CREATE INDEX idx_skill_claims_listing_id ON skill_claims(listing_id);
CREATE INDEX idx_skill_claims_claimant_id ON skill_claims(claimant_id);
CREATE INDEX idx_skill_claims_status ON skill_claims(status);
CREATE INDEX idx_skill_exchange_completions_provider_id ON skill_exchange_completions(provider_id);
CREATE INDEX idx_skill_exchange_completions_claimant_id ON skill_exchange_completions(claimant_id);