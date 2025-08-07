//! Yew components for statistical financial analysis
//!
//! This module provides UI components for displaying statistical analysis results
//! with cooperative values-aligned explanations.

use yew::prelude::*;
#[cfg(feature = "statistics")]
use cpc_statistics_core::{
    ConfidenceInterval,
    SignificanceResult,
    SignificanceLevel,
};

/// Props for the StatisticalExplanation component
#[derive(Properties, PartialEq)]
pub struct StatisticalExplanationProps {
    /// The statistical explanation text
    pub explanation: String,
    
    /// Optional methodology source
    #[prop_or_default]
    pub methodology_source: Option<String>,
    
    /// Optional confidence interval information
    #[prop_or_default]
    pub confidence_interval: Option<ConfidenceInterval>,
    
    /// Optional significance result
    #[prop_or_default]
    pub significance_result: Option<SignificanceResult>,
}

/// Component to display statistical explanations with "Explain This" functionality
#[function_component(StatisticalExplanation)]
pub fn statistical_explanation(props: &StatisticalExplanationProps) -> Html {
    let show_details = use_state(|| false);
    let onclick = {
        let show_details = show_details.clone();
        Callback::from(move |_| show_details.set(!*show_details))
    };
    
    html! {
        <div class="statistical-explanation">
            <div class="explanation-summary">
                <p>{ &props.explanation }</p>
                <button 
                    class="explain-button" 
                    onclick={onclick}
                >
                    { "Explain This" }
                </button>
            </div>
            
            if *show_details {
                <div class="explanation-details">
                    if let Some(source) = &props.methodology_source {
                        <div class="methodology-source">
                            <h4>{ "Methodology Source" }</h4>
                            <p>{ source }</p>
                        </div>
                    }
                    
                    if let Some(ci) = &props.confidence_interval {
                        <div class="confidence-interval">
                            <h4>{ "Confidence Interval" }</h4>
                            <p>{ format!("{}% confidence interval: [{:.2}, {:.2}]", 
                                (ci.confidence_level * 100.0) as i32, 
                                ci.lower, 
                                ci.upper) }</p>
                            <p>{ format!("Interval width: {:.2}", ci.width()) }</p>
                        </div>
                    }
                    
                    if let Some(sig) = &props.significance_result {
                        <div class="significance-result">
                            <h4>{ "Significance Test" }</h4>
                            <p>{ format!("p-value: {:.4}", sig.p_value) }</p>
                            <div class="significance-indicator" style={format!("color: {}", sig.level.color())}>
                                { sig.level.description() }
                            </div>
                        </div>
                    }
                    
                    <div class="cooperative-values">
                        <h4>{ "Cooperative Values" }</h4>
                        <p>{ "All statistical methods used are transparent and accessible to cooperative members. \
                            Our analysis prioritizes community benefit over individual returns." }</p>
                    </div>
                </div>
            }
        </div>
    }
}

/// Props for the ForecastVisualization component
#[derive(Properties, PartialEq)]
pub struct ForecastVisualizationProps {
    /// The forecast explanation
    pub explanation: String,
    
    /// Optional chart image data (base64 encoded)
    #[prop_or_default]
    pub chart_data: Option<String>,
    
    /// Statistical explanation props
    pub statistical_props: StatisticalExplanationProps,
}

/// Component to display forecast visualizations with statistical analysis
#[function_component(ForecastVisualization)]
pub fn forecast_visualization(props: &ForecastVisualizationProps) -> Html {
    html! {
        <div class="forecast-visualization">
            <h3>{ "Financial Forecast" }</h3>
            
            if let Some(chart_data) = &props.chart_data {
                <div class="chart-container">
                    <img src={format!("data:image/png;base64,{}", chart_data)} alt="Forecast Chart" />
                </div>
            }
            
            <StatisticalExplanation 
                explanation={props.explanation.clone()}
                methodology_source={props.statistical_props.methodology_source.clone()}
                confidence_interval={props.statistical_props.confidence_interval.clone()}
                significance_result={props.statistical_props.significance_result.clone()}
            />
        </div>
    }
}

/// Props for the ImpactAnalysis component
#[derive(Properties, PartialEq)]
pub struct ImpactAnalysisProps {
    /// The impact analysis explanation
    pub explanation: String,
    
    /// Statistical explanation props
    pub statistical_props: StatisticalExplanationProps,
}

/// Component to display impact analysis with cooperative values
#[function_component(ImpactAnalysis)]
pub fn impact_analysis(props: &ImpactAnalysisProps) -> Html {
    html! {
        <div class="impact-analysis">
            <h3>{ "Cooperative Impact Analysis" }</h3>
            
            <StatisticalExplanation 
                explanation={props.explanation.clone()}
                methodology_source={props.statistical_props.methodology_source.clone()}
                confidence_interval={props.statistical_props.confidence_interval.clone()}
                significance_result={props.statistical_props.significance_result.clone()}
            />
            
            <div class="community-benefit">
                <h4>{ "Community Benefit" }</h4>
                <p>{ "Your financial practices contribute to the cooperative's mission of economic empowerment for all members. \
                    By participating in transparent financial analysis, you're helping build a more equitable financial system." }</p>
            </div>
        </div>
    }
}

// Fallback implementations when statistics feature is disabled
#[cfg(not(feature = "statistics"))]
#[derive(Properties, PartialEq)]
pub struct StatisticalExplanationProps {
    pub explanation: String,
    #[prop_or_default]
    pub methodology_source: Option<String>,
    #[prop_or_default]
    pub confidence_interval: Option<()>,
    #[prop_or_default]
    pub significance_result: Option<()>,
}

#[cfg(not(feature = "statistics"))]
#[derive(Properties, PartialEq)]
pub struct ForecastVisualizationProps {
    pub explanation: String,
    #[prop_or_default]
    pub chart_data: Option<String>,
    pub statistical_props: StatisticalExplanationProps,
}

#[cfg(not(feature = "statistics"))]
#[derive(Properties, PartialEq)]
pub struct ImpactAnalysisProps {
    pub explanation: String,
    pub statistical_props: StatisticalExplanationProps,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_component_compiles() {
        // This test just verifies that the components compile
        // Actual UI testing would require more complex setup
        assert!(true);
    }
}