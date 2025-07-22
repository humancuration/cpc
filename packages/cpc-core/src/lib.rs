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

#[cfg(target_os = "android")]
mod android_lifecycle;

#[cfg(target_os = "ios")]
mod ios_lifecycle;

#[cfg(target_os = "macos")]
mod macos_lifecycle;

#[cfg(target_os = "windows")]
mod windows_lifecycle;

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