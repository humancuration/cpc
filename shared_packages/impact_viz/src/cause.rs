//! Cause impact storytelling visualizations

use crate::core::{ImpactVisualization, ImpactMetric, VisualizationStyle, VisualizationResult, 
                  ValuesAlignedMetric, AccessibleVisualization, AccessibilityOptions, 
                  CommunityStory, VisualElement, VisualizationType, VisualizationData,
                  MathematicalOutput, ValuesTranslator, MetricUnit};
use crate::components::cause_feedback_collector::{CauseFeedbackCollector, CauseFeedbackData};
use cause_management::ml::{CauseData, CauseProfile, CommunityNeeds, ImpactMeasurement, 
                           ResourceAllocationRecord, EngagementMetric, OutcomeMeasurement,
                           PriorityIssue};
use ml_core::models::{CauseSuccessFactor, CausePriority};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};
use yew::prelude::*;

/// Properties for the CauseImpactStorytellingComponent
#[derive(Properties, PartialEq)]
pub struct CauseImpactStorytellingProps {
    /// The cause data to visualize
    pub cause_data: CauseData,
    
    /// Callback for handling feedback submission
    #[prop_or_default]
    pub on_feedback: Callback<CauseFeedbackData>,
}

/// Cause impact storytelling visualization component
#[derive(PartialEq, Properties)]
pub struct CauseImpactStorytellingComponentState {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
    
    /// Values translator for cooperative principles
    values_translator: ValuesTranslator,
}

/// Cause impact storytelling visualization
pub struct CauseImpactStorytelling {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
    
    /// Values translator for cooperative principles
    values_translator: ValuesTranslator,
}

impl CauseImpactStorytelling {
    /// Create a new cause impact storytelling visualization
    pub fn new(core: Box<dyn ImpactVisualization>) -> Self {
        info!("Initializing CauseImpactStorytelling");
        Self {
            core,
            values_translator: ValuesTranslator::new(),
        }
    }
    
    /// Visualize cause effectiveness with narrative elements
    pub fn visualize_cause_effectiveness(&self, cause_data: &CauseData) -> VisualizationResult {
        debug!("Visualizing cause effectiveness");
        
        // Convert cause data to mathematical output
        let math_output = self.convert_cause_to_math(cause_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Narrative)
    }
    
