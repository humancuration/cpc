# Testing Environment

## Quickstart

### Windows PowerShell
```powershell
$env:WGPU_BACKEND="dx12"
cargo test -p video_editor -- --test-threads=1
```

### Windows Command Prompt
```cmd
set WGPU_BACKEND=dx12
cargo test -p video_editor -- --test-threads=1
```

## Supported Backends
- `dx12`: Primary backend for Windows CI and development
- `vulkan`: Supported for cross-platform testing
- `metal`: Supported for macOS/iOS (not used in CI)
- `gl`: OpenGL backend for compatibility

Note: Windows CI standardizes on `dx12` for consistent results

## Running Tests
```bash
WGPU_BACKEND=dx12 cargo test -p video_editor -- --test-threads=1
```

## Test Categories
- **GPU-accelerated tests**: Require WGPU_BACKEND set and compatible hardware
- **CPU-only tests**: Run without GPU dependencies
- **Behavior without WGPU_BACKEND**: GPU tests will be skipped with a clear message

## Test Script
Run `scripts/test_win.ps1` for one-command testing with DX12 backend.

## Stress Testing
Large timeline stress tests are located in `tests/stress_test.rs` and can be run with:
```bash
WGPU_BACKEND=dx12 cargo test -p video_editor -- stress
```

## Fuzz Testing
Fuzz tests are located in `tests/fuzz_test.rs` and can be run with:
```bash
WGPU_BACKEND=dx12 cargo test -p video_editor -- fuzz
```

We're actively working to:
1. Add automatic test categorization (#[gpu_test] attribute)
2. Implement conditional skipping of GPU tests when backend unavailable
3. Monitor flakiness metrics for potential CI retry mechanisms

## Texture Recycling Metrics
- Tests validate ≥50% allocation reduction
- Actual performance depends on workload patterns