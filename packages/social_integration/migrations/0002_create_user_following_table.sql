CREATE TABLE user_following (
    follower_id UUID NOT NULL,
    followed_id UUID NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (follower_id, followed_id)
);
CREATE INDEX idx_user_following_follower_id ON user_following (follower_id);
CREATE INDEX idx_user_following_followed_id ON user_following (followed_id);