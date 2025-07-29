-- Create savings_goals table for personal finance module
CREATE TABLE savings_goals (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    name TEXT NOT NULL,
    target_amount NUMERIC(18, 2) NOT NULL,
    current_amount NUMERIC(18, 2) NOT NULL DEFAULT 0,
    deadline TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_savings_goals_user ON savings_goals(user_id);