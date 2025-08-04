# One-command test runner for Windows with DX12 backend
Param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [String[]]$CargoArgs
)

$ErrorActionPreference = "Stop"

# Set DX12 backend for wgpu
$env:WGPU_BACKEND = "dx12"

Write-Host "Running cargo tests with WGPU_BACKEND=$($env:WGPU_BACKEND)`n"

# Run tests for the video editor crate; pass through any extra args
if ($CargoArgs -and $CargoArgs.Count -gt 0) {
    cargo test -p video_editor --% $CargoArgs
} else {
    cargo test -p video_editor
}