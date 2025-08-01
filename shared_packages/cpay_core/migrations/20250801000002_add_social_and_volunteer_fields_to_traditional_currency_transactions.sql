-- Add social and volunteerism fields to traditional_currency_transactions table
ALTER TABLE traditional_currency_transactions 
ADD COLUMN IF NOT EXISTS social_post_id UUID,
ADD COLUMN IF NOT EXISTS volunteer_hours DECIMAL(10, 2);

-- Add indexes for the new columns
CREATE INDEX IF NOT EXISTS idx_traditional_currency_transactions_social_post_id ON traditional_currency_transactions(social_post_id);
CREATE INDEX IF NOT EXISTS idx_traditional_currency_transactions_volunteer_hours ON traditional_currency_transactions(volunteer_hours);