# PowerShell script to verify all Shtairir demo applications are working correctly
# This script runs a quick check of all examples to ensure they compile and execute

Write-Host "🔍 Checking Shtairir Demo Applications"
Write-Host "======================================"

# Check if we're in the right directory
if (-not (Test-Path "Cargo.toml")) {
    Write-Host "❌ Error: This script must be run from the shtairir_demos directory"
    exit 1
}

# Array of example names
$examples = @("data_processing", "user_profiles", "ml_features")

# Track overall success
$overall_success = $true

# Check each example
foreach ($example in $examples) {
    Write-Host ""
    Write-Host "🧪 Testing $example..."
    Write-Host "------------------------"
    
    # Check if the example compiles
    Write-Host "📦 Checking compilation..."
    $check_result = cargo check -p "shtairir_demos_$example" --quiet
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Compilation successful"
    } else {
        Write-Host "❌ Compilation failed"
        $overall_success = $false
        continue
    }
    
    # Run the example in test mode (quick execution)
    Write-Host "🏃 Running quick test..."
    $run_result = cargo run -p "shtairir_demos_$example" --quiet -- --test-mode
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Execution successful"
    } else {
        Write-Host "❌ Execution failed"
        $overall_success = $false
    }
}

# Run integration tests
Write-Host ""
Write-Host "🧪 Running integration tests..."
Write-Host "-------------------------------"
$test_result = cargo test --quiet
if ($LASTEXITCODE -eq 0) {
    Write-Host "✅ All integration tests passed"
} else {
    Write-Host "❌ Some integration tests failed"
    $overall_success = $false
}

# Final summary
Write-Host ""
Write-Host "📋 Final Summary"
Write-Host "================"
if ($overall_success) {
    Write-Host "🎉 All Shtairir demo applications are working correctly!"
    exit 0
} else {
    Write-Host "⚠️  Some issues were detected. Please check the output above."
    exit 1
}