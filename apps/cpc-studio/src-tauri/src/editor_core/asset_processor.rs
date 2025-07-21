use super::assets::{AssetMetadata, AssetType};
use anyhow::{Context, Result};
use bevy::{
    prelude::*,
    render::{
        camera::RenderTarget,
        render_resource::{
            Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
        },
        texture::Image,
        view::RenderLayers,
    },
    window::WindowPlugin,
};
use crossbeam_channel::{bounded, Receiver};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::time::timeout;

/// Generates a thumbnail for the given asset
pub async fn generate_thumbnail(metadata: &AssetMetadata) -> Result<PathBuf> {
    // Create thumbnail directory if it doesn't exist
    let thumb_dir = PathBuf::from("assets/thumbnails");
    tokio::fs::create_dir_all(&thumb_dir)
        .await
        .context("Failed to create thumbnail directory")?;

    // Define output path
    let thumb_path = thumb_dir.join(format!("{}.png", metadata.asset_id));

    match metadata.asset_type {
        AssetType::Texture => {
            tokio::task::spawn_blocking({
                let path = metadata.path.clone();
                let thumb_path = thumb_path.clone();
                move || {
                    // Load and resize image
                    let img = ImageReader::open(&path)
                        .context("Failed to open image")?
                        .decode()
                        .context("Failed to decode image")?;
                    
                    let thumbnail = img.thumbnail(128, 128);
                    
                    // Save as PNG
                    thumbnail.save(&thumb_path)
                        .context("Failed to save thumbnail")?;
                    
                    Ok::<_, anyhow::Error>(())
                }
                
                /// Renders a 3D model to a PNG thumbnail using headless Bevy
                async fn render_model_headless(model_path: &PathBuf, output_path: &PathBuf) -> Result<()> {
                    // Channel to receive the rendered image
                    let (sender, receiver) = bounded(1);
                    let receiver = Arc::new(Mutex::new(receiver));
                
                    // Run Bevy app in a blocking thread
                    let model_path = model_path.clone();
                    let output_path = output_path.clone();
                    let handle = std::thread::spawn(move || {
                        let mut app = App::new();
                
                        // Minimal plugins needed for headless rendering
                        app.add_plugins(MinimalPlugins);
                        app.add_plugin(AssetPlugin::default());
                        app.add_plugin(bevy::render::RenderPlugin::default());
                        app.add_plugin(bevy::window::WindowPlugin {
                            primary_window: None,
                            exit_condition: bevy::window::ExitCondition::DontExit,
                            close_when_requested: false,
                        });
                        app.add_plugin(bevy::core::CorePlugin::default());
                        app.add_plugin(bevy::scene::ScenePlugin);
                        app.add_plugin(bevy_gltf::GltfPlugin);
                
                        // Create render texture
                        let size = Extent3d {
                            width: 256,
                            height: 256,
                            ..default()
                        };
                
                        let mut image = Image {
                            texture_descriptor: TextureDescriptor {
                                label: None,
                                size,
                                dimension: TextureDimension::D2,
                                format: TextureFormat::Rgba8UnormSrgb,
                                mip_level_count: 1,
                                sample_count: 1,
                                usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::COPY_SRC,
                                view_formats: &[],
                            },
                            ..default()
                        };
                
                        // Fill image data with transparent pixels
                        image.resize(size);
                
                        let image_handle = app.world.spawn(image).id();
                        let images = &mut app.world.resource_mut::<Assets<Image>>();
                        let image_handle = images.add(image);
                
                        // Camera
                        let camera = Camera3dBundle {
                            camera: Camera {
                                target: RenderTarget::Image(image_handle.clone()),
                                ..default()
                            },
                            transform: Transform::from_xyz(2.0, 2.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
                            ..default()
                        };
                        app.world.spawn(camera).insert(RenderLayers::layer(1));
                
                        // Light
                        app.world.spawn(DirectionalLightBundle {
                            directional_light: DirectionalLight {
                                illuminance: 1000.0,
                                ..default()
                            },
                            transform: Transform::from_xyz(4.0, 7.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
                            ..default()
                        });
                
                        // Load model
                        let asset_server = app.world.resource::<AssetServer>();
                        let model_handle: Handle<Gltf> = asset_server.load(model_path);
                
                        // System to spawn model once loaded
                        app.add_system(move |mut commands: Commands, models: Res<Assets<Gltf>>| {
                            if let Some(model) = models.get(&model_handle) {
                                // Spawn the first scene in the GLTF
                                if let Some(scene) = model.scenes.first() {
                                    commands.spawn(SceneBundle {
                                        scene: scene.clone(),
                                        ..default()
                                    });
                                }
                            }
                        });
                
                        // System to capture rendered image
                        app.add_system(move |images: Res<Assets<Image>>| {
                            if let Some(image) = images.get(&image_handle) {
                                if image.data.len() >= 4 * 256 * 256 {
                                    let _ = sender.send(image.data.clone());
                                }
                            }
                        });
                
                        // Run the app for a few frames to load and render
                        for _ in 0..10 {
                            app.update();
                        }
                    });
                
                    // Wait for rendering to complete with timeout
                    let render_result = timeout(Duration::from_secs(5), async {
                        let receiver = receiver.lock().unwrap();
                        receiver.recv().map_err(|_| anyhow::anyhow!("Render failed"))
                    })
                    .await;
                
                    // Join the Bevy thread
                    handle.join().map_err(|_| anyhow::anyhow!("Bevy thread panicked"))?;
                
                    // Process the rendered image
                    let image_data = render_result??;
                    let image_buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(256, 256, image_data)
                        .ok_or_else(|| anyhow::anyhow!("Invalid image dimensions"))?;
                
                    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);
                    dynamic_image
                        .save(output_path)
                        .context("Failed to save rendered thumbnail")?;
                
                    Ok(())
                }
            })
            .await??;
        }
        AssetType::Model => {
            // Use headless Bevy rendering for models
            render_model_headless(&metadata.path, &thumb_path).await?;
        }
        _ => {
            // Return error for unsupported asset types
            anyhow::bail!(
                "Thumbnail generation not supported for {:?} assets",
                metadata.asset_type
            )
        }
    }

    Ok(thumb_path)
}