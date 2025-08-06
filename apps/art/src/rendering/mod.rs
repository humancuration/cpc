//! Rendering module for the Art application
//!
//! This module handles all rendering functionality using Bevy engine,
//! including canvas rendering, zoom, pan, and layer visualization.

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use crate::core::models::{Project, Layer, LayerType};
use std::collections::HashMap;
use crate::main::RenderBackend;

// Import new modules
pub mod tile_manager;
pub mod texture_cache;
pub mod render_manager;
pub mod pipeline;
pub mod abstract_pipeline;
pub mod quality_manager;
pub mod scaling;
pub mod effects;
pub mod selection;
pub mod transform;

use tile_manager::TileManager;
use texture_cache::TextureCache;
use pipeline::{ArtRenderPipeline, dispatch_blending_compute};
use quality_manager::{QualityManager, RenderQuality};
use scaling::ScalingManager;
use effects::dispatch_effects_compute;
use selection::render_selection_overlays;
use transform::dispatch_transform_compute;

/// Plugin for art rendering functionality
pub struct ArtRenderingPlugin;

impl Plugin for ArtRenderingPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TextureCache>()
            .init_resource::<RenderSettings>()
            .init_resource::<DirtyRegionTracker>()
            .init_resource::<TileManager>()
            .init_resource::<QualityManager>()
            .init_resource::<ScalingManager>()
            .add_systems(Startup, setup_canvas)
            .add_systems(Update, (
                update_camera,
                render_layers,
                handle_input,
                dispatch_blending_compute,
                dispatch_effects_compute,
                render_selection_overlays,
                dispatch_transform_compute,
                quality_manager::apply_quality_settings,
                scaling::update_scaling,
            ));
        
        // Add render app setup for custom pipeline
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<ArtRenderPipeline>();
    }
}

/// Component for the art canvas
#[derive(Component)]
pub struct Canvas {
    pub width: f32,
    pub height: f32,
    pub project_id: uuid::Uuid,
}

/// Component for a layer in the rendering system
#[derive(Component)]
pub struct RenderLayer {
    pub layer_id: uuid::Uuid,
    pub visible: bool,
    pub texture_handle: Handle<Image>,
}

/// Resource for camera control
#[derive(Resource)]
pub struct CameraController {
    pub zoom: f32,
    pub pan_x: f32,
    pub pan_y: f32,
    pub is_panning: bool,
    pub last_mouse_position: Vec2,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            zoom: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
            is_panning: false,
            last_mouse_position: Vec2::ZERO,
        }
    }
}

/// Resource for tracking dirty regions for optimized rendering
#[derive(Resource, Default)]
pub struct DirtyRegionTracker {
    /// Dirty regions by layer ID
    pub dirty_regions: HashMap<uuid::Uuid, Vec<Rect>>,
}

/// Rectangle bounds for positioning
#[derive(Debug, Clone)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl DirtyRegionTracker {
    /// Mark a region as dirty for a layer
    pub fn mark_dirty_region(&mut self, layer_id: uuid::Uuid, region: Rect) {
        self.dirty_regions
            .entry(layer_id)
            .or_insert_with(Vec::new)
            .push(region);
    }
    
    /// Get dirty regions for a layer
    pub fn get_dirty_regions(&self, layer_id: uuid::Uuid) -> Option<&Vec<Rect>> {
        self.dirty_regions.get(&layer_id)
    }
    
    /// Clear dirty regions for a layer
    pub fn clear_dirty_regions(&mut self, layer_id: uuid::Uuid) {
        self.dirty_regions.remove(&layer_id);
    }
}

/// Render quality settings
#[derive(Resource, Debug, Clone, PartialEq)]
pub struct RenderSettings {
    pub quality: RenderQuality,
    pub enable_tiling: bool,
    pub tile_size: u32,
}

impl Default for RenderSettings {
    fn default() -> Self {
        Self {
            quality: RenderQuality::High,
            enable_tiling: true,
            tile_size: 256,
        }
    }
}

/// Resource for the current project being rendered
#[derive(Resource)]
pub struct RenderProject {
    pub project: Project,
}

/// Set up the initial canvas and camera
fn setup_canvas(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Camera with controller
    commands.insert_resource(CameraController::default());
    commands.insert_resource(RenderSettings::default());
    commands.insert_resource(DirtyRegionTracker::default());
    commands.insert_resource(TileManager::new(256)); // Default tile size
    
    commands.spawn((
        Camera2dBundle {
            projection: OrthographicProjection {
                scale: 1.0,
                ..default()
            },
            ..default()
        },
        Name::new("MainCamera"),
    ));

    // Background grid
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
            transform: Transform::from_scale(Vec3::new(10000.0, 10000.0, 1.0)),
            material: materials.add(ColorMaterial::from(Color::rgb(0.2, 0.2, 0.2))),
            ..default()
        },
        Name::new("Background"),
    ));
}

/// Update camera based on controller
fn update_camera(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), With<Camera2d>>,
    controller: Res<CameraController>,
) {
    if let Ok((mut projection, mut transform)) = camera_query.get_single_mut() {
        projection.scale = 1.0 / controller.zoom;
        transform.translation.x = -controller.pan_x;
        transform.translation.y = -controller.pan_y;
    }
}

