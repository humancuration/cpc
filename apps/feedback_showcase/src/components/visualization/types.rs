//! Common types for visualization components

use yew::{Properties, Callback};
use reviews::Review;
use crate::data_generator::generators::products::Product;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Properties, PartialEq)]
pub struct VisualizationProps {
    pub reviews: Vec<Review<Product>>,
    pub loading: bool,
    #[prop_or_default]
    pub on_share: Callback<ShareAction>,
    #[prop_or(true)]
    pub enable_sharing: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum VisualizationComponent {
    Summary,
    Ratings,
    WordCloud,
    Sentiment,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ShareAction {
    Federation,
    Embed,
    Image,
    Social,
}

/// Annotation struct for visualization comments
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Annotation {
    pub id: Uuid,
    pub share_id: String,
    pub user_id: String,
    pub timestamp: DateTime<Utc>,
    pub content: String,
    pub position: Option<(f32, f32)>, // Normalized coordinates
}