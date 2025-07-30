# Finance Module Migration Guide

This document describes how to migrate from the old personal-finance app to the new finance module in cpc-core.

## Overview

The personal finance functionality has been moved from `apps/personal-finance/` to `packages/cpc-core/finance/` to follow the screaming architecture principles. All finance domain logic now exists as vertical slices within the core package.

## Migration Steps

1. Update your Cargo.toml dependencies:
   ```toml
   # Remove this line:
   # personal-finance = { path = "../apps/personal-finance" }
   
   # Add this line:
   cpc-core = { path = "../packages/cpc-core", features = ["finance"] }
   ```

2. Update your imports:
   ```rust
   // Old imports:
   // use personal_finance::savings_goals::domain::models::SavingsGoal;
   
   // New imports:
   use cpc_core::finance::domain::savings_goal::SavingsGoal;
   ```

3. Update database migrations:
   - Move migration files from `apps/backend/migrations/20250728_*` to `packages/cpc-core/migrations/`
   - Update migration filenames to follow CPC Core format: `YYYYMMDDHHMMSS_description.sql`

4. Update service initialization:
   ```rust
   // Old initialization:
   // let finance_service = personal_finance::init_service(
   //     database_url,
   //     ubi_config,
   //     treasury_config,
   //     ocr_config,
   // ).await?;
   
   // New initialization:
   // let finance_module = cpc_core::finance::initialize_finance_module(
   //     db_pool,
   //     p2p_manager,
   //     user_consent_store,
   // );
   ```

## Breaking Changes

1. GraphQL API has been removed - all sensitive financial operations now use p2p channels only
2. REST API endpoints have been removed - use the new p2p communication layer
3. Database table names remain the same but are now managed through cpc-core migrations

## New Features

1. p2p data sharing with Double Ratchet encryption
2. Bevy visualization components for financial data
3. Improved privacy controls with explicit user consent