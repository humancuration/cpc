-- Create wallet_transactions table for storing wallet transaction history
CREATE TABLE IF NOT EXISTS wallet_transactions (
    id UUID PRIMARY KEY,
    wallet_id UUID NOT NULL,
    transaction_type VARCHAR(10) NOT NULL, -- 'credit' or 'debit'
    amount DECIMAL(20, 0) NOT NULL,
    description TEXT,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Add indexes for common queries
CREATE INDEX IF NOT EXISTS idx_wallet_transactions_wallet_id ON wallet_transactions(wallet_id);
CREATE INDEX IF NOT EXISTS idx_wallet_transactions_timestamp ON wallet_transactions(timestamp);

-- Add foreign key constraint
ALTER TABLE wallet_transactions
    ADD CONSTRAINT fk_wallet_transactions_wallet_id
    FOREIGN KEY (wallet_id)
    REFERENCES wallets(id)
    ON DELETE CASCADE;