use std::path::{Path, PathBuf};
use cpc_core::asset_browser::{PreviewData, AssetMetadata};
use crate::services::asset_storage::AssetStorageService;
use uuid::Uuid;

pub struct AssetPreviewService {
    storage: AssetStorageService,
}

impl AssetPreviewService {
    pub fn new(storage: AssetStorageService) -> Self {
        Self { storage }
    }

    pub async fn get_asset_preview(&self, id: Uuid) -> Result<PreviewData, Box<dyn std::error::Error>> {
        let metadata = self.storage.get_asset_metadata(id).await?
            .ok_or("Asset not found")?;
            
        match metadata.asset_type {
            cpc_core::asset_browser::AssetType::Image => {
                self.get_image_preview(&metadata).await
            }
            cpc_core::asset_browser::AssetType::Video => {
                self.get_video_preview(&metadata).await
            }
            cpc_core::asset_browser::AssetType::Audio => {
                self.get_audio_preview(&metadata).await
            }
            cpc_core::asset_browser::AssetType::Document => {
                self.get_document_preview(&metadata).await
            }
            _ => Ok(PreviewData::Unavailable),
        }
    }

    async fn get_image_preview(&self, metadata: &AssetMetadata) -> Result<PreviewData, Box<dyn std::error::Error>> {
        let (width, height) = if let Some(thumb_path) = &metadata.thumbnail_path {
            // Use thumbnail if available
            let img = image::open(thumb_path)?;
            (img.width(), img.height())
        } else {
            // Try to load original image
            let img = image::open(&metadata.path)?;
            (img.width(), img.height())
        };
        
        Ok(PreviewData::Image {
            path: metadata.thumbnail_path.clone().unwrap_or(metadata.path.clone()),
            width,
            height,
        })
    }

    async fn get_video_preview(&self, metadata: &AssetMetadata) -> Result<PreviewData, Box<dyn std::error::Error>> {
        // For now, return unavailable - in a real implementation, we'd use ffmpeg to extract metadata
        let thumbnail_path = metadata.thumbnail_path.clone()
            .unwrap_or_else(|| PathBuf::from("assets/video-placeholder.png"));
            
        Ok(PreviewData::Video {
            thumbnail_path,
            duration: 0.0, // Placeholder
        })
    }

    async fn get_audio_preview(&self, metadata: &AssetMetadata) -> Result<PreviewData, Box<dyn std::error::Error>> {
        // For now, return unavailable - in a real implementation, we'd generate a waveform
        let waveform_path = PathBuf::from("assets/audio-placeholder.png");
        
        Ok(PreviewData::Audio {
            waveform_path,
        })
    }

    async fn get_document_preview(&self, metadata: &AssetMetadata) -> Result<PreviewData, Box<dyn std::error::Error>> {
        // For now, return unavailable - in a real implementation, we'd extract page count
        Ok(PreviewData::Document {
            page_count: 1,
        })
    }

    pub async fn generate_thumbnail(&self, id: Uuid) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        let metadata = self.storage.get_asset_metadata(id).await?
            .ok_or("Asset not found")?;

        match metadata.asset_type {
            cpc_core::asset_browser::AssetType::Image => {
                self.generate_image_thumbnail(&metadata).await
            }
            _ => Ok(None), // Skip thumbnail generation for other types for now
        }
    }

    async fn generate_image_thumbnail(&self, metadata: &AssetMetadata) -> Result<Option<PathBuf>, Box<dyn std::error::Error>> {
        let img = image::open(&metadata.path)?;
        
        // Generate thumbnail (200x200 max)
        let thumbnail = img.thumbnail(200, 200);
        
        let thumbnail_name = format!("{}_thumb.jpg", metadata.id);
        let thumbnail_path = self.storage.get_thumbnails_dir().join(thumbnail_name);
        
        thumbnail.save(&thumbnail_path)?;
        
        Ok(Some(thumbnail_path))
    }
}