-- Migration script for adding universal feed fields to social_activities table

-- Add new columns for universal feed
ALTER TABLE social_activities
    ADD COLUMN content_type VARCHAR(50) NOT NULL DEFAULT 'SocialPost',
    ADD COLUMN source_package VARCHAR(255) NOT NULL DEFAULT 'social_graph',
    ADD COLUMN visibility VARCHAR(20) NOT NULL DEFAULT 'Public';

-- Remove unused columns
ALTER TABLE social_activities DROP COLUMN activity_type;
ALTER TABLE social_activities DROP COLUMN target_id;
ALTER TABLE social_activities DROP COLUMN target_type;