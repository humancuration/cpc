//! Bevy visualization components for sales pipeline analysis
//!
//! This module contains the Bevy systems and components for visualizing sales pipeline data.

use bevy::prelude::*;
use uuid::Uuid;
use crate::application::reporting_service::{PipelineAnalysis, StageData, AgingData, ForecastData};

/// Component to represent a sales pipeline visualization
#[derive(Component)]
pub struct SalesPipelineVisualization {
    pub analysis: PipelineAnalysis,
    pub selected_stage: Option<DealStatus>,
}

/// Component to represent a deal stage visualization
#[derive(Component)]
pub struct DealStageVisualization {
    pub stage: DealStatus,
    pub deal_count: u32,
    pub total_value: f32, // in thousands for visualization
    pub average_age: f32, // in days
}

/// Component to represent an aging visualization
#[derive(Component)]
pub struct AgingVisualization {
    pub days_range: String,
    pub deal_count: u32,
    pub total_value: f32, // in thousands for visualization
}

/// Component to represent a forecast visualization
#[derive(Component)]
pub struct ForecastVisualization {
    pub probability: f32,
    pub expected_value: f32, // in thousands for visualization
    pub confidence_interval: (f32, f32), // in thousands for visualization
}

/// Resource to manage sales pipeline visualization data
#[derive(Resource)]
pub struct SalesPipelineVisualizationData {
    pub analysis: Option<PipelineAnalysis>,
    pub selected_stage: Option<DealStatus>,
}

impl SalesPipelineVisualizationData {
    pub fn new() -> Self {
        Self {
            analysis: None,
            selected_stage: None,
        }
    }
    
    pub fn update_analysis(&mut self, analysis: PipelineAnalysis) {
        self.analysis = Some(analysis);
    }
    
    pub fn select_stage(&mut self, stage: DealStatus) {
        self.selected_stage = Some(stage);
    }
}

/// System to create sales pipeline visualizations
pub fn create_sales_pipeline_visualizations(
    mut commands: Commands,
    visualization_data: Res<SalesPipelineVisualizationData>,
) {
    // Clear existing visualizations
    // In a real implementation, we would be more selective about what to clear
    
    if let Some(analysis) = &visualization_data.analysis {
        // Create main pipeline visualization
        commands.spawn((
            SalesPipelineVisualization {
                analysis: analysis.clone(),
                selected_stage: visualization_data.selected_stage.clone(),
            },
            SpatialBundle::default(),
        ));
        
        // Create visualizations for each stage
        for stage_data in &analysis.pipeline_by_stage {
            commands.spawn((
                DealStageVisualization {
                    stage: stage_data.stage.clone(),
                    deal_count: stage_data.deal_count,
                    total_value: stage_data.total_value as f32 / 1000.0, // Convert to thousands
                    average_age: stage_data.average_age,
                },
                SpatialBundle::default(),
            ));
        }
        
        // Create visualizations for aging data
        for aging_data in &analysis.aging_analysis {
            commands.spawn((
                AgingVisualization {
                    days_range: aging_data.days_range.clone(),
                    deal_count: aging_data.deal_count,
                    total_value: aging_data.total_value as f32 / 1000.0, // Convert to thousands
                },
                SpatialBundle::default(),
            ));
        }
        
        // Create forecast visualization
        commands.spawn((
            ForecastVisualization {
                probability: analysis.forecast.probability,
                expected_value: analysis.forecast.expected_value as f32 / 1000.0, // Convert to thousands
                confidence_interval: (
                    analysis.forecast.confidence_interval.0 as f32 / 1000.0,
                    analysis.forecast.confidence_interval.1 as f32 / 1000.0,
                ),
            },
            SpatialBundle::default(),
        ));
    }
}

/// System to update sales pipeline visualizations
pub fn update_sales_pipeline_visualizations(
    mut visualization_query: Query<&mut SalesPipelineVisualization>,
    mut stage_query: Query<&mut DealStageVisualization>,
    mut aging_query: Query<&mut AgingVisualization>,
    mut forecast_query: Query<&mut ForecastVisualization>,
    visualization_data: Res<SalesPipelineVisualizationData>,
) {
    if visualization_data.is_changed() {
        if let Some(analysis) = &visualization_data.analysis {
            // Update main pipeline visualization
            for mut visualization in visualization_query.iter_mut() {
                visualization.analysis = analysis.clone();
                visualization.selected_stage = visualization_data.selected_stage.clone();
            }
            
            // Update stage visualizations
            // This is a simplified approach - in a real implementation, we would match by stage
            for (i, mut stage_viz) in stage_query.iter_mut().enumerate() {
                if i < analysis.pipeline_by_stage.len() {
                    let stage_data = &analysis.pipeline_by_stage[i];
                    stage_viz.deal_count = stage_data.deal_count;
                    stage_viz.total_value = stage_data.total_value as f32 / 1000.0;
                    stage_viz.average_age = stage_data.average_age;
                }
            }
            
            // Update aging visualizations
            for (i, mut aging_viz) in aging_query.iter_mut().enumerate() {
                if i < analysis.aging_analysis.len() {
                    let aging_data = &analysis.aging_analysis[i];
                    aging_viz.days_range = aging_data.days_range.clone();
                    aging_viz.deal_count = aging_data.deal_count;
                    aging_viz.total_value = aging_data.total_value as f32 / 1000.0;
                }
            }
            
            // Update forecast visualization
            for mut forecast_viz in forecast_query.iter_mut() {
                forecast_viz.probability = analysis.forecast.probability;
                forecast_viz.expected_value = analysis.forecast.expected_value as f32 / 1000.0;
                forecast_viz.confidence_interval = (
                    analysis.forecast.confidence_interval.0 as f32 / 1000.0,
                    analysis.forecast.confidence_interval.1 as f32 / 1000.0,
                );
            }
        }
    }
}

