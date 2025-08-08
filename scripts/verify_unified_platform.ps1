# Unified Platform Implementation Verification Script

Write-Host "Verifying Unified Platform Implementation..."
Write-Host "============================================="

# Check that all required directories exist
Write-Host "Checking directory structure..."
$RequiredDirs = @(
    "apps/launcher",
    "apps/launcher/src",
    "apps/launcher/icons",
    "apps/web_portal",
    "apps/web_portal/src",
    "shared_packages/core_app_logic",
    "shared_packages/core_app_logic/src",
    "shared_packages/core_app_logic/src/domain",
    "shared_packages/core_app_logic/src/services",
    "shared_packages/core_app_logic/src/utils",
    "shared_packages/ui_toolkit",
    "shared_packages/ui_toolkit/src",
    "shared_packages/ui_toolkit/src/components",
    "shared_packages/ui_toolkit/src/themes",
    "shared_packages/ui_toolkit/src/hooks"
)

foreach ($dir in $RequiredDirs) {
    if (-not (Test-Path $dir)) {
        Write-Error "ERROR: Directory $dir does not exist"
        exit 1
    }
}
Write-Host "✓ All required directories exist"

# Check that all required files exist
Write-Host "Checking required files..."
$RequiredFiles = @(
    "apps/launcher/Cargo.toml",
    "apps/launcher/tauri.conf.json",
    "apps/launcher/src/main.rs",
    "apps/launcher/README.md",
    "apps/web_portal/Cargo.toml",
    "apps/web_portal/src/main.rs",
    "apps/web_portal/README.md",
    "shared_packages/core_app_logic/Cargo.toml",
    "shared_packages/core_app_logic/src/lib.rs",
    "shared_packages/core_app_logic/src/domain/mod.rs",
    "shared_packages/core_app_logic/src/services/mod.rs",
    "shared_packages/core_app_logic/src/utils/mod.rs",
    "shared_packages/core_app_logic/README.md",
    "shared_packages/ui_toolkit/Cargo.toml",
    "shared_packages/ui_toolkit/src/lib.rs",
    "shared_packages/ui_toolkit/src/components/mod.rs",
    "shared_packages/ui_toolkit/src/components/button.rs",
    "shared_packages/ui_toolkit/src/themes/mod.rs",
    "shared_packages/ui_toolkit/src/hooks/mod.rs",
    "shared_packages/ui_toolkit/README.md"
)

foreach ($file in $RequiredFiles) {
    if (-not (Test-Path $file)) {
        Write-Error "ERROR: File $file does not exist"
        exit 1
    }
}
Write-Host "✓ All required files exist"

# Check that workspace Cargo.toml includes our packages
Write-Host "Checking workspace configuration..."
$CargoContent = Get-Content Cargo.toml -Raw

if (-not ($CargoContent -match "apps/launcher")) {
    Write-Error "ERROR: apps/launcher not found in workspace members"
    exit 1
}

if (-not ($CargoContent -match "apps/web_portal")) {
    Write-Error "ERROR: apps/web_portal not found in workspace members"
    exit 1
}

if (-not ($CargoContent -match "shared_packages/core_app_logic")) {
    Write-Error "ERROR: shared_packages/core_app_logic not found in workspace members"
    exit 1
}

if (-not ($CargoContent -match "shared_packages/ui_toolkit")) {
    Write-Error "ERROR: shared_packages/ui_toolkit not found in workspace members"
    exit 1
}
Write-Host "✓ Workspace configuration is correct"

# Check that documentation files are not empty
Write-Host "Checking documentation files..."
$DocFiles = @(
    "apps/launcher/README.md",
    "apps/web_portal/README.md",
    "shared_packages/core_app_logic/README.md",
    "shared_packages/ui_toolkit/README.md"
)

foreach ($doc in $DocFiles) {
    if ((Get-Item $doc).Length -eq 0) {
        Write-Error "ERROR: Documentation file $doc is empty"
        exit 1
    }
}
Write-Host "✓ All documentation files have content"

Write-Host ""
Write-Host "============================================="
Write-Host "✓ Unified Platform Implementation Verification Complete - All checks passed!"
Write-Host "============================================="