# Social Apps Database Schema

## Overview
This document defines the database schema for the Allat and Yapper social applications, including their integrated features.

## Allat Database Schema

### Communities Table
```sql
CREATE TABLE communities (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    rules TEXT[],
    created_by UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_communities_name ON communities(name);
CREATE INDEX idx_communities_created_by ON communities(created_by);
```

### Community Moderators Table
```sql
CREATE TABLE community_moderators (
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    role VARCHAR(50) NOT NULL DEFAULT 'moderator',
    granted_by UUID NOT NULL REFERENCES users(id),
    granted_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (community_id, user_id)
);

CREATE INDEX idx_community_moderators_user ON community_moderators(user_id);
```

### Community Members Table
```sql
CREATE TABLE community_members (
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    user_id UUID NOT NULL REFERENCES users(id),
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    PRIMARY KEY (community_id, user_id)
);

CREATE INDEX idx_community_members_user ON community_members(user_id);
```

### Posts Table
```sql
CREATE TABLE posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title VARCHAR(300) NOT NULL,
    content TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id),
    community_id UUID NOT NULL REFERENCES communities(id) ON DELETE CASCADE,
    media_ids UUID[],
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_posts_community ON posts(community_id, created_at DESC);
CREATE INDEX idx_posts_author ON posts(author_id, created_at DESC);
CREATE INDEX idx_posts_created_at ON posts(created_at DESC);
```

### Comments Table
```sql
CREATE TABLE comments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id),
    post_id UUID NOT NULL REFERENCES posts(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_comments_post ON comments(post_id, created_at ASC);
CREATE INDEX idx_comments_author ON comments(author_id, created_at DESC);
CREATE INDEX idx_comments_parent ON comments(parent_id, created_at ASC);
```

### Votes Table
```sql
CREATE TABLE votes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    post_id UUID REFERENCES posts(id) ON DELETE CASCADE,
    comment_id UUID REFERENCES comments(id) ON DELETE CASCADE,
    direction SMALLINT NOT NULL CHECK (direction IN (-1, 1)), -- -1 for downvote, 1 for upvote
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Ensure either post_id or comment_id is set, but not both
    CONSTRAINT check_vote_target CHECK (
        (post_id IS NOT NULL AND comment_id IS NULL) OR 
        (post_id IS NULL AND comment_id IS NOT NULL)
    ),
    
    -- Ensure unique vote per user per target
    UNIQUE(user_id, post_id),
    UNIQUE(user_id, comment_id)
);

CREATE INDEX idx_votes_post ON votes(post_id, direction, created_at);
CREATE INDEX idx_votes_comment ON votes(comment_id, direction, created_at);
CREATE INDEX idx_votes_user ON votes(user_id);
```

### User Karma Table
```sql
CREATE TABLE user_karma (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    total_karma INTEGER NOT NULL DEFAULT 0,
    post_karma INTEGER NOT NULL DEFAULT 0,
    comment_karma INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_user_karma_total ON user_karma(total_karma DESC);
```

## Yapper Database Schema

### Yapper Posts Table
```sql
CREATE TABLE yapper_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    author_id UUID NOT NULL REFERENCES users(id),
    reply_to_id UUID REFERENCES yapper_posts(id),
    media_ids UUID[],
    hashtags VARCHAR(50)[],
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_yapper_posts_author ON yapper_posts(author_id, created_at DESC);
CREATE INDEX idx_yapper_posts_reply_to ON yapper_posts(reply_to_id);
CREATE INDEX idx_yapper_posts_created_at ON yapper_posts(created_at DESC);
CREATE INDEX idx_yapper_posts_hashtags ON yapper_posts USING GIN(hashtags);
```

### Likes Table
```sql
CREATE TABLE yapper_likes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    post_id UUID NOT NULL REFERENCES yapper_posts(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Ensure unique like per user per post
    UNIQUE(user_id, post_id)
);

CREATE INDEX idx_yapper_likes_post ON yapper_likes(post_id, created_at);
CREATE INDEX idx_yapper_likes_user ON yapper_likes(user_id, created_at);
```

### Shares Table
```sql
CREATE TABLE yapper_shares (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    post_id UUID NOT NULL REFERENCES yapper_posts(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_yapper_shares_post ON yapper_shares(post_id, created_at);
CREATE INDEX idx_yapper_shares_user ON yapper_shares(user_id, created_at);
```

### Follows Table
```sql
CREATE TABLE yapper_follows (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    follower_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    following_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    
    -- Ensure unique follow relationship
    UNIQUE(follower_id, following_id),
    
    -- Ensure users can't follow themselves
    CONSTRAINT check_not_self_follow CHECK (follower_id != following_id)
);

CREATE INDEX idx_yapper_follows_follower ON yapper_follows(follower_id);
CREATE INDEX idx_yapper_follows_following ON yapper_follows(following_id);
```

### Hashtag Trends Table
```sql
CREATE TABLE hashtag_trends (
    tag VARCHAR(50) PRIMARY KEY,
    count INTEGER NOT NULL DEFAULT 1,
    last_used TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    is_trending BOOLEAN NOT NULL DEFAULT false
);

CREATE INDEX idx_hashtag_trends_count ON hashtag_trends(count DESC);
CREATE INDEX idx_hashtag_trends_trending ON hashtag_trends(is_trending, count DESC);
```

## Cross-App Integration Schema

