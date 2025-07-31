-- DEPRECATED: This file has been moved to the wallet package
-- The tip functionality has been moved to the wallet package
CREATE TABLE tip_transactions (
    id UUID PRIMARY KEY,
    sender_id UUID NOT NULL,
    recipient_id UUID NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    transaction_type VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_tip_transactions_sender_id ON tip_transactions (sender_id);
CREATE INDEX idx_tip_transactions_recipient_id ON tip_transactions (recipient_id);