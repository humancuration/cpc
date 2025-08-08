//! Skill development mapping visualizations

use crate::core::{ImpactVisualization, ImpactMetric, VisualizationStyle, VisualizationResult, 
                  ValuesAlignedMetric, AccessibleVisualization, AccessibilityOptions, 
                  CommunityStory, VisualElement, VisualizationType, VisualizationData,
                  MathematicalOutput, ValuesTranslator, MetricUnit};
use skill_development::ml::{SkillData, LearnerProfile, LearningExperience, CommunityData};
use ml_core::models::{LearningPathway, SkillOpportunity};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Skill development visualization
pub struct SkillDevelopmentViz {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
    
    /// Values translator for cooperative principles
    values_translator: ValuesTranslator,
}

impl SkillDevelopmentViz {
    /// Create a new skill development visualization
    pub fn new(core: Box<dyn ImpactVisualization>) -> Self {
        info!("Initializing SkillDevelopmentViz");
        Self {
            core,
            values_translator: ValuesTranslator::new(),
        }
    }
    
    /// Visualize community skill landscapes with gap analysis
    pub fn visualize_skill_landscape(&self, community_data: &CommunityData) -> VisualizationResult {
        debug!("Visualizing community skill landscape");
        
        // Convert community data to mathematical output
        let math_output = self.convert_community_to_math(community_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Show individual skill growth within community context
    pub fn visualize_individual_growth(&self, profile: &LearnerProfile) -> VisualizationResult {
        debug!("Visualizing individual skill growth");
        
        // Convert profile data to mathematical output
        let math_output = self.convert_profile_to_math(profile);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::TrendBased)
    }
    
    /// Map skill development to community needs
    pub fn visualize_skill_needs_mapping(&self, community_data: &CommunityData) -> VisualizationResult {
        debug!("Visualizing skill needs mapping");
        
        // Convert community data to mathematical output
        let math_output = self.convert_community_to_math(community_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Heatmap)
    }
    
    /// Create pathway visualizations connecting learning to impact
    pub fn visualize_learning_pathways(&self, pathways: &Vec<LearningPathway>) -> VisualizationResult {
        debug!("Visualizing learning pathways");
        
        // Convert pathways to mathematical output
        let math_output = self.convert_pathways_to_math(pathways);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Narrative)
    }
    
    /// Convert community data to mathematical output
    fn convert_community_to_math(&self, community_data: &CommunityData) -> MathematicalOutput {
        // Calculate skill distribution metrics
        let total_skills = community_data.skill_distribution.len() as f64;
        let total_needs = community_data.projected_needs.len() as f64;
        let gap_score = (total_needs - total_skills) / total_needs;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("community_data".to_string()));
        metadata.insert("total_skills".to_string(), serde_json::Value::Number(serde_json::Number::from(community_data.skill_distribution.len())));
        metadata.insert("total_needs".to_string(), serde_json::Value::Number(serde_json::Number::from(community_data.projected_needs.len())));
        metadata.insert("gap_score".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(gap_score).unwrap_or(serde_json::Number::from(0))));
        
        MathematicalOutput {
            value: gap_score,
            confidence_interval: None, // Would be calculated in real implementation
            significance: None, // Would be calculated in real implementation
            metadata,
        }
    }
    
    /// Convert profile data to mathematical output
    fn convert_profile_to_math(&self, profile: &LearnerProfile) -> MathematicalOutput {
        // Calculate learner progress metrics
        let current_skill_count = profile.current_skills.len() as f64;
        let goal_count = profile.learning_goals.len() as f64;
        let progress_ratio = if goal_count > 0.0 { current_skill_count / goal_count } else { 0.0 };
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("learner_profile".to_string()));
        metadata.insert("current_skill_count".to_string(), serde_json::Value::Number(serde_json::Number::from(profile.current_skills.len())));
        metadata.insert("goal_count".to_string(), serde_json::Value::Number(serde_json::Number::from(profile.learning_goals.len())));
        metadata.insert("progress_ratio".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(progress_ratio).unwrap_or(serde_json::Number::from(0))));
        
        MathematicalOutput {
            value: progress_ratio,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
    
    /// Convert pathways to mathematical output
    fn convert_pathways_to_math(&self, pathways: &Vec<LearningPathway>) -> MathematicalOutput {
        let pathway_count = pathways.len() as f64;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("learning_pathways".to_string()));
        metadata.insert("pathway_count".to_string(), serde_json::Value::Number(serde_json::Number::from(pathways.len())));
        
        MathematicalOutput {
            value: pathway_count,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
}

impl ImpactVisualization for SkillDevelopmentViz {
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

/// Skill opportunity visualization
pub struct SkillOpportunityViz {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
}

impl SkillOpportunityViz {
    /// Create a new skill opportunity visualization
    pub fn new(core: Box<dyn ImpactVisualization>) -> Self {
        Self { core }
    }
    
    /// Visualize skill-building opportunities for learners
    pub fn visualize_opportunities(&self, opportunities: &Vec<SkillOpportunity>) -> VisualizationResult {
        debug!("Visualizing skill opportunities");
        
        // Convert opportunities to mathematical output
        let math_output = self.convert_opportunities_to_math(opportunities);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Convert opportunities to mathematical output
    fn convert_opportunities_to_math(&self, opportunities: &Vec<SkillOpportunity>) -> MathematicalOutput {
        let opportunity_count = opportunities.len() as f64;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("skill_opportunities".to_string()));
        metadata.insert("opportunity_count".to_string(), serde_json::Value::Number(serde_json::Number::from(opportunities.len())));
        
        MathematicalOutput {
            value: opportunity_count,
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
    use std::collections::HashMap;
    
    #[test]
    fn test_skill_development_viz_creation() {
        let core = Box::new(ImpactVizCore::new());
        let skill_viz = SkillDevelopmentViz::new(core);
        assert!(true); // Viz should be created successfully
    }
    
    #[test]
    fn test_visualize_individual_growth() {
        let core = Box::new(ImpactVizCore::new());
        let skill_viz = SkillDevelopmentViz::new(core);
        
        let mut current_skills = HashMap::new();
        current_skills.insert("Rust Programming".to_string(), 0.8);
        current_skills.insert("Data Analysis".to_string(), 0.6);
        
        let learning_history = vec![
            LearningExperience {
                skill: "Rust Programming".to_string(),
                time_taken: 40.0,
                satisfaction: 8,
                completion_date: chrono::Utc::now(),
            }
        ];
        
        let profile = LearnerProfile {
            current_skills,
            learning_pace: 7,
            learning_styles: vec!["visual".to_string(), "hands-on".to_string()],
            available_time: 10.0,
            learning_goals: vec!["Master Rust".to_string(), "Data Science".to_string()],
            learning_history,
        };
        
        let viz_result = skill_viz.visualize_individual_growth(&profile);
        assert_eq!(viz_result.viz_type, VisualizationType::TrendBased);
        assert!(!viz_result.data.json_data.is_empty());
    }
    
    #[test]
    fn test_visualize_learning_pathways() {
        let core = Box::new(ImpactVizCore::new());
        let skill_viz = SkillDevelopmentViz::new(core);
        
        let pathways = vec![
            LearningPathway {
                skills: vec!["Rust".to_string(), "Data Structures".to_string()],
                estimated_time: 100.0,
                resources_needed: vec!["Book".to_string(), "Online Course".to_string()],
            }
        ];
        
        let viz_result = skill_viz.visualize_learning_pathways(&pathways);
        assert_eq!(viz_result.viz_type, VisualizationType::Narrative);
        assert!(!viz_result.data.json_data.is_empty());
    }
}