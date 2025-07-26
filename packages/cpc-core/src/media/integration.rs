use crate::media::*;
use anyhow::Result;
use std::path::PathBuf;
use std::sync::Arc;

/// High-level media service that integrates all media processing components
pub struct MediaService {
    uploader: MediaUploader,
    processor: MediaProcessor,
    thumbnail_generator: ThumbnailGenerator,
    storage: Arc<dyn MediaStorage>,
}

impl MediaService {
    /// Create a new media service with local storage
    pub fn new_with_local_storage(base_dir: PathBuf) -> Self {
        let storage = Arc::new(LocalStorage::new(base_dir.join("storage")));
        let uploader = MediaUploader::new(base_dir.join("media"));
        
        Self {
            uploader,
            processor: MediaProcessor::new(),
            thumbnail_generator: ThumbnailGenerator::new(),
            storage,
        }
    }

    /// Create a new media service with hybrid storage (local + P2P)
    pub fn new_with_hybrid_storage(base_dir: PathBuf) -> Self {
        let storage = Arc::new(HybridStorage::new(base_dir.join("storage")));
        let uploader = MediaUploader::new(base_dir.join("media"));
        
        Self {
            uploader,
            processor: MediaProcessor::new(),
            thumbnail_generator: ThumbnailGenerator::new(),
            storage,
        }
    }

    /// Initialize the media service
    pub async fn initialize(&self) -> Result<()> {
        self.uploader.initialize().await?;
        
        // Initialize storage if it's a local or hybrid storage
        if let Ok(local_storage) = self.storage.as_ref().downcast_ref::<LocalStorage>() {
            local_storage.initialize().await?;
        } else if let Ok(hybrid_storage) = self.storage.as_ref().downcast_ref::<HybridStorage>() {
            hybrid_storage.initialize().await?;
        }
        
        log::info!("Media service initialized successfully");
        Ok(())
    }

    /// Complete media upload and processing workflow
    pub async fn upload_and_process(
        &self,
        filename: &str,
        content_type: &str,
        data: &[u8],
        processing_config: Option<MediaProcessingConfig>,
        thumbnail_config: Option<ThumbnailConfig>,
    ) -> Result<ProcessedMediaResult> {
        // Validate upload
        crate::media::upload::validation::validate_upload(filename, data.len() as u64)?;
        
        // Upload file
        let upload = self.uploader.upload_from_bytes(filename, content_type, data).await?;
        log::info!("Uploaded media file: {} (ID: {})", filename, upload.id);
        
        // Process and generate thumbnail
        let upload = self.uploader.complete_upload(
            upload,
            processing_config,
            thumbnail_config,
        ).await?;
        
        // Store in persistent storage
        let storage_id = if let Some(processed_path) = &upload.processed_path {
            let processed_data = tokio::fs::read(processed_path).await?;
            self.storage.store(&processed_data, filename).await?
        } else {
            let original_data = tokio::fs::read(&upload.upload_path).await?;
            self.storage.store(&original_data, filename).await?
        };
        
        // Store thumbnail if available
        let thumbnail_storage_id = if let Some(thumbnail_path) = &upload.thumbnail_path {
            let thumbnail_data = tokio::fs::read(thumbnail_path).await?;
            let thumbnail_filename = format!("thumb_{}", filename);
            Some(self.storage.store(&thumbnail_data, &thumbnail_filename).await?)
        } else {
            None
        };
        
        Ok(ProcessedMediaResult {
            upload_id: upload.id,
            storage_id,
            thumbnail_storage_id,
            metadata: upload.metadata,
            processed_at: upload.processed_at,
        })
    }

    /// Retrieve media file from storage
    pub async fn get_media(&self, storage_id: &str) -> Result<Vec<u8>> {
        self.storage.retrieve(storage_id).await
    }

    /// Get media metadata
    pub async fn get_media_metadata(&self, storage_id: &str) -> Result<Option<MediaMetadata>> {
        self.storage.get_metadata(storage_id).await
    }

