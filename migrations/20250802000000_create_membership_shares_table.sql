-- Create user_shares table for tracking cooperative membership
CREATE TABLE user_shares (
    user_id UUID NOT NULL,
    campaign_id UUID NOT NULL REFERENCES campaigns(id),
    granted_at TIMESTAMPTZ DEFAULT NOW(),
    PRIMARY KEY (user_id, campaign_id)
);

-- Create validation function
CREATE OR REPLACE FUNCTION validate_membership_uniqueness()
RETURNS TRIGGER AS $$
BEGIN
  IF EXISTS (
    SELECT 1 FROM user_shares us
    JOIN campaigns c ON us.campaign_id = c.id
    WHERE us.user_id = NEW.user_id
      AND c.type = 'cooperative_membership'
      AND us.campaign_id != NEW.campaign_id
  ) THEN
    RAISE EXCEPTION 'User already has a membership share';
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Create constraint trigger
CREATE CONSTRAINT TRIGGER membership_uniqueness_trigger
AFTER INSERT OR UPDATE ON user_shares
FOR EACH ROW EXECUTE FUNCTION validate_membership_uniqueness();