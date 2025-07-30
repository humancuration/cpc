#!/bin/bash

# Script to verify all documentation files were created

echo "Verifying Documentation Files..."
echo "================================"

DOCS_DIR="docs/architectural_improvements"

# List of expected documentation files
DOCS_FILES=(
    "composite_adapter_sync_queue.md"
    "IMPLEMENTATION_SUMMARY.md"
    "MIGRATION_GUIDE.md"
    "FINAL_IMPLEMENTATION_REPORT.md"
    "PROJECT_COMPLETION_SUMMARY.md"
    "FILES_CHANGED.md"
    "TASK_COMPLETION_REPORT.md"
    "VERIFICATION_CHECKLIST.md"
)

MISSING_DOCS=0
for doc in "${DOCS_FILES[@]}"; do
    if [ ! -f "$DOCS_DIR/$doc" ]; then
        echo "❌ Missing documentation file: $DOCS_DIR/$doc"
        MISSING_DOCS=1
    else
        echo "✅ Found: $DOCS_DIR/$doc"
    fi
done

if [ $MISSING_DOCS -eq 1 ]; then
    echo "Error: Some documentation files are missing"
    exit 1
fi

echo ""
echo "Verifying key content in documentation files..."

# Check that key files have content
KEY_FILES=(
    "IMPLEMENTATION_SUMMARY.md"
    "MIGRATION_GUIDE.md"
    "FINAL_IMPLEMENTATION_REPORT.md"
)

for key_file in "${KEY_FILES[@]}"; do
    if [ -f "$DOCS_DIR/$key_file" ]; then
        lines=$(wc -l < "$DOCS_DIR/$key_file")
        if [ "$lines" -gt 50 ]; then
            echo "✅ $key_file has sufficient content ($lines lines)"
        else
            echo "⚠️  $key_file may be missing content ($lines lines)"
        fi
    fi
done

echo ""
echo "Documentation verification complete!"
echo "All required documentation files are present."