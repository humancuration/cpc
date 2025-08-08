-- Create financial impact tracking tables

-- Financial impact records table
CREATE TABLE IF NOT EXISTS financial_impact_records (
    id UUID PRIMARY KEY,
    timestamp TIMESTAMPTZ NOT NULL,
    transaction_id UUID REFERENCES transactions(id),
    event_type TEXT NOT NULL,
    amount DECIMAL NOT NULL,
    currency TEXT NOT NULL,
    category TEXT NOT NULL,
    description TEXT NOT NULL,
    metadata JSONB NOT NULL,
    impact_score DECIMAL NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_financial_impact_records_timestamp ON financial_impact_records(timestamp);
CREATE INDEX IF NOT EXISTS idx_financial_impact_records_category ON financial_impact_records(category);
CREATE INDEX IF NOT EXISTS idx_financial_impact_records_transaction ON financial_impact_records(transaction_id);

-- Financial cause links table (for linking financial activities to causes)
CREATE TABLE IF NOT EXISTS financial_cause_links (
    transaction_id UUID REFERENCES transactions(id),
    cause_id UUID NOT NULL,
    alignment_score DECIMAL NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (transaction_id, cause_id)
);

-- Financial volunteer links table (for linking financial activities to volunteer work)
CREATE TABLE IF NOT EXISTS financial_volunteer_links (
    transaction_id UUID REFERENCES transactions(id),
    volunteer_id UUID NOT NULL,
    hours_contributed DECIMAL NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (transaction_id, volunteer_id)
);

-- Financial learning links table (for linking financial activities to learning outcomes)
CREATE TABLE IF NOT EXISTS financial_learning_links (
    transaction_id UUID REFERENCES transactions(id),
    learning_program_id UUID NOT NULL,
    impact_score DECIMAL NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (transaction_id, learning_program_id)
);

-- Financial impact reports table
CREATE TABLE IF NOT EXISTS financial_impact_reports (
    id UUID PRIMARY KEY,
    generated_at TIMESTAMPTZ NOT NULL,
    period_start TIMESTAMPTZ NOT NULL,
    period_end TIMESTAMPTZ NOT NULL,
    report_data JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Indexes for reports
CREATE INDEX IF NOT EXISTS idx_financial_impact_reports_period ON financial_impact_reports(period_start, period_end);
CREATE INDEX IF NOT EXISTS idx_financial_impact_reports_generated ON financial_impact_reports(generated_at);

-- Function to update updated_at timestamp
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers to automatically update updated_at
CREATE TRIGGER update_financial_impact_records_updated_at 
    BEFORE UPDATE ON financial_impact_records 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_financial_cause_links_updated_at 
    BEFORE UPDATE ON financial_cause_links 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_financial_volunteer_links_updated_at 
    BEFORE UPDATE ON financial_volunteer_links 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_financial_learning_links_updated_at 
    BEFORE UPDATE ON financial_learning_links 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_financial_impact_reports_updated_at 
    BEFORE UPDATE ON financial_impact_reports 
    FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();