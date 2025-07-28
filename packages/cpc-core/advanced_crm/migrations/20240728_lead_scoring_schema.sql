-- Lead Scoring Schema Migration
-- This migration adds tables for lead scoring functionality

-- Create table for lead scoring models
CREATE TABLE lead_scoring_models (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    weights JSONB NOT NULL,  -- Scoring weights configuration
    thresholds JSONB NOT NULL,  -- Score thresholds configuration
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create table for lead scores
CREATE TABLE lead_scores (
    lead_id UUID PRIMARY KEY REFERENCES crm_contacts(id),
    base_score SMALLINT NOT NULL CHECK (base_score BETWEEN 0 AND 100),
    engagement_score SMALLINT NOT NULL CHECK (engagement_score BETWEEN 0 AND 100),
    fit_score SMALLINT NOT NULL CHECK (fit_score BETWEEN 0 AND 100),
    wellness_score SMALLINT NOT NULL CHECK (wellness_score BETWEEN 0 AND 100),
    total_score SMALLINT NOT NULL CHECK (total_score BETWEEN 0 AND 100),
    scoring_factors JSONB NOT NULL,
    last_updated TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    scoring_model_id UUID NOT NULL REFERENCES lead_scoring_models(id)
);

-- Create indexes for better query performance
CREATE INDEX idx_lead_scores_total_score ON lead_scores(total_score);
CREATE INDEX idx_lead_scores_last_updated ON lead_scores(last_updated);
CREATE INDEX idx_lead_scoring_models_default ON lead_scoring_models(is_default) WHERE is_default = true;

-- Insert a default scoring model
INSERT INTO lead_scoring_models (id, name, description, weights, thresholds, is_default, created_at, updated_at)
VALUES (
    '00000000-0000-0000-0000-000000000001',
    'Default Scoring Model',
    'Default lead scoring model with balanced weights',
    '{"business_metrics": 0.6, "wellness_metrics": 0.4, "base_score": 0.3, "engagement_score": 0.4, "fit_score": 0.3}',
    '{"hot_lead": 80, "warm_lead": 60, "cold_lead": 40}',
    true,
    NOW(),
    NOW()
);