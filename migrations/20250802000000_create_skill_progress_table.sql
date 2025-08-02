-- Create skill_progress table
CREATE TABLE IF NOT EXISTS skill_progress (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    skill_id UUID NOT NULL,
    current_level VARCHAR(20) NOT NULL CHECK (current_level IN ('Beginner', 'Intermediate', 'Advanced', 'Expert', 'Master')),
    target_level VARCHAR(20) NOT NULL CHECK (target_level IN ('Beginner', 'Intermediate', 'Advanced', 'Expert', 'Master')),
    progress_percentage FLOAT NOT NULL DEFAULT 0.0 CHECK (progress_percentage >= 0 AND progress_percentage <= 100),
    milestones_completed JSONB NOT NULL DEFAULT '[]'::jsonb,
    total_hours_invested INTEGER NOT NULL DEFAULT 0 CHECK (total_hours_invested >= 0),
    last_practice_date TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for skill_progress
CREATE INDEX IF NOT EXISTS idx_skill_progress_user_id ON skill_progress(user_id);
CREATE INDEX IF NOT EXISTS idx_skill_progress_skill_id ON skill_progress(skill_id);
CREATE INDEX IF NOT EXISTS idx_skill_progress_user_skill ON skill_progress(user_id, skill_id);

-- Create trigger to update updated_at
CREATE OR REPLACE FUNCTION update_skill_progress_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_skill_progress_updated_at
    BEFORE UPDATE ON skill_progress
    FOR EACH ROW
    EXECUTE FUNCTION update_skill_progress_updated_at();