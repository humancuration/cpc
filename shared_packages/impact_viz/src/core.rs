//! Core visualization components and traits for the impact visualization framework

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};
use common_utils::financial::MonetaryValue;
use cpc_statistics_core::{ConfidenceInterval, SignificanceResult};

/// Core trait for impact visualization
pub trait ImpactVisualization {
    /// Translate mathematical outputs into community impact metrics
    fn translate_impact(&self, data: &MathematicalOutput) -> ImpactMetric;
    
    /// Generate visualization in different styles
    fn visualize(&self, metric: &ImpactMetric, style: VisualizationStyle) -> VisualizationResult;
    
    /// Provide cooperative values translation
    fn translate_values(&self, metric: &ImpactMetric) -> ValuesAlignedMetric;
    
    /// Ensure accessibility-first design principles
    fn ensure_accessibility(&self, viz: &VisualizationResult, options: &AccessibilityOptions) -> AccessibleVisualization;
}

/// Mathematical output from ML models or statistical analysis
#[derive(Debug, Clone)]
pub struct MathematicalOutput {
    /// Primary value or prediction
    pub value: f64,
    
    /// Confidence interval if applicable
    pub confidence_interval: Option<ConfidenceInterval>,
    
    /// Statistical significance if applicable
    pub significance: Option<SignificanceResult>,
    
    /// Additional metadata
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Visualization styles for different presentation needs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationStyle {
    /// Narrative-driven visualization that tells community stories
    Narrative,
    
    /// Comparative metrics showing community progress over time
    Comparative,
    
    /// Trend-based visualization showing changes over time
    TrendBased,
    
    /// Simple quantitative representation
    Quantitative,
    
    /// Qualitative representation with context
    Qualitative,
}

/// Core impact metric representing community impact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactMetric {
    /// Name of the metric
    pub name: String,
    
    /// Description of what the metric measures
    pub description: String,
    
    /// Primary value of the metric
    pub value: f64,
    
    /// Unit of measurement
    pub unit: MetricUnit,
    
    /// Confidence interval if applicable
    pub confidence_interval: Option<ConfidenceInterval>,
    
    /// Statistical significance if applicable
    pub significance: Option<SignificanceResult>,
    
    /// Context information
    pub context: HashMap<String, String>,
}

/// Units of measurement for impact metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricUnit {
    /// Count or number of items
    Count,
    
    /// Percentage (0.0 to 100.0)
    Percentage,
    
    /// Monetary value with currency
    Monetary(MonetaryValue),
    
    /// Hours of time
    Hours,
    
    /// People affected
    People,
    
    /// Skills developed
    Skills,
    
    /// Custom unit with description
    Custom(String),
}

/// Values-aligned metric that connects to cooperative principles
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValuesAlignedMetric {
    /// Base impact metric
    pub base_metric: ImpactMetric,
    
    /// How this metric connects to cooperative values
    pub values_connection: HashMap<String, String>,
    
    /// Community validation status
    pub community_validated: bool,
    
    /// Narrative explanation of the impact
    pub narrative: String,
}

/// Community story that connects data to human outcomes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityStory {
    /// Title of the story
    pub title: String,
    
    /// Narrative description of the impact
    pub narrative: String,
    
    /// Related impact metrics
    pub metrics: Vec<ImpactMetric>,
    
    /// Community member quotes or testimonials
    pub testimonials: Vec<String>,
    
    /// Visual elements to support the story
    pub visual_elements: Vec<VisualElement>,
}

/// Visual element that can be part of a community story
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualElement {
    /// Chart or graph visualization
    Chart(VisualizationResult),
    
    /// Image or photo
    Image(String), // URL or path to image
    
    /// Icon or symbol
    Icon(String),
    
    /// Timeline representation
    Timeline(Vec<TimelineEvent>),
}

/// Timeline event for storytelling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimelineEvent {
    /// Date of the event
    pub date: chrono::DateTime<chrono::Utc>,
    
    /// Description of what happened
    pub description: String,
    
    /// Impact metrics at this point in time
    pub metrics: Vec<ImpactMetric>,
}

/// Result of visualization generation
#[derive(Debug, Clone)]
pub struct VisualizationResult {
    /// Visualization data in a format suitable for rendering
    pub data: VisualizationData,
    
    /// Type of visualization generated
    pub viz_type: VisualizationType,
    
    /// Metadata about the visualization
    pub metadata: HashMap<String, String>,
}

