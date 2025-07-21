pub mod bridge;
pub mod p2p;
pub mod bevy;
pub mod native;
pub mod thumbnail_ffi;

#[cfg(target_os = "android")]
mod android_lifecycle;

// Re-export android lifecycle functions
#[cfg(target_os = "android")]
pub use android_lifecycle::*;