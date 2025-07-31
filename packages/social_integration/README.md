# Social Integration Crate

This crate provides functionality for integrating social features across CPC apps, including unified feeds, cross-posting, and social tipping.

## Features

- Unified post model for cross-app social content
- Social event tracking
- Tipping system for social interactions
- Unified feed generation
- Integration with Allat (Reddit-style forums) and Yapper (Twitter-style microblogging)

## Modules

### Domain

The domain layer contains the core models and concepts:

- `post`: Unified post model and related types
- `social_event`: Social events for tracking user interactions

### Application

The application layer contains the business logic:

- `social_integration_service`: Main service for integrating social features
- `tip_service`: Service for handling social tipping
- `feed_service`: Service for generating unified feeds

### Infrastructure

The infrastructure layer contains implementations:

- `repositories`: Storage implementations for social data
- `clients`: Clients for integrating with social apps

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
social_integration = { path = "../packages/social_integration" }
```

### Creating a Unified Post

```rust
use social_integration::domain::post::{UnifiedPost, AppSource, PostMetadata, EngagementMetrics, PrivacySettings};
use uuid::Uuid;
use chrono::Utc;

let post_id = Uuid::new_v4();
let author_id = Uuid::new_v4();
let content = "Hello, world!".to_string();

let metadata = PostMetadata {
    created_at: Utc::now(),
    updated_at: Utc::now(),
    engagement: EngagementMetrics::new(),
    media_attachments: Vec::new(),
    hashtags: Vec::new(),
    privacy: PrivacySettings {
        is_public: true,
        allowed_viewers: Vec::new(),
        shareable: true,
    },
};

let post = UnifiedPost::new(
    AppSource::Allat,
    post_id,
    author_id,
    content,
    metadata,
);
```

### Sending Tips Between Users

```rust
use social_integration::domain::tip_transaction::TipTransaction;
use social_integration::application::tip_service::TipService;
use cpc_wallet::domain::primitives::{Money, Currency};
use uuid::Uuid;
use chrono::Utc;
use rust_decimal_macros::dec;

// Create a tip service with a wallet service
let tip_service = TipService::new(
    Box::new(wallet_service),
    Box::new(tip_transaction_repository),
);

// Send a tip from one user to another
let sender_id = Uuid::new_v4();
let recipient_id = Uuid::new_v4();
let amount = Money::new(dec!(10), Currency::Dabloons);
let note = Some("Thanks for the great post!".to_string());

tip_service.send_tip(sender_id, recipient_id, amount, note).await?;
```

## Testing

Run tests with:

```bash
cargo test
```

## License

This crate is part of the CPC software ecosystem and is licensed under the CPC license.