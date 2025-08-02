CREATE TABLE skills (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE skill_progress (
    id UUID PRIMARY KEY,
    skill_id UUID REFERENCES skills(id),
    user_id UUID NOT NULL,
    progress SMALLINT NOT NULL CHECK (progress BETWEEN 0 AND 100),
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE certifications (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    issuing_organization VARCHAR(255) NOT NULL,
    issue_date DATE NOT NULL,
    user_id UUID NOT NULL,
    skill_id UUID REFERENCES skills(id),
    certification_type SMALLINT NOT NULL,
    level_achieved SMALLINT CHECK (level_achieved BETWEEN 0 AND 4),
    verification_code VARCHAR(50) UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE learning_paths (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    skill_id UUID REFERENCES skills(id),
    title VARCHAR(255) NOT NULL,
    description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE milestones (
    id UUID PRIMARY KEY,
    learning_path_id UUID REFERENCES learning_paths(id) ON DELETE CASCADE,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    is_completed BOOLEAN DEFAULT FALSE,
    order_index INTEGER NOT NULL,
    estimated_duration_hours INTEGER
);

-- Create indexes for better query performance
CREATE INDEX idx_skill_progress_skill_id ON skill_progress(skill_id);
CREATE INDEX idx_skill_progress_user_id ON skill_progress(user_id);
CREATE INDEX idx_certifications_user_id ON certifications(user_id);
CREATE INDEX idx_certifications_skill_id ON certifications(skill_id);
CREATE INDEX idx_learning_paths_user_id ON learning_paths(user_id);
CREATE INDEX idx_learning_paths_skill_id ON learning_paths(skill_id);