### Cross-Posts Table
```sql
CREATE TABLE cross_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    source_app VARCHAR(20) NOT NULL, -- 'allat' or 'yapper'
    source_id UUID NOT NULL,
    target_app VARCHAR(20) NOT NULL, -- 'allat' or 'yapper'
    target_id UUID NOT NULL,
    user_id UUID NOT NULL REFERENCES users(id),
    privacy_level VARCHAR(20) NOT NULL DEFAULT 'public', -- 'public', 'followers', 'private'
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_cross_posts_source ON cross_posts(source_app, source_id);
CREATE INDEX idx_cross_posts_target ON cross_posts(target_app, target_id);
CREATE INDEX idx_cross_posts_user ON cross_posts(user_id, created_at DESC);
```

### Unified User Profiles Table
```sql
CREATE TABLE unified_user_profiles (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    display_name VARCHAR(100),
    bio TEXT,
    avatar_url TEXT,
    allat_karma INTEGER NOT NULL DEFAULT 0,
    yapper_followers INTEGER NOT NULL DEFAULT 0,
    total_posts INTEGER NOT NULL DEFAULT 0,
    total_comments INTEGER NOT NULL DEFAULT 0,
    cross_posts INTEGER NOT NULL DEFAULT 0,
    last_updated TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_unified_profiles_allat_karma ON unified_user_profiles(allat_karma DESC);
CREATE INDEX idx_unified_profiles_yapper_followers ON unified_user_profiles(yapper_followers DESC);
```

### Unified Feed Cache Table
```sql
CREATE TABLE unified_feed_cache (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    post_id UUID NOT NULL,
    post_type VARCHAR(20) NOT NULL, -- 'allat_post' or 'yapper_post'
    source_app VARCHAR(20) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    sort_key TIMESTAMP WITH TIME ZONE NOT NULL
);

CREATE INDEX idx_unified_feed_user ON unified_feed_cache(user_id, sort_key DESC);
CREATE INDEX idx_unified_feed_post ON unified_feed_cache(post_id, post_type);
```

## Views for Analytics

### Community Activity View
```sql
CREATE VIEW community_activity AS
SELECT 
    c.id AS community_id,
    c.name AS community_name,
    COUNT(p.id) AS total_posts,
    COUNT(DISTINCT p.author_id) AS active_users,
    MAX(p.created_at) AS last_activity,
    AVG(p.created_at::date - c.created_at::date) AS avg_days_since_creation
FROM communities c
LEFT JOIN posts p ON p.community_id = c.id
GROUP BY c.id, c.name;
```

### User Engagement View
```sql
CREATE VIEW user_engagement AS
SELECT 
    u.id AS user_id,
    u.username,
    COALESCE(uk.total_karma, 0) AS allat_karma,
    COALESCE(yf.followers_count, 0) AS yapper_followers,
    COALESCE(ya.total_posts, 0) AS yapper_posts,
    COALESCE(aa.total_posts, 0) AS allat_posts,
    (COALESCE(ya.total_posts, 0) + COALESCE(aa.total_posts, 0)) AS total_posts,
    COALESCE(cp.cross_post_count, 0) AS cross_posts
FROM users u
LEFT JOIN user_karma uk ON uk.user_id = u.id
LEFT JOIN (
    SELECT following_id AS user_id, COUNT(*) AS followers_count
    FROM yapper_follows
    GROUP BY following_id
) yf ON yf.user_id = u.id
LEFT JOIN (
    SELECT author_id AS user_id, COUNT(*) AS total_posts
    FROM yapper_posts
    GROUP BY author_id
) ya ON ya.user_id = u.id
LEFT JOIN (
    SELECT author_id AS user_id, COUNT(*) AS total_posts
    FROM posts
    GROUP BY author_id
) aa ON aa.user_id = u.id
LEFT JOIN (
    SELECT user_id, COUNT(*) AS cross_post_count
    FROM cross_posts
    GROUP BY user_id
) cp ON cp.user_id = u.id;
```

## Indexes for Performance

### Composite Indexes
```sql
-- For efficient feed generation
CREATE INDEX idx_posts_community_created ON posts(community_id, created_at DESC);
CREATE INDEX idx_yapper_posts_author_created ON yapper_posts(author_id, created_at DESC);
CREATE INDEX idx_comments_post_created ON comments(post_id, created_at ASC);

-- For efficient voting queries
CREATE INDEX idx_votes_post_direction ON votes(post_id, direction);
CREATE INDEX idx_votes_comment_direction ON votes(comment_id, direction);

-- For efficient search
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_communities_name_trgm ON communities USING GIN (name gin_trgm_ops);
```

## Security Considerations

### Row Level Security Policies
```sql
-- Example policy for posts (to be implemented based on privacy settings)
-- ALTER TABLE posts ENABLE ROW LEVEL SECURITY;
-- CREATE POLICY posts_viewable_by_community_members ON posts
--     FOR SELECT
--     USING (community_id IN (
--         SELECT community_id FROM community_members WHERE user_id = CURRENT_USER_ID()
--     ));
```

### Data Encryption
- All sensitive data should be encrypted at rest
- Media assets should be stored with appropriate access controls
- User sessions should be properly secured

## TODO
- [ ] Add audit trails for all modifications
- [ ] Implement data retention policies
- [ ] Add partitioning for large tables
- [ ] Implement materialized views for complex analytics