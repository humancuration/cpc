use crate::media::types::*;
use crate::media::processor::MediaProcessor;
use crate::media::thumbnail::ThumbnailGenerator;
use anyhow::Result;
use chrono::Utc;
use std::path::{Path, PathBuf};
use tokio::io::AsyncWriteExt;
use uuid::Uuid;

/// Media uploader that handles file uploads and processing
pub struct MediaUploader {
    upload_dir: PathBuf,
    processed_dir: PathBuf,
    thumbnail_dir: PathBuf,
    processor: MediaProcessor,
    thumbnail_generator: ThumbnailGenerator,
}

impl MediaUploader {
    pub fn new(base_dir: PathBuf) -> Self {
        let upload_dir = base_dir.join("uploads");
        let processed_dir = base_dir.join("processed");
        let thumbnail_dir = base_dir.join("thumbnails");
        
        Self {
            upload_dir,
            processed_dir,
            thumbnail_dir,
            processor: MediaProcessor::new(),
            thumbnail_generator: ThumbnailGenerator::new(),
        }
    }

    /// Initialize upload directories
    pub async fn initialize(&self) -> Result<()> {
        tokio::fs::create_dir_all(&self.upload_dir).await?;
        tokio::fs::create_dir_all(&self.processed_dir).await?;
        tokio::fs::create_dir_all(&self.thumbnail_dir).await?;
        
        log::info!("Media uploader initialized with directories:");
        log::info!("  Uploads: {:?}", self.upload_dir);
        log::info!("  Processed: {:?}", self.processed_dir);
        log::info!("  Thumbnails: {:?}", self.thumbnail_dir);
        
        Ok(())
    }

    /// Upload media file from bytes
    pub async fn upload_from_bytes(
        &self,
        filename: &str,
        content_type: &str,
        data: &[u8],
    ) -> Result<MediaUpload> {
        let upload_id = Uuid::new_v4();
        let file_extension = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("bin");
        
        let upload_filename = format!("{}_{}.{}", upload_id, filename, file_extension);
        let upload_path = self.upload_dir.join(&upload_filename);
        
        // Write file to upload directory
        let mut file = tokio::fs::File::create(&upload_path).await?;
        file.write_all(data).await?;
        file.flush().await?;
        
        log::info!("Uploaded file: {} ({} bytes)", filename, data.len());
        
        let upload = MediaUpload {
            id: upload_id,
            original_filename: filename.to_string(),
            content_type: content_type.to_string(),
            file_size: data.len() as u64,
            upload_path,
            processed_path: None,
            thumbnail_path: None,
            metadata: None,
            uploaded_at: Utc::now(),
            processed_at: None,
        };
        
        Ok(upload)
    }

    /// Upload media file from path
    pub async fn upload_from_path(
        &self,
        source_path: &Path,
        content_type: Option<&str>,
    ) -> Result<MediaUpload> {
        let data = tokio::fs::read(source_path).await?;
        let filename = source_path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown");
        
        let content_type = content_type.unwrap_or_else(|| {
            self.detect_content_type(source_path)
        });
        
        self.upload_from_bytes(filename, content_type, &data).await
    }

    /// Process uploaded media file
    pub async fn process_upload(
        &self,
        mut upload: MediaUpload,
        config: Option<MediaProcessingConfig>,
    ) -> Result<MediaUpload> {
        let config = config.unwrap_or_default();
        
        // Generate processed filename
        let processed_filename = format!(
            "processed_{}",
            upload.upload_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
        );
        
        let processed_path = self.processed_dir.join(processed_filename);
        
        // Process the media file
        let result = self.processor.process_media(
            upload.upload_path.clone(),
            processed_path.clone(),
            config,
        ).await?;
        
        if result.success {
            upload.processed_path = result.output_path;
            upload.metadata = result.metadata;
            upload.processed_at = Some(Utc::now());
            
            log::info!("Successfully processed upload: {}", upload.id);
        } else {
            log::error!("Failed to process upload {}: {:?}", 
                       upload.id, result.error_message);
            return Err(anyhow::anyhow!("Processing failed: {:?}", result.error_message));
        }
        
        Ok(upload)
    }

    /// Generate thumbnail for uploaded media
    pub async fn generate_thumbnail(
        &self,
        mut upload: MediaUpload,
        config: Option<ThumbnailConfig>,
    ) -> Result<MediaUpload> {
        let config = config.unwrap_or_default();
        
        // Use processed file if available, otherwise use original upload
        let source_path = upload.processed_path.as_ref()
            .unwrap_or(&upload.upload_path);
        
        // Generate thumbnail filename
        let thumbnail_filename = format!(
            "thumb_{}_{}x{}.png",
            upload.id,
            config.width,
            config.height
        );
        
        let thumbnail_path = self.thumbnail_dir.join(thumbnail_filename);
        
        // Generate thumbnail
        let result_path = self.thumbnail_generator.generate_thumbnail(
            source_path,
            &thumbnail_path,
            config,
        ).await?;
        
        upload.thumbnail_path = Some(result_path);
        
        log::info!("Generated thumbnail for upload: {}", upload.id);
        
        Ok(upload)
    }

    /// Complete upload processing (process + thumbnail)
    pub async fn complete_upload(
        &self,
        upload: MediaUpload,
        processing_config: Option<MediaProcessingConfig>,
        thumbnail_config: Option<ThumbnailConfig>,
    ) -> Result<MediaUpload> {
        // Process the media file
        let upload = self.process_upload(upload, processing_config).await?;
        
        // Generate thumbnail
        let upload = self.generate_thumbnail(upload, thumbnail_config).await?;
        
        Ok(upload)
    }

