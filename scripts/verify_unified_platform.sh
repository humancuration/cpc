#!/bin/bash

# Unified Platform Implementation Verification Script

echo "Verifying Unified Platform Implementation..."
echo "============================================="

# Check that all required directories exist
echo "Checking directory structure..."
REQUIRED_DIRS=(
    "apps/launcher"
    "apps/launcher/src"
    "apps/launcher/icons"
    "apps/web_portal"
    "apps/web_portal/src"
    "shared_packages/core_app_logic"
    "shared_packages/core_app_logic/src"
    "shared_packages/core_app_logic/src/domain"
    "shared_packages/core_app_logic/src/services"
    "shared_packages/core_app_logic/src/utils"
    "shared_packages/ui_toolkit"
    "shared_packages/ui_toolkit/src"
    "shared_packages/ui_toolkit/src/components"
    "shared_packages/ui_toolkit/src/themes"
    "shared_packages/ui_toolkit/src/hooks"
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
    "apps/launcher/Cargo.toml"
    "apps/launcher/tauri.conf.json"
    "apps/launcher/src/main.rs"
    "apps/launcher/README.md"
    "apps/web_portal/Cargo.toml"
    "apps/web_portal/src/main.rs"
    "apps/web_portal/README.md"
    "shared_packages/core_app_logic/Cargo.toml"
    "shared_packages/core_app_logic/src/lib.rs"
    "shared_packages/core_app_logic/src/domain/mod.rs"
    "shared_packages/core_app_logic/src/services/mod.rs"
    "shared_packages/core_app_logic/src/utils/mod.rs"
    "shared_packages/core_app_logic/README.md"
    "shared_packages/ui_toolkit/Cargo.toml"
    "shared_packages/ui_toolkit/src/lib.rs"
    "shared_packages/ui_toolkit/src/components/mod.rs"
    "shared_packages/ui_toolkit/src/components/button.rs"
    "shared_packages/ui_toolkit/src/themes/mod.rs"
    "shared_packages/ui_toolkit/src/hooks/mod.rs"
    "shared_packages/ui_toolkit/README.md"
)

for file in "${REQUIRED_FILES[@]}"; do
    if [ ! -f "$file" ]; then
        echo "ERROR: File $file does not exist"
        exit 1
    fi
done
echo "✓ All required files exist"

# Check that workspace Cargo.toml includes our packages
echo "Checking workspace configuration..."
if ! grep -q "apps/launcher" Cargo.toml; then
    echo "ERROR: apps/launcher not found in workspace members"
    exit 1
fi

if ! grep -q "apps/web_portal" Cargo.toml; then
    echo "ERROR: apps/web_portal not found in workspace members"
    exit 1
fi

if ! grep -q "shared_packages/core_app_logic" Cargo.toml; then
    echo "ERROR: shared_packages/core_app_logic not found in workspace members"
    exit 1
fi

if ! grep -q "shared_packages/ui_toolkit" Cargo.toml; then
    echo "ERROR: shared_packages/ui_toolkit not found in workspace members"
    exit 1
fi
echo "✓ Workspace configuration is correct"

# Check that documentation files are not empty
echo "Checking documentation files..."
DOC_FILES=(
    "apps/launcher/README.md"
    "apps/web_portal/README.md"
    "shared_packages/core_app_logic/README.md"
    "shared_packages/ui_toolkit/README.md"
)

for doc in "${DOC_FILES[@]}"; do
    if [ ! -s "$doc" ]; then
        echo "ERROR: Documentation file $doc is empty"
        exit 1
    fi
done
echo "✓ All documentation files have content"

echo ""
echo "============================================="
echo "✓ Unified Platform Implementation Verification Complete - All checks passed!"
echo "============================================="