-- Create tables for the live streaming module

-- Channels table
CREATE TABLE channels (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    owner_id UUID NOT NULL,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    profile_image_url TEXT,
    banner_image_url TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    settings JSONB NOT NULL DEFAULT '{}',
    stats JSONB NOT NULL DEFAULT '{}'
);

-- Create index on owner_id for faster lookups
CREATE INDEX idx_channels_owner_id ON channels(owner_id);

-- Create index on name for search
CREATE INDEX idx_channels_name ON channels(name);

-- Streams table
CREATE TABLE streams (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id UUID NOT NULL REFERENCES channels(id),
    stream_key VARCHAR(255) NOT NULL UNIQUE,
    title VARCHAR(255) NOT NULL,
    category VARCHAR(255),
    started_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMP WITH TIME ZONE,
    viewer_count INTEGER NOT NULL DEFAULT 0,
    metadata JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true
);

-- Create index on channel_id for faster lookups
CREATE INDEX idx_streams_channel_id ON streams(channel_id);

-- Create index on stream_key for faster lookups
CREATE INDEX idx_streams_stream_key ON streams(stream_key);

-- Create index on is_active for active stream queries
CREATE INDEX idx_streams_is_active ON streams(is_active);

-- Subscriptions table
CREATE TABLE subscriptions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    subscriber_id UUID NOT NULL,
    channel_owner_id UUID NOT NULL,
    tier_id UUID NOT NULL,
    subscribed_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    renews_at TIMESTAMP WITH TIME ZONE NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    is_gift BOOLEAN NOT NULL DEFAULT false,
    gifted_by UUID,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create index on subscriber_id for faster lookups
CREATE INDEX idx_subscriptions_subscriber_id ON subscriptions(subscriber_id);

-- Create index on channel_owner_id for faster lookups
CREATE INDEX idx_subscriptions_channel_owner_id ON subscriptions(channel_owner_id);

-- Create index on is_active for active subscription queries
CREATE INDEX idx_subscriptions_is_active ON subscriptions(is_active);

-- Subscription tiers table
CREATE TABLE subscription_tiers (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id UUID NOT NULL REFERENCES channels(id),
    name VARCHAR(255) NOT NULL,
    description TEXT,
    price_cents INTEGER NOT NULL,
    level INTEGER NOT NULL,
    benefits JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create index on channel_id for faster lookups
CREATE INDEX idx_subscription_tiers_channel_id ON subscription_tiers(channel_id);

-- Custom emotes table
CREATE TABLE custom_emotes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    channel_id UUID NOT NULL REFERENCES channels(id),
    name VARCHAR(255) NOT NULL,
    image_url TEXT NOT NULL,
    subscriber_only BOOLEAN NOT NULL DEFAULT false,
    tier_required INTEGER,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create index on channel_id for faster lookups
CREATE INDEX idx_custom_emotes_channel_id ON custom_emotes(channel_id);

-- Create index on name for faster lookups
CREATE INDEX idx_custom_emotes_name ON custom_emotes(name);

-- Media segments table for adaptive streaming
CREATE TABLE media_segments (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    stream_id UUID NOT NULL REFERENCES streams(id),
    sequence_number INTEGER NOT NULL,
    duration NUMERIC NOT NULL,
    location TEXT NOT NULL,
    is_keyframe BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
);

-- Create index on stream_id for faster lookups
CREATE INDEX idx_media_segments_stream_id ON media_segments(stream_id);

-- Create index on sequence_number for faster lookups
CREATE INDEX idx_media_segments_sequence_number ON media_segments(sequence_number);