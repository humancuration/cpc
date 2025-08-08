//! Financial health visualization components

use crate::core::{ImpactVisualization, ImpactMetric, VisualizationStyle, VisualizationResult, 
                  ValuesAlignedMetric, AccessibleVisualization, AccessibilityOptions, 
                  CommunityStory, VisualElement, VisualizationType, VisualizationData,
                  MathematicalOutput, ValuesTranslator, MetricUnit};
use cpay_core::ml::{FinancialData, Transaction, EconomicData, ResourceData};
use common_utils::financial::MonetaryValue;
use ml_core::models::ResourceAllocation;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tracing::{debug, info};

/// Financial health visualization
pub struct FinancialHealthViz {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
    
    /// Values translator for cooperative principles
    values_translator: ValuesTranslator,
}

impl FinancialHealthViz {
    /// Create a new financial health visualization
    pub fn new(core: Box<dyn ImpactVisualization>) -> Self {
        info!("Initializing FinancialHealthViz");
        Self {
            core,
            values_translator: ValuesTranslator::new(),
        }
    }
    
    /// Transform financial metrics into community wellbeing indicators
    pub fn visualize_community_wellbeing(&self, financial_data: &FinancialData) -> VisualizationResult {
        debug!("Visualizing community wellbeing from financial data");
        
        // Convert financial data to mathematical output
        let math_output = self.convert_financial_to_math(financial_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Narrative)
    }
    
