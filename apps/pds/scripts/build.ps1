# Build script for Windows PowerShell
param(
    [string]$Target = "all",
    [switch]$Release = $false
)

Write-Host "Building CPC Desktop Application..." -ForegroundColor Green

# Install frontend dependencies
Write-Host "Installing frontend dependencies..." -ForegroundColor Yellow
Set-Location frontend
npm install
Set-Location ..

# Build frontend
Write-Host "Building frontend..." -ForegroundColor Yellow
Set-Location frontend
npm run build
Set-Location ..

# Build Tauri application
$BuildArgs = @("tauri", "build")

if ($Target -ne "all") {
    $BuildArgs += "--target", $Target
}

if ($Release) {
    Write-Host "Building in release mode..." -ForegroundColor Green
} else {
    Write-Host "Building in debug mode..." -ForegroundColor Green
    $BuildArgs += "--debug"
}

Write-Host "Running: cargo $($BuildArgs -join ' ')" -ForegroundColor Cyan
& cargo @BuildArgs

if ($LASTEXITCODE -eq 0) {
    Write-Host "Build completed successfully!" -ForegroundColor Green
} else {
    Write-Error "Build failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}