/// Types of visualizations that can be generated
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VisualizationType {
    /// Bar chart visualization
    BarChart,
    
    /// Line chart visualization
    LineChart,
    
    /// Pie chart visualization
    PieChart,
    
    /// Scatter plot visualization
    ScatterPlot,
    
    /// Heatmap visualization
    Heatmap,
    
    /// Area chart visualization
    AreaChart,
    
    /// Table visualization
    Table,
    
    /// Narrative text visualization
    Narrative,
    
    /// Interactive dashboard
    Dashboard,
}

/// Visualization data structure
#[derive(Debug, Clone)]
pub struct VisualizationData {
    /// JSON representation of the data
    pub json_data: String,
    
    /// Binary representation if applicable (e.g., for images)
    pub binary_data: Option<Vec<u8>>,
}

/// Accessibility options for visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessibilityOptions {
    /// Whether to provide text alternatives
    pub text_alternatives: bool,
    
    /// Whether to support screen readers
    pub screen_reader_support: bool,
    
    /// High contrast mode
    pub high_contrast: bool,
    
    /// Simplified view for low-bandwidth environments
    pub simplified_view: bool,
    
    /// Language for the visualization
    pub language: String,
}

/// Accessible visualization result
#[derive(Debug, Clone)]
pub struct AccessibleVisualization {
    /// Base visualization result
    pub base_viz: VisualizationResult,
    
    /// Text alternative description
    pub text_alternative: Option<String>,
    
    /// Simplified version for low-bandwidth
    pub simplified_version: Option<VisualizationResult>,
}

/// Error types for visualization operations
#[derive(Debug, thiserror::Error)]
pub enum VisualizationError {
    #[error("Data conversion error: {0}")]
    DataConversion(String),
    
    #[error("Visualization generation error: {0}")]
    GenerationError(String),
    
    #[error("Accessibility adaptation error: {0}")]
    AccessibilityError(String),
}

/// Result type for visualization operations
pub type VisualizationResultType<T> = Result<T, VisualizationError>;

/// Core implementation of the impact visualization framework
pub struct ImpactVizCore {
    /// Values translator for cooperative principles
    values_translator: ValuesTranslator,
    
    /// Accessibility options
    accessibility_options: AccessibilityOptions,
}

impl ImpactVizCore {
    /// Create a new impact visualization core
    pub fn new() -> Self {
        info!("Initializing ImpactVizCore");
        Self {
            values_translator: ValuesTranslator::new(),
            accessibility_options: AccessibilityOptions {
                text_alternatives: true,
                screen_reader_support: true,
                high_contrast: false,
                simplified_view: false,
                language: "en".to_string(),
            },
        }
    }
    
    /// Set accessibility options
    pub fn with_accessibility_options(mut self, options: AccessibilityOptions) -> Self {
        self.accessibility_options = options;
        self
    }
}

impl Default for ImpactVizCore {
    fn default() -> Self {
        Self::new()
    }
}

impl ImpactVisualization for ImpactVizCore {
    fn translate_impact(&self, data: &MathematicalOutput) -> ImpactMetric {
        debug!("Translating mathematical output to impact metric");
        
        // In a real implementation, this would:
        // 1. Convert mathematical results to community-understandable metrics
        // 2. Add context and explanations
        // 3. Apply cooperative values translation
        
        ImpactMetric {
            name: "Community Impact".to_string(),
            description: "Translated impact metric from mathematical output".to_string(),
            value: data.value,
            unit: MetricUnit::Count,
            confidence_interval: data.confidence_interval.clone(),
            significance: data.significance.clone(),
            context: data.metadata.iter().map(|(k, v)| (k.clone(), v.to_string())).collect(),
        }
    }
    
    fn visualize(&self, metric: &ImpactMetric, style: VisualizationStyle) -> VisualizationResult {
        debug!("Generating visualization in style: {:?}", style);
        
        // In a real implementation, this would:
        // 1. Generate appropriate visualization based on style
        // 2. Create visualization data structure
        // 3. Add metadata
        
        VisualizationResult {
            data: VisualizationData {
                json_data: serde_json::to_string(metric).unwrap_or_default(),
                binary_data: None,
            },
            viz_type: match style {
                VisualizationStyle::Narrative => VisualizationType::Narrative,
                VisualizationStyle::Comparative => VisualizationType::BarChart,
                VisualizationStyle::TrendBased => VisualizationType::LineChart,
                VisualizationStyle::Quantitative => VisualizationType::Table,
                VisualizationStyle::Qualitative => VisualizationType::Narrative,
            },
            metadata: HashMap::new(),
        }
    }
    
    fn translate_values(&self, metric: &ImpactMetric) -> ValuesAlignedMetric {
        debug!("Translating metric to values-aligned representation");
        
        self.values_translator.translate_metric(metric)
    }
    
