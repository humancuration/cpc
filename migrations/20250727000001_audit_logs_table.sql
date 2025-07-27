-- Audit Logs Table Migration
-- This migration creates the audit_logs table for HIPAA compliance

CREATE TABLE audit_logs (
    id UUID PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    accessed_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    data_type TEXT NOT NULL,
    data_id UUID NOT NULL,
    access_type TEXT NOT NULL,
    purpose TEXT NOT NULL,
    source_ip INET,
    device_info TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create indexes for common query patterns
CREATE INDEX idx_audit_logs_user_id ON audit_logs(user_id);
CREATE INDEX idx_audit_logs_data_type ON audit_logs(data_type);
CREATE INDEX idx_audit_logs_accessed_at ON audit_logs(accessed_at);
CREATE INDEX idx_audit_logs_purpose ON audit_logs(purpose);