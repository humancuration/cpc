# Domain Models

This document describes the core domain models for the Live Streaming module.

## Channel

A channel represents a content creator's streaming presence on the platform.

### Properties

- `id`: Unique identifier for the channel
- `owner_id`: User ID of the channel owner
- `name`: Channel display name
- `description`: Channel description
- `profile_image_url`: URL to the channel's profile image
- `banner_image_url`: URL to the channel's banner image
- `created_at`: When the channel was created
- `updated_at`: When the channel was last updated
- `settings`: Channel settings (notifications, content preferences, etc.)
- `stats`: Channel statistics (follower count, view count, etc.)

### Methods

- `new(owner_id, name, description)`: Create a new channel
- `update_info(name, description)`: Update channel information
- `update_settings(settings)`: Update channel settings
- `add_emote(emote)`: Add a custom emote to the channel
- `remove_emote(emote_id)`: Remove a custom emote from the channel
- `update_stats(stats)`: Update channel statistics

## Stream

A stream represents a live broadcast from a channel.

### Properties

- `id`: Unique identifier for the stream
- `stream_key`: Key used for broadcasting
- `channel_id`: ID of the channel this stream belongs to
- `title`: Stream title
- `category`: Stream category/game
- `started_at`: When the stream started
- `viewer_count`: Current number of viewers
- `metadata`: Stream metadata (resolution, bitrate, etc.)

### Methods

- `new(channel_id, stream_key, title, category, metadata)`: Create a new stream

## Subscription

A subscription represents a user's paid subscription to a channel.

### Properties

- `id`: Unique identifier for the subscription
- `subscriber_id`: ID of the user who subscribed
- `channel_owner_id`: ID of the channel owner
- `tier_id`: ID of the subscription tier
- `subscribed_at`: When the subscription started
- `renews_at`: When the subscription renews
- `is_active`: Whether the subscription is active
- `is_gift`: Whether the subscription was gifted
- `gifted_by`: ID of the user who gifted the subscription (if applicable)

### Methods

- `new(subscriber_id, channel_owner_id, tier_id, is_gift, gifted_by)`: Create a new subscription

## SubscriptionTier

A subscription tier represents a level of subscription with specific benefits.

### Properties

- `id`: Unique identifier for the tier
- `channel_id`: ID of the channel this tier belongs to
- `name`: Tier name (e.g., "Tier 1", "Tier 2")
- `description`: Tier description
- `price_cents`: Monthly price in cents
- `level`: Tier level (1, 2, 3, etc.)
- `benefits`: Benefits included in this tier

### Methods

- `new(channel_id, name, description, price_cents, level, benefits)`: Create a new subscription tier

## CustomEmote

A custom emote is an image that can be used in chat by channel subscribers.

### Properties

- `id`: Unique identifier for the emote
- `channel_id`: ID of the channel this emote belongs to
- `name`: Emote name
- `image_url`: URL to the emote image
- `subscriber_only`: Whether the emote is subscriber-only
- `tier_required`: Tier required to use the emote (if subscriber-only)

### Methods

- `new(channel_id, name, image_url, subscriber_only, tier_required)`: Create a new custom emote