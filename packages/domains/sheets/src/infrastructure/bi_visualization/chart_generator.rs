//! Chart generator using the BI Visualization Toolkit

use cpc_core::bi_visualization::{
    VisualizationService,
    InteractiveConfig,
    ChartType,
    DataSeries,
};
use crate::domain::{Sheet, ChartSpec};

/// Chart generator for creating interactive charts
pub struct ChartGenerator;

impl ChartGenerator {
    pub fn new() -> Self {
        Self
    }
    
    /// Create an interactive chart from sheet data
    pub fn create_interactive_chart(
        &self,
        sheet: &Sheet,
        chart_spec: &ChartSpec,
    ) -> Result<impl bevy::prelude::Bundle, Box<dyn std::error::Error>> {
        let interactive_config = InteractiveConfig {
            chart_type: chart_spec.chart_type.clone(),
            title: chart_spec.title.clone(),
            dimensions: (chart_spec.options.width, chart_spec.options.height),
            interactive_elements: vec![
                cpc_core::bi_visualization::domain::chart::InteractiveElement::Tooltip,
                cpc_core::bi_visualization::domain::chart::InteractiveElement::Zoom,
                cpc_core::bi_visualization::domain::chart::InteractiveElement::Selection,
            ],
        };
        
        // In a real implementation, we would transform the sheet data
        // For now, we'll create a simple data series
        let data_points = vec![
            cpc_core::bi_visualization::domain::data::TimeSeriesPoint::new(1.0, 10.0),
            cpc_core::bi_visualization::domain::data::TimeSeriesPoint::new(2.0, 20.0),
            cpc_core::bi_visualization::domain::data::TimeSeriesPoint::new(3.0, 15.0),
        ];
        let data_series = DataSeries::from_time_series("Sample Data".to_string(), data_points);
        
        // Create interactive chart
        let chart_bundle = VisualizationService::create_interactive_chart(interactive_config, data_series)?;
        
        Ok(chart_bundle)
    }
}