/// System to render deal stage visualizations as bars
pub fn render_deal_stage_bars(
    mut gizmos: Gizmos,
    stage_query: Query<(&Transform, &DealStageVisualization)>,
) {
    for (transform, stage) in stage_query.iter() {
        // Color based on stage
        let color = stage_to_color(&stage.stage);
        
        // Draw a bar representing the stage
        let height = stage.deal_count as f32 * 0.1; // Scale factor
        let width = 0.5;
        
        gizmos.cuboid(
            Transform::from_translation(transform.translation + Vec3::new(0.0, height / 2.0, 0.0))
                .with_scale(Vec3::new(width, height, 0.1)),
            color,
        );
        
        // Draw a label with the stage name
        // Note: Text rendering would require additional setup in a real implementation
    }
}

/// System to render aging visualizations as pie charts
pub fn render_aging_pie_charts(
    mut gizmos: Gizmos,
    aging_query: Query<(&Transform, &AgingVisualization)>,
) {
    for (transform, aging) in aging_query.iter() {
        // Draw segments of a pie chart
        let total_value: f32 = aging_query.iter().map(|(_, a)| a.total_value).sum();
        if total_value > 0.0 {
            let angle = (aging.total_value / total_value) * 360.0;
            
            // Draw a sector of a circle
            // This is a simplified representation
            gizmos.arc_2d(
                transform.translation.truncate(),
                0.0,
                angle.to_radians(),
                1.0,
                Color::rgb(0.2, 0.7, 0.9),
            );
        }
    }
}

/// System to render forecast visualizations
pub fn render_forecast_visualizations(
    mut gizmos: Gizmos,
    forecast_query: Query<(&Transform, &ForecastVisualization)>,
) {
    for (transform, forecast) in forecast_query.iter() {
        // Draw a line representing the expected value
        let expected_position = Vec3::new(0.0, forecast.expected_value / 100.0, 0.0);
        gizmos.line(
            transform.translation,
            transform.translation + expected_position,
            Color::GREEN,
        );
        
        // Draw lines for confidence interval
        let lower_position = Vec3::new(-0.5, forecast.confidence_interval.0 / 100.0, 0.0);
        let upper_position = Vec3::new(0.5, forecast.confidence_interval.1 / 100.0, 0.0);
        
        gizmos.line(
            transform.translation + lower_position,
            transform.translation + expected_position,
            Color::YELLOW,
        );
        
        gizmos.line(
            transform.translation + expected_position,
            transform.translation + upper_position,
            Color::YELLOW,
        );
    }
}

/// Helper function to convert stage to color
fn stage_to_color(stage: &DealStatus) -> Color {
    match stage {
        DealStatus::Prospecting => Color::rgb(0.2, 0.6, 0.8),    // Blue
        DealStatus::Qualified => Color::rgb(0.3, 0.7, 0.5),      // Green
        DealStatus::Proposal => Color::rgb(0.8, 0.7, 0.2),       // Yellow
        DealStatus::Negotiation => Color::rgb(0.9, 0.5, 0.2),    // Orange
        DealStatus::Won => Color::rgb(0.2, 0.8, 0.2),            // Bright Green
        DealStatus::Lost => Color::rgb(0.8, 0.2, 0.2),           // Red
    }
}

/// Enum representing deal status for visualization
#[derive(Debug, Clone, PartialEq)]
pub enum DealStatus {
    Prospecting,
    Qualified,
    Proposal,
    Negotiation,
    Won,
    Lost,
}

impl From<crate::application::reporting_service::DealStatus> for DealStatus {
    fn from(status: crate::application::reporting_service::DealStatus) -> Self {
        match status {
            crate::application::reporting_service::DealStatus::Prospecting => DealStatus::Prospecting,
            crate::application::reporting_service::DealStatus::Qualified => DealStatus::Qualified,
            crate::application::reporting_service::DealStatus::Proposal => DealStatus::Proposal,
            crate::application::reporting_service::DealStatus::Negotiation => DealStatus::Negotiation,
            crate::application::reporting_service::DealStatus::Won => DealStatus::Won,
            crate::application::reporting_service::DealStatus::Lost => DealStatus::Lost,
        }
    }
}

/// Plugin to add sales pipeline visualization functionality to Bevy
pub struct SalesPipelineVisualizationPlugin;

impl Plugin for SalesPipelineVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SalesPipelineVisualizationData>()
            .add_systems(Update, (
                create_sales_pipeline_visualizations,
                update_sales_pipeline_visualizations,
                render_deal_stage_bars,
                render_aging_pie_charts,
                render_forecast_visualizations,
            ));
    }
}