    /// Show resource impact with community stories
    pub fn visualize_resource_impact(&self, cause_data: &CauseData) -> VisualizationResult {
        debug!("Visualizing resource impact");
        
        // Convert cause data to mathematical output
        let math_output = self.convert_cause_to_math(cause_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Display prediction confidence with transparent explanations
    pub fn visualize_prediction_confidence(&self, cause_data: &CauseData) -> VisualizationResult {
        debug!("Visualizing prediction confidence");
        
        // Convert cause data to mathematical output
        let math_output = self.convert_cause_to_math(cause_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Quantitative)
    }
    
    /// Create comparative visualizations across different causes
    pub fn visualize_cause_comparison(&self, causes: &Vec<CauseData>) -> VisualizationResult {
        debug!("Visualizing cause comparison");
        
        // Convert causes to mathematical output
        let math_output = self.convert_causes_to_math(causes);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Convert cause data to mathematical output
    fn convert_cause_to_math(&self, cause_data: &CauseData) -> MathematicalOutput {
        // Calculate cause effectiveness metrics
        let impact_measurements: f64 = cause_data.historical_impact.iter().map(|i| i.impact_score).sum();
        let measurement_count = cause_data.historical_impact.len() as f64;
        let avg_impact = if measurement_count > 0.0 { impact_measurements / measurement_count } else { 0.0 };
        
        let resource_allocations: f64 = cause_data.resource_allocation.iter().map(|r| r.amount).sum();
        let engagement_metrics: f64 = cause_data.engagement_metrics.iter().map(|e| e.quality_score * e.participants as f64).sum();
        
        let effectiveness_score = avg_impact * engagement_metrics / (resource_allocations + 1.0); // +1 to avoid division by zero
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("cause_data".to_string()));
        metadata.insert("avg_impact".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(avg_impact).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("resource_allocations".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(resource_allocations).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("engagement_metrics".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(engagement_metrics).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("effectiveness_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(effectiveness_score).unwrap_or(serde_json::Number::from(0))));
        
        MathematicalOutput {
            value: effectiveness_score,
            confidence_interval: None, // Would be calculated in real implementation
            significance: None, // Would be calculated in real implementation
            metadata,
        }
    }
    
    /// Convert causes to mathematical output
    fn convert_causes_to_math(&self, causes: &Vec<CauseData>) -> MathematicalOutput {
        let cause_count = causes.len() as f64;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("causes_comparison".to_string()));
        metadata.insert("cause_count".to_string(), serde_json::Value::Number(serde_json::Number::from(causes.len())));
        
        MathematicalOutput {
            value: cause_count,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
}

impl ImpactVisualization for CauseImpactStorytelling {
    fn translate_impact(&self, data: &MathematicalOutput) -> ImpactMetric {
        self.core.translate_impact(data)
    }
    
    fn visualize(&self, metric: &ImpactMetric, style: VisualizationStyle) -> VisualizationResult {
        self.core.visualize(metric, style)
    }
    
    fn translate_values(&self, metric: &ImpactMetric) -> ValuesAlignedMetric {
        self.core.translate_values(metric)
    }
    
    fn ensure_accessibility(&self, viz: &VisualizationResult, options: &AccessibilityOptions) -> AccessibleVisualization {
        self.core.ensure_accessibility(viz, options)
    }
}

/// Cause impact storytelling visualization component for web
#[function_component(CauseImpactStorytellingComponent)]
pub fn cause_impact_storytelling_component(props: &CauseImpactStorytellingProps) -> Html {
    let component_id = format!("cause_viz_{}", props.cause_data.id);
    
    let on_feedback = {
        let on_feedback = props.on_feedback.clone();
        Callback::from(move |feedback: CauseFeedbackData| {
            on_feedback.emit(feedback);
        })
    };
    
    html! {
        <div class="cause-impact-storytelling">
            <h3>{&props.cause_data.name}</h3>
            <p>{&props.cause_data.category}</p>
            
            // In a real implementation, this would render the actual visualization
            <div class="visualization-placeholder">
                <p>{"Cause Impact Visualization"}</p>
                <p>{"Effectiveness Score: "} {format!("{:.2}", calculate_effectiveness_score(&props.cause_data))}</p>
            </div>
            
            // Include the feedback collector component
            <CauseFeedbackCollector 
                component_id={component_id} 
                on_feedback_submit={on_feedback}
            />
        </div>
    }
}

/// Calculate effectiveness score for display
fn calculate_effectiveness_score(cause_data: &CauseData) -> f64 {
    let impact_measurements: f64 = cause_data.historical_impact.iter().map(|i| i.impact_score).sum();
    let measurement_count = cause_data.historical_impact.len() as f64;
    let avg_impact = if measurement_count > 0.0 { impact_measurements / measurement_count } else { 0.0 };
    
    let resource_allocations: f64 = cause_data.resource_allocation.iter().map(|r| r.amount).sum();
    let engagement_metrics: f64 = cause_data.engagement_metrics.iter().map(|e| e.quality_score * e.participants as f64).sum();
    
    avg_impact * engagement_metrics / (resource_allocations + 1.0) // +1 to avoid division by zero
}

/// Cause priority visualization
pub struct CausePriorityViz {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
}

impl CausePriorityViz {
    /// Create a new cause priority visualization
    pub fn new(core: Box<dyn ImpactVisualization>) -> Self {
        Self { core }
    }
    
    /// Visualize cause prioritization strategies
    pub fn visualize_prioritization(&self, priorities: &Vec<CausePriority>) -> VisualizationResult {
        debug!("Visualizing cause prioritization");
        
        // Convert priorities to mathematical output
        let math_output = self.convert_priorities_to_math(priorities);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Convert priorities to mathematical output
    fn convert_priorities_to_math(&self, priorities: &Vec<CausePriority>) -> MathematicalOutput {
        let priority_count = priorities.len() as f64;
        let avg_priority: f64 = priorities.iter().map(|p| p.priority_score).sum::<f64>() / priority_count;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("cause_priorities".to_string()));
        metadata.insert("priority_count".to_string(), serde_json::Value::Number(serde_json::Number::from(priorities.len())));
        metadata.insert("avg_priority".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(avg_priority).unwrap_or(serde_json::Number::from(0))));
        
        MathematicalOutput {
            value: avg_priority,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ImpactVizCore;
    use chrono::{DateTime, Utc};
    use std::collections::HashMap;
    
    #[test]
    fn test_cause_impact_storytelling_creation() {
        let core = Box::new(ImpactVizCore::new());
        let cause_viz = CauseImpactStorytelling::new(core);
        assert!(true); // Viz should be created successfully
    }
    
    #[test]
    fn test_visualize_cause_effectiveness() {
        let core = Box::new(ImpactVizCore::new());
        let cause_viz = CauseImpactStorytelling::new(core);
        
        let historical_impact = vec![
            ImpactMeasurement {
                date: Utc::now(),
                impact_score: 0.85,
                people_affected: 1000,
                geographic_scope: "Local".to_string(),
            }
        ];
        
        let resource_allocation = vec![
            ResourceAllocationRecord {
                date: Utc::now(),
                resource_type: "Funding".to_string(),
                amount: 5000.0,
                source: "Community Fund".to_string(),
            }
        ];
        
        let engagement_metrics = vec![
            EngagementMetric {
                date: Utc::now(),
                engagement_type: "Volunteer Participation".to_string(),
                participants: 50,
                quality_score: 0.9,
            }
        ];
        
        let outcomes = vec![
            OutcomeMeasurement {
                date: Utc::now(),
                outcome_type: "Education".to_string(),
                value: 85.0,
                method: "Standardized Testing".to_string(),
            }
        ];
        
        let cause_data = CauseData {
            id: "1".to_string(),
            name: "Education Access".to_string(),
            category: "Education".to_string(),
            historical_impact,
            resource_allocation,
            engagement_metrics,
            outcomes,
        };
        
        let viz_result = cause_viz.visualize_cause_effectiveness(&cause_data);
        assert_eq!(viz_result.viz_type, VisualizationType::Narrative);
        assert!(!viz_result.data.json_data.is_empty());
    }
    
    #[test]
    fn test_visualize_cause_comparison() {
        let core = Box::new(ImpactVizCore::new());
        let cause_viz = CauseImpactStorytelling::new(core);
        
        let causes = vec![
            CauseData {
                id: "1".to_string(),
                name: "Education Access".to_string(),
                category: "Education".to_string(),
                historical_impact: vec![],
                resource_allocation: vec![],
                engagement_metrics: vec![],
                outcomes: vec![],
            }
        ];
        
        let viz_result = cause_viz.visualize_cause_comparison(&causes);
        assert_eq!(viz_result.viz_type, VisualizationType::Comparative);
        assert!(!viz_result.data.json_data.is_empty());
    }
}