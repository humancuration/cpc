// Mipmap generation compute shader for the Art application
// Generates lower resolution versions of textures

// Texture bindings
@group(0) @binding(0) var src_texture: texture_2d<f32>;
@group(0) @binding(1) var dst_texture: texture_storage_2d<rgba8unorm, write>;

// Uniforms
struct Uniforms {
    src_level: u32,
    dst_level: u32,
}

@group(0) @binding(2) var<uniform> uniforms: Uniforms;

// Work group size
@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // Get destination texture dimensions
    let dst_dims = textureDimensions(dst_texture);
    
    // Check bounds
    if (global_id.x >= dst_dims.x || global_id.y >= dst_dims.y) {
        return;
    }
    
    // Calculate corresponding position in source texture
    let scale = f32(1 << uniforms.dst_level); // 2^dst_level
    let src_pos = vec2<u32>(global_id.xy * u32(scale));
    
    // Get source texture dimensions
    let src_dims = textureDimensions(src_texture, uniforms.src_level);
    
    // Check bounds in source texture
    if (src_pos.x >= src_dims.x || src_pos.y >= src_dims.y) {
        // Write transparent pixel
        textureStore(dst_texture, global_id.xy, vec4<u32>(0, 0, 0, 0));
        return;
    }
    
    // Simple downsampling - read 4 pixels and average them
    let p0 = textureLoad(src_texture, src_pos, uniforms.src_level);
    let p1 = textureLoad(src_texture, src_pos + vec2<u32>(1, 0), uniforms.src_level);
    let p2 = textureLoad(src_texture, src_pos + vec2<u32>(0, 1), uniforms.src_level);
    let p3 = textureLoad(src_texture, src_pos + vec2<u32>(1, 1), uniforms.src_level);
    
    // Average the pixels
    let avg = (p0 + p1 + p2 + p3) / 4.0;
    
    // Convert to u32 and write result
    let result = vec4<u32>(avg * 255.0);
    textureStore(dst_texture, global_id.xy, result);
}