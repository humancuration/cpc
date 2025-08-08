# Script to verify that the new volunteer impact tracking packages compile correctly

Write-Host "Verifying volunteer impact tracking packages..."

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "Error: This script must be run from the project root directory"
    exit 1
}

# Verify that the packages compile
Write-Host "Checking volunteer_impact_tracker package..."
cargo check -p volunteer_impact_tracker

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: volunteer_impact_tracker failed to compile"
    exit 1
}

Write-Host "Checking learning_impact_tracker package..."
cargo check -p learning_impact_tracker

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: learning_impact_tracker failed to compile"
    exit 1
}

Write-Host "Checking impact_viz package..."
cargo check -p impact_viz

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: impact_viz failed to compile"
    exit 1
}

Write-Host "Checking volunteer_coordination_admin package..."
cargo check -p volunteer-coordination-admin

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: volunteer-coordination-admin failed to compile"
    exit 1
}

Write-Host "All volunteer impact tracking packages compiled successfully!"
Write-Host "Running unit tests..."

# Run unit tests for the new packages
Write-Host "Running tests for volunteer_impact_tracker..."
cargo test -p volunteer_impact_tracker

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: volunteer_impact_tracker tests failed"
    exit 1
}

Write-Host "Running tests for learning_impact_tracker..."
cargo test -p learning_impact_tracker

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: learning_impact_tracker tests failed"
    exit 1
}

Write-Host "Running tests for impact_viz..."
cargo test -p impact_viz

if ($LASTEXITCODE -ne 0) {
    Write-Host "Error: impact_viz tests failed"
    exit 1
}

Write-Host "All tests passed successfully!"
Write-Host "Verification complete."