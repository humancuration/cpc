//! Financial visualization components for Bevy

use bevy::prelude::*;
use std::collections::HashMap;
use crate::domain::{
    budget::Budget,
    savings_goal::SavingsGoal,
    wallet::Wallet,
};

/// Build a budget vs actual comparison chart
#[cfg(feature = "visualization")]
pub fn build_budget_chart(
    budget: &Budget,
    actual: &HashMap<String, f64>
) -> Vec<u8> {
    // Use default theme for financial visualizations
    // In a real implementation, this would use plotters to generate a chart
    
    // Placeholder implementation - in a real implementation this would generate
    // a chart using plotters and return the image data
    vec![]
}

/// Build a dual-currency budget comparison chart
#[cfg(feature = "visualization")]
pub fn build_dual_currency_budget_chart(
    budget: &Budget,
) -> impl Fn(&mut Commands) {
    move |commands: &mut Commands| {
        // Create side-by-side bar charts showing both currency types
        // Primary currency as blue bar
        let primary_allocated_height = budget.allocation.primary.amount.to_f64().unwrap_or(0.0) as f32 / 100.0;
        let primary_spent_height = budget.spent.primary.amount.to_f64().unwrap_or(0.0) as f32 / 100.0;
        
        // Dabloons as gold/yellow bar
        let dabloons_allocated_height = budget.allocation.dabloons.amount.to_f64().unwrap_or(0.0) as f32 / 100.0;
        let dabloons_spent_height = budget.spent.dabloons.amount.to_f64().unwrap_or(0.0) as f32 / 100.0;
        
        // Create primary currency bars (blue)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, primary_allocated_height, 0.3))),
            material: materials.add(StandardMaterial {
                base_color: Color::BLUE,
                ..default()
            }),
            transform: Transform::from_xyz(-0.5, primary_allocated_height / 2.0, 0.0),
            ..default()
        });
        
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, primary_spent_height, 0.3))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.0, 0.0, 0.5), // Darker blue for spent
                ..default()
            }),
            transform: Transform::from_xyz(-0.5, primary_spent_height / 2.0, 0.1),
            ..default()
        });
        
        // Create Dabloons bars (gold)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, dabloons_allocated_height, 0.3))),
            material: materials.add(StandardMaterial {
                base_color: Color::GOLD,
                ..default()
            }),
            transform: Transform::from_xyz(0.5, dabloons_allocated_height / 2.0, 0.0),
            ..default()
        });
        
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(0.3, dabloons_spent_height, 0.3))),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.8, 0.7, 0.0), // Darker gold for spent
                ..default()
            }),
            transform: Transform::from_xyz(0.5, dabloons_spent_height / 2.0, 0.1),
            ..default()
        });
    }
}

/// Build a 3D progress ring visualization for savings goals
#[cfg(feature = "visualization")]
pub fn build_savings_progress_ring(
    goal: &SavingsGoal,
) -> impl Fn(&mut Commands) {
    move |commands: &mut Commands| {
        // Calculate progress as a value between 0.0 and 1.0
        // For mixed currency goals, we'll show both progress rings
        let primary_progress = if goal.target.primary.is_zero() {
            0.0
        } else {
            (goal.current.primary.amount / goal.target.primary.amount).to_f64().unwrap_or(0.0) as f32
        };
        
        let dabloons_progress = if goal.target.dabloons.is_zero() {
            0.0
        } else {
            (goal.current.dabloons.amount / goal.target.dabloons.amount).to_f64().unwrap_or(0.0) as f32
        };
        
        // Create outer ring for primary currency (blue)
        commands.spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Torus {
                radius: 1.0,
                ring_radius: 0.2,
                subdivisions: 64,
            })),
            material: materials.add(StandardMaterial {
                base_color: Color::rgba(0.2, 0.6, 1.0, primary_progress * 0.8 + 0.2), // Blue with alpha based on progress
                ..default()
            }),
            transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
            ..default()
        });
        
        // Create inner ring for Dabloons (gold/yellow)
        if dabloons_progress > 0.0 {
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Torus {
                    radius: 0.7,
                    ring_radius: 0.15,
                    subdivisions: 64,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(1.0, 0.84, 0.0, dabloons_progress * 0.8 + 0.2), // Gold with alpha based on progress
                    ..default()
                }),
                transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
                ..default()
            });
        }
    }
}

/// Bevy plugin for financial visualizations
pub struct FinancialVisualizationPlugin;

impl Plugin for FinancialVisualizationPlugin {
    fn build(&self, app: &mut App) {
        // Add systems for financial visualization
        app.add_systems(Update, update_financial_visualizations);
    }
}

/// System to update financial visualizations
fn update_financial_visualizations() {
    // In a real implementation, this would update visualizations based on financial data
}

/// Build a 3D coin visualization for wallet balance
#[cfg(feature = "visualization")]
pub fn build_wallet_coin_visualization(
    wallet: &Wallet,
) -> impl Fn(&mut Commands) {
    move |commands: &mut Commands| {
        // Calculate the number of coins to display based on balance
        // For visualization purposes, we'll cap this at 100 coins
        let coin_count = (wallet.balance.amount.to_f64().unwrap_or(0.0) as usize).min(100);
        
        // Create a stack of Dabloons coins using Bevy's 3D primitives
        // Dabloons are represented with a special "dabloon yellow" color
        for i in 0..coin_count {
            let y_position = i as f32 * 0.05; // Stack coins with small gaps
            
            commands.spawn(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cylinder {
                    radius: 0.5,
                    height: 0.1,
                    resolution: 32,
                    segments: 1,
                })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgb(1.0, 0.84, 0.0), // "Dabloon yellow" - slightly different from gold
                    metallic: 0.9,
                    perceptual_roughness: 0.1,
                    ..default()
                }),
                transform: Transform::from_xyz(0.0, y_position, 0.0),
                ..default()
            });
        }
    }
}