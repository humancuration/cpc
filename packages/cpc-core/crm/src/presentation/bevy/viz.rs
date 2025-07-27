//! Bevy visualization plugin for CRM functionality
//!
//! This module contains 3D visualization components for the CRM module.

use bevy::prelude::*;
use crate::domain::contact::Contact;
use crate::domain::deal::Deal;
use crate::domain::pipeline::Pipeline;
use crate::presentation::bevy::pipeline_viz::PipelineVisualizationPlugin;

/// Plugin for CRM visualization
pub struct CrmVisualizationPlugin;
impl Plugin for CrmVisualizationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(PipelineVisualizationPlugin)
            .add_systems(Startup, setup_crm_visualization)
            .add_systems(Update, update_crm_visualization);
    }
}
}

/// Component for contact visualization
#[derive(Component)]
pub struct ContactVisual {
    pub contact_id: String,
}

/// Component for deal visualization
#[derive(Component)]
pub struct DealVisual {
    pub deal_id: String,
}

/// Component for pipeline visualization
#[derive(Component)]
pub struct PipelineVisual {
    pub pipeline_id: String,
}

/// Set up the CRM visualization
fn setup_crm_visualization(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Add a light source
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });

    // Add a camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    // Create visual representations for CRM entities
    // In a real implementation, this would be based on actual CRM data
    create_contact_visuals(&mut commands, &mut meshes, &mut materials);
    create_deal_visuals(&mut commands, &mut meshes, &mut materials);
    create_pipeline_visuals(&mut commands, &mut meshes, &mut materials);
}

/// Create visual representations for contacts
fn create_contact_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create a sphere for each contact
    // In a real implementation, this would be based on actual contact data
    for i in 0..5 {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Icosphere {
                    radius: 0.5,
                    subdivisions: 32,
                })),
                material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
                transform: Transform::from_xyz(i as f32 * 2.0 - 4.0, 0.5, 0.0),
                ..default()
            })
            .insert(ContactVisual {
                contact_id: format!("contact_{}", i),
            });
    }
}

/// Create visual representations for deals
fn create_deal_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create a cube for each deal
    // In a real implementation, this would be based on actual deal data
    for i in 0..3 {
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                material: materials.add(Color::rgb(0.6, 0.8, 0.7).into()),
                transform: Transform::from_xyz(0.0, i as f32 * 2.0 + 1.0, i as f32 * 2.0 - 2.0),
                ..default()
            })
            .insert(DealVisual {
                deal_id: format!("deal_{}", i),
            });
    }
}

/// Create visual representations for pipelines
fn create_pipeline_visuals(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
    // Create a cylinder for each pipeline
    // In a real implementation, this would be based on actual pipeline data
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cylinder {
                radius: 1.0,
                height: 0.2,
                resolution: 32,
                segments: 1,
            })),
            material: materials.add(Color::rgb(0.7, 0.6, 0.8).into()),
            transform: Transform::from_xyz(0.0, 0.1, 0.0),
            ..default()
        })
        .insert(PipelineVisual {
            pipeline_id: "main_pipeline".to_string(),
        });
}

/// Update the CRM visualization
fn update_crm_visualization(
    mut contact_query: Query<(&ContactVisual, &mut Transform)>,
    mut deal_query: Query<(&DealVisual, &mut Transform)>,
    time: Res<Time>,
) {
    // Animate contacts
    for (_contact, mut transform) in contact_query.iter_mut() {
        // Simple floating animation
        transform.translation.y = 0.5 + (time.elapsed_seconds() * 2.0).sin() * 0.2;
    }

    // Animate deals
    for (_deal, mut transform) in deal_query.iter_mut() {
        // Simple pulsing animation
        let scale = 1.0 + (time.elapsed_seconds() * 3.0).sin() * 0.1;
        transform.scale = Vec3::splat(scale);
    }
}