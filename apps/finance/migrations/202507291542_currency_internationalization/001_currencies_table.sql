-- Create currencies table
-- This table stores all supported currencies with their properties

CREATE TABLE currencies (
    code CHAR(3) PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    symbol VARCHAR(10) NOT NULL,
    decimal_places SMALLINT NOT NULL DEFAULT 2,
    is_dabloon BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX idx_currencies_is_dabloon ON currencies(is_dabloon);
CREATE INDEX idx_currencies_name ON currencies(name);

-- Insert the Dabloons currency
INSERT INTO currencies (code, name, symbol, decimal_places, is_dabloon) 
VALUES ('DABLOONS', 'Dabloons', 'ᴅ', 2, TRUE);

-- Add comment for documentation
COMMENT ON TABLE currencies IS 'Supported currencies with their properties';
COMMENT ON COLUMN currencies.code IS 'ISO 4217 currency code';
COMMENT ON COLUMN currencies.name IS 'Full currency name';
COMMENT ON COLUMN currencies.symbol IS 'Currency symbol for display';
COMMENT ON COLUMN currencies.decimal_places IS 'Number of decimal places for this currency';
COMMENT ON COLUMN currencies.is_dabloon IS 'Whether this is the platform''s internal currency';