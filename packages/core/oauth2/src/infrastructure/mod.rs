//! Infrastructure layer for OAuth2 authentication
//! 
//! Contains provider adapters, storage implementations, and API interfaces.

pub mod providers;
pub mod storage;
pub mod api;

pub use storage::StorageAdapter;