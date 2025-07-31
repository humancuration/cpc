//! # CPC OAuth2 Crate
//!
//! A reusable authentication crate for CPC apps supporting major providers: TikTok, Google, Facebook, Twitter.
//! Integrates with consent_manager for permission handling and supports web/Tauri flows.
//!
//! ## Features
//! - OAuth2 authentication with major providers
//! - Encrypted token storage using RustCrypto (AES-GCM)
//! - Integration with consent_manager for scope permissions
//! - Support for both web redirect and native deep link flows
//! - gRPC service and REST API endpoints

pub mod domain;
pub mod application;
pub mod infrastructure;

// Re-export key types
pub use domain::{OAuthProvider, OAuthToken, OAuthProfile, AuthError, AuthConfig};
pub use application::{AuthService, TokenService};

// Re-export provider adapters
pub use infrastructure::providers::tiktok::TikTokAdapter;
pub use infrastructure::providers::google::GoogleAdapter;
pub use infrastructure::providers::facebook::FacebookAdapter;
#[cfg(feature = "twitter")]
pub use infrastructure::providers::twitter::TwitterAdapter;

// Re-export storage adapters
#[cfg(feature = "sled_storage")]
pub use infrastructure::storage::sled_storage::SledStorageAdapter;

#[cfg(feature = "postgres_storage")]
pub use infrastructure::storage::postgres_storage::PostgresStorageAdapter;