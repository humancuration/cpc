// Blending compute shader for the Art application
// Implements various blend modes for layer composition

// Texture bindings
@group(0) @binding(0) var src_texture: texture_storage_2d<rgba8unorm, read_write>;
@group(0) @binding(1) var dst_texture: texture_storage_2d<rgba8unorm, read_write>;

// Work group size
@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Get texture dimensions
    let dims = textureDimensions(src_texture);
    
    // Check bounds
    if (global_id.x >= dims.x || global_id.y >= dims.y) {
        return;
    }
    
    // Read source and destination pixels
    let src_pixel = textureLoad(src_texture, global_id.xy);
    let dst_pixel = textureLoad(dst_texture, global_id.xy);
    
    // Perform normal blending (source over destination)
    // Convert to float for calculations
    let src_f = vec4<f32>(src_pixel) / 255.0;
    let dst_f = vec4<f32>(dst_pixel) / 255.0;
    
    // Simple alpha blending: result = src + dst * (1 - src_alpha)
    let result_a = src_f.a + dst_f.a * (1.0 - src_f.a);
    let result_rgb = (src_f.rgb * src_f.a + dst_f.rgb * dst_f.a * (1.0 - src_f.a)) / result_a;
    let result = vec4<f32>(result_rgb, result_a);
    
    // Convert back to u32 and write result
    let result_u32 = vec4<u32>(result * 255.0);
    textureStore(dst_texture, global_id.xy, result_u32);
}