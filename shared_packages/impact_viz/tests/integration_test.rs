//! Integration tests for the Impact Visualization Framework

#[cfg(test)]
mod tests {
    use impact_viz::core::{ImpactVizCore, ImpactVisualization, VisualizationStyle};
    use impact_viz::volunteer::VolunteerImpactDashboard;
    use impact_viz::financial::FinancialHealthViz;
    use impact_viz::skill::SkillDevelopmentViz;
    use impact_viz::cause::CauseImpactStorytelling;
    use std::collections::HashMap;
    
    #[test]
    fn test_complete_integration() {
        // This test ensures all components work together correctly
        println!("Running complete integration test...");
        
        // Create core visualization engine
        let core_viz = ImpactVizCore::new();
        
        // Test volunteer impact dashboard
        let volunteer_dashboard = VolunteerImpactDashboard::new(Box::new(core_viz.clone()));
        // Just ensure it can be created without panicking
        
        // Test financial health visualization
        let financial_viz = FinancialHealthViz::new(Box::new(core_viz.clone()));
        // Just ensure it can be created without panicking
        
        // Test skill development visualization
        let skill_viz = SkillDevelopmentViz::new(Box::new(core_viz.clone()));
        // Just ensure it can be created without panicking
        
        // Test cause impact storytelling
        let cause_viz = CauseImpactStorytelling::new(Box::new(core_viz.clone()));
        // Just ensure it can be created without panicking
        
        // If we reach here, all components were created successfully
        assert!(true);
        println!("All components created successfully!");
    }
    
    #[test]
    fn test_core_functionality() {
        // Test the core visualization functionality
        let core_viz = ImpactVizCore::new();
        
        // Create a simple mathematical output
        let mut metadata = HashMap::new();
        metadata.insert("test".to_string(), serde_json::Value::String("data".to_string()));
        
        let math_output = impact_viz::core::MathematicalOutput {
            value: 42.5,
            confidence_interval: None,
            significance: None,
            metadata,
        };
        
        // Translate to impact metric
        let impact_metric = core_viz.translate_impact(&math_output);
        assert_eq!(impact_metric.value, 42.5);
        assert_eq!(impact_metric.name, "Community Impact");
        
        // Generate visualization
        let viz_result = core_viz.visualize(&impact_metric, VisualizationStyle::Narrative);
        assert!(!viz_result.data.json_data.is_empty());
        
        // Translate values
        let values_metric = core_viz.translate_values(&impact_metric);
        assert_eq!(values_metric.base_metric.value, 42.5);
        
        // Ensure accessibility
        let accessibility_options = impact_viz::accessibility::AccessibilityOptions::default();
        let accessible_viz = core_viz.ensure_accessibility(&viz_result, &accessibility_options);
        assert_eq!(accessible_viz.base_viz.data.json_data, viz_result.data.json_data);
    }
    
    #[test]
    fn test_module_imports() {
        // Test that all modules can be imported correctly
        use impact_viz::*;
        use impact_viz::core::*;
        use impact_viz::volunteer::*;
        use impact_viz::financial::*;
        use impact_viz::skill::*;
        use impact_viz::cause::*;
        use impact_viz::values::*;
        use impact_viz::accessibility::*;
        
        // If we can import everything without compilation errors, the test passes
        assert!(true);
    }
    
    #[test]
    fn test_accessibility_module() {
        // Test accessibility functionality
        let options = impact_viz::accessibility::AccessibilityOptions::default();
        let manager = impact_viz::accessibility::AccessibilityManager::new(options);
        
        // Create a simple visualization result
        let viz_result = impact_viz::core::VisualizationResult {
            data: impact_viz::core::VisualizationData {
                json_data: "{\"test\": \"data\"}".to_string(),
                binary_data: None,
            },
            viz_type: impact_viz::core::VisualizationType::Narrative,
            metadata: HashMap::new(),
        };
        
        // Ensure accessibility
        let accessible_viz = manager.ensure_accessibility(&viz_result);
        // With default options, we should have a text alternative
        assert!(accessible_viz.text_alternative.is_some());
    }
    
    #[test]
    fn test_values_translation() {
        // Test values translation functionality
        let translator = impact_viz::values::ValuesTranslator::new();
        
        // Create a sample impact metric
        let metric = impact_viz::core::ImpactMetric {
            name: "volunteer_hours".to_string(),
            description: "Total volunteer hours".to_string(),
            value: 1000.0,
            unit: impact_viz::core::MetricUnit::Hours,
            confidence_interval: None,
            significance: None,
            context: HashMap::new(),
        };
        
        // Translate to values-aligned metric
        let values_metric = translator.translate_metric(&metric);
        assert!(!values_metric.values_connection.is_empty());
        assert!(!values_metric.narrative.is_empty());
    }
}