    /// Get upload by ID (placeholder - would typically use database)
    pub async fn get_upload(&self, upload_id: Uuid) -> Result<Option<MediaUpload>> {
        // TODO: Implement database lookup
        // For now, this is a placeholder
        Ok(None)
    }

    /// List all uploads (placeholder - would typically use database)
    pub async fn list_uploads(&self) -> Result<Vec<MediaUpload>> {
        // TODO: Implement database query
        // For now, return empty list
        Ok(Vec::new())
    }

    /// Delete upload and associated files
    pub async fn delete_upload(&self, upload: &MediaUpload) -> Result<()> {
        // Delete original upload file
        if upload.upload_path.exists() {
            tokio::fs::remove_file(&upload.upload_path).await?;
            log::info!("Deleted upload file: {:?}", upload.upload_path);
        }
        
        // Delete processed file
        if let Some(processed_path) = &upload.processed_path {
            if processed_path.exists() {
                tokio::fs::remove_file(processed_path).await?;
                log::info!("Deleted processed file: {:?}", processed_path);
            }
        }
        
        // Delete thumbnail
        if let Some(thumbnail_path) = &upload.thumbnail_path {
            if thumbnail_path.exists() {
                tokio::fs::remove_file(thumbnail_path).await?;
                log::info!("Deleted thumbnail file: {:?}", thumbnail_path);
            }
        }
        
        Ok(())
    }

    /// Detect content type from file extension
    fn detect_content_type(&self, path: &Path) -> &'static str {
        match path.extension().and_then(|e| e.to_str()) {
            // Video types
            Some("mp4") => "video/mp4",
            Some("webm") => "video/webm",
            Some("avi") => "video/x-msvideo",
            Some("mov") => "video/quicktime",
            Some("mkv") => "video/x-matroska",
            
            // Audio types
            Some("mp3") => "audio/mpeg",
            Some("wav") => "audio/wav",
            Some("ogg") => "audio/ogg",
            Some("opus") => "audio/opus",
            Some("flac") => "audio/flac",
            
            // Image types
            Some("jpg") | Some("jpeg") => "image/jpeg",
            Some("png") => "image/png",
            Some("gif") => "image/gif",
            Some("bmp") => "image/bmp",
            Some("webp") => "image/webp",
            
            _ => "application/octet-stream",
        }
    }

    /// Clean up old uploads (remove files older than specified days)
    pub async fn cleanup_old_uploads(&self, days: u64) -> Result<usize> {
        let cutoff = Utc::now() - chrono::Duration::days(days as i64);
        let mut cleaned_count = 0;
        
        // TODO: Implement database query to find old uploads
        // For now, this is a placeholder
        
        log::info!("Cleaned up {} old uploads", cleaned_count);
        
        Ok(cleaned_count)
    }

    /// Get storage statistics
    pub async fn get_storage_stats(&self) -> Result<StorageStats> {
        let upload_size = self.calculate_directory_size(&self.upload_dir).await?;
        let processed_size = self.calculate_directory_size(&self.processed_dir).await?;
        let thumbnail_size = self.calculate_directory_size(&self.thumbnail_dir).await?;
        
        Ok(StorageStats {
            total_uploads: 0, // TODO: Get from database
            upload_size,
            processed_size,
            thumbnail_size,
            total_size: upload_size + processed_size + thumbnail_size,
        })
    }

    /// Calculate total size of files in directory
    async fn calculate_directory_size(&self, dir: &Path) -> Result<u64> {
        let mut total_size = 0u64;
        
        if dir.exists() {
            let mut entries = tokio::fs::read_dir(dir).await?;
            while let Some(entry) = entries.next_entry().await? {
                if let Ok(metadata) = entry.metadata().await {
                    if metadata.is_file() {
                        total_size += metadata.len();
                    }
                }
            }
        }
        
        Ok(total_size)
    }
}

/// Storage statistics
#[derive(Debug, Clone)]
pub struct StorageStats {
    pub total_uploads: usize,
    pub upload_size: u64,
    pub processed_size: u64,
    pub thumbnail_size: u64,
    pub total_size: u64,
}

/// Upload validation
pub mod validation {
    use super::*;
    
    /// Maximum file size (100MB)
    pub const MAX_FILE_SIZE: u64 = 100 * 1024 * 1024;
    
    /// Allowed file extensions
    pub const ALLOWED_EXTENSIONS: &[&str] = &[
        // Video
        "mp4", "webm", "avi", "mov", "mkv",
        // Audio
        "mp3", "wav", "ogg", "opus", "flac",
        // Image
        "jpg", "jpeg", "png", "gif", "bmp", "webp",
    ];
    
    /// Validate file size
    pub fn validate_file_size(size: u64) -> Result<()> {
        if size > MAX_FILE_SIZE {
            return Err(anyhow::anyhow!(
                "File size {} exceeds maximum allowed size {}",
                size, MAX_FILE_SIZE
            ));
        }
        Ok(())
    }
    
    /// Validate file extension
    pub fn validate_file_extension(filename: &str) -> Result<()> {
        let extension = Path::new(filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        if !ALLOWED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
            return Err(anyhow::anyhow!(
                "File extension '{}' is not allowed", extension
            ));
        }
        
        Ok(())
    }
    
    /// Validate upload
    pub fn validate_upload(filename: &str, size: u64) -> Result<()> {
        validate_file_extension(filename)?;
        validate_file_size(size)?;
        Ok(())
    }
}