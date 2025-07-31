# CPC OAuth2 Crate

A reusable authentication crate for CPC apps supporting major providers: TikTok, Google, Facebook, Twitter, and more.
Integrates with consent_manager for permission handling and supports web/Tauri flows.

## Features

- OAuth2 authentication with major providers
- Encrypted token storage using RustCrypto (AES-GCM)
- Integration with consent_manager for scope permissions
- Support for both web redirect and native deep link flows
- gRPC service and REST API endpoints

## Supported Providers

- TikTok
- Google
- Facebook
- Twitter
- YouTube
- WhatsApp
- Instagram
- Threads
- WeChat
- Messenger
- Snapchat
- Discord
- Twitch
- Gmail

## Architecture

The crate follows a hexagonal architecture with clear separation of concerns:

### Domain Layer
- `OAuthProvider`: Enum of supported providers
- `OAuthToken`: Encrypted token representation
- `OAuthProfile`: User profile information
- `AuthError`: Error types
- `AuthConfig`: Configuration structures

### Application Layer
- `AuthService`: Orchestrates authentication flows
- `TokenService`: Manages token storage/retrieval

### Infrastructure Layer
- Provider adapters for each OAuth provider
- Storage adapters (Sled, Postgres)
- API interfaces (gRPC, REST)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
cpc_oauth2 = { path = "../packages/core/oauth2" }
```

## Usage

### Basic Setup

```rust
use cpc_oauth2::{
    domain::{AuthConfig, ProviderConfig, OAuthProvider},
    application::{AuthService, TokenService},
    infrastructure::{
        storage::sled_storage::SledStorageAdapter,
        providers::tiktok::TikTokAdapter,
    },
};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

// Create storage adapter
let db = sled::open("oauth_db")?;
let storage_adapter = Arc::new(SledStorageAdapter::new(db));

// Create auth configuration
let encryption_key = AuthConfig::generate_encryption_key();
let mut auth_config = AuthConfig::new(
    "http://localhost:3000/callback".to_string(),
    encryption_key,
);

// Add provider configuration
let tiktok_config = ProviderConfig {
    client_id: "your_client_id".to_string(),
    client_secret: "your_client_secret".to_string(),
    auth_url: "https://open.tiktokapis.com/v2/oauth/authorize/".to_string(),
    token_url: "https://open.tiktokapis.com/v2/oauth/token/".to_string(),
    redirect_uris: vec!["http://localhost:3000/callback".to_string()],
    default_scopes: vec!["user.info.basic".to_string()],
};

auth_config.set_provider_config(OAuthProvider::TikTok, tiktok_config);

// Create provider adapters
let mut providers: HashMap<OAuthProvider, Arc<dyn ProviderAdapter>> = HashMap::new();
let tiktok_adapter = TikTokAdapter::new(
    "your_client_id".to_string(),
    "your_client_secret".to_string(),
    "http://localhost:3000/callback".to_string(),
)?;
providers.insert(OAuthProvider::TikTok, Arc::new(tiktok_adapter));

// Create services
let token_service = Arc::new(TokenService::new(storage_adapter, auth_config.clone()));
let auth_service = Arc::new(AuthService::new(providers, token_service, auth_config));
```

### Starting Authentication Flow

```rust
let user_id = Uuid::new_v4();
let auth_request = auth_service.start_auth(user_id, OAuthProvider::TikTok, None).await?;
println!("Redirect user to: {}", auth_request.auth_url);
```

### Handling Callback

```rust
let (user_id, token, profile) = auth_service.handle_callback(code, state).await?;
```

## Configuration

The crate can be configured with different features:

```toml
[dependencies.cpc_oauth2]
path = "../packages/core/oauth2"
features = ["sled_storage", "tiktok", "google"]
```

Available features:
- `sled_storage`: Enable Sled storage adapter
- `postgres_storage`: Enable Postgres storage adapter
- `tiktok`: Enable TikTok provider
- `google`: Enable Google provider
- `facebook`: Enable Facebook provider
- `twitter`: Enable Twitter provider

## Security

- Tokens are encrypted at rest using AES-GCM
- CSRF protection via state parameters
- Secure random generation for encryption keys
- Integration with consent_manager for permission handling

## Testing

Run tests with:

```bash
cargo test
```

## License

This crate is part of the CPC software ecosystem and is licensed under the CPC license.