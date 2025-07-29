//! Bevy visualization components for integrated calendar events
//!
//! This module implements the custom visual elements for CRM and Invoicing events
//! within the Bevy timeline view.

use bevy::prelude::*;
use crate::domain::EventType;

/// Plugin for calendar visualization components
pub struct CalendarVisualizationPlugin;

impl Plugin for CalendarVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                visualize_sales_pipeline_events,
                visualize_lead_follow_ups,
                visualize_payment_due_events,
                visualize_payment_status_changes,
            ));
    }
}

/// Visualize sales pipeline milestone events
fn visualize_sales_pipeline_events(
    mut commands: Commands,
    query: Query<(Entity, &EventType), Added<EventType>>,
) {
    for (entity, event_type) in query.iter() {
        if let EventType::SalesPipelineMilestone { stage, .. } = event_type {
            // Create a progress bar visualization for the sales stage
            let progress = match stage {
                crate::domain::SalesStage::Lead => 0.1,
                crate::domain::SalesStage::Qualified => 0.3,
                crate::domain::SalesStage::DemoScheduled => 0.5,
                crate::domain::SalesStage::ProposalSent => 0.7,
                crate::domain::SalesStage::Negotiation => 0.85,
                crate::domain::SalesStage::ClosedWon => 1.0,
                crate::domain::SalesStage::ClosedLost => 0.0,
            };
            
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    SalesPipelineVisual,
                    ProgressBar {
                        progress,
                        color: Color::srgb(0.2, 0.6, 0.9),
                    },
                    Transform::from_xyz(0.0, 15.0, 0.0),
                    GlobalTransform::default(),
                ));
            });
        }
    }
}

/// Visualize lead follow-up events
fn visualize_lead_follow_ups(
    mut commands: Commands,
    query: Query<(Entity, &EventType), Added<EventType>>,
) {
    for (entity, event_type) in query.iter() {
        if let EventType::LeadFollowUp { score_change, wellness_threshold, .. } = event_type {
            // Create a marker with color based on urgency
            let color = if *score_change > 30 {
                Color::srgb(0.9, 0.2, 0.2) // Red for high urgency
            } else if *score_change > 10 {
                Color::srgb(0.9, 0.7, 0.2) // Orange for medium urgency
            } else {
                Color::srgb(0.2, 0.7, 0.2) // Green for low urgency
            };
            
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    LeadFollowUpVisual,
                    Shape::Circle(Circle { radius: 8.0 }),
                    Fill::color(color),
                    Transform::from_xyz(0.0, 0.0, 5.0),
                    GlobalTransform::default(),
                ));
                
                // Add wellness threshold indicator if present
                if let Some(threshold) = wellness_threshold {
                    let wellness_color = if *threshold > 70 {
                        Color::srgb(0.9, 0.2, 0.2) // Red for high stress
                    } else if *threshold > 50 {
                        Color::srgb(0.9, 0.7, 0.2) // Orange for medium stress
                    } else {
                        Color::srgb(0.2, 0.7, 0.2) // Green for low stress
                    };
                    
                    parent.spawn((
                        WellnessThresholdIndicator,
                        Shape::Circle(Circle { radius: 4.0 }),
                        Fill::color(wellness_color),
                        Transform::from_xyz(0.0, 0.0, 6.0),
                        GlobalTransform::default(),
                    ));
                }
            });
        }
    }
}

/// Visualize payment due events
fn visualize_payment_due_events(
    mut commands: Commands,
    query: Query<(Entity, &EventType), Added<EventType>>,
) {
    for (entity, event_type) in query.iter() {
        if let EventType::PaymentDue { amount, status, .. } = event_type {
            // Create a payment due marker with color based on status
            let color = match status {
                crate::domain::PaymentStatus::Paid => Color::srgb(0.2, 0.7, 0.2), // Green
                crate::domain::PaymentStatus::Sent | 
                crate::domain::PaymentStatus::Viewed => Color::srgb(0.2, 0.6, 0.9), // Blue
                crate::domain::PaymentStatus::Partial => Color::srgb(0.9, 0.7, 0.2), // Orange
                crate::domain::PaymentStatus::Overdue => Color::srgb(0.9, 0.2, 0.2), // Red
                crate::domain::PaymentStatus::Draft => Color::srgb(0.5, 0.5, 0.5), // Gray
            };
            
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    PaymentDueVisual,
                    Shape::Rectangle(Rectangle::new(20.0, 20.0)),
                    Fill::color(color),
                    Transform::from_xyz(0.0, 0.0, 5.0),
                    GlobalTransform::default(),
                ));
                
                // Add amount label
                parent.spawn((
                    PaymentAmountLabel,
                    Text::new(format!("${:.2}", amount)),
                    TextFont {
                        font_size: 12.0,
                        ..default()
                    },
                    TextColor(Color::srgb(1.0, 1.0, 1.0)),
                    Transform::from_xyz(0.0, -15.0, 0.0),
                    GlobalTransform::default(),
                ));
            });
        }
    }
}

/// Visualize payment status changes
fn visualize_payment_status_changes(
    mut commands: Commands,
    query: Query<(Entity, &EventType), Added<EventType>>,
) {
    for (entity, event_type) in query.iter() {
        if let EventType::PaymentStatusChange { 
            previous_status, 
            new_status, 
            ..
        } = event_type {
            // Create a status change annotation
            let (color, label) = match (previous_status, new_status) {
                (crate::domain::PaymentStatus::Sent, crate::domain::PaymentStatus::Viewed) => {
                    (Color::srgb(0.2, 0.6, 0.9), "Viewed")
                }
                (crate::domain::PaymentStatus::Viewed, crate::domain::PaymentStatus::Paid) => {
                    (Color::srgb(0.2, 0.7, 0.2), "Paid")
                }
                (_, crate::domain::PaymentStatus::Overdue) => {
                    (Color::srgb(0.9, 0.2, 0.2), "Overdue")
                }
                _ => (Color::srgb(0.5, 0.5, 0.5), "Status Change"),
            };
            
            commands.entity(entity).with_children(|parent| {
                parent.spawn((
                    PaymentStatusChangeVisual,
                    Shape::Circle(Circle { radius: 6.0 }),
                    Fill::color(color),
                    Transform::from_xyz(0.0, 0.0, 5.0),
                    GlobalTransform::default(),
                ));
                
                // Add status change label
                parent.spawn((
                    PaymentStatusLabel,
                    Text::new(label.to_string()),
                    TextFont {
                        font_size: 10.0,
                        ..default()
                    },
                    TextColor(Color::srgb(0.2, 0.2, 0.2)),
                    Transform::from_xyz(0.0, -12.0, 0.0),
                    GlobalTransform::default(),
                ));
            });
        }
    }
}

// Component markers for visualization
#[derive(Component)]
struct SalesPipelineVisual;

#[derive(Component)]
struct LeadFollowUpVisual;

#[derive(Component)]
struct WellnessThresholdIndicator;

#[derive(Component)]
struct PaymentDueVisual;

#[derive(Component)]
struct PaymentAmountLabel;

#[derive(Component)]
struct PaymentStatusChangeVisual;

#[derive(Component)]
struct PaymentStatusLabel;

// Helper component for progress bars
#[derive(Component)]
struct ProgressBar {
    progress: f32,
    color: Color,
}