/// Render layers in the project
fn render_layers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_cache: ResMut<TextureCache>,
    mut tile_manager: ResMut<TileManager>,
    render_project: Option<Res<RenderProject>>,
    render_settings: Res<RenderSettings>,
    camera_controller: Res<CameraController>,
) {
    if let Some(render_project) = render_project {
        let project = &render_project.project;
        
        // Update tile manager with current tile size
        if tile_manager.tile_size != render_settings.tile_size {
            // In a real implementation, we would recreate tiles when size changes
            // For now, we'll just update the tile size
            tile_manager.tile_size = render_settings.tile_size;
        }
        
        // Calculate render scale based on DPI and zoom level
        let dpi_scale = project.resolution / 72.0; // 72 PPI as base
        let render_scale = dpi_scale * camera_controller.zoom;
        
        // Spawn or update canvas entity
        let canvas_entity = commands.spawn((
            Canvas {
                width: project.width as f32,
                height: project.height as f32,
                project_id: project.id,
            },
            Transform::from_xyz(0.0, 0.0, 0.0),
            GlobalTransform::default(),
        )).id();
        
        // If tiling is enabled, render tiles instead of full layers
        if render_settings.enable_tiling {
            render_tiled_layers(
                &mut commands,
                &mut meshes,
                &mut images,
                &mut materials,
                &mut texture_cache,
                &mut tile_manager,
                project,
                &render_settings,
                &camera_controller,
            );
        } else {
            // Render each layer (non-tiled approach)
            for layer in &project.layers {
                if layer.visible {
                    // Get or create texture for this layer
                    let texture_entry = texture_cache.get_or_create_texture(layer, &mut images);
                            
                    // Create mesh for the layer
                    let mesh_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                        layer.bounds.width,
                        layer.bounds.height,
                    ))));
                    
                    // Create material with the layer texture
                    let material_handle = materials.add(ColorMaterial {
                        texture: Some(texture_entry.bevy_handle.clone()),
                        color: Color::rgba(1.0, 1.0, 1.0, layer.opacity),
                    });
                    
                    // Spawn or update layer entity
                    commands.spawn((
                        RenderLayer {
                            layer_id: layer.id,
                            visible: layer.visible,
                            texture_handle,
                        },
                        MaterialMesh2dBundle {
                            mesh: mesh_handle.into(),
                            material: material_handle,
                            transform: Transform::from_xyz(
                                layer.bounds.x + layer.bounds.width / 2.0,
                                layer.bounds.y + layer.bounds.height / 2.0,
                                0.0, // Z-order based on layer index
                            ),
                            ..default()
                        },
                    ));
                }
            }
        }
    }
}

/// Render layers using tiled approach for better performance
fn render_tiled_layers(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    images: &mut Assets<Image>,
    materials: &mut Assets<ColorMaterial>,
    texture_cache: &mut TextureCache,
    tile_manager: &mut TileManager,
    project: &Project,
    render_settings: &RenderSettings,
    camera_controller: &CameraController,
) {
    // Process each layer
    for layer in &project.layers {
        if !layer.visible {
            continue;
        }
        
        // Divide layer into tiles if not already done
        if tile_manager.get_layer_tiles(layer.id).is_none() {
            tile_manager.divide_layer_into_tiles(layer);
        }
        
        // Get visible tiles for this layer
        let visible_tiles = tile_manager.calculate_visible_tiles(layer.id, camera_controller, project);
        
        // Get or create texture atlas for this layer
        let atlas = texture_cache.get_or_create_atlas(layer.id, images, render_settings.tile_size);
        
        // Render each visible tile
        for tile in visible_tiles {
            // In a real implementation, we would:
            // 1. Extract tile data from layer
            // 2. Update atlas with tile data if dirty
            // 3. Render tile using atlas coordinates
            
            // For now, we'll create a placeholder for each tile
            let tile_width = tile.width as f32;
            let tile_height = tile.height as f32;
            
            // Create mesh for the tile
            let mesh_handle = meshes.add(Mesh::from(shape::Quad::new(Vec2::new(
                tile_width,
                tile_height,
            ))));
            
            // Create a placeholder texture for the tile
            let size = Extent3d {
                width: tile.width,
                height: tile.height,
                depth_or_array_layers: 1,
            };
            
            let mut tile_image = Image::new_fill(
                size,
                TextureDimension::D2,
                &[100, 150, 200, 255], // Blue placeholder
                TextureFormat::Rgba8UnormSrgb,
            );
            
            tile_image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
            let tile_texture = images.add(tile_image);
            
            // Create material with the tile texture
            let material_handle = materials.add(ColorMaterial {
                texture: Some(tile_texture),
                color: Color::rgba(1.0, 1.0, 1.0, layer.opacity),
            });
            
            // Spawn tile entity with appropriate position
            commands.spawn((
                MaterialMesh2dBundle {
                    mesh: mesh_handle.into(),
                    material: material_handle,
                    transform: Transform::from_xyz(
                        tile.x as f32 + tile.width as f32 / 2.0,
                        tile.y as f32 + tile.height as f32 / 2.0,
                        0.0,
                    ),
                    ..default()
                },
                Name::new(format!("Tile_{}_{}_{}", layer.id, tile.x, tile.y)),
            ));
        }
    }
    
    info!("Rendered {} layers using tiled approach", project.layers.len());
}

