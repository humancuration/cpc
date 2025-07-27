//! 3D visualization components for invoices and quotes

use bevy::prelude::*;

/// Plugin for invoicing and quoting visualizations
pub struct InvoicingVisualizationPlugin;

impl Plugin for InvoicingVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, setup_invoice_visualization)
            .add_systems(Update, update_invoice_status_visualization);
    }
}

/// Component to represent an invoice in 3D space
#[derive(Component)]
pub struct InvoiceVisualization {
    pub invoice_id: String,
    pub status: InvoiceStatus,
    pub amount: f32,
}

/// Component to represent a quote in 3D space
#[derive(Component)]
pub struct QuoteVisualization {
    pub quote_id: String,
    pub status: QuoteStatus,
    pub amount: f32,
}

/// Enum for invoice status visualization
#[derive(Debug, Clone, PartialEq)]
pub enum InvoiceStatus {
    Draft,
    Sent,
    Viewed,
    Paid,
    Overdue,
    Partial,
}

/// Enum for quote status visualization
#[derive(Debug, Clone, PartialEq)]
pub enum QuoteStatus {
    Draft,
    Sent,
    Accepted,
    Rejected,
    Expired,
}

/// System to set up invoice visualization
fn setup_invoice_visualization(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create a simple cube to represent an invoice
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        InvoiceVisualization {
            invoice_id: "example_invoice".to_string(),
            status: InvoiceStatus::Draft,
            amount: 100.0,
        },
    ));
}

/// System to update invoice status visualization
fn update_invoice_status_visualization(
    mut invoices: Query<(&InvoiceVisualization, &mut Handle<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (invoice, mut material_handle) in invoices.iter_mut() {
        let color = match invoice.status {
            InvoiceStatus::Draft => Color::rgb(0.8, 0.8, 0.8), // Gray
            InvoiceStatus::Sent => Color::rgb(0.8, 0.8, 0.0),  // Yellow
            InvoiceStatus::Viewed => Color::rgb(0.0, 0.8, 0.8), // Cyan
            InvoiceStatus::Paid => Color::rgb(0.0, 0.8, 0.0),   // Green
            InvoiceStatus::Overdue => Color::rgb(0.8, 0.0, 0.0), // Red
            InvoiceStatus::Partial => Color::rgb(0.8, 0.4, 0.0), // Orange
        };
        
        if let Some(material) = materials.get_mut(&*material_handle) {
            material.base_color = color;
        }
    }
}