//! Shared visualization context for cross-app communication

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use uuid::Uuid;

/// Error types for context operations
#[derive(Error, Debug)]
pub enum ContextError {
    #[error("Invalid header format: {0}")]
    InvalidHeaderFormat(String),
    
    #[error("Missing required header: {0}")]
    MissingHeader(String),
    
    #[error("Invalid UUID format: {0}")]
    InvalidUuid(String),
}

/// Sharing scope for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SharingScope {
    Public,
    Team(Uuid),
    Private(Uuid),
}

/// Accessibility modes for visualizations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessibilityMode {
    Standard,
    HighContrast,
    ScreenReader,
    KeyboardNavigation,
}

/// Preferences for alt text generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AltTextPreferences {
    /// Level of detail in alt text (0 = brief, 1 = detailed, 2 = verbose)
    pub detail_level: u8,
    /// Focus of description (data values, trends, patterns)
    pub content_focus: String,
    /// Format of alt text (textual, structural)
    pub format: String,
}

impl Default for AltTextPreferences {
    fn default() -> Self {
        Self {
            detail_level: 1,
            content_focus: "data values".to_string(),
            format: "textual".to_string(),
        }
    }
}

/// Visualization context for cross-app communication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationContext {
    /// Originating app identifier
    pub originating_app: String,
    /// User ID making the request
    pub user_id: Uuid,
    /// Sharing scope for the visualization
    pub sharing_scope: SharingScope,
    /// Accessibility mode requested
    pub accessibility_mode: AccessibilityMode,
    /// Level of detail (LOD) for progressive loading
    pub lod_level: u8,
    /// Preferences for alt text generation
    pub alt_text_preferences: AltTextPreferences,
    /// Additional context metadata
    pub metadata: HashMap<String, String>,
}

impl VisualizationContext {
    /// Create a new visualization context
    pub fn new(
        originating_app: String,
        user_id: Uuid,
        sharing_scope: SharingScope,
        accessibility_mode: AccessibilityMode,
        lod_level: u8,
    ) -> Self {
        Self {
            originating_app,
            user_id,
            sharing_scope,
            accessibility_mode,
            lod_level,
            alt_text_preferences: AltTextPreferences::default(),
            metadata: HashMap::new(),
        }
    }
    
