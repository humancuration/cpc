#!/bin/bash

# Script to verify that all financial impact packages are properly set up

echo "Verifying Financial Impact Tracker packages..."

# Check if shared package exists
if [ ! -d "shared_packages/financial_impact_tracker" ]; then
    echo "ERROR: shared_packages/financial_impact_tracker directory not found"
    exit 1
fi

# Check if Cargo.toml exists
if [ ! -f "shared_packages/financial_impact_tracker/Cargo.toml" ]; then
    echo "ERROR: shared_packages/financial_impact_tracker/Cargo.toml not found"
    exit 1
fi

# Check if src/lib.rs exists
if [ ! -f "shared_packages/financial_impact_tracker/src/lib.rs" ]; then
    echo "ERROR: shared_packages/financial_impact_tracker/src/lib.rs not found"
    exit 1
fi

# Check if all modules exist
modules=("tracker" "analytics" "feedback" "improvement" "integration")
for module in "${modules[@]}"; do
    if [ ! -f "shared_packages/financial_impact_tracker/src/${module}.rs" ]; then
        echo "ERROR: shared_packages/financial_impact_tracker/src/${module}.rs not found"
        exit 1
    fi
done

# Check if tests directory exists
if [ ! -d "shared_packages/financial_impact_tracker/tests" ]; then
    echo "WARNING: shared_packages/financial_impact_tracker/tests directory not found"
fi

# Check if integration tests exist
if [ ! -f "shared_packages/financial_impact_tracker/tests/integration_tests.rs" ]; then
    echo "WARNING: shared_packages/financial_impact_tracker/tests/integration_tests.rs not found"
fi

# Check if examples directory exists
if [ ! -d "shared_packages/financial_impact_tracker/examples" ]; then
    echo "WARNING: shared_packages/financial_impact_tracker/examples directory not found"
fi

# Check if demo example exists
if [ ! -f "shared_packages/financial_impact_tracker/examples/financial_impact_demo.rs" ]; then
    echo "WARNING: shared_packages/financial_impact_tracker/examples/financial_impact_demo.rs not found"
fi

# Check if README exists
if [ ! -f "shared_packages/financial_impact_tracker/README.md" ]; then
    echo "WARNING: shared_packages/financial_impact_tracker/README.md not found"
fi

echo "Financial Impact Tracker shared package verification: PASSED"

# Check if finance admin dashboard app exists
if [ ! -d "apps/finance_admin_dashboard" ]; then
    echo "ERROR: apps/finance_admin_dashboard directory not found"
    exit 1
fi

# Check if Cargo.toml exists
if [ ! -f "apps/finance_admin_dashboard/Cargo.toml" ]; then
    echo "ERROR: apps/finance_admin_dashboard/Cargo.toml not found"
    exit 1
fi

# Check if src/main.rs exists
if [ ! -f "apps/finance_admin_dashboard/src/main.rs" ]; then
    echo "ERROR: apps/finance_admin_dashboard/src/main.rs not found"
    exit 1
fi

# Check if README exists
if [ ! -f "apps/finance_admin_dashboard/README.md" ]; then
    echo "WARNING: apps/finance_admin_dashboard/README.md not found"
fi

echo "Finance Admin Dashboard app verification: PASSED"

# Check if member feedback app exists
if [ ! -d "apps/member_feedback" ]; then
    echo "ERROR: apps/member_feedback directory not found"
    exit 1
fi

# Check if Cargo.toml exists
if [ ! -f "apps/member_feedback/Cargo.toml" ]; then
    echo "ERROR: apps/member_feedback/Cargo.toml not found"
    exit 1
fi

# Check if src/main.rs exists
if [ ! -f "apps/member_feedback/src/main.rs" ]; then
    echo "ERROR: apps/member_feedback/src/main.rs not found"
    exit 1
fi

# Check if README exists
if [ ! -f "apps/member_feedback/README.md" ]; then
    echo "WARNING: apps/member_feedback/README.md not found"
fi

echo "Member Feedback app verification: PASSED"

echo "All financial impact packages verified successfully!"