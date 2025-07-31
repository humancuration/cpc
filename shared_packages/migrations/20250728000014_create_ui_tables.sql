-- Create Universal Income configuration table
CREATE TABLE IF NOT EXISTS ui_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    daily_amount DECIMAL(10,0) NOT NULL DEFAULT 0,
    start_date DATE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create Universal Income distributions table
CREATE TABLE IF NOT EXISTS ui_distributions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    amount DECIMAL(10,0) NOT NULL,
    distribution_date DATE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for better query performance
CREATE INDEX IF NOT EXISTS idx_ui_distributions_user_id ON ui_distributions(user_id);
CREATE INDEX IF NOT EXISTS idx_ui_distributions_date ON ui_distributions(distribution_date);
CREATE INDEX IF NOT EXISTS idx_ui_distributions_user_date ON ui_distributions(user_id, distribution_date);

-- Add a comment to describe the tables
COMMENT ON TABLE ui_config IS 'Universal Income configuration settings';
COMMENT ON TABLE ui_distributions IS 'Records of Universal Income distributions to users';