//! Example visualization client for the Dashboard app
//!
//! This module demonstrates how to integrate with the visualization gateway
//! from a client application.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use visualization_context::{VisualizationContext, SharingScope, AccessibilityMode};

/// Visualization client for making requests to the API gateway
pub struct VisualizationClient {
    client: Client,
    gateway_url: String,
}

impl VisualizationClient {
    /// Create a new visualization client
    pub fn new(gateway_url: String) -> Self {
        Self {
            client: Client::new(),
            gateway_url,
        }
    }
    
    /// Get a 3D visualization
    pub async fn get_visualization(
        &self,
        visualization_id: Uuid,
        context: &VisualizationContext,
        width: Option<u32>,
        height: Option<u32>,
        lod_level: Option<u8>,
    ) -> Result<VisualizationResponse, Box<dyn std::error::Error>> {
        let mut url = format!("{}/visualizations/{}", self.gateway_url, visualization_id);
        
        // Add query parameters
        let mut params = Vec::new();
        if let Some(w) = width {
            params.push(format!("width={}", w));
        }
        if let Some(h) = height {
            params.push(format!("height={}", h));
        }
        if let Some(lod) = lod_level {
            params.push(format!("lod_level={}", lod));
        }
        
        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }
        
        // Convert context to headers
        let headers = context.to_headers();
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        let visualization_response = response.json::<VisualizationResponse>().await?;
        Ok(visualization_response)
    }
    
    /// Get visualization as image
    pub async fn get_visualization_image(
        &self,
        visualization_id: Uuid,
        context: &VisualizationContext,
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut url = format!("{}/visualizations/{}/image", self.gateway_url, visualization_id);
        
        // Add query parameters
        let mut params = Vec::new();
        if let Some(w) = width {
            params.push(format!("width={}", w));
        }
        if let Some(h) = height {
            params.push(format!("height={}", h));
        }
        
        if !params.is_empty() {
            url.push_str("?");
            url.push_str(&params.join("&"));
        }
        
        // Convert context to headers
        let headers = context.to_headers();
        
        let response = self.client
            .get(&url)
            .headers(headers)
            .send()
            .await?;
            
        let image_data = response.bytes().await?;
        Ok(image_data.to_vec())
    }
    
    /// Apply LOD configuration
    pub fn apply_lod_config(&self, config: AppLodConfig) -> AppLodConfig {
        // In a real implementation, this would apply dynamic detail adjustment
        // For now, we'll just return the config as-is
        config
    }
}

/// Response from visualization endpoint
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationResponse {
    pub visualization_data: VisualizationData,
    pub metadata: ResponseMetadata,
    pub compliance: ComplianceMetadata,
}

/// Visualization data payload
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum VisualizationData {
    Scene3D {
        payload: serde_json::Value,
        accessibility: AccessibilityMetadata,
    },
    Image {
        payload: String, // Base64 encoded image
        format: String,
        accessibility: AccessibilityMetadata,
    },
    Stream {
        endpoint: String,
        stream_id: String,
        accessibility: AccessibilityMetadata,
    },
}

/// Accessibility metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityMetadata {
    pub alt_text: String,
    pub navigation_map: std::collections::HashMap<String, NavigationHint>,
    pub aria_properties: AriaProperties,
}

/// Navigation hint for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationHint {
    pub label: String,
    pub key: String,
    pub position: [f32; 3],
}

/// ARIA properties for accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AriaProperties {
    pub role: String,
    pub live_region: String,
    pub keyboard_shortcuts: Vec<String>,
}

/// Response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseMetadata {
    pub cache_ttl: u64,
    pub lod_config: LodConfig,
    pub compliance_flags: Vec<String>,
}
/// Level of detail configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LodConfig {
    pub level: u8,
    pub max_points: u32,
}

/// Compliance metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceMetadata {
    pub data_sovereignty: String, // Country/region of origin
    pub pii_redacted: bool,       // Whether PII has been removed
    pub sharing_permissions: Vec<String>, // Who can access this data
}

/// App-specific LOD configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppLodConfig {
    pub default_lod: u8,
    pub max_points: u32,
}
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;
    
    #[tokio::test]
    async fn test_visualization_client_creation() {
        let client = VisualizationClient::new("http://localhost:3001".to_string());
        assert_eq!(client.gateway_url, "http://localhost:3001");
    }
    
    #[test]
    fn test_visualization_response_structs() {
        let response = VisualizationResponse {
            visualization_data: VisualizationData::Scene3D {
                payload: serde_json::json!({"test": "data"}),
                accessibility: AccessibilityMetadata {
                    alt_text: "Test visualization".to_string(),
                    navigation_map: std::collections::HashMap::new(),
                    aria_properties: AriaProperties {
                        role: "application".to_string(),
                        live_region: "off".to_string(),
                        keyboard_shortcuts: vec![],
                    },
                },
            },
            metadata: ResponseMetadata {
                cache_ttl: 300,
                lod_config: LodConfig {
                    level: 2,
                    max_points: 1000,
                },
                compliance_flags: vec![],
            },
        };
        
        match response.visualization_data {
            VisualizationData::Scene3D { payload, .. } => {
                assert_eq!(payload["test"], "data");
            }
            _ => panic!("Expected Scene3D variant"),
        }
    }
    
    #[test]
    fn test_visualization_response_with_compliance() {
        let response = VisualizationResponse {
            visualization_data: VisualizationData::Scene3D {
                payload: serde_json::json!({"test": "data"}),
                accessibility: AccessibilityMetadata {
                    alt_text: "Test visualization".to_string(),
                    navigation_map: std::collections::HashMap::new(),
                    aria_properties: AriaProperties {
                        role: "application".to_string(),
                        live_region: "off".to_string(),
                        keyboard_shortcuts: vec![],
                    },
                },
            },
            metadata: ResponseMetadata {
                cache_ttl: 300,
                lod_config: LodConfig {
                    level: 2,
                    max_points: 1000,
                },
                compliance_flags: vec![],
            },
            compliance: ComplianceMetadata {
                data_sovereignty: "US".to_string(),
                pii_redacted: true,
                sharing_permissions: vec!["user".to_string()],
            },
        };
        
        assert_eq!(response.compliance.data_sovereignty, "US");
        assert!(response.compliance.pii_redacted);
        assert_eq!(response.compliance.sharing_permissions.len(), 1);
    }
    
    #[test]
    fn test_lod_config_application() {
        let client = VisualizationClient::new("http://localhost:3001".to_string());
        let config = AppLodConfig {
            default_lod: 2,
            max_points: 1000,
        };
        
        let applied_config = client.apply_lod_config(config);
        assert_eq!(applied_config.default_lod, 2);
        assert_eq!(applied_config.max_points, 1000);
    }
}