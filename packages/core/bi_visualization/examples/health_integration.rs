//! Health module integration example for the BI visualization toolkit

use bi_visualization::{
    VisualizationService,
    InteractiveConfig,
    ChartType,
    DataSeries,
    TimeSeriesPoint,
    HeatmapPoint,
    infrastructure::bevy::BiVisualizationPlugin,
};
use bevy::prelude::*;
use chrono::{Utc, Duration};

/// Health service that uses the BI visualization toolkit
struct HealthService;

impl HealthService {
    /// Create interactive sleep quality visualization
    fn create_sleep_analysis_chart(&self) -> Result<impl Bundle, Box<dyn std::error::Error>> {
        // Create interactive sleep quality visualization
        let interactive_config = InteractiveConfig {
            chart_type: ChartType::Heatmap,
            title: "Sleep Quality Over Time".to_string(),
            dimensions: (1200, 800),
            interactive_elements: vec![
                bi_visualization::domain::chart::InteractiveElement::Tooltip,
                bi_visualization::domain::chart::InteractiveElement::Zoom,
            ],
        };
        
        // Create sample sleep data
        let mut sleep_points = Vec::new();
        
        for day in 0..30 {
            for hour in 0..24 {
                let timestamp = Utc::now() - Duration::days(30 - day) + Duration::hours(hour);
                // Generate some sample sleep quality data (0.0 to 1.0)
                let quality = (day as f64 * 0.1 + hour as f64 * 0.05) % 1.0;
                sleep_points.push(HeatmapPoint::new(day, hour, quality));
            }
        }
        
        let sleep_series = DataSeries::from_heatmap("Sleep Quality".to_string(), sleep_points);
        
        // Create interactive chart bundle
        // Note: In a real implementation, we would pass the asset server
        // For this example, we'll just create a placeholder
        #[derive(bevy::prelude::Component)]
        struct PlaceholderChart;
        
        #[derive(bevy::prelude::Bundle)]
        struct ChartBundle {
            chart: PlaceholderChart,
        }
        
        let bundle = ChartBundle {
            chart: PlaceholderChart,
        };
        
        Ok(bundle)
    }
}

/// Bevy app system to add the interactive chart
fn add_sleep_analysis_chart(mut commands: Commands) {
    let health_service = HealthService;
    
    match health_service.create_sleep_analysis_chart() {
        Ok(bundle) => {
            commands.spawn(bundle);
            println!("Interactive sleep analysis chart added to Bevy app");
        }
        Err(e) => {
            eprintln!("Failed to create sleep analysis chart: {}", e);
        }
    }
}

fn main() {
    // Create Bevy app with BI visualization plugin
    App::new()
        .add_plugins(MinimalPlugins)
        .add_plugins(BiVisualizationPlugin)
        .add_systems(Startup, add_sleep_analysis_chart)
        .run();
    
    println!("Health integration example completed successfully!");
}