    /// Extract context from HTTP headers
    pub fn from_headers(headers: &http::HeaderMap) -> Result<Self, ContextError> {
        let originating_app = headers
            .get("X-Originating-App")
            .ok_or_else(|| ContextError::MissingHeader("X-Originating-App".to_string()))?
            .to_str()
            .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))?
            .to_string();
            
        let user_id = headers
            .get("X-User-ID")
            .ok_or_else(|| ContextError::MissingHeader("X-User-ID".to_string()))?
            .to_str()
            .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))
            .and_then(|s| {
                Uuid::parse_str(s).map_err(|e| ContextError::InvalidUuid(e.to_string()))
            })?;
            
        let sharing_scope = headers
            .get("X-Sharing-Scope")
            .ok_or_else(|| ContextError::MissingHeader("X-Sharing-Scope".to_string()))?
            .to_str()
            .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))
            .and_then(|s| match s {
                "public" => Ok(SharingScope::Public),
                "private" => Ok(SharingScope::Private(user_id)),
                "team" => {
                    let team_id = headers
                        .get("X-Team-ID")
                        .ok_or_else(|| ContextError::MissingHeader("X-Team-ID".to_string()))?
                        .to_str()
                        .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))
                        .and_then(|s| {
                            Uuid::parse_str(s).map_err(|e| ContextError::InvalidUuid(e.to_string()))
                        })?;
                    Ok(SharingScope::Team(team_id))
                }
                _ => Err(ContextError::InvalidHeaderFormat("Invalid sharing scope".to_string())),
            })?;
            
        let accessibility_mode = headers
            .get("X-Accessibility-Mode")
            .ok_or_else(|| ContextError::MissingHeader("X-Accessibility-Mode".to_string()))?
            .to_str()
            .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))
            .and_then(|s| match s {
                "standard" => Ok(AccessibilityMode::Standard),
                "high-contrast" => Ok(AccessibilityMode::HighContrast),
                "screen-reader" => Ok(AccessibilityMode::ScreenReader),
                "keyboard" => Ok(AccessibilityMode::KeyboardNavigation),
                _ => Err(ContextError::InvalidHeaderFormat("Invalid accessibility mode".to_string())),
            })?;
            
        let lod_level = headers
            .get("X-LOD-Level")
            .ok_or_else(|| ContextError::MissingHeader("X-LOD-Level".to_string()))?
            .to_str()
            .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))
            .and_then(|s| {
                s.parse::<u8>()
                    .map_err(|e| ContextError::InvalidHeaderFormat(e.to_string()))
            })?;
            
        let detail_level = headers
            .get("X-Alt-Text-Detail-Level")
            .and_then(|h| h.to_str().ok())
            .and_then(|s| s.parse::<u8>().ok())
            .unwrap_or(1); // Default to detailed
            
        let content_focus = headers
            .get("X-Alt-Text-Content-Focus")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "data values".to_string());
            
        let format = headers
            .get("X-Alt-Text-Format")
            .and_then(|h| h.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "textual".to_string());
            
        let alt_text_preferences = AltTextPreferences {
            detail_level,
            content_focus,
            format,
        };
            
        let mut metadata = HashMap::new();
        for (key, value) in headers.iter() {
            if key.as_str().starts_with("X-Context-") {
                if let Ok(value_str) = value.to_str() {
                    let context_key = key.as_str().trim_start_matches("X-Context-").to_string();
                    metadata.insert(context_key, value_str.to_string());
                }
            }
        }
        
        Ok(Self {
            originating_app,
            user_id,
            sharing_scope,
            accessibility_mode,
            lod_level,
            alt_text_preferences,
            metadata,
        })
    }
    
    /// Convert context to HTTP headers
    pub fn to_headers(&self) -> http::HeaderMap {
        let mut headers = http::HeaderMap::new();
        
        headers.insert(
            "X-Originating-App",
            http::HeaderValue::from_str(&self.originating_app).unwrap(),
        );
        
        headers.insert(
            "X-User-ID",
            http::HeaderValue::from_str(&self.user_id.to_string()).unwrap(),
        );
        
        let sharing_scope_str = match &self.sharing_scope {
            SharingScope::Public => "public",
            SharingScope::Team(_) => "team",
            SharingScope::Private(_) => "private",
        };
        headers.insert(
            "X-Sharing-Scope",
            http::HeaderValue::from_str(sharing_scope_str).unwrap(),
        );
        
        if let SharingScope::Team(team_id) = &self.sharing_scope {
            headers.insert(
                "X-Team-ID",
                http::HeaderValue::from_str(&team_id.to_string()).unwrap(),
            );
        }
        
        let accessibility_mode_str = match self.accessibility_mode {
            AccessibilityMode::Standard => "standard",
            AccessibilityMode::HighContrast => "high-contrast",
            AccessibilityMode::ScreenReader => "screen-reader",
            AccessibilityMode::KeyboardNavigation => "keyboard",
        };
        headers.insert(
            "X-Accessibility-Mode",
            http::HeaderValue::from_str(accessibility_mode_str).unwrap(),
        );
        
        headers.insert(
            "X-LOD-Level",
            http::HeaderValue::from_str(&self.lod_level.to_string()).unwrap(),
        );
        
        headers.insert(
            "X-Alt-Text-Detail-Level",
            http::HeaderValue::from_str(&self.alt_text_preferences.detail_level.to_string()).unwrap(),
        );
        
        headers.insert(
            "X-Alt-Text-Content-Focus",
            http::HeaderValue::from_str(&self.alt_text_preferences.content_focus).unwrap(),
        );
        
        headers.insert(
            "X-Alt-Text-Format",
            http::HeaderValue::from_str(&self.alt_text_preferences.format).unwrap(),
        );
        
        for (key, value) in &self.metadata {
            headers.insert(
                format!("X-Context-{}", key),
                http::HeaderValue::from_str(value).unwrap(),
            );
        }
        
        headers
    }
    
    /// Check if the user has access to the visualization
    pub fn has_access(&self, user_id: Uuid) -> bool {
        match &self.sharing_scope {
            SharingScope::Public => true,
            SharingScope::Team(team_id) => {
                // In a real implementation, check if user belongs to the team
                true
            }
            SharingScope::Private(owner_id) => *owner_id == user_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use http::HeaderMap;
    
    #[test]
    fn test_context_serialization() {
        let user_id = Uuid::new_v4();
        let team_id = Uuid::new_v4();
        
        let context = VisualizationContext::new(
            "dashboard".to_string(),
            user_id,
            SharingScope::Team(team_id),
            AccessibilityMode::ScreenReader,
            2,
        );
        
        let headers = context.to_headers();
        let deserialized = VisualizationContext::from_headers(&headers).unwrap();
        
        assert_eq!(deserialized.originating_app, context.originating_app);
        assert_eq!(deserialized.user_id, context.user_id);
        assert_eq!(deserialized.lod_level, context.lod_level);
        assert_eq!(deserialized.alt_text_preferences.detail_level, context.alt_text_preferences.detail_level);
        assert_eq!(deserialized.alt_text_preferences.content_focus, context.alt_text_preferences.content_focus);
        assert_eq!(deserialized.alt_text_preferences.format, context.alt_text_preferences.format);
    }
    
    #[test]
    fn test_access_check() {
        let user_id = Uuid::new_v4();
        let other_user_id = Uuid::new_v4();
        
        // Public scope
        let context = VisualizationContext::new(
            "dashboard".to_string(),
            user_id,
            SharingScope::Public,
            AccessibilityMode::Standard,
            1,
        );
        assert!(context.has_access(user_id));
        assert!(context.has_access(other_user_id));
        
        // Private scope
        let context = VisualizationContext::new(
            "dashboard".to_string(),
            user_id,
            SharingScope::Private(user_id),
            AccessibilityMode::Standard,
            1,
        );
        assert!(context.has_access(user_id));
        assert!(!context.has_access(other_user_id));
    }
}