    /// Delete media from storage
    pub async fn delete_media(&self, storage_id: &str) -> Result<()> {
        self.storage.delete(storage_id).await
    }

    /// Generate additional thumbnails for existing media
    pub async fn generate_additional_thumbnails(
        &self,
        storage_id: &str,
        sizes: &[(u32, u32)],
    ) -> Result<Vec<String>> {
        // Retrieve original media
        let media_data = self.storage.retrieve(storage_id).await?;
        
        // Create temporary file
        let temp_dir = std::env::temp_dir();
        let temp_file = temp_dir.join(format!("temp_media_{}", uuid::Uuid::new_v4()));
        tokio::fs::write(&temp_file, &media_data).await?;
        
        // Generate thumbnails
        let thumbnail_paths = self.thumbnail_generator.generate_multiple_thumbnails(
            &temp_file,
            &temp_dir,
            sizes,
        ).await?;
        
        // Store thumbnails and collect storage IDs
        let mut thumbnail_storage_ids = Vec::new();
        for (i, thumbnail_path) in thumbnail_paths.iter().enumerate() {
            let thumbnail_data = tokio::fs::read(thumbnail_path).await?;
            let thumbnail_filename = format!("thumb_{}_{}.png", storage_id, i);
            let thumbnail_storage_id = self.storage.store(&thumbnail_data, &thumbnail_filename).await?;
            thumbnail_storage_ids.push(thumbnail_storage_id);
            
            // Clean up temporary thumbnail file
            let _ = tokio::fs::remove_file(thumbnail_path).await;
        }
        
        // Clean up temporary media file
        let _ = tokio::fs::remove_file(&temp_file).await;
        
        Ok(thumbnail_storage_ids)
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<crate::media::upload::StorageStats> {
        self.uploader.get_storage_stats().await
    }

    /// Clean up old uploads
    pub async fn cleanup_old_uploads(&self, days: u64) -> Result<usize> {
        self.uploader.cleanup_old_uploads(days).await
    }

    /// Process existing file (not uploaded through this service)
    pub async fn process_existing_file(
        &self,
        file_path: &std::path::Path,
        config: Option<MediaProcessingConfig>,
    ) -> Result<ProcessingResult> {
        let output_path = file_path.with_extension("processed");
        let config = config.unwrap_or_default();
        
        self.processor.process_media(
            file_path.to_path_buf(),
            output_path,
            config,
        ).await
    }
}

/// Result of complete media processing
#[derive(Debug, Clone)]
pub struct ProcessedMediaResult {
    pub upload_id: uuid::Uuid,
    pub storage_id: String,
    pub thumbnail_storage_id: Option<String>,
    pub metadata: Option<MediaMetadata>,
    pub processed_at: Option<chrono::DateTime<chrono::Utc>>,
}

/// Media service configuration
#[derive(Debug, Clone)]
pub struct MediaServiceConfig {
    pub base_directory: PathBuf,
    pub use_p2p_storage: bool,
    pub default_processing_config: MediaProcessingConfig,
    pub default_thumbnail_config: ThumbnailConfig,
    pub cleanup_interval_days: u64,
}

impl Default for MediaServiceConfig {
    fn default() -> Self {
        Self {
            base_directory: PathBuf::from("./media"),
            use_p2p_storage: false,
            default_processing_config: MediaProcessingConfig::default(),
            default_thumbnail_config: ThumbnailConfig::default(),
            cleanup_interval_days: 30,
        }
    }
}

/// Utility functions for media service
pub mod utils {
    use super::*;
    
    /// Create media service from configuration
    pub fn create_media_service(config: MediaServiceConfig) -> MediaService {
        if config.use_p2p_storage {
            MediaService::new_with_hybrid_storage(config.base_directory)
        } else {
            MediaService::new_with_local_storage(config.base_directory)
        }
    }

