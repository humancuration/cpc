pub mod bridge;
pub mod p2p;
pub mod bevy;
pub mod native;
pub mod thumbnail_ffi;
pub mod error;
pub mod auth;
pub mod models;
pub mod utils;  // Add datetime utilities module

#[cfg(target_os = "android")]
mod android_lifecycle;

// Re-export android lifecycle functions
#[cfg(target_os = "android")]
pub use android_lifecycle::*;