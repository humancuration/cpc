-- Sales Reporting Schema Migration
-- This migration adds tables for sales reporting functionality

-- Create table for sales report definitions
CREATE TABLE sales_report_definitions (
    id UUID PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    filters JSONB NOT NULL,
    visualization_type VARCHAR(50) NOT NULL,
    created_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create table for sales report instances
CREATE TABLE sales_report_instances (
    id UUID PRIMARY KEY,
    report_id UUID NOT NULL REFERENCES sales_report_definitions(id) ON DELETE CASCADE,
    generated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data JSONB NOT NULL,
    format VARCHAR(20) NOT NULL CHECK (format IN ('csv', 'pdf', 'json')),
    generated_by UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE
);

-- Create indexes for better query performance
CREATE INDEX idx_sales_report_definitions_created_by ON sales_report_definitions(created_by);
CREATE INDEX idx_sales_report_definitions_created_at ON sales_report_definitions(created_at);
CREATE INDEX idx_sales_report_instances_report_id ON sales_report_instances(report_id);
CREATE INDEX idx_sales_report_instances_generated_at ON sales_report_instances(generated_at);
CREATE INDEX idx_sales_report_instances_format ON sales_report_instances(format);

-- Create table for dashboard widgets
CREATE TABLE dashboard_widgets (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    widget_type VARCHAR(50) NOT NULL,
    configuration JSONB NOT NULL,
    position INTEGER NOT NULL,
    is_visible BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for dashboard widgets
CREATE INDEX idx_dashboard_widgets_user_id ON dashboard_widgets(user_id);
CREATE INDEX idx_dashboard_widgets_position ON dashboard_widgets(position);

-- Insert default report definitions
INSERT INTO sales_report_definitions (id, name, description, filters, visualization_type, created_by, created_at, updated_at)
VALUES 
    ('00000000-0000-0000-0000-000000000002', 'Sales Pipeline Overview', 'Overview of current sales pipeline', '{}', 'pipeline', '00000000-0000-0000-0000-000000000000', NOW(), NOW()),
    ('00000000-0000-0000-0000-000000000003', 'Team Performance Report', 'Performance metrics by team', '{}', 'bar_chart', '00000000-0000-0000-0000-000000000000', NOW(), NOW()),
    ('00000000-0000-0000-0000-000000000004', 'Deal Aging Analysis', 'Analysis of deal ages in pipeline', '{}', 'line_chart', '00000000-0000-0000-0000-000000000000', NOW(), NOW());