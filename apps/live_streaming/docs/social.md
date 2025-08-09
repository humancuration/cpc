# Social Features

This document describes the social features implemented in the Live Streaming module.

## Overview

The social features enable users to connect, interact, and build communities around live streaming content. These features integrate with the broader CPC social ecosystem.

## Architecture

The social system consists of:

1. **Following System**: Track favorite channels
2. **Subscription System**: Support content creators financially
3. **Notifications**: Stay updated on social activities
4. **Integration**: Connect with CPC's shared social packages

## Implementation

### Following

Location: `src/social/follow.rs`

The following system allows users to track their favorite channels:

- Follow/unfollow channels
- View followed channels
- Receive notifications about live streams

Key methods:
- `follow_channel()`: Follow a channel
- `unfollow_channel()`: Unfollow a channel
- `get_following_channels()`: Get list of followed channels
- `is_following()`: Check if following a specific channel

### Subscriptions

Location: `src/social/subscription.rs`

The subscription system enables financial support for content creators:

- Tiered subscription model
- Recurring payments
- Exclusive benefits for subscribers

Key components:
- `Subscription`: Represents a user's subscription to a channel
- `SubscriptionTier`: Defines subscription levels and benefits
- `SubscriptionService`: Manages subscription operations

Key methods:
- `create_tier()`: Create a subscription tier
- `subscribe_user()`: Subscribe a user to a channel
- `cancel_subscription()`: Cancel a subscription
- `get_user_subscriptions()`: Get user's subscriptions

## Integration with CPC Social Packages

### Social Integration

The module integrates with `social_integration` for:

- Following relationships
- Social graph management
- Cross-application social features

### Notifications

The module integrates with `cpc-notification-core` for:

- Stream start notifications
- New follower notifications
- Subscription renewal reminders

## Features

### Follow System

Users can follow channels to:

- Receive notifications when channels go live
- See streams in their "Following" feed
- Support channels without financial commitment

Implementation details:
- Follow relationships stored in database
- Integration with CPC's social graph
- Privacy controls for following status

### Subscription System

Users can subscribe to channels for:

- Financial support of content creators
- Exclusive benefits (emotes, badges, etc.)
- Ad-free viewing experience

Subscription tiers:
- Tier 1: Basic benefits
- Tier 2: Enhanced benefits
- Tier 3: Premium benefits

Benefits system:
- Subscriber-only emotes
- Custom badges
- Ad-free viewing
- Higher quality streams
- Subscriber-only chat

### Notifications

Social activities trigger notifications:

- When followed channels go live
- When someone follows a user's channel
- Subscription renewal reminders
- Special events or announcements

Notification types:
- Push notifications (mobile)
- In-app notifications
- Email notifications (optional)

## API

### Following

```rust
// Follow a channel
follow_service.follow_channel(follower_id, channel_owner_id).await?;

// Unfollow a channel
follow_service.unfollow_channel(follower_id, channel_owner_id).await?;

// Check if following
let is_following = follow_service.is_following(follower_id, channel_owner_id).await?;

// Get following count
let follower_count = follow_service.get_follower_count(channel_owner_id).await?;
```

### Subscriptions

```rust
// Create subscription tier
let tier = subscription_service.create_tier(
    channel_id,
    name,
    description,
    price_cents,
    level,
    benefits
);

// Subscribe user
let subscription = subscription_service.subscribe_user(
    subscriber_id,
    channel_owner_id,
    tier_id,
    is_gift,
    gifted_by
)?;

// Cancel subscription
subscription_service.cancel_subscription(subscription_id)?;

// Get user subscriptions
let subscriptions = subscription_service.get_user_subscriptions(subscriber_id);
```

## Data Models

### Follow

```rust
pub struct Follow {
    pub follower_id: Uuid,
    pub followed_id: Uuid,
    pub followed_at: DateTime<Utc>,
}
```

### Subscription

```rust
pub struct Subscription {
    pub id: Uuid,
    pub subscriber_id: Uuid,
    pub channel_owner_id: Uuid,
    pub tier_id: Uuid,
    pub subscribed_at: DateTime<Utc>,
    pub renews_at: DateTime<Utc>,
    pub is_active: bool,
    pub is_gift: bool,
    pub gifted_by: Option<Uuid>,
}
```

### Subscription Tier

```rust
pub struct SubscriptionTier {
    pub id: Uuid,
    pub channel_id: Uuid,
    pub name: String,
    pub description: String,
    pub price_cents: u32,
    pub level: u8,
    pub benefits: SubscriptionBenefits,
}
```

### Subscription Benefits

```rust
pub struct SubscriptionBenefits {
    pub subscriber_emotes: bool,
    pub ad_free: bool,
    pub higher_quality: bool,
    pub custom_badges: bool,
    pub subscriber_chat: bool,
    pub special_badge: bool,
    pub custom_benefits: Vec<String>,
}
```

## Testing

Social features can be tested using:

- Unit tests for individual components
- Integration tests with database
- End-to-end tests for user flows
- Performance tests for scalability

## Privacy and Security

### Data Protection

- Follow relationships are private by default
- Subscription information is private
- Users control their privacy settings

### Consent Management

- Integration with CPC's consent management system
- Clear opt-in for notifications
- Granular privacy controls

## Performance Considerations

### Scalability

- Efficient database queries for follow relationships
- Caching of frequently accessed data
- Asynchronous processing for notifications

### Real-time Updates

- WebSocket connections for live updates
- Efficient event broadcasting
- Minimal latency for social interactions

## Future Enhancements

Planned improvements:

- Enhanced analytics for content creators
- Community features (groups, forums)
- Integration with CPC's skill development system
- Advanced moderation tools
- Social streaming (watch parties)
- Integration with CPC's cooperative fundraising system