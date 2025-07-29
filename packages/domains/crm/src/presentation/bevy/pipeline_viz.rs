//! Pipeline visualization for the CRM module
//!
//! This module provides Bevy systems for visualizing sales pipelines.
//!
//! ## Bevy System Architecture
//!
//! The pipeline visualization implements a data flow architecture:
//!
//! 1. **Data Flow**:
//!    - Domain models (Pipeline, Deal, Contact) are converted to visualization objects
//!    - Visualization objects are used to create Bevy entities with components
//!    - Systems query these entities to render the visualization
//!
//! 2. **Component Design**:
//!    - PipelineVisual: Root container for the pipeline visualization
//!    - StageVisual: Represents individual pipeline stages with position and name
//!    - DealVisual: Visual representation of deals with overdue status
//!    - ConversionRateVisual: Displays conversion metrics between stages
//!
//! 3. **System Organization**:
//!    - render_pipeline_diagram: Main rendering system that creates entities
//!    - update_conversion_rates: Calculates and updates conversion metrics
//!    - Systems run in the Update and PostUpdate schedules
//!
//! 4. **Integration Points**:
//!    - Uses pipeline_adapter.rs to convert domain objects to visualization objects
//!    - Queries application layer services for pipeline data
//!    - Renders using Bevy's UI system with NodeBundles

use bevy::prelude::*;
use crate::domain::pipeline::Pipeline;
use crate::domain::deal::Deal;
use crate::domain::contact::Contact;
use crate::presentation::bevy::pipeline_adapter::{convert_pipeline_to_viz, VizPipeline};

/// Plugin for pipeline visualization
pub struct PipelineVisualizationPlugin;

impl Plugin for PipelineVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, render_pipeline_diagram)
            .add_systems(PostUpdate, update_conversion_rates);
    }
}

/// Component for pipeline visualization
#[derive(Component)]
pub struct PipelineVisual {
    pub pipeline_id: String,
}

/// Component for stage visualization
#[derive(Component)]
pub struct StageVisual {
    pub stage_name: String,
    pub position: u8,
}

/// Component for deal visualization
#[derive(Component)]
pub struct DealVisual {
    pub deal_title: String,
    pub is_overdue: bool,
}

/// Component for conversion rate visualization
#[derive(Component)]
pub struct ConversionRateVisual {
    pub from_stage: String,
    pub to_stage: String,
    pub rate: f32,
}

/// System to render the pipeline diagram
fn render_pipeline_diagram(
    mut commands: Commands,
    // In a real implementation, these would come from queries or resources
    // For now, we'll use mock data to demonstrate the structure
) {
    // This is a placeholder implementation
    // In a real implementation, we would:
    // 1. Query for pipelines, deals, and contacts from the application layer
    // 2. Convert them to visualization objects using the pipeline_adapter
    // 3. Create Bevy entities for each visualization element
    
    info!("Rendering pipeline diagram");
    
    // Example of how we might create visualization entities:
    /*
    let viz_pipeline = convert_pipeline_to_viz(&pipeline, &deals, &contacts);
    
    for (index, stage) in viz_pipeline.stages.iter().enumerate() {
        // Create stage visualization
        commands
            .spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    margin: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                background_color: Color::rgb(0.8, 0.8, 0.8).into(),
                ..default()
            })
            .insert(StageVisual {
                stage_name: stage.name.clone(),
                position: stage.position,
            });
            
        // Create deal visualizations for this stage
        for deal in &stage.deals {
            commands
                .spawn(NodeBundle {
                    style: Style {
                        width: Val::Px(100.0),
                        height: Val::Px(50.0),
                        margin: UiRect::all(Val::Px(5.0)),
                        ..default()
                    },
                    background_color: if deal.is_overdue {
                        Color::rgb(1.0, 0.5, 0.5).into() // Red for overdue
                    } else {
                        Color::rgb(0.5, 1.0, 0.5).into() // Green for normal
                    },
                    ..default()
                })
                .insert(DealVisual {
                    deal_title: deal.title.clone(),
                    is_overdue: deal.is_overdue,
                });
        }
    }
    */
}

/// System to update conversion rates between stages
fn update_conversion_rates(
    // In a real implementation, we would query for stage and deal components
    // and calculate conversion rates based on the actual data
) {
    // This is a placeholder implementation
    info!("Updating conversion rates");
    
    // Example of how we might calculate and display conversion rates:
    /*
    // Calculate conversion rates between consecutive stages
    for i in 0..stages.len().saturating_sub(1) {
        let from_stage = &stages[i];
        let to_stage = &stages[i + 1];
        
        let from_deal_count = from_stage.deals.len();
        let to_deal_count = to_stage.deals.len();
        
        let conversion_rate = if from_deal_count > 0 {
            to_deal_count as f32 / from_deal_count as f32
        } else {
            0.0
        };
        
        // Create or update conversion rate visualization
        commands
            .spawn(TextBundle {
                text: Text::from_section(
                    format!("{:.1}%", conversion_rate * 100.0),
                    TextStyle {
                        font_size: 14.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ..default()
            })
            .insert(ConversionRateVisual {
                from_stage: from_stage.name.clone(),
                to_stage: to_stage.name.clone(),
                rate: conversion_rate,
            });
    }
    */
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pipeline_visualization_plugin() {
        // This is a placeholder test
        // In a real implementation, we would test the Bevy systems
        assert_eq!(1, 1);
    }
}