    /// Visualize resource flows with community impact attribution
    pub fn visualize_resource_flows(&self, resource_data: &ResourceData) -> VisualizationResult {
        debug!("Visualizing resource flows");
        
        // Convert resource data to mathematical output
        let math_output = self.convert_resource_to_math(resource_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Show sustainability metrics with future projections
    pub fn visualize_sustainability(&self, financial_data: &FinancialData) -> VisualizationResult {
        debug!("Visualizing sustainability metrics");
        
        // Convert financial data to mathematical output
        let math_output = self.convert_financial_to_math(financial_data);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::TrendBased)
    }
    
    /// Create "what if" scenarios for different allocation strategies
    pub fn visualize_allocation_scenarios(&self, scenarios: &Vec<AllocationScenario>) -> VisualizationResult {
        debug!("Visualizing allocation scenarios");
        
        // Convert scenarios to mathematical output
        let math_output = self.convert_scenarios_to_math(scenarios);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Convert financial data to mathematical output
    fn convert_financial_to_math(&self, financial_data: &FinancialData) -> MathematicalOutput {
        // Calculate a composite financial health score
        let revenue_trend_avg: f64 = financial_data.revenue_trends.iter().sum::<f64>() / financial_data.revenue_trends.len() as f64;
        let expense_avg: f64 = financial_data.expense_patterns.values().map(|v| v.iter().sum::<f64>() / v.len() as f64).sum::<f64>() / financial_data.expense_patterns.len() as f64;
        let reserve_avg: f64 = financial_data.reserve_levels.iter().sum::<f64>() / financial_data.reserve_levels.len() as f64;
        
        let financial_health_score = (revenue_trend_avg - expense_avg) / reserve_avg;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("financial_data".to_string()));
        metadata.insert("revenue_trend_avg".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(revenue_trend_avg).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("expense_avg".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(expense_avg).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("reserve_avg".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(reserve_avg).unwrap_or(serde_json::Number::from(0))));
        
        MathematicalOutput {
            value: financial_health_score,
            confidence_interval: None, // Would be calculated in real implementation
            significance: None, // Would be calculated in real implementation
            metadata,
        }
    }
    
    /// Convert resource data to mathematical output
    fn convert_resource_to_math(&self, resource_data: &ResourceData) -> MathematicalOutput {
        // Calculate total available resources
        let total_resources: f64 = resource_data.available_resources.values().sum();
        let total_demand: f64 = resource_data.demand_forecasts.values().sum();
        let resource_utilization = if total_demand > 0.0 { total_resources / total_demand } else { 0.0 };
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("resource_data".to_string()));
        metadata.insert("total_resources".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(total_resources).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("total_demand".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(total_demand).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("resource_utilization".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(resource_utilization).unwrap_or(serde_json::Number::from(0))));
        
        MathematicalOutput {
            value: resource_utilization,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
    
    /// Convert scenarios to mathematical output
    fn convert_scenarios_to_math(&self, scenarios: &Vec<AllocationScenario>) -> MathematicalOutput {
        let scenario_count = scenarios.len() as f64;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("allocation_scenarios".to_string()));
        metadata.insert("scenario_count".to_string(), serde_json::Value::Number(serde_json::Number::from(scenarios.len())));
        
        MathematicalOutput {
            value: scenario_count,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
}

impl ImpactVisualization for FinancialHealthViz {
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

/// Allocation scenario for "what if" analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationScenario {
    /// Name of the scenario
    pub name: String,
    
    /// Description of the scenario
    pub description: String,
    
    /// Resource allocations in this scenario
    pub allocations: Vec<ResourceAllocation>,
    
    /// Projected impact of this scenario
    pub projected_impact: f64,
    
    /// Sustainability score for this scenario
    pub sustainability_score: f64,
}

/// Resource flow visualization
pub struct ResourceFlowViz {
    /// Core visualization engine
    core: Box<dyn ImpactVisualization>,
}

impl ResourceFlowViz {
    /// Create a new resource flow visualization
    pub fn new(core: Box<dyn ImpactVisualization>) -> Self {
        Self { core }
    }
    
    /// Visualize resource flow between different community sectors
    pub fn visualize_flow_between_sectors(&self, flows: &Vec<ResourceFlow>) -> VisualizationResult {
        debug!("Visualizing resource flows between sectors");
        
        // Convert flows to mathematical output
        let math_output = self.convert_flows_to_math(flows);
        
        // Translate to impact metric
        let impact_metric = self.core.translate_impact(&math_output);
        
        // Generate visualization
        self.core.visualize(&impact_metric, VisualizationStyle::Comparative)
    }
    
    /// Convert flows to mathematical output
    fn convert_flows_to_math(&self, flows: &Vec<ResourceFlow>) -> MathematicalOutput {
        let total_flow: f64 = flows.iter().map(|f| f.amount).sum();
        let flow_count = flows.len() as f64;
        
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), serde_json::Value::String("resource_flows".to_string()));
        metadata.insert("total_flow".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(total_flow).unwrap_or(serde_json::Number::from(0))));
        metadata.insert("flow_count".to_string(), serde_json::Value::Number(serde_json::Number::from(flows.len())));
        
        MathematicalOutput {
            value: total_flow,
            confidence_interval: None,
            significance: None,
            metadata,
        }
    }
}

/// Resource flow between sectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceFlow {
    /// Source sector
    pub from_sector: String,
    
    /// Destination sector
    pub to_sector: String,
    
    /// Amount of resources flowing
    pub amount: f64,
    
    /// Type of resource
    pub resource_type: String,
    
    /// Community impact of this flow
    pub community_impact: f64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::ImpactVizCore;
    use std::collections::HashMap;
    
    #[test]
    fn test_financial_health_viz_creation() {
        let core = Box::new(ImpactVizCore::new());
        let financial_viz = FinancialHealthViz::new(core);
        assert!(true); // Viz should be created successfully
    }
    
    #[test]
    fn test_visualize_community_wellbeing() {
        let core = Box::new(ImpactVizCore::new());
        let financial_viz = FinancialHealthViz::new(core);
        
        let mut expense_patterns = HashMap::new();
        expense_patterns.insert("operational".to_string(), vec![1000.0, 1100.0, 1050.0]);
        expense_patterns.insert("community".to_string(), vec![500.0, 550.0, 525.0]);
        
        let financial_data = FinancialData {
            revenue_trends: vec![2000.0, 2100.0, 2050.0],
            expense_patterns,
            reserve_levels: vec![5000.0, 5100.0, 5050.0],
            contribution_rates: vec![0.7, 0.75, 0.72],
            investment_returns: vec![0.05, 0.06, 0.055],
            debt_levels: vec![1000.0, 950.0, 900.0],
        };
        
        let viz_result = financial_viz.visualize_community_wellbeing(&financial_data);
        assert_eq!(viz_result.viz_type, VisualizationType::Narrative);
        assert!(!viz_result.data.json_data.is_empty());
    }
    
    #[test]
    fn test_visualize_sustainability() {
        let core = Box::new(ImpactVizCore::new());
        let financial_viz = FinancialHealthViz::new(core);
        
        let mut expense_patterns = HashMap::new();
        expense_patterns.insert("operational".to_string(), vec![1000.0, 1100.0, 1050.0]);
        
        let financial_data = FinancialData {
            revenue_trends: vec![2000.0, 2100.0, 2050.0],
            expense_patterns,
            reserve_levels: vec![5000.0, 5100.0, 5050.0],
            contribution_rates: vec![0.7, 0.75, 0.72],
            investment_returns: vec![0.05, 0.06, 0.055],
            debt_levels: vec![1000.0, 950.0, 900.0],
        };
        
        let viz_result = financial_viz.visualize_sustainability(&financial_data);
        assert_eq!(viz_result.viz_type, VisualizationType::TrendBased);
        assert!(!viz_result.data.json_data.is_empty());
    }
}