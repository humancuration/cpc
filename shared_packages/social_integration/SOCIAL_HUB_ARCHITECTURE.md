# Social Hub Architecture

## Overview
The Social Hub app integrates social features with the CPC ecosystem, focusing on connectivity and voluntary sharing rather than incentivized content creation. Key principles:
- No automatic rewards for content creation to prevent spamming
- Tipping as the primary reward mechanism (DEPRECATED - moved to wallet package)
- Flexible currency support
- Social sharing via OAuth

## Architecture Diagram
```
┌──────────────────────┐       ┌──────────────────────┐
│      UI Layer        │       │   Application Layer  │
│ (Tauri + Yew)        │◄─────►│ (Services)           │
│ - Feed Component     │       │ - FeedService        │
│ - Profile Component  │       │ - ProfileService     │
│ - Tip Component      │       │ - TipService (DEPRECATED - moved to wallet package) │
└──────────────────────┘       └──────────────────────┘
       ▲                               ▲
       │                               │
       ▼                               ▼
┌──────────────────────┐       ┌──────────────────────┐
│   Infrastructure     │       │      Domain           │
│   Layer              │       │   (Core Logic)       │
│ - Post Repositories  │       │ - UnifiedPost        │
│ - Following Repos    │       │ - UserFollowing      │
│ - Tip Repository     │       │ - TipTransaction (DEPRECATED - moved to wallet package) │
└──────────────────────┘       └──────────────────────┘
       ▲                               ▲
       │                               │
       ▼                               ▼
┌──────────────────────┐       ┌──────────────────────┐
│    External          │       │    Core Wallet       │
│   Integrations       │       │   (Dabloons)         │
│ - OAuth Providers    │       │ - Transactions       │
│ - Social Platforms   │       │ - Balances           │
└──────────────────────┘       └──────────────────────┘
```

## Key Components

### UI Layer
- **Feed View**: Shows posts from followed users/sources
- **Profile View**: User profiles with following/followers
- **Tip Button**: One-click tipping on posts (DEPRECATED - moved to wallet package)
- **Share Button**: Share to external platforms

### Application Services
- `FeedService`: Aggregates posts from multiple sources
- `ProfileService`: Manages user relationships
- `TipService`: Handles tipping transactions (DEPRECATED - moved to wallet package)
- `ShareService`: Manages social sharing via OAuth

### Domain Models
- `UnifiedPost`: Aggregated post content
- `UserFollowing`: Following relationships
- `TipTransaction`: Record of voluntary tips (DEPRECATED - moved to wallet package)
- `SocialShare`: Record of shared content

### Infrastructure
- `PostgresUnifiedPostRepository`: Post storage
- `PostgresUserFollowingRepository`: Relationship storage
- `PostgresTipRepository`: Tip transaction storage (DEPRECATED - moved to wallet package)
- `OAuthClient`: Social platform integrations

### Wallet Integration
- Uses `cpc_wallet` crate for transactions
- Supports multiple currencies (Dabloons + traditional)
- Tipping deducts from sender's wallet, adds to recipient's (DEPRECATED - moved to wallet package)

## Data Flow
1. User views feed → `FeedService` fetches posts
2. User tips a post → `TipService` creates transaction via wallet (DEPRECATED - moved to wallet package)
3. User follows another → `ProfileService` updates relationships
4. User shares post → `ShareService` uses OAuth to post externally

## Testing Strategy
- Unit tests for domain logic
- Integration tests for repository implementations
- E2E tests for UI workflows
- Mock wallet service for testing transactions (DEPRECATED - moved to wallet package)

## Reward Philosophy
- **No automatic rewards** for content creation
- **Tipping only** when users voluntarily reward content (DEPRECATED - moved to wallet package)
- Prevents spamming by removing financial incentives for low-quality content