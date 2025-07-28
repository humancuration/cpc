//! Bevy visualization components for payment status flow
//!
//! This module contains the Bevy systems and components for visualizing payment status changes.

use bevy::prelude::*;
use uuid::Uuid;
use crate::domain::status::PaymentStatus;
use crate::domain::payment::Invoice;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

/// Component to represent an invoice in the visualization
#[derive(Component)]
pub struct InvoiceVisualization {
    pub invoice_id: Uuid,
    pub current_status: PaymentStatus,
    pub status_history: Vec<StatusHistoryEntry>,
    pub position: Vec3,
}

/// Component to represent a status node in the visualization
#[derive(Component)]
pub struct StatusNode {
    pub status: PaymentStatus,
    pub position: Vec3,
    pub invoice_count: u32,
}

/// Component to represent a status transition in the visualization
#[derive(Component)]
pub struct StatusTransitionVisualization {
    pub from_status: PaymentStatus,
    pub to_status: PaymentStatus,
    pub count: u32,
    pub start_position: Vec3,
    pub end_position: Vec3,
}

/// Status history entry for visualization
#[derive(Debug, Clone)]
pub struct StatusHistoryEntry {
    pub status: PaymentStatus,
    pub timestamp: DateTime<Utc>,
}

/// Resource to manage payment status visualization data
#[derive(Resource)]
pub struct PaymentStatusVisualizationData {
    pub invoices: Vec<InvoiceVisualization>,
    pub status_nodes: HashMap<PaymentStatus, StatusNode>,
    pub transitions: Vec<StatusTransitionVisualization>,
}

impl PaymentStatusVisualizationData {
    pub fn new() -> Self {
        Self {
            invoices: Vec::new(),
            status_nodes: HashMap::new(),
            transitions: Vec::new(),
        }
    }
    
    pub fn update_invoices(&mut self, invoices: Vec<Invoice>) {
        self.invoices.clear();
        
        // Convert invoices to visualizations
        for invoice in invoices {
            let visualization = InvoiceVisualization {
                invoice_id: invoice.id,
                current_status: invoice.status.clone(),
                status_history: vec![], // In a real implementation, we would populate this
                position: Vec3::ZERO,
            };
            self.invoices.push(visualization);
        }
        
        // Update status nodes
        self.update_status_nodes();
        
        // Update transitions
        self.update_transitions();
    }
    
    fn update_status_nodes(&mut self) {
        self.status_nodes.clear();
        
        // Count invoices by status
        let mut status_counts = HashMap::new();
        for invoice in &self.invoices {
            *status_counts.entry(invoice.current_status.clone()).or_insert(0) += 1;
        }
        
        // Create status nodes
        let statuses = [
            PaymentStatus::Draft,
            PaymentStatus::Sent,
            PaymentStatus::Viewed,
            PaymentStatus::Paid,
            PaymentStatus::Overdue,
            PaymentStatus::Partial,
            PaymentStatus::PaymentFailed,
            PaymentStatus::Pending,
        ];
        
        for (i, status) in statuses.iter().enumerate() {
            let count = *status_counts.get(status).unwrap_or(&0);
            
            // Position nodes in a circle
            let angle = (i as f32) * 2.0 * std::f32::consts::PI / (statuses.len() as f32);
            let radius = 3.0;
            let position = Vec3::new(
                radius * angle.cos(),
                radius * angle.sin(),
                0.0,
            );
            
            let node = StatusNode {
                status: status.clone(),
                position,
                invoice_count: count,
            };
            
            self.status_nodes.insert(status.clone(), node);
        }
    }
    
    fn update_transitions(&mut self) {
        self.transitions.clear();
        
        // In a real implementation, we would analyze status history to determine transitions
        // For now, we'll create some example transitions
        let transitions = [
            (PaymentStatus::Draft, PaymentStatus::Sent),
            (PaymentStatus::Sent, PaymentStatus::Viewed),
            (PaymentStatus::Viewed, PaymentStatus::Paid),
            (PaymentStatus::Sent, PaymentStatus::Overdue),
            (PaymentStatus::Overdue, PaymentStatus::PaymentFailed),
            (PaymentStatus::PaymentFailed, PaymentStatus::Pending),
            (PaymentStatus::Pending, PaymentStatus::Paid),
        ];
        
        for (from, to) in &transitions {
            let start_position = self.status_nodes.get(from).map(|n| n.position).unwrap_or(Vec3::ZERO);
            let end_position = self.status_nodes.get(to).map(|n| n.position).unwrap_or(Vec3::ZERO);
            
            let transition = StatusTransitionVisualization {
                from_status: from.clone(),
                to_status: to.clone(),
                count: 1, // In a real implementation, we would count actual transitions
                start_position,
                end_position,
            };
            
            self.transitions.push(transition);
        }
    }
}

