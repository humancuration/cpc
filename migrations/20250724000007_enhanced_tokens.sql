-- Enhance existing tokens table with additional fields
ALTER TABLE tokens ADD COLUMN IF NOT EXISTS token_type VARCHAR(50) NOT NULL DEFAULT 'refresh';
ALTER TABLE tokens ADD COLUMN IF NOT EXISTS id UUID DEFAULT gen_random_uuid();

-- Add primary key to tokens table if it doesn't exist
DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1 FROM information_schema.table_constraints 
        WHERE table_name = 'tokens' AND constraint_type = 'PRIMARY KEY'
    ) THEN
        ALTER TABLE tokens ADD PRIMARY KEY (id);
    END IF;
END $$;

-- Create indexes for token management
CREATE INDEX IF NOT EXISTS idx_tokens_user_id ON tokens(user_id);
CREATE INDEX IF NOT EXISTS idx_tokens_refresh_token ON tokens(refresh_token);
CREATE INDEX IF NOT EXISTS idx_tokens_token_type ON tokens(token_type);
CREATE INDEX IF NOT EXISTS idx_tokens_expires_at ON tokens(expires_at);
CREATE INDEX IF NOT EXISTS idx_tokens_created_at ON tokens(created_at DESC);