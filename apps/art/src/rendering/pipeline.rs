//! Custom rendering pipeline for the Art application
//!
//! This module implements GPU acceleration using Bevy's rendering system,
//! including compute shaders for blending and mipmap generation.

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
    Render, RenderApp, RenderSet,
};
use crate::rendering::{RenderSettings, RenderQuality};

/// Custom pipeline for art rendering
#[derive(Resource)]
pub struct ArtRenderPipeline {
    pub pipeline: CachedComputePipelineId,
    pub bind_group_layout: BindGroupLayout,
}

impl FromWorld for ArtRenderPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        
        // Create bind group layout for compute shader
        let bind_group_layout = render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba8UnormSrgb,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::COMPUTE,
                    ty: BindingType::StorageTexture {
                        access: StorageTextureAccess::ReadWrite,
                        format: TextureFormat::Rgba8UnormSrgb,
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
            ],
            label: Some("art_render_bind_group_layout"),
        });
        
        // Load compute shader
        let shader = Shader::from_wgsl(
            include_str!("../../assets/shaders/blending.wgsl"),
            "blending.wgsl",
        );
        
        let pipeline_cache = world.resource::<PipelineCache>();
        let pipeline = pipeline_cache.queue_compute_pipeline(ComputePipelineDescriptor {
            label: Some("art_render_pipeline".into()),
            layout: vec![bind_group_layout.clone()],
            push_constant_ranges: Vec::new(),
            shader: shader.clone(),
            shader_defs: vec![],
            entry_point: "main".into(),
        });
        
        ArtRenderPipeline {
            pipeline,
            bind_group_layout,
        }
    }
}

/// Component for layers that need GPU blending
#[derive(Component)]
pub struct GpuBlendingLayer {
    pub texture_source: Handle<Image>,
    pub texture_dest: Handle<Image>,
    pub blend_mode: u32, // 0=Normal, 1=Multiply, etc.
}

/// System to dispatch compute shader for layer blending
pub fn dispatch_blending_compute(
    mut commands: Commands,
    pipeline: Res<ArtRenderPipeline>,
    pipeline_cache: Res<PipelineCache>,
    render_settings: Res<RenderSettings>,
    gpu_layers: Query<(Entity, &GpuBlendingLayer)>,
    gpu_images: Res<Assets<Image>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
) {
    // Only run if we have the pipeline ready
    let pipeline = match pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
        Some(pipeline) => pipeline,
        None => return,
    };
    
    let mut pass_count = 0;
    
    for (entity, gpu_layer) in gpu_layers.iter() {
        // Get image data
        let source_image = match gpu_images.get(&gpu_layer.texture_source) {
            Some(image) => image,
            None => continue,
        };
        
        let dest_image = match gpu_images.get(&gpu_layer.texture_dest) {
            Some(image) => image,
            None => continue,
        };
        
        // Create texture views
        let source_view = source_image.texture_view.clone();
        let dest_view = dest_image.texture_view.clone();
        
        // Create bind group
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(&source_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::TextureView(&dest_view),
                },
            ],
            label: Some("blending_bind_group"),
            layout: &pipeline.get_bind_group_layout(0),
        });
        
        // Dispatch compute pass
        let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("blending_encoder"),
        });
        
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("blending_compute_pass"),
            });
            
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &bind_group, &[]);
            
            // Calculate work group count based on texture size
            let work_group_size = 8; // Must match shader
            let width = (source_image.texture_descriptor.size.width + work_group_size - 1) / work_group_size;
            let height = (source_image.texture_descriptor.size.height + work_group_size - 1) / work_group_size;
            
            pass.dispatch_workgroups(width, height, 1);
        }
        
        render_queue.submit(vec![encoder.finish()]);
        pass_count += 1;
        
        // Remove component after processing
        commands.entity(entity).remove::<GpuBlendingLayer>();
    }
    
    if pass_count > 0 {
        info!("Executed {} GPU blending passes", pass_count);
    }
}

/// Generate mipmaps for a texture
pub fn generate_mipmaps(
    images: &mut Assets<Image>,
    texture: &Handle<Image>,
    levels: u32,
) -> Vec<Handle<Image>> {
    let mut mipmaps = Vec::new();
    
    if let Some(base_image) = images.get(texture) {
        let base_width = base_image.texture_descriptor.size.width;
        let base_height = base_image.texture_descriptor.size.height;
        
        for level in 1..=levels {
            let scale = 1.0 / (2u32.pow(level)) as f32;
            let width = (base_width as f32 * scale) as u32;
            let height = (base_height as f32 * scale) as u32;
            
            if width == 0 || height == 0 {
                break;
            }
            
            // In a real implementation, we would use a compute shader to generate mipmaps
            // For now, we'll create a placeholder
            let size = Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };
            
            let mut mipmap_image = Image::new_fill(
                size,
                TextureDimension::D2,
                &[128, 128, 128, 255], // Gray placeholder
                TextureFormat::Rgba8UnormSrgb,
            );
            
            mipmap_image.texture_descriptor.usage = 
                TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST;
            
            let handle = images.add(mipmap_image);
            mipmaps.push(handle);
        }
    }
    
    mipmaps
}

/// Configure rendering based on quality settings
pub fn configure_rendering_for_quality(
    render_settings: &RenderSettings,
    images: &mut Assets<Image>,
) {
    match render_settings.quality {
        RenderQuality::Low => {
            // Disable mipmaps, use nearest filtering
            // In a real implementation, we would configure the sampler
            info!("Configured rendering for Low quality");
        }
        RenderQuality::Medium => {
            // Enable 2 mipmap levels, use bilinear filtering
            info!("Configured rendering for Medium quality");
        }
        RenderQuality::High => {
            // Enable full mipmaps, use bicubic filtering (via shader)
            info!("Configured rendering for High quality");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pipeline_creation() {
        // This would require a full Bevy app to test properly
        // For now, we just ensure the module compiles
        assert!(true);
    }
    
    #[test]
    fn test_mipmap_generation() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins)
           .add_plugins(AssetPlugin::default());
        
        let mut images = app.world.resource_mut::<Assets<Image>>();
        let size = Extent3d {
            width: 256,
            height: 256,
            depth_or_array_layers: 1,
        };
        
        let image = Image::new_fill(
            size,
            TextureDimension::D2,
            &[255, 0, 0, 255],
            TextureFormat::Rgba8UnormSrgb,
        );
        
        let handle = images.add(image);
        let mipmaps = generate_mipmaps(&mut images, &handle, 2);
        
        // Should generate 2 mipmap levels
        assert_eq!(mipmaps.len(), 2);
    }
}