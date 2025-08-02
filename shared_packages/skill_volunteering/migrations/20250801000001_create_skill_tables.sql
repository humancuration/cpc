-- Create skills table
CREATE TABLE skills (
  id UUID PRIMARY KEY,
  name TEXT NOT NULL UNIQUE,
  category TEXT NOT NULL,
  description TEXT,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  updated_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create volunteer opportunities table
CREATE TABLE volunteer_opportunities (
  id UUID PRIMARY KEY,
  cause_id UUID NOT NULL,
  title TEXT NOT NULL,
  description TEXT NOT NULL,
  required_skills UUID[],
  estimated_hours INT,
  created_at TIMESTAMPTZ DEFAULT NOW(),
  deadline TIMESTAMPTZ,
  created_by UUID NOT NULL
);

-- Create opportunity applications table
CREATE TABLE opportunity_applications (
  id UUID PRIMARY KEY,
  opportunity_id UUID REFERENCES volunteer_opportunities(id) ON DELETE CASCADE,
  user_id UUID NOT NULL,
  applied_at TIMESTAMPTZ DEFAULT NOW(),
  status TEXT CHECK(status IN ('pending', 'accepted', 'rejected', 'completed')) DEFAULT 'pending',
  volunteer_hours DECIMAL(10,2),
  UNIQUE(opportunity_id, user_id)
); 

-- Create volunteer impact records table
CREATE TABLE volunteer_impacts (
  id UUID PRIMARY KEY,
  opportunity_id UUID REFERENCES volunteer_opportunities(id) ON DELETE CASCADE,
  user_id UUID NOT NULL,
  skill_id UUID REFERENCES skills(id) ON DELETE SET NULL,
  hours_contributed DECIMAL(10,2) NOT NULL,
  impact_description TEXT,
  recorded_at TIMESTAMPTZ DEFAULT NOW()
);

-- Create indexes for better performance
CREATE INDEX idx_volunteer_opportunities_cause_id ON volunteer_opportunities(cause_id);
CREATE INDEX idx_volunteer_opportunities_deadline ON volunteer_opportunities(deadline);
CREATE INDEX idx_opportunity_applications_opportunity_id ON opportunity_applications(opportunity_id);
CREATE INDEX idx_opportunity_applications_user_id ON opportunity_applications(user_id);
CREATE INDEX idx_opportunity_applications_status ON opportunity_applications(status);
CREATE INDEX idx_volunteer_impacts_opportunity_id ON volunteer_impacts(opportunity_id);
CREATE INDEX idx_volunteer_impacts_user_id ON volunteer_impacts(user_id);
CREATE INDEX idx_volunteer_impacts_skill_id ON volunteer_impacts(skill_id);