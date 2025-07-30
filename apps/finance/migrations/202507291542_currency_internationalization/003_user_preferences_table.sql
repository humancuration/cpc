-- Create user_currency_preferences table
-- This table stores user-specific currency preferences

CREATE TABLE user_currency_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    default_currency CHAR(3) NOT NULL REFERENCES currencies(code),
    preferred_locale VARCHAR(10) NOT NULL DEFAULT 'en-US',
    show_currency_symbols BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX idx_user_currency_preferences_currency ON user_currency_preferences(default_currency);
CREATE INDEX idx_user_currency_preferences_locale ON user_currency_preferences(preferred_locale);

-- Add comment for documentation
COMMENT ON TABLE user_currency_preferences IS 'User-specific currency preferences';
COMMENT ON COLUMN user_currency_preferences.user_id IS 'The user this preference belongs to';
COMMENT ON COLUMN user_currency_preferences.default_currency IS 'The user''s default currency';
COMMENT ON COLUMN user_currency_preferences.preferred_locale IS 'The user''s preferred locale for formatting';
COMMENT ON COLUMN user_currency_preferences.show_currency_symbols IS 'Whether to show currency symbols or codes';