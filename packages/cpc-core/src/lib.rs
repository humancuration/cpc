pub mod bridge;
pub mod p2p;
pub mod bevy;
pub mod native;
pub mod thumbnail_ffi;
pub mod error;
pub mod auth;
pub mod models;
pub mod utils;  // Add datetime utilities module
pub mod texture_manifest; // Add texture manifest module
pub mod vision; // Image recognition functionality
pub mod invoicing; // Invoice management system
pub mod expenses; // Expense tracking system
pub mod asset_browser; // Asset Browser module
pub mod accounting; // Accounting and financial management
pub mod business; // Business intelligence and forecasting
pub mod impact; // Impact calculation and reporting
pub mod serialization; // Protobuf serialization for Android
pub mod product; // Product management module
pub mod product::extensions; // Extension traits for protobuf models

#[cfg(target_os = "android")]
mod android_lifecycle;

#[cfg(target_os = "ios")]
mod ios_lifecycle;

#[cfg(target_os = "macos")]
mod macos_lifecycle;

#[cfg(target_os = "windows")]
mod windows_lifecycle;

// Android FFI module for social features
#[cfg(target_os = "android")]
pub mod ffi {
    pub mod android;
}

// Re-export android lifecycle functions
#[cfg(target_os = "android")]
pub use android_lifecycle::*;

// Re-export ios lifecycle functions
#[cfg(target_os = "ios")]
pub use ios_lifecycle::*;

// Re-export macos lifecycle functions
#[cfg(target_os = "macos")]
pub use macos_lifecycle::*;

// Re-export windows lifecycle functions
#[cfg(target_os = "windows")]
pub use windows_lifecycle::*;

// Re-export texture manifest
pub use texture_manifest::TextureManifest;

// Re-export models
pub use models::{Proposal, FeedItem, SupplyChain};
pub use product::model::Product;

// Re-export serialization function
pub use serialization::to_protobuf;