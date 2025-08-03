//! Metrics panel component for displaying generation metrics

use yew::prelude::*;
use crate::services::generator_service::GenerationMetrics;

#[derive(Properties, PartialEq)]
pub struct MetricsPanelProps {
    pub metrics: GenerationMetrics,
}

#[function_component(MetricsPanel)]
pub fn metrics_panel(props: &MetricsPanelProps) -> Html {
    let metrics = &props.metrics;
    
    html! {
        <div class="metrics-panel">
            <h2>{"Generation Metrics"}</h2>
            
            <div class="metric">
                <label>{"Progress:"}</label>
                <progress value={metrics.progress} max="100"></progress>
                <span>{format!("{}%", metrics.progress)}</span>
            </div>
            
            <div class="metric">
                <label>{"Items Processed:"}</label>
                <span>{metrics.items_processed.to_string()}</span>
            </div>
            
            <div class="metric">
                <label>{"Items/Second:"}</label>
                <span>{format!("{:.2}", metrics.items_per_second)}</span>
            </div>
            
            <div class="metric">
                <label>{"Memory Usage:"}</label>
                <span>{format!("{} MB", metrics.memory_usage)}</span>
            </div>
        </div>
    }
}