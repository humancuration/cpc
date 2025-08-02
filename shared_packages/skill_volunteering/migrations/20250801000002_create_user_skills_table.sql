-- Create a custom enum type for skill levels for data consistency.
CREATE TYPE skill_level_enum AS ENUM ('beginner', 'intermediate', 'advanced');

-- Create the user_skills table
CREATE TABLE user_skills (
  user_id UUID NOT NULL, -- Assuming a users table exists
  skill_id UUID NOT NULL REFERENCES skills(id) ON DELETE CASCADE,
  skill_level skill_level_enum NOT NULL,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW(),
  PRIMARY KEY (user_id, skill_id)
);

-- Create indexes for efficient querying
CREATE INDEX idx_user_skills_user_id ON user_skills(user_id);
CREATE INDEX idx_user_skills_skill_id ON user_skills(skill_id);