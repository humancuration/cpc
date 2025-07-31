-- DEPRECATED: This file has been replaced by 0003_create_tip_transactions_table.sql
CREATE TABLE reward_transactions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    amount NUMERIC(20, 8) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    event_type VARCHAR(50) NOT NULL,
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE INDEX idx_reward_transactions_user_id ON reward_transactions (user_id);