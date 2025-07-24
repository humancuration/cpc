//! AR rendering system for 3D overlays
//!
//! Handles rendering of 3D models, text annotations, and interactive elements
//! on detected AR markers with occlusion handling and performance optimizations.

use bevy::prelude::*;
use bevy::render::view::RenderLayers;
use bevy::pbr::{StandardMaterial, PbrBundle};
use bevy::text::{Text, Text2dBundle, TextStyle};
use bevy::gltf::Gltf;

use crate::ar_tracking::{ARTrackedEntity, ARTrackingState};

/// AR overlay component for 3D models
#[derive(Component, Debug)]
pub struct AROverlay3D {
    pub marker_id: String,
    pub model_path: Option<String>,
    pub scale: f32,
    pub auto_scale: bool,
    pub occlusion_mode: OcclusionMode,
    pub lod_level: LODLevel,
}

/// AR text annotation component
#[derive(Component, Debug)]
pub struct ARTextAnnotation {
    pub marker_id: String,
    pub text: String,
    pub font_size: f32,
    pub color: Color,
    pub billboard: bool,
}

/// AR interactive element component
#[derive(Component, Debug)]
pub struct ARInteractiveElement {
    pub marker_id: String,
    pub interaction_type: InteractionType,
    pub bounds: Vec3,
    pub enabled: bool,
}

/// Occlusion handling modes
#[derive(Debug, Clone, Copy)]
pub enum OcclusionMode {
    None,
    DepthTest,
    DepthWrite,
    FullOcclusion,
}

/// Level of detail for performance optimization
#[derive(Debug, Clone, Copy)]
pub enum LODLevel {
    Low,
    Medium,
    High,
    Ultra,
}

/// Interaction types for AR elements
#[derive(Debug, Clone, Copy)]
pub enum InteractionType {
    Button,
    Slider,
    Toggle,
    Custom,
}

/// AR rendering configuration
#[derive(Resource, Debug)]
pub struct ARRenderingConfig {
    pub enable_occlusion: bool,
    pub lod_distance: Vec<f32>, // [low, medium, high] distances
    pub texture_quality: TextureQuality,
    pub enable_shadows: bool,
    pub enable_post_processing: bool,
}

/// Texture quality settings
#[derive(Debug, Clone, Copy)]
pub enum TextureQuality {
    Low,
    Medium,
    High,
    Ultra,
}

/// AR render layer for compositing
#[derive(Resource, Debug)]
pub struct ARRenderLayer {
    pub layer_id: u8,
    pub render_layers: RenderLayers,
}

/// System to spawn AR overlays on detected markers
pub fn spawn_ar_overlays(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tracked_entities: Query<(Entity, &ARTrackedEntity), Added<ARTrackedEntity>>,
    overlay_query: Query<&AROverlay3D>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, tracked) in tracked_entities.iter() {
        // Check if overlay already exists for this marker
        let existing_overlay = overlay_query.iter()
            .any(|overlay| overlay.marker_id == tracked.marker_id);
        
        if !existing_overlay {
            // Create 3D overlay
            let material = materials.add(StandardMaterial {
                base_color: Color::srgb(0.2, 0.8, 1.0),
                metallic: 0.5,
                perceptual_roughness: 0.3,
                ..default()
            });
            
            let overlay = commands.spawn((
                PbrBundle {
                    mesh: asset_server.load("models/cube.glb#Mesh0/Primitive0"),
                    material,
                    transform: Transform::from_scale(Vec3::splat(0.1)),
                    ..default()
                },
                AROverlay3D {
                    marker_id: tracked.marker_id.clone(),
                    model_path: Some("models/cube.glb".to_string()),
                    scale: 0.1,
                    auto_scale: true,
                    occlusion_mode: OcclusionMode::DepthTest,
                    lod_level: LODLevel::Medium,
                },
                ARTrackedEntity {
                    marker_id: tracked.marker_id.clone(),
                    content_hash: tracked.content_hash.clone(),
                    confidence: tracked.confidence,
                    last_update: tracked.last_update,
                },
            )).id();
            
            commands.entity(entity).add_child(overlay);
        }
    }
}

