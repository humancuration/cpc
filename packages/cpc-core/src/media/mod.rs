pub mod processor;
pub mod storage;
pub mod types;
pub mod thumbnail;
pub mod upload;
pub mod integration;
pub mod distribution;

pub use processor::MediaProcessor;
pub use storage::{MediaStorage, LocalStorage, P2PStorage, HybridStorage};
pub use types::*;
pub use thumbnail::ThumbnailGenerator;
pub use upload::{MediaUploader, StorageStats};
pub use integration::{MediaService, ProcessedMediaResult, MediaServiceConfig, DistributedMediaService, DistributedMediaResult};
pub use distribution::{MediaDistributionManager, DistributionNode, DistributionStats};