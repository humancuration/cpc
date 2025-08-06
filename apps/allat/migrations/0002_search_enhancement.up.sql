-- Add search vector columns
ALTER TABLE posts ADD COLUMN IF NOT EXISTS search_vector tsvector;
ALTER TABLE communities ADD COLUMN IF NOT EXISTS search_vector tsvector;

-- Populate initial search vectors
UPDATE posts SET search_vector = 
    setweight(to_tsvector('english', title), 'A') || 
    setweight(to_tsvector('english', content), 'B');

UPDATE communities SET search_vector = 
    setweight(to_tsvector('english', name), 'A') || 
    setweight(to_tsvector('english', description), 'B');

-- Create GIN indexes for efficient searching
CREATE INDEX IF NOT EXISTS idx_posts_search ON posts USING GIN(search_vector);
CREATE INDEX IF NOT EXISTS idx_communities_search ON communities USING GIN(search_vector);

-- Create functions to update search vectors
CREATE OR REPLACE FUNCTION update_post_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', COALESCE(NEW.title, '')), 'A') || 
        setweight(to_tsvector('english', COALESCE(NEW.content, '')), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION update_community_search_vector() RETURNS trigger AS $$
BEGIN
    NEW.search_vector := 
        setweight(to_tsvector('english', COALESCE(NEW.name, '')), 'A') || 
        setweight(to_tsvector('english', COALESCE(NEW.description, '')), 'B');
    RETURN NEW;
END
$$ LANGUAGE plpgsql;

-- Create triggers to automatically update search vectors
CREATE TRIGGER IF NOT EXISTS update_post_search_vector_trigger
    BEFORE INSERT OR UPDATE ON posts
    FOR EACH ROW EXECUTE FUNCTION update_post_search_vector();

CREATE TRIGGER IF NOT EXISTS update_community_search_vector_trigger
    BEFORE INSERT OR UPDATE ON communities
    FOR EACH ROW EXECUTE FUNCTION update_community_search_vector();