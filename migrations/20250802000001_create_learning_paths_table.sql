-- Create learning_paths table
CREATE TABLE IF NOT EXISTS learning_paths (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    creator_id UUID NOT NULL,
    skills JSONB NOT NULL DEFAULT '[]'::jsonb,
    estimated_duration_hours INTEGER NOT NULL DEFAULT 0 CHECK (estimated_duration_hours >= 0),
    difficulty_level VARCHAR(20) NOT NULL CHECK (difficulty_level IN ('Beginner', 'Intermediate', 'Advanced')),
    tags TEXT[] NOT NULL DEFAULT '{}',
    is_public BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create indexes for learning_paths
CREATE INDEX IF NOT EXISTS idx_learning_paths_creator_id ON learning_paths(creator_id);
CREATE INDEX IF NOT EXISTS idx_learning_paths_difficulty_level ON learning_paths(difficulty_level);
CREATE INDEX IF NOT EXISTS idx_learning_paths_is_public ON learning_paths(is_public) WHERE is_public = TRUE;
CREATE INDEX IF NOT EXISTS idx_learning_paths_tags ON learning_paths USING GIN(tags);

-- Create trigger to update updated_at
CREATE OR REPLACE FUNCTION update_learning_paths_updated_at()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_update_learning_paths_updated_at
    BEFORE UPDATE ON learning_paths
    FOR EACH ROW
    EXECUTE FUNCTION update_learning_paths_updated_at();