//! Selection rendering for the Art application
//!
//! This module implements GPU-accelerated rendering of selections,
//! including marching ants animation and selection mask visualization.

use bevy::prelude::*;
use bevy::render::{
    render_resource::*,
    renderer::{RenderDevice, RenderQueue},
};
use crate::core::selection::{SelectionState, SelectionArea};
use crate::rendering::ArtRenderPipeline;

/// Component for rendering selection overlays
#[derive(Component)]
pub struct SelectionOverlay {
    pub selection_state: SelectionState,
    pub texture_dest: Handle<Image>,
    pub animation_time: f32,
}

/// System to render selection overlays
pub fn render_selection_overlays(
    mut commands: Commands,
    pipeline: Res<ArtRenderPipeline>,
    pipeline_cache: Res<PipelineCache>,
    selection_overlays: Query<(Entity, &SelectionOverlay)>,
    gpu_images: Res<Assets<Image>>,
    render_device: Res<RenderDevice>,
    render_queue: Res<RenderQueue>,
    time: Res<Time>,
) {
    // Only run if we have the pipeline ready
    let pipeline = match pipeline_cache.get_compute_pipeline(pipeline.pipeline) {
        Some(pipeline) => pipeline,
        None => return,
    };
    
    let mut pass_count = 0;
    
    for (entity, overlay) in selection_overlays.iter() {
        // Get destination image
        let dest_image = match gpu_images.get(&overlay.texture_dest) {
            Some(image) => image,
            None => continue,
        };
        
        // Update animation time
        let animation_time = overlay.animation_time + time.delta_seconds();
        
        // Render each selection area
        for selection in &overlay.selection_state.selections {
            // Create texture view for destination
            let dest_view = dest_image.texture_view.clone();
            
            // Create bind group
            let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(&dest_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::TextureView(&dest_view),
                    },
                ],
                label: Some("selection_bind_group"),
                layout: &pipeline.get_bind_group_layout(0),
            });
            
            // Dispatch compute pass for selection rendering
            let mut encoder = render_device.create_command_encoder(&CommandEncoderDescriptor {
                label: Some("selection_encoder"),
            });
            
            {
                let mut pass = encoder.begin_compute_pass(&ComputePassDescriptor {
                    label: Some("selection_compute_pass"),
                });
                
                pass.set_pipeline(pipeline);
                pass.set_bind_group(0, &bind_group, &[]);
                
                // Calculate work group count based on texture size
                let work_group_size = 8; // Must match shader
                let width = (dest_image.texture_descriptor.size.width + work_group_size - 1) / work_group_size;
                let height = (dest_image.texture_descriptor.size.height + work_group_size - 1) / work_group_size;
                
                pass.dispatch_workgroups(width, height, 1);
            }
            
            render_queue.submit(vec![encoder.finish()]);
            pass_count += 1;
        }
        
        // Remove component after processing
        commands.entity(entity).remove::<SelectionOverlay>();
    }
    
    if pass_count > 0 {
        info!("Rendered {} selection overlays", pass_count);
    }
}

/// Render a marching ants animation for a selection boundary
pub fn render_marching_ants(
    device: &RenderDevice,
    queue: &RenderQueue,
    dest: &TextureView,
    selection: &SelectionArea,
    animation_time: f32,
) {
    // In a real implementation, this would use a compute shader to render the marching ants
    // For now, we'll just log that the animation would be rendered
    info!("Rendering marching ants for selection at time {}", animation_time);
}

/// Render a selection mask overlay
pub fn render_selection_mask(
    device: &RenderDevice,
    queue: &RenderQueue,
    dest: &TextureView,
    selection: &SelectionArea,
) {
    // In a real implementation, this would use a compute shader to render the selection mask
    // For now, we'll just log that the mask would be rendered
    info!("Rendering selection mask");
}