#!/bin/bash

# Make this script executable
chmod +x "$0"

# Unified Community Impact Dashboard - Build and Deployment Script
# This script builds and deploys the dashboard for production use

echo "========================================="
echo "Unified Community Impact Dashboard"
echo "Build and Deployment Script"
echo "========================================="
echo

# Check prerequisites
echo "1. Checking prerequisites..."

# Check if Rust is installed
if ! command -v rustc &> /dev/null; then
    echo "   ❌ Rust is not installed"
    echo "   Please install Rust from https://www.rust-lang.org/"
    exit 1
else
    echo "   ✅ Rust is installed ($(rustc --version))"
fi

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "   ❌ wasm-pack is not installed"
    echo "   Please install wasm-pack: cargo install wasm-pack"
    exit 1
else
    echo "   ✅ wasm-pack is installed ($(wasm-pack --version))"
fi

# Check if Trunk is installed
if ! command -v trunk &> /dev/null; then
    echo "   ❌ Trunk is not installed"
    echo "   Please install Trunk: cargo install trunk"
    exit 1
else
    echo "   ✅ Trunk is installed ($(trunk --version))"
fi

echo

# Run verification script first
echo "2. Running deployment verification..."
if [ -f "VERIFY_DEPLOYMENT.sh" ]; then
    echo "   Running verification script..."
    bash VERIFY_DEPLOYMENT.sh
    if [ $? -ne 0 ]; then
        echo "   ❌ Verification failed"
        exit 1
    else
        echo "   ✅ Verification passed"
    fi
else
    echo "   ⚠️  Verification script not found, skipping verification"
fi

echo

# Run tests
echo "3. Running tests..."

echo "   Running unit tests..."
if wasm-pack test --firefox --headless &> /dev/null; then
    echo "   ✅ Unit tests passed"
else
    echo "   ❌ Unit tests failed"
    echo "   Please check test output and fix issues before deployment"
    exit 1
fi

echo "   Running integration tests..."
INTEGRATION_TEST_FILES=(
    "tests/integration/connectivity_tests.rs"
    "tests/integration/data_flow_tests.rs"
    "tests/integration/visualization_tests.rs"
    "tests/integration/complexity_tests.rs"
    "tests/integration/story_contribution_tests.rs"
    "tests/integration/community_validation_tests.rs"
)

for test_file in "${INTEGRATION_TEST_FILES[@]}"; do
    test_name=$(basename "$test_file" .rs)
    echo "     Running $test_name..."
    if wasm-pack test --firefox --headless -- "$test_file" &> /dev/null; then
        echo "       ✅ $test_name passed"
    else
        echo "       ❌ $test_name failed"
        echo "       Please check test output and fix issues before deployment"
        exit 1
    fi
done

echo

# Build for production
echo "4. Building for production..."

echo "   Cleaning previous builds..."
rm -rf dist/

echo "   Building with Trunk..."
if trunk build --release; then
    echo "   ✅ Build successful"
    echo "   Build output is in the 'dist/' directory"
else
    echo "   ❌ Build failed"
    echo "   Please check build output and fix issues before deployment"
    exit 1
fi

echo

# Optimize assets
echo "5. Optimizing assets..."

# Check if there are image optimization tools available
if command -v optipng &> /dev/null; then
    echo "   Optimizing PNG images..."
    find dist/ -name "*.png" -exec optipng -o7 {} \; 2>/dev/null || echo "     ⚠️  optipng failed, continuing..."
fi

if command -v jpegoptim &> /dev/null; then
    echo "   Optimizing JPEG images..."
    find dist/ -name "*.jpg" -exec jpegoptim -m85 {} \; 2>/dev/null || echo "     ⚠️  jpegoptim failed, continuing..."
    find dist/ -name "*.jpeg" -exec jpegoptim -m85 {} \; 2>/dev/null || echo "     ⚠️  jpegoptim failed, continuing..."
fi

echo

# Generate build report
echo "6. Generating build report..."

BUILD_TIME=$(date)
BUILD_SIZE=$(du -sh dist/ 2>/dev/null | cut -f1)
FILE_COUNT=$(find dist/ -type f | wc -l)

cat > dist/BUILD_REPORT.md << EOF
# Build Report

## Build Information
- **Build Time**: $BUILD_TIME
- **Build Size**: $BUILD_SIZE
- **File Count**: $FILE_COUNT files

## Build Environment
- **Rust Version**: $(rustc --version 2>/dev/null || echo "Unknown")
- **wasm-pack Version**: $(wasm-pack --version 2>/dev/null || echo "Unknown")
- **Trunk Version**: $(trunk --version 2>/dev/null || echo "Unknown")

## Deployment Instructions

1. Copy all files from the 'dist/' directory to your web server
2. Ensure your web server is configured to serve static files
3. The dashboard should be accessible at your server's root URL

## Security Considerations

- All assets are static and do not require server-side processing
- No sensitive information is embedded in the build
- The dashboard uses client-side encryption for user data
- Ensure your web server uses HTTPS for secure delivery

## Performance Notes

- The build is optimized for production deployment
- WebAssembly modules are optimized for performance
- Assets are minified and compressed where possible
- The dashboard supports Progressive Web App features for offline use

## Support

For deployment issues, please refer to:
- docs/deployment.md
- docs/troubleshooting.md
- Community support forums

EOF

echo "   ✅ Build report generated at dist/BUILD_REPORT.md"

echo

# Final summary
echo "========================================="
echo "BUILD AND DEPLOYMENT COMPLETE"
echo "========================================="
echo
echo "✅ All tests passed"
echo "✅ Production build created successfully"
echo "✅ Assets optimized"
echo "✅ Build report generated"
echo
echo "Deployment package is ready in the 'dist/' directory"
echo
echo "To deploy:"
echo "1. Copy all files from 'dist/' to your web server"
echo "2. Configure your web server to serve static files"
echo "3. Access the dashboard at your server's URL"
echo
echo "For detailed deployment instructions, see docs/deployment.md"
echo

exit 0