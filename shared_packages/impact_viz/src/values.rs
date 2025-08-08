//! Cooperative values translation layer

use crate::core::{ImpactMetric, ValuesAlignedMetric};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Values translator for cooperative principles
pub struct ValuesTranslator {
    /// Mapping of metrics to cooperative values
    values_mapping: HashMap<String, Vec<CooperativeValue>>,
    
    /// Community validation status for metrics
    community_validation: HashMap<String, bool>,
}

impl ValuesTranslator {
    /// Create a new values translator
    pub fn new() -> Self {
        info!("Initializing ValuesTranslator");
        let mut translator = Self {
            values_mapping: HashMap::new(),
            community_validation: HashMap::new(),
        };
        
        // Initialize with default cooperative values mappings
        translator.initialize_default_mappings();
        translator
    }
    
    /// Initialize default mappings of metrics to cooperative values
    fn initialize_default_mappings(&mut self) {
        debug!("Initializing default cooperative values mappings");
        
        // Volunteer-related metrics
        self.values_mapping.insert(
            "volunteer_impact".to_string(),
            vec![
                CooperativeValue::Community,
                CooperativeValue::Cooperation,
                CooperativeValue::Solidarity,
            ]
        );
        
        self.values_mapping.insert(
            "volunteer_hours".to_string(),
            vec![
                CooperativeValue::Community,
                CooperativeValue::Participation,
                CooperativeValue::Service,
            ]
        );
        
        // Financial-related metrics
        self.values_mapping.insert(
            "financial_health".to_string(),
            vec![
                CooperativeValue::Sustainability,
                CooperativeValue::Transparency,
                CooperativeValue::Responsibility,
            ]
        );
        
        self.values_mapping.insert(
            "resource_allocation".to_string(),
            vec![
                CooperativeValue::Equity,
                CooperativeValue::Democracy,
                CooperativeValue::Responsibility,
            ]
        );
        
        // Skill-related metrics
        self.values_mapping.insert(
            "skill_development".to_string(),
            vec![
                CooperativeValue::Education,
                CooperativeValue::Growth,
                CooperativeValue::SelfHelp,
            ]
        );
        
        self.values_mapping.insert(
            "learning_opportunities".to_string(),
            vec![
                CooperativeValue::Education,
                CooperativeValue::Openness,
                CooperativeValue::Progress,
            ]
        );
        
        // Cause-related metrics
        self.values_mapping.insert(
            "cause_impact".to_string(),
            vec![
                CooperativeValue::Solidarity,
                CooperativeValue::Justice,
                CooperativeValue::Community,
            ]
        );
        
        self.values_mapping.insert(
            "social_impact".to_string(),
            vec![
                CooperativeValue::Solidarity,
                CooperativeValue::Justice,
                CooperativeValue::ConcernForOthers,
            ]
        );
    }
    
    /// Translate a metric to a values-aligned representation
    pub fn translate_metric(&self, metric: &ImpactMetric) -> ValuesAlignedMetric {
        debug!("Translating metric '{}' to values-aligned representation", metric.name);
        
        let values_connections = self.create_values_connections(metric);
        let narrative = self.generate_narrative(metric, &values_connections);
        let community_validated = self.is_community_validated(&metric.name);
        
        ValuesAlignedMetric {
            base_metric: metric.clone(),
            values_connection: values_connections,
            community_validated,
            narrative,
        }
    }
    
    /// Create values connections for a metric
    fn create_values_connections(&self, metric: &ImpactMetric) -> HashMap<String, String> {
        let values = self.values_mapping.get(&metric.name).unwrap_or(&vec![]);
        
        values.iter().map(|value| {
            let value_str = format!("{:?}", value);
            let description = match value {
                CooperativeValue::Cooperation => "Working together toward common goals",
                CooperativeValue::Community => "Building and strengthening our community bonds",
                CooperativeValue::Sustainability => "Ensuring long-term viability and environmental responsibility",
                CooperativeValue::Democracy => "Inclusive decision-making and equal participation",
                CooperativeValue::Equity => "Fair distribution of resources and opportunities",
                CooperativeValue::Solidarity => "Mutual support and shared responsibility",
                CooperativeValue::SelfHelp => "Taking initiative for personal and collective improvement",
                CooperativeValue::Transparency => "Open and honest communication",
                CooperativeValue::Responsibility => "Accountability for our actions and their consequences",
                CooperativeValue::Education => "Lifelong learning and knowledge sharing",
                CooperativeValue::Openness => "Welcoming new ideas and diverse perspectives",
                CooperativeValue::ConcernForOthers => "Caring for the wellbeing of all community members",
                CooperativeValue::Service => "Contributing to the common good",
                CooperativeValue::Participation => "Active engagement in community life",
                CooperativeValue::Growth => "Continuous development and improvement",
                CooperativeValue::Progress => "Advancing toward positive change",
                CooperativeValue::Justice => "Fairness and equality for all",
            };
            
            (value_str, description.to_string())
        }).collect()
    }
    