/// System to update AR overlay positions
pub fn update_ar_overlays(
    mut overlay_query: Query<(&mut Transform, &AROverlay3D)>,
    tracked_query: Query<&ARTrackedEntity>,
    tracking_state: Res<ARTrackingState>,
) {
    for (mut transform, overlay) in overlay_query.iter_mut() {
        if let Some(tracked) = tracking_state.active_markers.get(&overlay.marker_id) {
            // Update position based on tracked entity
            if let Some(tracked_entity) = tracked_query.iter()
                .find(|e| e.marker_id == overlay.marker_id) {
                // Apply scale based on LOD
                let scale = match overlay.lod_level {
                    LODLevel::Low => overlay.scale * 0.5,
                    LODLevel::Medium => overlay.scale,
                    LODLevel::High => overlay.scale * 1.5,
                    LODLevel::Ultra => overlay.scale * 2.0,
                };
                
                transform.scale = Vec3::splat(scale);
            }
        }
    }
}

/// System to handle occlusion
pub fn handle_occlusion(
    mut overlay_query: Query<(&mut Visibility, &AROverlay3D, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<ARCamera>>,
    tracking_state: Res<ARTrackingState>,
) {
    if !tracking_state.tracking_enabled {
        return;
    }
    
    for (mut visibility, overlay, transform) in overlay_query.iter_mut() {
        match overlay.occlusion_mode {
            OcclusionMode::None => {
                *visibility = Visibility::Visible;
            }
            OcclusionMode::DepthTest => {
                // Implement depth testing logic
                *visibility = Visibility::Visible;
            }
            OcclusionMode::DepthWrite => {
                // Implement depth writing
                *visibility = Visibility::Visible;
            }
            OcclusionMode::FullOcclusion => {
                // Implement full occlusion
                *visibility = Visibility::Visible;
            }
        }
    }
}

/// System to spawn text annotations
pub fn spawn_text_annotations(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    tracked_entities: Query<(Entity, &ARTrackedEntity), Added<ARTrackedEntity>>,
    text_query: Query<&ARTextAnnotation>,
) {
    for (entity, tracked) in tracked_entities.iter() {
        // Check if text annotation already exists
        let existing_text = text_query.iter()
            .any(|text| text.marker_id == tracked.marker_id);
        
        if !existing_text {
            let font = asset_server.load("fonts/FiraSans-Bold.ttf");
            
            let text_entity = commands.spawn((
                Text2dBundle {
                    text: Text::from_section(
                        "AR Annotation",
                        TextStyle {
                            font,
                            font_size: 20.0,
                            color: Color::WHITE,
                        },
                    ),
                    transform: Transform::from_xyz(0.0, 0.2, 0.0),
                    ..default()
                },
                ARTextAnnotation {
                    marker_id: tracked.marker_id.clone(),
                    text: "AR Annotation".to_string(),
                    font_size: 20.0,
                    color: Color::WHITE,
                    billboard: true,
                },
            )).id();
            
            commands.entity(entity).add_child(text_entity);
        }
    }
}

/// System to update text annotations
pub fn update_text_annotations(
    mut text_query: Query<(&mut Text, &ARTextAnnotation)>,
    tracking_state: Res<ARTrackingState>,
) {
    for (mut text, annotation) in text_query.iter_mut() {
        if let Some(tracked) = tracking_state.active_markers.get(&annotation.marker_id) {
            // Update text content based on tracking confidence
            let confidence_text = format!("Confidence: {:.2}%", tracked.confidence * 100.0);
            text.sections[0].value = confidence_text;
        }
    }
}

/// LOD system for performance optimization
pub fn lod_system(
    mut overlay_query: Query<(&mut Handle<StandardMaterial>, &mut AROverlay3D, &GlobalTransform)>,
    camera_query: Query<(&Camera, &GlobalTransform), With<ARCamera>>,
    materials: Res<Assets<StandardMaterial>>,
    mut material_assets: ResMut<Assets<StandardMaterial>>,
    config: Res<ARRenderingConfig>,
) {
    for (camera, camera_transform) in camera_query.iter() {
        for (mut material_handle, mut overlay, transform) in overlay_query.iter_mut() {
            let distance = camera_transform.translation().distance(transform.translation());
            
            // Update LOD level based on distance
            overlay.lod_level = if distance < config.lod_distance[0] {
                LODLevel::Ultra
            } else if distance < config.lod_distance[1] {
                LODLevel::High
            } else if distance < config.lod_distance[2] {
                LODLevel::Medium
            } else {
                LODLevel::Low
            };
            
            // Update material quality based on LOD
            if let Some(material) = materials.get_mut(&material_handle) {
                match overlay.lod_level {
                    LODLevel::Low => {
                        material.perceptual_roughness = 0.8;
                        material.metallic = 0.1;
                    }
                    LODLevel::Medium => {
                        material.perceptual_roughness = 0.5;
                        material.metallic = 0.3;
                    }
                    LODLevel::High => {
                        material.perceptual_roughness = 0.3;
                        material.metallic = 0.5;
                    }
                    LODLevel::Ultra => {
                        material.perceptual_roughness = 0.1;
                        material.metallic = 0.7;
                    }
                }
            }
        }
    }
}

/// Texture streaming system
pub fn texture_streaming_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    overlay_query: Query<&AROverlay3D>,
    config: Res<ARRenderingConfig>,
) {
    // Implement texture streaming based on distance and quality settings
    match config.texture_quality {
        TextureQuality::Low => {
            // Load low-resolution textures
        }
        TextureQuality::Medium => {
            // Load medium-resolution textures
        }
        TextureQuality::High => {
            // Load high-resolution textures
        }
        TextureQuality::Ultra => {
            // Load ultra-resolution textures
        }
    }
}

