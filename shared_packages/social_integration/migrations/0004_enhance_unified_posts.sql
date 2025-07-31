ALTER TABLE unified_posts
ADD COLUMN original_id UUID,
ALTER COLUMN content TYPE TEXT,
ALTER COLUMN properties TYPE JSONB USING properties::jsonb;