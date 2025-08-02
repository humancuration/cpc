-- Create certifications table
CREATE TABLE IF NOT EXISTS certifications (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    skill_id UUID NOT NULL,
    certification_type VARCHAR(50) NOT NULL CHECK (certification_type IN ('CourseCompletion', 'PeerEndorsement', 'SkillAssessment', 'ProjectReview', 'PortfolioReview')),
    level_achieved VARCHAR(20) NOT NULL CHECK (level_achieved IN ('Beginner', 'Intermediate', 'Advanced', 'Expert', 'Master')),
    issued_by UUID NOT NULL,
    issued_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE,
    credential_url TEXT,
    verification_code VARCHAR(255) NOT NULL UNIQUE,
    metadata JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for certifications
CREATE INDEX IF NOT EXISTS idx_certifications_user_id ON certifications(user_id);
CREATE INDEX IF NOT EXISTS idx_certifications_skill_id ON certifications(skill_id);
CREATE INDEX IF NOT EXISTS idx_certifications_issued_by ON certifications(issued_by);
CREATE INDEX IF NOT EXISTS idx_certifications_verification_code ON certifications(verification_code);
CREATE INDEX IF NOT EXISTS idx_certifications_user_skill ON certifications(user_id, skill_id);

-- Create trigger to update updated_at
CREATE OR REPLACE FUNCTION update_certifications_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_certifications_updated_at
    BEFORE UPDATE ON certifications
    FOR EACH ROW
    EXECUTE FUNCTION update_certifications_updated_at();