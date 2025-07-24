-- Drop trigger and function
DROP TRIGGER IF EXISTS trigger_log_job_status_change ON jobs;
DROP FUNCTION IF EXISTS log_job_status_change();

-- Drop tables
DROP TABLE IF EXISTS job_events;
DROP TABLE IF EXISTS jobs;