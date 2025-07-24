use async_graphql::*;
use std::path::PathBuf;
use cpc_core::asset_browser::{AssetMetadata as CoreAssetMetadata, PreviewData as CorePreviewData};
use crate::services::asset_storage::AssetStorageService;
use crate::services::asset_preview::AssetPreviewService;
use uuid::Uuid;

#[derive(SimpleObject, Clone)]
pub struct AssetMetadata {
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size: u64,
    pub asset_type: AssetType,
    pub created_at: String,
    pub updated_at: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum AssetType {
    Image,
    Video,
    Audio,
    Document,
    Model3D,
    Other,
}

impl From<CoreAssetMetadata> for AssetMetadata {
    fn from(core: CoreAssetMetadata) -> Self {
        Self {
            id: core.id,
            name: core.name,
            path: core.path.to_string_lossy().to_string(),
            size: core.size,
            asset_type: match core.asset_type {
                cpc_core::asset_browser::AssetType::Image => AssetType::Image,
                cpc_core::asset_browser::AssetType::Video => AssetType::Video,
                cpc_core::asset_browser::AssetType::Audio => AssetType::Audio,
                cpc_core::asset_browser::AssetType::Document => AssetType::Document,
                cpc_core::asset_browser::AssetType::Model3D => AssetType::Model3D,
                cpc_core::asset_browser::AssetType::Other => AssetType::Other,
            },
            created_at: core.created_at.to_rfc3339(),
            updated_at: core.updated_at.to_rfc3339(),
            thumbnail_path: core.thumbnail_path.map(|p| p.to_string_lossy().to_string()),
        }
    }
}

#[derive(SimpleObject, Clone)]
pub struct PreviewData {
    pub kind: PreviewKind,
    pub data: Option<PreviewDataContent>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum PreviewKind {
    Image,
    Video,
    Audio,
    Document,
    Unavailable,
}

#[derive(SimpleObject, Clone)]
pub struct PreviewDataContent {
    pub path: Option<String>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration: Option<f32>,
    pub page_count: Option<u32>,
}

impl From<CorePreviewData> for PreviewData {
    fn from(core: CorePreviewData) -> Self {
        let (kind, data) = match core {
            CorePreviewData::Image { path, width, height } => (
                PreviewKind::Image,
                Some(PreviewDataContent {
                    path: Some(path.to_string_lossy().to_string()),
                    width: Some(width),
                    height: Some(height),
                    duration: None,
                    page_count: None,
                }),
            ),
            CorePreviewData::Video { thumbnail_path, duration } => (
                PreviewKind::Video,
                Some(PreviewDataContent {
                    path: Some(thumbnail_path.to_string_lossy().to_string()),
                    width: None,
                    height: None,
                    duration: Some(duration),
                    page_count: None,
                }),
            ),
            CorePreviewData::Audio { waveform_path } => (
                PreviewKind::Audio,
                Some(PreviewDataContent {
                    path: Some(waveform_path.to_string_lossy().to_string()),
                    width: None,
                    height: None,
                    duration: None,
                    page_count: None,
                }),
            ),
            CorePreviewData::Document { page_count } => (
                PreviewKind::Document,
                Some(PreviewDataContent {
                    path: None,
                    width: None,
                    height: None,
                    duration: None,
                    page_count: Some(page_count),
                }),
            ),
            CorePreviewData::Unavailable => (PreviewKind::Unavailable, None),
        };

        Self { kind, data }
    }
}

#[derive(Default)]
pub struct AssetBrowserQuery;

#[Object]
impl AssetBrowserQuery {
    async fn get_assets_in_path(
        &self,
        ctx: &Context<'_>,
        path: String,
    ) -> Result<Vec<AssetMetadata>> {
        let storage = ctx.data_unchecked::<AssetStorageService>();
        
        let path_buf = PathBuf::from(path);
        let assets = storage.get_assets_in_path(&path_buf)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
            
        Ok(assets.into_iter().map(AssetMetadata::from).collect())
    }

