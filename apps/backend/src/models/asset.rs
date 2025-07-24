use sea_orm::entity::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "assets")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub name: String,
    pub path: String,
    pub size: i64,
    pub asset_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub thumbnail_path: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Model {
    pub fn to_core_metadata(&self) -> cpc_core::asset_browser::AssetMetadata {
        cpc_core::asset_browser::AssetMetadata {
            id: self.id,
            name: self.name.clone(),
            path: PathBuf::from(&self.path),
            size: self.size as u64,
            asset_type: match self.asset_type.as_str() {
                "Image" => cpc_core::asset_browser::AssetType::Image,
                "Video" => cpc_core::asset_browser::AssetType::Video,
                "Audio" => cpc_core::asset_browser::AssetType::Audio,
                "Document" => cpc_core::asset_browser::AssetType::Document,
                "Model3D" => cpc_core::asset_browser::AssetType::Model3D,
                _ => cpc_core::asset_browser::AssetType::Other,
            },
            created_at: self.created_at,
            updated_at: self.updated_at,
            thumbnail_path: self.thumbnail_path.as_ref().map(|p| PathBuf::from(p)),
        }
    }

    pub fn from_core_metadata(metadata: &cpc_core::asset_browser::AssetMetadata) -> Self {
        Self {
            id: metadata.id,
            name: metadata.name.clone(),
            path: metadata.path.to_string_lossy().to_string(),
            size: metadata.size as i64,
            asset_type: match metadata.asset_type {
                cpc_core::asset_browser::AssetType::Image => "Image".to_string(),
                cpc_core::asset_browser::AssetType::Video => "Video".to_string(),
                cpc_core::asset_browser::AssetType::Audio => "Audio".to_string(),
                cpc_core::asset_browser::AssetType::Document => "Document".to_string(),
                cpc_core::asset_browser::AssetType::Model3D => "Model3D".to_string(),
                cpc_core::asset_browser::AssetType::Other => "Other".to_string(),
            },
            created_at: metadata.created_at,
            updated_at: metadata.updated_at,
            thumbnail_path: metadata.thumbnail_path.as_ref().map(|p| p.to_string_lossy().to_string()),
        }
    }
}