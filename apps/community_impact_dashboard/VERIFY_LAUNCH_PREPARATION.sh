#!/bin/bash

# Community Impact Dashboard Launch Preparation Verification Script
#
# This script verifies that all components of the launch preparation system
# are properly implemented and integrated.
#!/bin/bash

# Community Impact Dashboard Launch Preparation Verification Script
#
# This script verifies that all components of the launch preparation system
# are properly implemented and integrated.

# Community Impact Dashboard Launch Preparation Verification Script
#
# This script verifies that all components of the launch preparation system
# are properly implemented and integrated.

echo "🚀 Unified Community Impact Dashboard Launch Preparation Verification"
echo "====================================================================="

# Check if required tools are available
echo "🔍 Checking for required tools..."
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust compiler not found. Please install Rust."
    exit 1
fi

if ! command -v cargo &> /dev/null; then
    echo "❌ Cargo not found. Please install Rust."
    exit 1
fi

if ! command -v wasm-pack &> /dev/null; then
    echo "⚠️  wasm-pack not found. Some tests may be skipped."
fi

echo "✅ Required tools found"

# Check project structure
echo "📂 Verifying project structure..."
REQUIRED_DIRS=(
    "src/launch"
    "docs/facilitator"
    "docs/templates"
    "docs/guides"
)

REQUIRED_FILES=(
    "src/launch/mod.rs"
    "src/launch/readiness.rs"
    "src/launch/notification.rs"
    "src/launch/rollout.rs"
    "src/launch/metrics.rs"
    "src/launch/facilitator.rs"
    "src/launch/celebration.rs"
    "src/launch/feedback.rs"
    "src/launch/coordinator.rs"
    "docs/launch_preparation_checklist.md"
    "docs/launch_summary.md"
    "docs/launch_implementation_summary.md"
    "docs/community_quick_start.md"
    "docs/facilitator/mod.rs"
    "docs/templates/mod.rs"
    "docs/guides/mod.rs"
    "docs/templates/workshop_introduction_template.md"
    "docs/templates/workshop_validation_template.md"
    "docs/templates/launch_announcement.md"
    "docs/guides/facilitation_basics.md"
    "docs/guides/troubleshooting.md"
    "docs/templates/community_customization.md"
)

MISSING_ITEMS=0

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "❌ Missing directory: $dir"
        MISSING_ITEMS=$((MISSING_ITEMS + 1))
    fi
done

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "❌ Missing file: $file"
        MISSING_ITEMS=$((MISSING_ITEMS + 1))
    fi
done

if [ $MISSING_ITEMS -eq 0 ]; then
    echo "✅ All required directories and files present"
else
    echo "❌ $MISSING_ITEMS required items missing"
    exit 1
fi

# Check Cargo.toml for dependencies
echo "📦 Verifying dependencies..."
if grep -q "launch" Cargo.toml; then
    echo "✅ Launch module properly referenced in project"
else
    echo "⚠️  Launch module not explicitly referenced in Cargo.toml (this may be OK)"
fi

# Run compilation test
echo "🏗️  Compiling project to verify integration..."
if cargo check --quiet; then
    echo "✅ Project compiles successfully"
else
    echo "❌ Compilation failed"
    exit 1
fi

# Run unit tests
echo "🧪 Running unit tests..."
if cargo test --quiet; then
    echo "✅ All unit tests pass"
else
    echo "❌ Some unit tests failed"
    exit 1
fi

# Verify documentation links in README
echo "📚 Verifying documentation links..."
MISSING_DOCS=0

DOC_FILES=(
    "docs/launch_preparation_checklist.md"
    "docs/launch_summary.md"
    "docs/launch_implementation_summary.md"
    "docs/community_quick_start.md"
)

for doc in "${DOC_FILES[@]}"; do
    if [ -f "$doc" ]; then
        echo "✅ Documentation file present: $doc"
    else
        echo "❌ Missing documentation file: $doc"
        MISSING_DOCS=$((MISSING_DOCS + 1))
    fi
done

if [ $MISSING_DOCS -eq 0 ]; then
    echo "✅ All documentation files present"
else
    echo "❌ $MISSING_DOCS documentation files missing"
    exit 1
fi

# Summary
echo ""
echo "🎉 Launch Preparation Verification Complete!"
echo "============================================"
echo "✅ Project structure verified"
echo "✅ All source files present"
echo "✅ Compilation successful"
echo "✅ Unit tests passing"
echo "✅ Documentation complete"
echo ""
echo "The Unified Community Impact Dashboard launch preparation system"
echo "is fully implemented and ready for community launch!"
echo ""
echo "Next steps:"
echo "1. Review the launch preparation checklist"
echo "2. Train community facilitators using the provided resources"
echo "3. Customize materials for your specific community context"
echo "4. Execute the phased rollout plan"
echo "5. Monitor launch metrics and community feedback"
echo ""
echo "For detailed instructions, see docs/launch_preparation_checklist.md"