    async fn get_asset_metadata(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<Option<AssetMetadata>> {
        let storage = ctx.data_unchecked::<AssetStorageService>();
        
        let metadata = storage.get_asset_metadata(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
            
        Ok(metadata.map(AssetMetadata::from))
    }

    async fn get_asset_preview(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<PreviewData> {
        let preview_service = ctx.data_unchecked::<AssetPreviewService>();
        
        let preview = preview_service.get_asset_preview(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
            
        Ok(PreviewData::from(preview))
    }
}

#[derive(Default)]
pub struct AssetBrowserMutation;

#[Object]
impl AssetBrowserMutation {
    async fn import_asset(
        &self,
        ctx: &Context<'_>,
        file_path: String,
        asset_type: AssetType,
    ) -> Result<Uuid> {
        let storage = ctx.data_unchecked::<AssetStorageService>();
        let preview_service = ctx.data_unchecked::<AssetPreviewService>();
        
        let path = PathBuf::from(file_path);
        let core_asset_type = match asset_type {
            AssetType::Image => cpc_core::asset_browser::AssetType::Image,
            AssetType::Video => cpc_core::asset_browser::AssetType::Video,
            AssetType::Audio => cpc_core::asset_browser::AssetType::Audio,
            AssetType::Document => cpc_core::asset_browser::AssetType::Document,
            AssetType::Model3D => cpc_core::asset_browser::AssetType::Model3D,
            AssetType::Other => cpc_core::asset_browser::AssetType::Other,
        };
        
        let asset_id = storage.import_asset(&path, core_asset_type)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        // Generate thumbnail in background
        let storage_clone = storage.clone();
        let preview_service_clone = preview_service.clone();
        tokio::spawn(async move {
            if let Ok(Some(thumb_path)) = preview_service_clone.generate_thumbnail(asset_id).await {
                let _ = storage_clone.update_asset_thumbnail(asset_id, Some(thumb_path)).await;
            }
        });
        
        // Emit asset created event through subscription
        ctx.data_unchecked::<async_graphql::Data>()
            .get::<crate::graphql::subscriptions::AssetSubscription>()
            .map(|sub| {
                sub.asset_created(asset_id);
            });
        
        Ok(asset_id)
    }

    async fn import_multiple_assets(
        &self,
        ctx: &Context<'_>,
        files: Vec<String>,
        asset_types: Vec<AssetType>,
    ) -> Result<Vec<Uuid>> {
        if files.len() != asset_types.len() {
            return Err(Error::new("Files and asset types count must match"));
        }
        
        let mut results = Vec::new();
        
        for (file_path, asset_type) in files.into_iter().zip(asset_types.into_iter()) {
            match self.import_asset(ctx, file_path, asset_type).await {
                Ok(id) => results.push(id),
                Err(e) => {
                    // Log error but continue with other files
                    tracing::error!("Failed to import asset: {}", e);
                }
            }
        }
        
        Ok(results)
    }

    async fn delete_asset(
        &self,
        ctx: &Context<'_>,
        id: Uuid,
    ) -> Result<bool> {
        let storage = ctx.data_unchecked::<AssetStorageService>();
        
        storage.delete_asset(id)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        
        // Emit asset deleted event through subscription
        ctx.data_unchecked::<async_graphql::Data>()
            .get::<crate::graphql::subscriptions::AssetSubscription>()
            .map(|sub| {
                sub.asset_deleted(id);
            });
        
        Ok(true)
    }
}

#[derive(Default)]
pub struct AssetBrowserSubscription;

#[Subscription]
impl AssetBrowserSubscription {
    async fn asset_created(&self) -> impl futures::Stream<Item = AssetMetadata> {
        // This would be implemented with a real subscription system
        // For now, returning an empty stream
        futures::stream::empty()
    }

    async fn asset_updated(&self) -> impl futures::Stream<Item = AssetMetadata> {
        futures::stream::empty()
    }

    async fn asset_deleted(&self) -> impl futures::Stream<Item = Uuid> {
        futures::stream::empty()
    }

    async fn asset_conflict(&self) -> impl futures::Stream<Item = String> {
        futures::stream::empty()
    }
}