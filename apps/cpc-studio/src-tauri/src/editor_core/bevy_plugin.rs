use bevy::prelude::*;
use bevy::render::camera::RenderTarget;
use bevy::render::render_resource::{
    Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::window::WindowId;
use std::sync::{Arc, Mutex};
use wry::webview::WebView;
use crate::event_bridge::{NetworkEventBridge, editor_event_to_p2p, p2p_event_to_editor};
use cpc_core::p2p::NetworkHandler;
use image;
use base64::{self, engine::general_purpose::STANDARD};
use crate::editor_core::scene::hierarchy::SceneHierarchy;

pub struct EditorBevyPlugin {
    pub webview: Arc<Mutex<Option<WebView>>>,
    pub network_handler: Arc<NetworkHandler>,
}

impl Plugin for EditorBevyPlugin {
    fn build(&self, app: &mut App) {
        // Create network event bridge
        let device_id = "editor_device"; // In production would use actual device ID
        let bridge = NetworkEventBridge::new(self.network_handler.clone(), device_id);
        
        // Create asset manager
        let asset_manager = AssetManager::new(self.network_handler.clone());
        
        // Create scene hierarchy
        let scene_hierarchy = SceneHierarchy::default();
        
        app.insert_resource(EditorTextureResource::default())
            .insert_resource(bridge)
            .insert_resource(asset_manager)
            .insert_resource(scene_hierarchy)
            .add_event::<crate::event_bridge::EditorEvent>()
            .add_event::<crate::event_bridge::EditorCommand>()
            .add_startup_system(setup_editor_camera)
            .add_system(update_texture)
            .add_system(editor_event_to_p2p)
            .add_system(p2p_event_to_editor)
            .add_system(update_scene_hierarchy);
        
        // Setup asset manager systems
        app.resource::<AssetManager>().setup_bevy(app);
    }
}

/// System to update scene hierarchy transforms
fn update_scene_hierarchy(mut scene_hierarchy: ResMut<SceneHierarchy>) {
    scene_hierarchy.update_transforms();
}

#[derive(Default)]
pub struct EditorTextureResource {
    pub texture: Option<Handle<Image>>,
}

fn setup_editor_camera(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut editor_texture: ResMut<EditorTextureResource>,
) {
    // Create texture for offscreen rendering
    let size = Extent3d {
        width: 800,
        height: 600,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("editor_texture"),
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        },
        ..Default::default()
    };

    // Fill image data with placeholder
    image.resize(size);

    let image_handle = images.add(image);
    editor_texture.texture = Some(image_handle.clone());

    // Create camera that renders to texture
    commands.spawn(Camera2dBundle {
        camera: Camera {
            target: RenderTarget::Image(image_handle),
            ..default()
        },
        ..default()
    });
fn image_to_png(image: &Image) -> Result<Vec<u8>, image::ImageError> {
    let size = image.texture_descriptor.size;
    let data = image.data.as_slice();

    // Check the length
    if data.len() != (size.width * size.height * 4) as usize {
        return Err(image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        )));
    }

    // Convert BGRA to RGBA
    let mut rgba_data = Vec::with_capacity(data.len());
    for i in (0..data.len()).step_by(4) {
        rgba_data.push(data[i + 2]); // R
        rgba_data.push(data[i + 1]); // G
        rgba_data.push(data[i]);     // B
        rgba_data.push(data[i + 3]); // A
    }

    let img_buffer = image::ImageBuffer::from_vec(size.width, size.height, rgba_data)
        .ok_or_else(|| image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        )))?;

    let mut png_data = Vec::new();
    let encoder = image::codecs::png::PngEncoder::new(&mut png_data);
    encoder.encode(
        &img_buffer,
        size.width,
        size.height,
        image::ColorType::Rgba8,
    )?;

    Ok(png_data)
}

fn update_texture(
    webview: Res<Arc<Mutex<Option<WebView>>>>,
    editor_texture: Res<EditorTextureResource>,
    images: Res<Assets<Image>>,
) {
    if let Some(webview) = webview.lock().unwrap().as_ref() {
        if let Some(texture_handle) = &editor_texture.texture {
            if let Some(image) = images.get(texture_handle) {
                match image_to_png(image) {
                    Ok(png_data) => {
                        let base64 = STANDARD.encode(&png_data);
                        let js = format!(
                            "window.dispatchEvent(new CustomEvent('texture-update', {{ detail: '{}' }}));",
                            base64
                        );
                        if let Err(e) = webview.evaluate_script(&js) {
                            eprintln!("Failed to evaluate script: {}", e);
                        }
                    }
                    Err(e) => eprintln!("Failed to convert image to PNG: {}", e),
                }
            }
        }
    }
}
    }
}