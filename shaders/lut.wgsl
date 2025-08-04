@group(0) @binding(0) var base_tex: texture_2d<f32>;
@group(0) @binding(1) var base_samp: sampler;
@group(0) @binding(2) var lut_tex: texture_3d<f32>;

struct VsOut {
    @builtin(position) pos: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) pos: vec2<f32>, @location(1) uv: vec2<f32>) -> VsOut {
    var out: VsOut;
    out.pos = vec4(pos, 0.0, 1.0);
    out.uv = uv;
    return out;
}

@fragment
fn fs_main(inf: VsOut) -> @location(0) vec4<f32> {
    // sample base color
    let color = textureSample(base_tex, base_samp, inf.uv);
    // sample 3D LUT using rgb as coordinates (expects normalized 0..1 input)
    let graded = textureSampleLevel(lut_tex, color.rgb, 0.0);
    return vec4(graded.rgb, color.a);
}