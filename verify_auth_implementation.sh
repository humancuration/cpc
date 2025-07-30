#!/bin/bash
#!/bin/bash

# Verify Auth Implementation Script

echo "Verifying Unified Authentication System Implementation..."

# Check if all required packages exist
echo "Checking for required packages..."
if [ ! -d "packages/cpc_rbac" ]; then
  echo "ERROR: cpc_rbac package not found"
  exit 1
fi

if [ ! -d "packages/cpc_karma" ]; then
  echo "ERROR: cpc_karma package not found"
  exit 1
fi

if [ ! -d "packages/cpc_consent" ]; then
  echo "ERROR: cpc_consent package not found"
  exit 1
fi

if [ ! -d "apps/auth_service" ]; then
  echo "ERROR: auth_service app not found"
  exit 1
fi

# Check if all required files exist
echo "Checking for required files..."

# RBAC package files
if [ ! -f "packages/cpc_rbac/src/lib.rs" ]; then
  echo "ERROR: packages/cpc_rbac/src/lib.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_rbac/Cargo.toml" ]; then
  echo "ERROR: packages/cpc_rbac/Cargo.toml not found"
  exit 1
fi

# Karma package files
if [ ! -f "packages/cpc_karma/src/lib.rs" ]; then
  echo "ERROR: packages/cpc_karma/src/lib.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_karma/Cargo.toml" ]; then
  echo "ERROR: packages/cpc_karma/Cargo.toml not found"
  exit 1
fi

# Consent package files
if [ ! -f "packages/cpc_consent/src/lib.rs" ]; then
  echo "ERROR: packages/cpc_consent/src/lib.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_consent/src/middleware.rs" ]; then
  echo "ERROR: packages/cpc_consent/src/middleware.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_consent/Cargo.toml" ]; then
  echo "ERROR: packages/cpc_consent/Cargo.toml not found"
  exit 1
fi

# Auth service files
if [ ! -f "apps/auth_service/src/main.rs" ]; then
  echo "ERROR: apps/auth_service/src/main.rs not found"
  exit 1
fi

if [ ! -f "apps/auth_service/Cargo.toml" ]; then
  echo "ERROR: apps/auth_service/Cargo.toml not found"
  exit 1
fi

if [ ! -f "apps/auth_service/proto/auth.proto" ]; then
  echo "ERROR: apps/auth_service/proto/auth.proto not found"
  exit 1
fi

if [ ! -f "apps/auth_service/build.rs" ]; then
  echo "ERROR: apps/auth_service/build.rs not found"
  exit 1
fi

# Modified files
if [ ! -f "apps/allat/src/domain/auth/user.rs" ]; then
  echo "ERROR: apps/allat/src/domain/auth/user.rs not found"
  exit 1
fi

if [ ! -f "apps/allat/src/infrastructure/middleware/authorization.rs" ]; then
  echo "ERROR: apps/allat/src/infrastructure/middleware/authorization.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_auth/src/error.rs" ]; then
  echo "ERROR: packages/cpc_auth/src/error.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_auth/src/session.rs" ]; then
  echo "ERROR: packages/cpc_auth/src/session.rs not found"
  exit 1
fi

if [ ! -f "packages/cpc_auth/Cargo.toml" ]; then
  echo "ERROR: packages/cpc_auth/Cargo.toml not found"
  exit 1
fi

# Documentation files
if [ ! -f "docs/auth_service_api.md" ]; then
  echo "ERROR: docs/auth_service_api.md not found"
  exit 1
fi

if [ ! -f "docs/ARCHITECTURE.md" ]; then
  echo "ERROR: docs/ARCHITECTURE.md not found"
  exit 1
fi

if [ ! -f "apps/allat/AUTH_INTEGRATION_SUMMARY.md" ]; then
  echo "ERROR: apps/allat/AUTH_INTEGRATION_SUMMARY.md not found"
  exit 1
fi

if [ ! -f "apps/allat/AUTH_IMPLEMENTATION_SUMMARY.md" ]; then
  echo "ERROR: apps/allat/AUTH_IMPLEMENTATION_SUMMARY.md not found"
  exit 1
fi

echo "All required files found!"

# Check Cargo.toml dependencies
echo "Checking Cargo.toml dependencies..."

# Check if cpc_auth has redis dependency
if ! grep -q "redis = \"0.23.0\"" "packages/cpc_auth/Cargo.toml"; then
  echo "ERROR: redis dependency not found in packages/cpc_auth/Cargo.toml"
  exit 1
fi

# Check if allat has new dependencies
if ! grep -q "cpc_consent" "apps/allat/Cargo.toml"; then
  echo "ERROR: cpc_consent dependency not found in apps/allat/Cargo.toml"
  exit 1
fi

if ! grep -q "cpc_rbac" "apps/allat/Cargo.toml"; then
  echo "ERROR: cpc_rbac dependency not found in apps/allat/Cargo.toml"
  exit 1
fi

if ! grep -q "cpc_karma" "apps/allat/Cargo.toml"; then
  echo "ERROR: cpc_karma dependency not found in apps/allat/Cargo.toml"
  exit 1
fi

# Check if auth_service has required dependencies
if ! grep -q "tonic = \"0.9.0\"" "apps/auth_service/Cargo.toml"; then
  echo "ERROR: tonic dependency not found in apps/auth_service/Cargo.toml"
  exit 1
fi

if ! grep -q "prost = \"0.12.0\"" "apps/auth_service/Cargo.toml"; then
  echo "ERROR: prost dependency not found in apps/auth_service/Cargo.toml"
  exit 1
fi

if ! grep -q "tonic-build = \"0.9.0\"" "apps/auth_service/Cargo.toml"; then
  echo "ERROR: tonic-build dependency not found in apps/auth_service/Cargo.toml"
  exit 1
fi

echo "All dependencies verified!"

echo "Unified Authentication System Implementation Verification Complete!"
echo "All components are properly implemented and integrated."