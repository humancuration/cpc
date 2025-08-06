//! Transform rendering for the Art application
//!
//! This module implements GPU-accelerated rendering of transformed layers,
//! including move, scale, and rotate operations with transform matrices.

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};
use crate::services::transform::TransformMatrix;
use crate::rendering::ArtRenderPipeline;

/// Component for layers that need GPU transformation
#[derive(Component)]
pub struct GpuTransformLayer {
    pub transform_matrix: TransformMatrix,
    pub texture_source: Handle<Image>,
    pub texture_dest: Handle<Image>,
}

/// System to dispatch compute shaders for layer transformations
pub fn dispatch_transform_compute(
    mut commands: Commands,
    pipeline: Res<ArtRenderPipeline>,
    pipeline_cache: Res<PipelineCache>,
    transform_layers: Query<(Entity, &GpuTransformLayer)>,
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
    
    for (entity, transform_layer) in transform_layers.iter() {
        // Get image data
        let source_image = match gpu_images.get(&transform_layer.texture_source) {
            Some(image) => image,
            None => continue,
        };
        
        let dest_image = match gpu_images.get(&transform_layer.texture_dest) {
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
            label: Some("transform_bind_group"),
            layout: &pipeline.get_bind_group_layout(0),
        });
        
        // Dispatch compute pass
        let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
            label: Some("transform_encoder"),
        });
        
        {
            let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                label: Some("transform_compute_pass"),
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
        commands.entity(entity).remove::<GpuTransformLayer>();
    }
    
    if pass_count > 0 {
        info!("Executed {} GPU transform passes", pass_count);
    }
}

/// Apply a transformation matrix to a texture
pub fn apply_transform(
    device: &RenderDevice,
    queue: &RenderQueue,
    source: &TextureView,
    dest: &TextureView,
    matrix: &TransformMatrix,
) {
    // In a real implementation, this would use a compute shader to apply the transformation
    // For now, we'll just log that the transform would be applied
    info!("Applying transformation matrix: {:?}", matrix.matrix);
}

/// Render a transformed layer with bilinear interpolation
pub fn render_transformed_layer(
    device: &RenderDevice,
    queue: &RenderQueue,
    source: &TextureView,
    dest: &TextureView,
    matrix: &TransformMatrix,
) {
    // In a real implementation, this would use a compute shader to render the transformed layer
    // For now, we'll just log that the layer would be rendered
    info!("Rendering transformed layer");
}