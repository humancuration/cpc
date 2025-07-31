//! Infrastructure layer for social integration

pub mod repositories;
pub mod clients;

pub use repositories::InMemoryUnifiedPostRepository;
pub use clients::{AllatClient, YapperClient};