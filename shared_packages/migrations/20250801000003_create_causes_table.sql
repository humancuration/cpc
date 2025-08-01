-- Create causes table for storing cause information
CREATE TABLE IF NOT EXISTS causes (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    image_url VARCHAR(512),
    total_donations DECIMAL(20, 2) NOT NULL DEFAULT 0.00,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX IF NOT EXISTS idx_causes_name ON causes(name);
CREATE INDEX IF NOT EXISTS idx_causes_created_at ON causes(created_at);
CREATE INDEX IF NOT EXISTS idx_causes_total_donations ON causes(total_donations);