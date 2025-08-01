#!/bin/bash

# CPay Implementation Verification Script

echo "Verifying CPay Implementation..."
echo "================================="

# Check that all required directories exist
echo "Checking directory structure..."
REQUIRED_DIRS=(
    "apps/cpay"
    "apps/cpay/src"
    "apps/cpay/ui"
    "shared_packages/cpay_core"
    "shared_packages/cpay_core/src"
    "shared_packages/cpay_core/src/repositories"
    "shared_packages/cpay_core/proto"
    "shared_packages/cpay_core/migrations"
    "shared_packages/cpay_core/tests"
    "docs"
)

for dir in "${REQUIRED_DIRS[@]}"; do
    if [ ! -d "$dir" ]; then
        echo "ERROR: Directory $dir does not exist"
        exit 1
    fi
done
echo "✓ All required directories exist"

# Check that all required files exist
echo "Checking required files..."
REQUIRED_FILES=(
    "apps/cpay/Cargo.toml"
    "apps/cpay/tauri.conf.json"
    "apps/cpay/src/main.rs"
    "apps/cpay/ui/index.html"
    "apps/cpay/README.md"
    "apps/cpay/IMPLEMENTATION_SUMMARY.md"
    "shared_packages/cpay_core/Cargo.toml"
    "shared_packages/cpay_core/build.rs"
    "shared_packages/cpay_core/src/lib.rs"
    "shared_packages/cpay_core/src/models.rs"
    "shared_packages/cpay_core/src/transaction_engine.rs"
    "shared_packages/cpay_core/src/repositories.rs"
    "shared_packages/cpay_core/src/repositories/mock.rs"
    "shared_packages/cpay_core/proto/cpay.proto"
    "shared_packages/cpay_core/migrations/20250801000001_create_traditional_currency_transactions_table.sql"
    "shared_packages/cpay_core/tests/integration_test.rs"
    "shared_packages/cpay_core/tests/proto_compilation_test.rs"
    "shared_packages/cpay_core/README.md"
    "shared_packages/cpay_core/IMPLEMENTATION_SUMMARY.md"
    "docs/cpay_architecture.md"
    "docs/cpay_complete_implementation.md"
    "docs/cpay_file_inventory.md"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "ERROR: File $file does not exist"
        exit 1
    fi
done
echo "✓ All required files exist"

# Check that workspace Cargo.toml includes cpay packages
echo "Checking workspace configuration..."
if ! grep -q "apps/cpay" Cargo.toml; then
    echo "ERROR: apps/cpay not found in workspace members"
    exit 1
fi

if ! grep -q "shared_packages/cpay_core" Cargo.toml; then
    echo "ERROR: shared_packages/cpay_core not found in workspace members"
    exit 1
fi
echo "✓ Workspace configuration is correct"

# Check that documentation files are not empty
echo "Checking documentation files..."
DOC_FILES=(
    "apps/cpay/README.md"
    "apps/cpay/IMPLEMENTATION_SUMMARY.md"
    "shared_packages/cpay_core/README.md"
    "shared_packages/cpay_core/IMPLEMENTATION_SUMMARY.md"
    "docs/cpay_architecture.md"
    "docs/cpay_complete_implementation.md"
    "docs/cpay_file_inventory.md"
)

for doc in "${DOC_FILES[@]}"; do
    if [ ! -s "$doc" ]; then
        echo "ERROR: Documentation file $doc is empty"
        exit 1
    fi
done
echo "✓ All documentation files have content"

echo ""
echo "================================="
echo "✓ CPay Implementation Verification Complete - All checks passed!"
echo "================================="