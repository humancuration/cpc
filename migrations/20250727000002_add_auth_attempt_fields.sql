-- Add authentication attempt fields to audit_logs table
-- This migration adds fields to track authentication attempts for HIPAA compliance

ALTER TABLE audit_logs 
ADD COLUMN IF NOT EXISTS attempt_type TEXT NOT NULL DEFAULT 'Success';

ALTER TABLE audit_logs 
ADD COLUMN IF NOT EXISTS attempt_correlation_id UUID;

ALTER TABLE audit_logs 
ADD COLUMN IF NOT EXISTS risk_score INTEGER NOT NULL DEFAULT 0 
    CONSTRAINT valid_risk_score CHECK (risk_score >= 0 AND risk_score <= 100);

ALTER TABLE audit_logs
ADD COLUMN IF NOT EXISTS failure_reason TEXT;

-- Add performance indexes for pattern detection
CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_logs_user_attempts
ON audit_logs(user_id, accessed_at)
WHERE attempt_type IS NOT NULL;

CREATE INDEX CONCURRENTLY IF NOT EXISTS idx_audit_logs_ip_attempts
ON audit_logs(source_ip, accessed_at)
WHERE attempt_type IS NOT NULL AND source_ip IS NOT NULL;