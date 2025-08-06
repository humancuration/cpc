-- Drop triggers
DROP TRIGGER IF EXISTS update_post_search_vector_trigger ON posts;
DROP TRIGGER IF EXISTS update_community_search_vector_trigger ON communities;

-- Drop functions
DROP FUNCTION IF EXISTS update_post_search_vector();
DROP FUNCTION IF EXISTS update_community_search_vector();

-- Drop indexes
DROP INDEX IF EXISTS idx_posts_search;
DROP INDEX IF EXISTS idx_communities_search;

-- Drop columns
ALTER TABLE posts DROP COLUMN IF EXISTS search_vector;
ALTER TABLE communities DROP COLUMN IF EXISTS search_vector;