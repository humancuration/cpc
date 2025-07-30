-- Create exchange_rates table
-- This table stores exchange rates between currencies

CREATE TABLE exchange_rates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_currency CHAR(3) NOT NULL REFERENCES currencies(code),
    to_currency CHAR(3) NOT NULL REFERENCES currencies(code),
    rate DECIMAL(20, 10) NOT NULL,
    provider VARCHAR(50) NOT NULL,
    fetched_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Ensure we don't have duplicate rates for the same currency pair from the same provider
    UNIQUE(from_currency, to_currency, provider, fetched_at)
);

-- Add indexes for common queries
CREATE INDEX idx_exchange_rates_from_to ON exchange_rates(from_currency, to_currency);
CREATE INDEX idx_exchange_rates_fetched_at ON exchange_rates(fetched_at DESC);
CREATE INDEX idx_exchange_rates_provider ON exchange_rates(provider);

-- Add comment for documentation
COMMENT ON TABLE exchange_rates IS 'Exchange rates between currencies from various providers';
COMMENT ON COLUMN exchange_rates.from_currency IS 'The base currency code';
COMMENT ON COLUMN exchange_rates.to_currency IS 'The target currency code';
COMMENT ON COLUMN exchange_rates.rate IS 'The exchange rate value';
COMMENT ON COLUMN exchange_rates.provider IS 'The provider that supplied this rate';
COMMENT ON COLUMN exchange_rates.fetched_at IS 'When this rate was fetched from the provider';