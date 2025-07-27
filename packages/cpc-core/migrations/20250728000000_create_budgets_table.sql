-- Create budgets table for personal finance module
CREATE TABLE budgets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id),
    category TEXT NOT NULL,
    allocated_amount NUMERIC(18, 2) NOT NULL,
    spent_amount NUMERIC(18, 2) NOT NULL DEFAULT 0,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_budgets_user ON budgets(user_id);