/// AR rendering plugin
pub struct ARRenderingPlugin;

impl Plugin for ARRenderingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ARRenderingConfig>()
            .init_resource::<ARRenderLayer>()
            .add_systems(Update, (
                spawn_ar_overlays,
                update_ar_overlays,
                handle_occlusion,
                spawn_text_annotations,
                update_text_annotations,
                lod_system,
                texture_streaming_system,
            ));
    }
}

impl Default for ARRenderingConfig {
    fn default() -> Self {
        Self {
            enable_occlusion: true,
            lod_distance: vec![2.0, 5.0, 10.0], // [low, medium, high] distances
            texture_quality: TextureQuality::High,
            enable_shadows: false,
            enable_post_processing: true,
        }
    }
}

impl Default for ARRenderLayer {
    fn default() -> Self {
        Self {
            layer_id: 1,
            render_layers: RenderLayers::layer(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;

    #[test]
    fn test_ar_overlay_creation() {
        let overlay = AROverlay3D {
            marker_id: "test".to_string(),
            model_path: Some("test.glb".to_string()),
            scale: 1.0,
            auto_scale: true,
            occlusion_mode: OcclusionMode::DepthTest,
            lod_level: LODLevel::Medium,
        };
        
        assert_eq!(overlay.marker_id, "test");
        assert_eq!(overlay.scale, 1.0);
    }

    #[test]
    fn test_ar_text_annotation() {
        let annotation = ARTextAnnotation {
            marker_id: "test".to_string(),
            text: "Hello AR".to_string(),
            font_size: 16.0,
            color: Color::WHITE,
            billboard: true,
        };
        
        assert_eq!(annotation.text, "Hello AR");
        assert_eq!(annotation.font_size, 16.0);
    }
}
           