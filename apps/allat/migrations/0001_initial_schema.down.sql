-- Drop indexes first
DROP INDEX IF EXISTS idx_votes_user_id;
DROP INDEX IF EXISTS idx_votes_post_id;
DROP INDEX IF EXISTS idx_media_assets_post_id;
DROP INDEX IF EXISTS idx_posts_created_at;
DROP INDEX IF EXISTS idx_posts_parent_id;
DROP INDEX IF EXISTS idx_posts_user_id;
DROP INDEX IF EXISTS idx_posts_community_id;

-- Drop tables in reverse order of creation
DROP TABLE IF EXISTS votes;
DROP TABLE IF EXISTS media_assets;
DROP TABLE IF EXISTS posts;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS communities;