/// Convert layer pixels to a Bevy Image texture
fn layer_to_image(layer: &Layer) -> Image {
    // Create image with RGBA8 format
    let width = layer.bounds.width as u32;
    let height = layer.bounds.height as u32;
    
    // Create image data from layer pixels
    // Layer pixels are stored as RGBA values
    let pixel_data = layer.pixels.clone();
    
    let size = Extent3d {
        width,
        height,
        depth_or_array_layers: 1,
    };
    
    let mut image = Image::new(
        size,
        TextureDimension::D2,
        pixel_data,
        TextureFormat::Rgba8UnormSrgb,
    );
    
    // Set texture usage for rendering
    image.texture_descriptor.usage = TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
    
    image
}

/// Handle user input for camera controls
fn handle_input(
    mut camera_controller: ResMut<CameraController>,
    mouse_input: Res<Input<MouseButton>>,
    scroll_events: EventReader<MouseWheel>,
    windows: Query<&Window>,
) {
    let window = windows.single();
    
    // Handle panning
    if mouse_input.just_pressed(MouseButton::Middle) {
        camera_controller.is_panning = true;
        if let Some(cursor_position) = window.cursor_position() {
            camera_controller.last_mouse_position = cursor_position;
        }
    }
    
    if mouse_input.just_released(MouseButton::Middle) {
        camera_controller.is_panning = false;
    }
    
    if camera_controller.is_panning {
        if let Some(cursor_position) = window.cursor_position() {
            let delta = cursor_position - camera_controller.last_mouse_position;
            camera_controller.pan_x -= delta.x / camera_controller.zoom;
            camera_controller.pan_y += delta.y / camera_controller.zoom;
            camera_controller.last_mouse_position = cursor_position;
        }
    }
    
    // Handle zooming
    for event in scroll_events.read() {
        match event.unit {
            MouseScrollUnit::Line => {
                camera_controller.zoom *= 1.0 + event.y * 0.1;
            }
            MouseScrollUnit::Pixel => {
                camera_controller.zoom *= 1.0 + event.y * 0.01;
            }
        }
        // Clamp zoom to reasonable values
        camera_controller.zoom = camera_controller.zoom.clamp(0.1, 10.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::ecs::system::RunSystemOnce;
    
    #[test]
    fn test_camera_controller_default() {
        let controller = CameraController::default();
        assert_eq!(controller.zoom, 1.0);
        assert_eq!(controller.pan_x, 0.0);
        assert_eq!(controller.pan_y, 0.0);
        assert_eq!(controller.is_panning, false);
    }
    
    #[test]
    fn test_canvas_component() {
        let canvas = Canvas {
            width: 800.0,
            height: 600.0,
            project_id: uuid::Uuid::new_v4(),
        };
        assert_eq!(canvas.width, 800.0);
        assert_eq!(canvas.height, 600.0);
    }
    
    #[test]
    fn test_render_settings_default() {
        let settings = RenderSettings::default();
        assert_eq!(settings.quality, RenderQuality::High);
        assert_eq!(settings.enable_tiling, true);
        assert_eq!(settings.tile_size, 256);
    }
    
    #[test]
    fn test_dirty_region_tracker() {
        let mut tracker = DirtyRegionTracker::default();
        let layer_id = uuid::Uuid::new_v4();
        
        assert!(tracker.get_dirty_regions(layer_id).is_none());
        
        let region = Rect {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        
        tracker.mark_dirty_region(layer_id, region.clone());
        assert!(tracker.get_dirty_regions(layer_id).is_some());
        
        let regions = tracker.get_dirty_regions(layer_id).unwrap();
        assert_eq!(regions.len(), 1);
        assert_eq!(regions[0].x, 0.0);
        assert_eq!(regions[0].y, 0.0);
        assert_eq!(regions[0].width, 100.0);
        assert_eq!(regions[0].height, 100.0);
        
        tracker.clear_dirty_regions(layer_id);
        assert!(tracker.get_dirty_regions(layer_id).is_none());
    }
    
    #[test]
    fn test_layer_to_image() {
        let mut layer = Layer::new("Test Layer".to_string(), 100, 100, LayerType::Raster);
        
        // Fill with red pixels
        for i in 0..layer.pixels.len() {
            if i % 4 == 0 {
                layer.pixels[i] = 255; // Red
            } else if i % 4 == 1 {
                layer.pixels[i] = 0; // Green
            } else if i % 4 == 2 {
                layer.pixels[i] = 0; // Blue
            } else {
                layer.pixels[i] = 255; // Alpha
            }
        }
        
        let image = layer_to_image(&layer);
        assert_eq!(image.texture_descriptor.size.width, 100);
        assert_eq!(image.texture_descriptor.size.height, 100);
        assert_eq!(image.data.len(), 100 * 100 * 4); // RGBA
    }
}