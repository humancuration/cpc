-- Create interviews table
CREATE TABLE interviews (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    application_id UUID NOT NULL REFERENCES applications(id),
    scheduled_time TIMESTAMP WITH TIME ZONE NOT NULL,
    location TEXT,
    meeting_link TEXT,
    notes TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create job_alerts table
CREATE TABLE job_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    candidate_id UUID NOT NULL REFERENCES candidates(id),
    keywords TEXT,
    location TEXT,
    employment_type TEXT CHECK (employment_type IN ('full_time', 'part_time', 'contract', 'internship')),
    is_remote BOOLEAN,
    min_salary NUMERIC,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create job_alert_matches table
CREATE TABLE job_alert_matches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_alert_id UUID NOT NULL REFERENCES job_alerts(id),
    job_id UUID NOT NULL REFERENCES jobs(id),
    matched_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create index for better performance
CREATE INDEX idx_jobs_status ON jobs(status);
CREATE INDEX idx_jobs_employer ON jobs(employer_id);
CREATE INDEX idx_applications_job ON applications(job_id);
CREATE INDEX idx_applications_candidate ON applications(candidate_id);
CREATE INDEX idx_interviews_application ON interviews(application_id);
CREATE INDEX idx_job_alerts_candidate ON job_alerts(candidate_id);