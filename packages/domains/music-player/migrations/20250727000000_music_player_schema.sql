-- Tracks table for storing audio metadata
CREATE TABLE tracks (
    id UUID PRIMARY KEY,
    artist_id UUID NOT NULL REFERENCES cooperative_members(id),
    album_id UUID REFERENCES albums(id),
    title VARCHAR(255) NOT NULL,
    duration_ms BIGINT NOT NULL CHECK (duration_ms > 0),
    media_cid VARCHAR(100) NOT NULL,
    waveform_data_cid VARCHAR(100),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Timestamped comments on tracks
CREATE TABLE track_comments (
    id UUID PRIMARY KEY,
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    timestamp_ms BIGINT NOT NULL CHECK (timestamp_ms >= 0),
    content TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_track_comments_track ON track_comments(track_id);
CREATE INDEX idx_track_comments_timestamp ON track_comments(track_id, timestamp_ms);

-- Track likes (many-to-many relationship)
CREATE TABLE track_likes (
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (track_id, user_id)
);

-- Artist follows (many-to-many relationship)
CREATE TABLE artist_follows (
    artist_id UUID NOT NULL REFERENCES cooperative_members(id),
    follower_id UUID NOT NULL REFERENCES cooperative_members(id),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (artist_id, follower_id)
);

-- User playlists
CREATE TABLE playlists (
    id UUID PRIMARY KEY,
    owner_id UUID NOT NULL REFERENCES cooperative_members(id),
    title VARCHAR(100) NOT NULL,
    description TEXT,
    is_public BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Tracks within playlists
CREATE TABLE playlist_tracks (
    playlist_id UUID NOT NULL REFERENCES playlists(id),
    track_id UUID NOT NULL REFERENCES tracks(id),
    position INTEGER NOT NULL,
    PRIMARY KEY (playlist_id, track_id)
);

-- Visualizer presets
CREATE TABLE visualizer_presets (
    id UUID PRIMARY KEY,
    name VARCHAR(50) NOT NULL,
    config JSONB NOT NULL,
    is_default BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Offline download management
CREATE TABLE offline_downloads (
    track_id UUID NOT NULL REFERENCES tracks(id),
    user_id UUID NOT NULL REFERENCES cooperative_members(id),
    download_manifest JSONB NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ,
    PRIMARY KEY (track_id, user_id)
);

-- Enable pgcrypto for UUID generation if not already enabled
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";