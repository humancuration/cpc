-- Create the project_status enum type
CREATE TYPE project_status AS ENUM (
    'not_started',
    'in_progress',
    'completed',
    'on_hold'
);

-- Create the projects table
CREATE TABLE projects (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    cooperative_id UUID NOT NULL,
    status project_status NOT NULL,
    start_date DATE,
    end_date DATE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create an index on the cooperative_id for faster lookups
CREATE INDEX idx_projects_cooperative_id ON projects(cooperative_id);

-- Create an index on the status for faster lookups
CREATE INDEX idx_projects_status ON projects(status);