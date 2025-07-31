use async_graphql::{SimpleObject, ID};
use chrono::{DateTime, Utc};
use crate::domain::media_asset::{MediaAsset, MediaType};

#[derive(SimpleObject)]
pub struct MediaAssetObject {
    pub id: ID,
    pub url: String,
    pub thumbnail_url: Option<String>,
    pub media_type: String,
    pub alt_text: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<MediaAsset> for MediaAssetObject {
    fn from(media_asset: MediaAsset) -> Self {
        Self {
            id: ID::from(media_asset.id.to_string()),
            url: media_asset.url,
            thumbnail_url: media_asset.thumbnail_url,
            media_type: match media_asset.media_type {
                MediaType::Image => "IMAGE".to_string(),
                MediaType::Video => "VIDEO".to_string(),
            },
            alt_text: media_asset.alt_text,
            created_at: media_asset.created_at,
        }
    }
}