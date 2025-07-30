//! Bevy visualization components for lead scoring trends
//!
//! This module contains the Bevy systems and components for visualizing lead scoring data.

use bevy::prelude::*;
use uuid::Uuid;
use crate::domain::lead_scoring::LeadScore;

/// Component to represent a lead score visualization
#[derive(Component)]
pub struct LeadScoreVisualization {
    pub lead_id: Uuid,
    pub current_score: u8,
    pub trend: ScoreTrend,
}

/// Component to represent a score trend visualization
#[derive(Component)]
pub struct ScoreTrend {
    pub values: Vec<(f32, f32)>, // (time, score) pairs for plotting
    pub color: Color,
}

/// Resource to manage lead scoring visualization data
#[derive(Resource)]
pub struct LeadScoringVisualizationData {
    pub scores: Vec<LeadScore>,
    pub selected_lead: Option<Uuid>,
}

impl LeadScoringVisualizationData {
    pub fn new() -> Self {
        Self {
            scores: Vec::new(),
            selected_lead: None,
        }
    }
    
    pub fn update_scores(&mut self, new_scores: Vec<LeadScore>) {
        self.scores = new_scores;
    }
    
    pub fn select_lead(&mut self, lead_id: Uuid) {
        self.selected_lead = Some(lead_id);
    }
}

/// System to create lead score visualizations
pub fn create_lead_score_visualizations(
    mut commands: Commands,
    visualization_data: Res<LeadScoringVisualizationData>,
) {
    // Clear existing visualizations
    // In a real implementation, we would be more selective about what to clear
    
    // Create visualizations for each lead score
    for score in &visualization_data.scores {
        let trend = calculate_score_trend(score);
        
        commands.spawn((
            LeadScoreVisualization {
                lead_id: score.lead_id,
                current_score: score.total_score,
                trend,
            },
            SpatialBundle::default(),
        ));
    }
}

/// System to update lead score visualizations
pub fn update_lead_score_visualizations(
    mut visualization_query: Query<(&mut LeadScoreVisualization, &mut ScoreTrend)>,
    visualization_data: Res<LeadScoringVisualizationData>,
) {
    if visualization_data.is_changed() {
        for (mut visualization, mut trend) in visualization_query.iter_mut() {
            // Find the corresponding score for this visualization
            if let Some(score) = visualization_data.scores.iter().find(|s| s.lead_id == visualization.lead_id) {
                visualization.current_score = score.total_score;
                *trend = calculate_score_trend(score);
            }
        }
    }
}

/// Helper function to calculate score trend from history
fn calculate_score_trend(score: &LeadScore) -> ScoreTrend {
    // In a real implementation, this would use historical data
    // For now, we'll create a simple trend based on the current score
    
    let values = vec![
        (-2.0, (score.total_score as f32 * 0.7).clamp(0.0, 100.0)),
        (-1.0, (score.total_score as f32 * 0.85).clamp(0.0, 100.0)),
        (0.0, score.total_score as f32),
    ];
    
    ScoreTrend {
        values,
        color: Color::rgb(0.2, 0.7, 0.9),
    }
}

/// System to render score trends as lines
pub fn render_score_trends(
    mut gizmos: Gizmos,
    trend_query: Query<(&Transform, &ScoreTrend)>,
) {
    for (transform, trend) in trend_query.iter() {
        // Draw the trend line
        if trend.values.len() > 1 {
            for i in 0..trend.values.len() - 1 {
                let start = Vec3::new(trend.values[i].0, trend.values[i].1 / 100.0, 0.0);
                let end = Vec3::new(trend.values[i + 1].0, trend.values[i + 1].1 / 100.0, 0.0);
                
                gizmos.line(
                    transform.translation + start,
                    transform.translation + end,
                    trend.color,
                );
            }
        }
        
        // Draw points for each data point
        for (x, y) in &trend.values {
            let position = Vec3::new(*x, *y / 100.0, 0.0);
            gizmos.sphere(
                transform.translation + position,
                Quat::IDENTITY,
                0.05,
                trend.color,
            );
        }
    }
}

/// System to render lead score indicators
pub fn render_lead_score_indicators(
    mut gizmos: Gizmos,
    visualization_query: Query<(&Transform, &LeadScoreVisualization)>,
) {
    for (transform, visualization) in visualization_query.iter() {
        // Color based on score (green for high, red for low)
        let color = score_to_color(visualization.current_score);
        
        // Draw a circle representing the lead score
        gizmos.circle(
            transform.translation,
            Dir3::Z,
            0.1 + (visualization.current_score as f32 / 1000.0),
            color,
        );
        
        // Draw a label with the score
        // Note: Text rendering would require additional setup in a real implementation
    }
}

/// Helper function to convert score to color
fn score_to_color(score: u8) -> Color {
    let normalized = score as f32 / 100.0;
    if normalized > 0.7 {
        Color::rgb(0.0, 0.8, 0.2) // Green for high scores
    } else if normalized > 0.4 {
        Color::rgb(0.8, 0.8, 0.0) // Yellow for medium scores
    } else {
        Color::rgb(0.8, 0.2, 0.2) // Red for low scores
    }
}

/// Plugin to add lead scoring visualization functionality to Bevy
pub struct LeadScoringVisualizationPlugin;

impl Plugin for LeadScoringVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<LeadScoringVisualizationData>()
            .add_systems(Update, (
                create_lead_score_visualizations,
                update_lead_score_visualizations,
                render_score_trends,
                render_lead_score_indicators,
            ));
    }
}