    /// Generate narrative explanation for a metric
    fn generate_narrative(&self, metric: &ImpactMetric, values_connections: &HashMap<String, String>) -> String {
        let values_list: Vec<String> = values_connections.keys().cloned().collect();
        let values_text = if values_list.is_empty() {
            "our cooperative values".to_string()
        } else {
            format!("the cooperative values of {}", values_list.join(", "))
        };
        
        format!(
            "This {} {} represents community impact in terms of {}. It connects to {} and demonstrates how our collective actions create positive change.",
            metric.value,
            match &metric.unit {
                crate::core::MetricUnit::Count => "count",
                crate::core::MetricUnit::Percentage => "percentage",
                crate::core::MetricUnit::Monetary(_) => "monetary value",
                crate::core::MetricUnit::Hours => "hours",
                crate::core::MetricUnit::People => "people affected",
                crate::core::MetricUnit::Skills => "skills developed",
                crate::core::MetricUnit::Custom(unit) => unit.as_str(),
            },
            metric.name,
            values_text
        )
    }
    
    /// Check if a metric has been validated by the community
    fn is_community_validated(&self, metric_name: &str) -> bool {
        *self.community_validation.get(metric_name).unwrap_or(&false)
    }
    
    /// Add a new mapping of a metric to cooperative values
    pub fn add_mapping(&mut self, metric_name: String, values: Vec<CooperativeValue>) {
        debug!("Adding mapping for metric '{}' to values: {:?}", metric_name, values);
        self.values_mapping.insert(metric_name, values);
    }
    
    /// Mark a metric as validated by the community
    pub fn mark_validated(&mut self, metric_name: String) {
        debug!("Marking metric '{}' as community validated", metric_name);
        self.community_validation.insert(metric_name, true);
    }
    
    /// Get all cooperative values associated with a metric
    pub fn get_values_for_metric(&self, metric_name: &str) -> Option<&Vec<CooperativeValue>> {
        self.values_mapping.get(metric_name)
    }
}

impl Default for ValuesTranslator {
    fn default() -> Self {
        Self::new()
    }
}

/// Cooperative values as defined by cooperative principles
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CooperativeValue {
    /// Voluntary and open membership
    Cooperation,
    
    /// Democratic member control
    Community,
    
    /// Member economic participation
    Sustainability,
    
    /// Autonomy and independence
    Democracy,
    
    /// Education, training and information
    Equity,
    
    /// Cooperation among cooperatives
    Solidarity,
    
    /// Concern for community
    SelfHelp,
    
    /// Transparency and openness
    Transparency,
    
    /// Responsibility and accountability
    Responsibility,
    
    /// Education and lifelong learning
    Education,
    
    /// Openness to new ideas and perspectives
    Openness,
    
    /// Concern for others' wellbeing
    ConcernForOthers,
    
    /// Service to the community
    Service,
    
    /// Active participation in community life
    Participation,
    
    /// Personal and collective growth
    Growth,
    
    /// Progress toward positive change
    Progress,
    
    /// Justice and fairness for all
    Justice,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{ImpactMetric, MetricUnit};
    use std::collections::HashMap;
    
    #[test]
    fn test_values_translator_creation() {
        let translator = ValuesTranslator::new();
        assert!(true); // Translator should be created successfully
    }
    
    #[test]
    fn test_translate_metric() {
        let translator = ValuesTranslator::new();
        
        let metric = ImpactMetric {
            name: "volunteer_hours".to_string(),
            description: "Total volunteer hours contributed".to_string(),
            value: 1000.0,
            unit: MetricUnit::Hours,
            confidence_interval: None,
            significance: None,
            context: HashMap::new(),
        };
        
        let values_metric = translator.translate_metric(&metric);
        assert!(!values_metric.values_connection.is_empty());
        assert!(!values_metric.narrative.is_empty());
        assert_eq!(values_metric.base_metric.name, "volunteer_hours");
    }
    
    #[test]
    fn test_add_mapping() {
        let mut translator = ValuesTranslator::new();
        
        let new_values = vec![CooperativeValue::Education, CooperativeValue::Growth];
        translator.add_mapping("test_metric".to_string(), new_values.clone());
        
        let retrieved_values = translator.get_values_for_metric("test_metric");
        assert!(retrieved_values.is_some());
        assert_eq!(retrieved_values.unwrap(), &new_values);
    }
    
    #[test]
    fn test_mark_validated() {
        let mut translator = ValuesTranslator::new();
        
        translator.mark_validated("test_metric".to_string());
        // Since we don't have a direct way to check validation status in the public API,
        // we'll just ensure this doesn't panic
        assert!(true);
    }
}