# Script to verify that all financial impact packages are properly set up

Write-Host "Verifying Financial Impact Tracker packages..."

# Check if shared package exists
if (-not (Test-Path "shared_packages/financial_impact_tracker")) {
    Write-Error "ERROR: shared_packages/financial_impact_tracker directory not found"
    exit 1
}

# Check if Cargo.toml exists
if (-not (Test-Path "shared_packages/financial_impact_tracker/Cargo.toml")) {
    Write-Error "ERROR: shared_packages/financial_impact_tracker/Cargo.toml not found"
    exit 1
}

# Check if src/lib.rs exists
if (-not (Test-Path "shared_packages/financial_impact_tracker/src/lib.rs")) {
    Write-Error "ERROR: shared_packages/financial_impact_tracker/src/lib.rs not found"
    exit 1
}

# Check if all modules exist
$modules = @("tracker", "analytics", "feedback", "improvement", "integration")
foreach ($module in $modules) {
    if (-not (Test-Path "shared_packages/financial_impact_tracker/src/$module.rs")) {
        Write-Error "ERROR: shared_packages/financial_impact_tracker/src/$module.rs not found"
        exit 1
    }
}

# Check if tests directory exists
if (-not (Test-Path "shared_packages/financial_impact_tracker/tests")) {
    Write-Warning "WARNING: shared_packages/financial_impact_tracker/tests directory not found"
}

# Check if integration tests exist
if (-not (Test-Path "shared_packages/financial_impact_tracker/tests/integration_tests.rs")) {
    Write-Warning "WARNING: shared_packages/financial_impact_tracker/tests/integration_tests.rs not found"
}

# Check if examples directory exists
if (-not (Test-Path "shared_packages/financial_impact_tracker/examples")) {
    Write-Warning "WARNING: shared_packages/financial_impact_tracker/examples directory not found"
}

# Check if demo example exists
if (-not (Test-Path "shared_packages/financial_impact_tracker/examples/financial_impact_demo.rs")) {
    Write-Warning "WARNING: shared_packages/financial_impact_tracker/examples/financial_impact_demo.rs not found"
}

# Check if README exists
if (-not (Test-Path "shared_packages/financial_impact_tracker/README.md")) {
    Write-Warning "WARNING: shared_packages/financial_impact_tracker/README.md not found"
}

Write-Host "Financial Impact Tracker shared package verification: PASSED"

# Check if finance admin dashboard app exists
if (-not (Test-Path "apps/finance_admin_dashboard")) {
    Write-Error "ERROR: apps/finance_admin_dashboard directory not found"
    exit 1
}

# Check if Cargo.toml exists
if (-not (Test-Path "apps/finance_admin_dashboard/Cargo.toml")) {
    Write-Error "ERROR: apps/finance_admin_dashboard/Cargo.toml not found"
    exit 1
}

# Check if src/main.rs exists
if (-not (Test-Path "apps/finance_admin_dashboard/src/main.rs")) {
    Write-Error "ERROR: apps/finance_admin_dashboard/src/main.rs not found"
    exit 1
}

# Check if README exists
if (-not (Test-Path "apps/finance_admin_dashboard/README.md")) {
    Write-Warning "WARNING: apps/finance_admin_dashboard/README.md not found"
}

Write-Host "Finance Admin Dashboard app verification: PASSED"

# Check if member feedback app exists
if (-not (Test-Path "apps/member_feedback")) {
    Write-Error "ERROR: apps/member_feedback directory not found"
    exit 1
}

# Check if Cargo.toml exists
if (-not (Test-Path "apps/member_feedback/Cargo.toml")) {
    Write-Error "ERROR: apps/member_feedback/Cargo.toml not found"
    exit 1
}

# Check if src/main.rs exists
if (-not (Test-Path "apps/member_feedback/src/main.rs")) {
    Write-Error "ERROR: apps/member_feedback/src/main.rs not found"
    exit 1
}

# Check if README exists
if (-not (Test-Path "apps/member_feedback/README.md")) {
    Write-Warning "WARNING: apps/member_feedback/README.md not found"
}

Write-Host "Member Feedback app verification: PASSED"

Write-Host "All financial impact packages verified successfully!"