    /// Create media service with distribution capabilities
    pub fn create_distributed_media_service(
        config: MediaServiceConfig,
        p2p_config: String,
    ) -> DistributedMediaService {
        DistributedMediaService::new(config, p2p_config)
    }
    
    /// Get supported media types
    pub fn get_supported_media_types() -> Vec<&'static str> {
        crate::media::upload::validation::ALLOWED_EXTENSIONS.to_vec()
    }
    
    /// Check if file type is supported
    pub fn is_supported_media_type(filename: &str) -> bool {
        crate::media::upload::validation::validate_file_extension(filename).is_ok()
    }
}
/
// Distributed media service with CDN-like capabilities
pub struct DistributedMediaService {
    media_service: MediaService,
    distribution_manager: crate::media::distribution::MediaDistributionManager,
}

impl DistributedMediaService {
    pub fn new(config: MediaServiceConfig, p2p_config: String) -> Self {
        let media_service = utils::create_media_service(config.clone());
        let distribution_manager = crate::media::distribution::MediaDistributionManager::new(
            p2p_config,
            std::time::Duration::from_secs(3600), // 1 hour cache TTL
            1024 * 1024 * 1024, // 1GB cache size
        );

        Self {
            media_service,
            distribution_manager,
        }
    }

    /// Initialize the distributed media service
    pub async fn initialize(&self) -> Result<()> {
        self.media_service.initialize().await?;
        self.distribution_manager.initialize().await?;
        log::info!("Distributed media service initialized");
        Ok(())
    }

    /// Upload and distribute content across the network
    pub async fn upload_and_distribute(
        &self,
        filename: &str,
        content_type: &str,
        data: &[u8],
        replication_factor: u32,
        processing_config: Option<MediaProcessingConfig>,
        thumbnail_config: Option<ThumbnailConfig>,
    ) -> Result<DistributedMediaResult> {
        // Process media using the regular media service
        let processed_result = self.media_service.upload_and_process(
            filename,
            content_type,
            data,
            processing_config,
            thumbnail_config,
        ).await?;

        // Distribute the processed content
        let distribution_id = self.distribution_manager.distribute_content(
            data,
            content_type,
            replication_factor,
        ).await?;

        Ok(DistributedMediaResult {
            processed_result,
            distribution_id,
            replication_factor,
        })
    }

    /// Retrieve content with intelligent routing
    pub async fn get_distributed_content(&self, distribution_id: &str) -> Result<Vec<u8>> {
        self.distribution_manager.retrieve_content(distribution_id).await
    }

    /// Get distribution statistics
    pub fn get_distribution_stats(&self) -> crate::media::distribution::DistributionStats {
        self.distribution_manager.get_distribution_stats()
    }

    /// Clean up cache and old content
    pub async fn cleanup(&self, max_age_days: u64) -> Result<CleanupStats> {
        let media_cleanup = self.media_service.cleanup_old_uploads(max_age_days).await?;
        let cache_cleanup = self.distribution_manager.cleanup_cache().await;

        Ok(CleanupStats {
            media_files_cleaned: media_cleanup,
            cache_entries_cleaned: cache_cleanup,
        })
    }

    /// Verify content integrity across the network
    pub async fn verify_content_integrity(
        &self,
        content_id: &str,
    ) -> Result<crate::media::distribution::verification::ContentIntegrityReport> {
        crate::media::distribution::verification::verify_content_integrity(
            &self.distribution_manager,
            content_id,
        ).await
    }
}

/// Result of distributed media processing
#[derive(Debug, Clone)]
pub struct DistributedMediaResult {
    pub processed_result: ProcessedMediaResult,
    pub distribution_id: String,
    pub replication_factor: u32,
}

/// Cleanup statistics
#[derive(Debug, Clone)]
pub struct CleanupStats {
    pub media_files_cleaned: usize,
    pub cache_entries_cleaned: usize,
}