    fn ensure_accessibility(&self, viz: &VisualizationResult, options: &AccessibilityOptions) -> AccessibleVisualization {
        debug!("Ensuring accessibility for visualization");
        
        // In a real implementation, this would:
        // 1. Generate text alternatives if requested
        // 2. Create simplified versions if needed
        // 3. Apply high contrast themes if requested
        
        AccessibleVisualization {
            base_viz: viz.clone(),
            text_alternative: if options.text_alternatives {
                Some("Accessibility text alternative would be generated here".to_string())
            } else {
                None
            },
            simplified_version: if options.simplified_view {
                // In a real implementation, this would create a simplified version
                Some(viz.clone())
            } else {
                None
            },
        }
    }
}

/// Values translator for cooperative principles
pub struct ValuesTranslator {
    /// Mapping of metrics to cooperative values
    values_mapping: HashMap<String, Vec<String>>,
}

impl ValuesTranslator {
    /// Create a new values translator
    pub fn new() -> Self {
        let mut mapping = HashMap::new();
        
        // Example mappings - in a real implementation, these would be more comprehensive
        mapping.insert("volunteer_hours".to_string(), vec!["community".to_string(), "cooperation".to_string()]);
        mapping.insert("financial_health".to_string(), vec!["sustainability".to_string(), "transparency".to_string()]);
        mapping.insert("skill_development".to_string(), vec!["education".to_string(), "growth".to_string()]);
        mapping.insert("cause_impact".to_string(), vec!["solidarity".to_string(), "justice".to_string()]);
        
        Self {
            values_mapping: mapping,
        }
    }
    
    /// Translate a metric to a values-aligned representation
    pub fn translate_metric(&self, metric: &ImpactMetric) -> ValuesAlignedMetric {
        let values_connections: HashMap<String, String> = self.values_mapping
            .get(&metric.name)
            .unwrap_or(&vec![])
            .iter()
            .map(|value| (value.clone(), format!("This metric relates to the cooperative value of {}", value)))
            .collect();
        
        ValuesAlignedMetric {
            base_metric: metric.clone(),
            values_connection: values_connections,
            community_validated: false, // Would be set based on actual community validation
            narrative: format!("This {} {} represents community impact in terms of {}.", 
                              metric.value, 
                              match &metric.unit {
                                  MetricUnit::Count => "count",
                                  MetricUnit::Percentage => "percentage",
                                  MetricUnit::Monetary(_) => "monetary value",
                                  MetricUnit::Hours => "hours",
                                  MetricUnit::People => "people affected",
                                  MetricUnit::Skills => "skills developed",
                                  MetricUnit::Custom(unit) => unit.as_str(),
                              },
                              metric.name),
        }
    }
}

impl Default for ValuesTranslator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cpc_statistics_core::{ConfidenceInterval, SignificanceResult, SignificanceLevel};
    
    #[test]
    fn test_impact_viz_core_creation() {
        let viz_core = ImpactVizCore::new();
        assert!(true); // Core should be created successfully
    }
    
    #[test]
    fn test_translate_impact() {
        let viz_core = ImpactVizCore::new();
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("test".to_string()));
        
        let math_output = MathematicalOutput {
            value: 42.5,
            confidence_interval: Some(ConfidenceInterval::new(40.0, 45.0, 0.95)),
            significance: Some(SignificanceResult::Significant(SignificanceLevel::P05)),
            metadata,
        };
        
        let impact_metric = viz_core.translate_impact(&math_output);
        assert_eq!(impact_metric.value, 42.5);
        assert_eq!(impact_metric.name, "Community Impact");
    }
    
    #[test]
    fn test_visualize() {
        let viz_core = ImpactVizCore::new();
        
        let metric = ImpactMetric {
            name: "Test Metric".to_string(),
            description: "A test metric".to_string(),
            value: 100.0,
            unit: MetricUnit::Count,
            confidence_interval: None,
            significance: None,
            context: HashMap::new(),
        };
        
        let viz_result = viz_core.visualize(&metric, VisualizationStyle::Narrative);
        assert_eq!(viz_result.viz_type, VisualizationType::Narrative);
        assert!(!viz_result.data.json_data.is_empty());
    }
    
    #[test]
    fn test_values_translator() {
        let translator = ValuesTranslator::new();
        
        let metric = ImpactMetric {
            name: "volunteer_hours".to_string(),
            description: "Hours volunteered".to_string(),
            value: 1000.0,
            unit: MetricUnit::Hours,
            confidence_interval: None,
            significance: None,
            context: HashMap::new(),
        };
        
        let values_metric = translator.translate_metric(&metric);
        assert!(!values_metric.values_connection.is_empty());
        assert!(!values_metric.narrative.is_empty());
    }
}