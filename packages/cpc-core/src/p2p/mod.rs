pub mod network;
pub mod storage;
pub mod sync;
pub mod events;
pub mod reconciliation;
pub mod media_storage;

// Platform-specific modules
#[cfg(target_os = "android")]
pub mod android;

// Re-export key components
pub use network::NetworkHandler;
pub use storage::MetadataStore;
pub use sync::SynchronizationManager;
pub use events::{EventSystem, P2PEvent};
pub use media_storage::{P2PMediaNetwork, P2PMediaStorage, ContentAddress, ContentStats};