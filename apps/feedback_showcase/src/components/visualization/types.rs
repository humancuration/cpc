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
    #[serde(default)]
    pub permissions: Vec<Permission>,
    #[serde(default)]
    pub mentions: Vec<String>,
    #[serde(default)]
    pub version: u32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Permission {
    pub user_id: String,
    pub level: PermissionLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum PermissionLevel {
    View,
    Comment,
    Edit,
}

impl Default for PermissionLevel {
    fn default() -> Self {
        PermissionLevel::Edit
    }
}

impl Annotation {
    /// Create a new annotation with backward compatibility
    pub fn new(
        id: Uuid,
        share_id: String,
        user_id: String,
        content: String,
        position: Option<(f32, f32)>,
    ) -> Self {
        Self {
            id,
            share_id,
            user_id: user_id.clone(),
            timestamp: Utc::now(),
            content,
            position,
            permissions: vec![Permission {
                user_id,
                level: PermissionLevel::Edit,
            }],
            mentions: Vec::new(),
            version: 1,
        }
    }
    
    /// Ensure backward compatibility for existing annotations
    pub fn ensure_compatibility(&mut self, default_user_id: &str) {
        if self.permissions.is_empty() {
            self.permissions.push(Permission {
                user_id: self.user_id.clone(),
                level: PermissionLevel::Edit,
            });
        }
        
        if self.version == 0 {
            self.version = 1;
        }
    }
}