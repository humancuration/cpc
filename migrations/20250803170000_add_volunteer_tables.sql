-- Add tables for volunteer tracking functionality

-- Create volunteer_activities table
CREATE TABLE volunteer_activities (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    organization_id UUID,
    description TEXT NOT NULL,
    hours DECIMAL(10,2) NOT NULL,
    verified BOOLEAN NOT NULL DEFAULT FALSE,
    verified_by UUID,
    verified_at TIMESTAMPTZ,
    converted_to_dabloons BOOLEAN NOT NULL DEFAULT FALSE,
    conversion_transaction_id UUID,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create volunteer_verifications table
CREATE TABLE volunteer_verifications (
    id UUID PRIMARY KEY,
    activity_id UUID NOT NULL REFERENCES volunteer_activities(id) ON DELETE CASCADE,
    verifier_id UUID NOT NULL,
    status TEXT CHECK(status IN ('pending', 'approved', 'rejected')) NOT NULL DEFAULT 'pending',
    notes TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ
);

-- Create dabloon_conversions table
CREATE TABLE dabloon_conversions (
    id UUID PRIMARY KEY,
    activity_id UUID NOT NULL REFERENCES volunteer_activities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    hours_converted DECIMAL(10,2) NOT NULL,
    dabloons_credited DECIMAL(10,2) NOT NULL,
    transaction_id UUID NOT NULL,
    skill_rate DECIMAL(10,2) NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_volunteer_activities_user_id ON volunteer_activities(user_id);
CREATE INDEX idx_volunteer_activities_organization_id ON volunteer_activities(organization_id);
CREATE INDEX idx_volunteer_activities_verified ON volunteer_activities(verified);
CREATE INDEX idx_volunteer_activities_converted ON volunteer_activities(converted_to_dabloons);
CREATE INDEX idx_volunteer_verifications_activity_id ON volunteer_verifications(activity_id);
CREATE INDEX idx_volunteer_verifications_verifier_id ON volunteer_verifications(verifier_id);
CREATE INDEX idx_volunteer_verifications_status ON volunteer_verifications(status);
CREATE INDEX idx_dabloon_conversions_user_id ON dabloon_conversions(user_id);
CREATE INDEX idx_dabloon_conversions_activity_id ON dabloon_conversions(activity_id);