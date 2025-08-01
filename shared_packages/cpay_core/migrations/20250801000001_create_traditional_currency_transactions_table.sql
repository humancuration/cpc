-- Create traditional_currency_transactions table for storing traditional currency transaction history
CREATE TABLE IF NOT EXISTS traditional_currency_transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    transaction_type VARCHAR(10) NOT NULL, -- 'credit' or 'debit'
    amount DECIMAL(20, 2) NOT NULL,
    currency VARCHAR(3) NOT NULL,
    external_reference VARCHAR(255),
    status VARCHAR(20) NOT NULL,
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX IF NOT EXISTS idx_traditional_currency_transactions_user_id ON traditional_currency_transactions(user_id);
CREATE INDEX IF NOT EXISTS idx_traditional_currency_transactions_currency ON traditional_currency_transactions(currency);
CREATE INDEX IF NOT EXISTS idx_traditional_currency_transactions_created_at ON traditional_currency_transactions(created_at);