/// System to create payment status visualizations
pub fn create_payment_status_visualizations(
    mut commands: Commands,
    visualization_data: Res<PaymentStatusVisualizationData>,
) {
    // Clear existing visualizations
    // In a real implementation, we would be more selective about what to clear
    
    // Create visualizations for each status node
    for (_, node) in &visualization_data.status_nodes {
        commands.spawn((
            StatusNodeVisualization {
                status: node.status.clone(),
                invoice_count: node.invoice_count,
            },
            SpatialBundle {
                transform: Transform::from_translation(node.position),
                ..default()
            },
        ));
    }
    
    // Create visualizations for each transition
    for transition in &visualization_data.transitions {
        commands.spawn((
            StatusTransitionVisualizationComponent {
                from_status: transition.from_status.clone(),
                to_status: transition.to_status.clone(),
                count: transition.count,
            },
            SpatialBundle::default(),
        ));
    }
}

/// Component to represent a status node visualization
#[derive(Component)]
pub struct StatusNodeVisualization {
    pub status: PaymentStatus,
    pub invoice_count: u32,
}

/// Component to represent a status transition visualization
#[derive(Component)]
pub struct StatusTransitionVisualizationComponent {
    pub from_status: PaymentStatus,
    pub to_status: PaymentStatus,
    pub count: u32,
}

/// System to update payment status visualizations
pub fn update_payment_status_visualizations(
    mut visualization_query: Query<(&mut StatusNodeVisualization, &mut Transform)>,
    visualization_data: Res<PaymentStatusVisualizationData>,
) {
    if visualization_data.is_changed() {
        for (mut visualization, mut transform) in visualization_query.iter_mut() {
            // Update invoice count for this status
            if let Some(node) = visualization_data.status_nodes.get(&visualization.status) {
                visualization.invoice_count = node.invoice_count;
                
                // Update position
                transform.translation = node.position;
            }
        }
    }
}

/// System to render status nodes
pub fn render_status_nodes(
    mut gizmos: Gizmos,
    node_query: Query<(&Transform, &StatusNodeVisualization)>,
) {
    for (transform, node) in node_query.iter() {
        // Color based on status
        let color = status_to_color(&node.status);
        
        // Draw a circle representing the status node
        gizmos.circle(
            transform.translation,
            Dir3::Z,
            0.3 + (node.invoice_count as f32 * 0.05).min(0.5), // Size based on invoice count
            color,
        );
        
        // Draw a label with the status name
        // Note: Text rendering would require additional setup in a real implementation
    }
}

/// System to render status transitions
pub fn render_status_transitions(
    mut gizmos: Gizmos,
    transition_query: Query<&StatusTransitionVisualizationComponent>,
    node_data: Res<PaymentStatusVisualizationData>,
) {
    for transition in transition_query.iter() {
        // Get positions from node data
        let start_pos = node_data.status_nodes.get(&transition.from_status)
            .map(|n| n.position)
            .unwrap_or(Vec3::ZERO);
            
        let end_pos = node_data.status_nodes.get(&transition.to_status)
            .map(|n| n.position)
            .unwrap_or(Vec3::ZERO);
        
        // Color based on transition frequency
        let color = transition_count_to_color(transition.count);
        
        // Draw an arrow representing the transition
        gizmos.arrow(
            start_pos,
            end_pos,
            color,
        );
    }
}

/// Helper function to convert status to color
fn status_to_color(status: &PaymentStatus) -> Color {
    match status {
        PaymentStatus::Draft => Color::rgb(0.5, 0.5, 0.5),      // Gray
        PaymentStatus::Sent => Color::rgb(0.2, 0.6, 0.8),       // Blue
        PaymentStatus::Viewed => Color::rgb(0.3, 0.7, 0.5),     // Green
        PaymentStatus::Paid => Color::rgb(0.2, 0.8, 0.2),       // Bright Green
        PaymentStatus::Overdue => Color::rgb(0.9, 0.7, 0.2),    // Yellow
        PaymentStatus::Partial => Color::rgb(0.8, 0.5, 0.2),    // Orange
        PaymentStatus::PaymentFailed => Color::rgb(0.8, 0.2, 0.2), // Red
        PaymentStatus::Pending => Color::rgb(0.6, 0.4, 0.8),    // Purple
    }
}

/// Helper function to convert transition count to color
fn transition_count_to_color(count: u32) -> Color {
    let intensity = (count as f32 / 10.0).min(1.0); // Normalize to 0-1
    Color::rgb(intensity, intensity, intensity)
}

/// Plugin to add payment status visualization functionality to Bevy
pub struct PaymentStatusVisualizationPlugin;

impl Plugin for PaymentStatusVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PaymentStatusVisualizationData>()
            .add_systems(Update, (
                create_payment_status_visualizations,
                update_payment_status_visualizations,
                render_status_nodes,
                render_status_transitions,
            ));
    }
}

/// Helper function to update visualization data with new invoices
pub fn update_visualization_with_invoices(
    visualization_data: &mut PaymentStatusVisualizationData,
    invoices: Vec<Invoice>,
) {
    visualization_data.update_invoices(invoices);
}