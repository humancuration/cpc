//! Visualization service for generating 3D visualizations using Bevy

use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::report::{Report, VisualizationType};
use crate::infrastructure::postgres_repository::PostgresBiRepository;
use crate::presentation::bevy_visualization::{BiVisualizationApp, AccessibilityMetadata};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum VisualizationError {
    #[error("Report not found: {0}")]
    ReportNotFound(Uuid),
    
    #[error("Failed to parse report data: {0}")]
    DataParsingError(String),
    
    #[error("Visualization generation failed: {0}")]
    GenerationError(String),
    
    #[error("Rendering failed: {0}")]
    RenderingError(String),
    
    #[error("Access denied for report: {0}")]
    AccessDenied(Uuid),
}

/// Navigation hint for keyboard accessibility
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationHint {
    pub label: String,
    pub key: String,
    pub position: [f32; 3],
}

/// Payload for 3D visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationPayload {
    pub scene_data: serde_json::Value, // Serialized 3D scene data (glTF format)
    pub alt_text: String,
    pub navigation_map: HashMap<String, NavigationHint>,
}

/// Base64 encoded image with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base64Image {
    pub image_data: String, // Base64-encoded PNG
    pub alt_text: String,
    pub width: i32,
    pub height: i32,
}

/// Service for generating visualizations
#[async_trait]
pub trait VisualizationService: Send + Sync {
    /// Generate 3D visualization for a report
    async fn generate_3d_visualization(
        &self,
        report_id: Uuid,
        user_id: Uuid,
    ) -> Result<VisualizationPayload, VisualizationError>;
    
    /// Generate static image representation of visualization
    async fn generate_visualization_image(
        &self,
        report_id: Uuid,
        user_id: Uuid,
        width: u32,
        height: u32,
    ) -> Result<Base64Image, VisualizationError>;
}

/// Implementation of the visualization service
pub struct BevyVisualizationService {
    repository: Arc<PostgresBiRepository>,
}

impl BevyVisualizationService {
    pub fn new(repository: Arc<PostgresBiRepository>) -> Self {
        Self { repository }
    }
    
    /// Check if user has access to the report
    async fn check_report_access(&self, report_id: Uuid, user_id: Uuid) -> Result<(), VisualizationError> {
        let report = self.repository.get_report(report_id)
            .await
            .map_err(|_| VisualizationError::ReportNotFound(report_id))?;
            
        // TODO: Implement proper authorization check
        // For now, allow access to all reports
        Ok(())
    }
    
    /// Generate accessibility metadata for a report
    fn generate_accessibility_metadata(
        &self,
        report: &Report,
    ) -> AccessibilityMetadata {
        let alt_text = format!(
            "{} visualization showing {}",
            report.visualization_type.to_string().replace('_', " "),
            self.describe_data_distribution(report)
        );
        
        AccessibilityMetadata {
            alt_text,
            navigation_map: self.generate_navigation_map(report),
            live_region: "polite".to_string(),
        }
    }
    
    /// Describe the data distribution for accessibility
    fn describe_data_distribution(&self, report: &Report) -> String {
        // Simple implementation - can be enhanced with actual data analysis
        match report.visualization_type {
            VisualizationType::BarChart => "bar chart data".to_string(),
            VisualizationType::LineChart => "line chart data".to_string(),
            VisualizationType::PieChart => "pie chart data".to_string(),
            VisualizationType::ScatterPlot => "scatter plot data".to_string(),
            VisualizationType::Heatmap => "heatmap data".to_string(),
            VisualizationType::AreaChart => "area chart data".to_string(),
            VisualizationType::Table => "table data".to_string(),
        }
    }
    
    /// Generate navigation map for keyboard accessibility
    fn generate_navigation_map(&self, report: &Report) -> HashMap<String, NavigationHint> {
        let mut map = HashMap::new();
        
        // Add basic navigation hints
        map.insert("title".to_string(), NavigationHint {
            label: format!("Report: {}", report.name),
            key: "T".to_string(),
            position: [0.0, 2.0, 0.0],
        });
        
        map.insert("legend".to_string(), NavigationHint {
            label: "Legend".to_string(),
            key: "L".to_string(),
            position: [-2.0, 0.0, 0.0],
        });
        
        map.insert("data".to_string(), NavigationHint {
            label: "Main visualization".to_string(),
            key: "D".to_string(),
            position: [0.0, 0.0, 0.0],
        });
        
        map
    }
}

#[async_trait]
impl VisualizationService for BevyVisualizationService {
    async fn generate_3d_visualization(
        &self,
        report_id: Uuid,
        user_id: Uuid,
    ) -> Result<VisualizationPayload, VisualizationError> {
        self.check_report_access(report_id, user_id).await?;
        
        let report = self.repository.get_report(report_id)
            .await
            .map_err(|_| VisualizationError::ReportNotFound(report_id))?;
            
        let mut app = BiVisualizationApp::new_headless();
        app.add_report_visualization(&report);
        
        let scene_data = app.export_scene_data()
            .map_err(|e| VisualizationError::GenerationError(e))?;
            
        let accessibility = self.generate_accessibility_metadata(&report);
        
        Ok(VisualizationPayload {
            scene_data,
            alt_text: accessibility.alt_text,
            navigation_map: accessibility.navigation_map,
        })
    }
    
    async fn generate_visualization_image(
        &self,
        report_id: Uuid,
        user_id: Uuid,
        width: u32,
        height: u32,
    ) -> Result<Base64Image, VisualizationError> {
        self.check_report_access(report_id, user_id).await?;
        
        let report = self.repository.get_report(report_id)
            .await
            .map_err(|_| VisualizationError::ReportNotFound(report_id))?;
            
        let mut app = BiVisualizationApp::new_headless();
        app.add_report_visualization(&report);
        
        let image_data = app.render_to_image(width, height)
            .map_err(|e| VisualizationError::RenderingError(e))?;
            
        let accessibility = self.generate_accessibility_metadata(&report);
        
        Ok(Base64Image {
            image_data: base64::encode(image_data),
            alt_text: accessibility.alt_text,
            width: width as i32,
            height: height as i32,
        })
    }
}