// GPU Keyframe Interpolation Compute Shader (stub)
//
// This shader will read keyframe times/values and write interpolated
// values for the current timeline time into an output buffer.
// The full binding layout will be defined in the runner; for now we
// keep a minimal no-op kernel to validate pipeline wiring.

@group(0) @binding(0)
var<storage, read> key_times: array<u32>;   // ms

@group(0) @binding(1)
var<storage, read> key_values: array<f32>;  // value per key

@group(0) @binding(2)
var<storage, read_write> out_values: array<f32>; // per property output

@group(0) @binding(3)
var<uniform> time_ms: u32;

// Notes:
// - In a real implementation we would also have an index buffer to map properties
//   to keyframe ranges, plus interpolation type per segment (linear/hold/bezier).
// - Workgroup X dimension can be the number of properties to evaluate in parallel.

@compute @workgroup_size(64)
fn interpolate_keyframes(@builtin(global_invocation_id) gid: vec3<u32>) {
    // Stub: write a deterministic placeholder value so we can validate dispatch
    let idx = gid.x;
    if (idx < arrayLength(&out_values)) {
        // For now, pass through the last key value if present, otherwise 0.
        let n = arrayLength(&key_values);
        var v: f32 = 0.0;
        if (n > 0u) {
            v = key_values[n - 1u];
        }
        out_values[idx] = v;
    }
}