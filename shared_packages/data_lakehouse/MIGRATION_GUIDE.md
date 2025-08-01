# Data Lakehouse Migration Guide

This guide provides instructions for migrating to the data lakehouse module.

## Overview

The data lakehouse module provides a federation-wide data management solution that serves as our primary big data repository. It implements several key features:

- Data ingestion from multiple sources (PostgreSQL, Sled, APIs, file drops)
- WebM columnar storage using royalty-free AV1 codec
- Change Data Capture (CDC) for real-time data synchronization
- Extended audit logging with privacy-preserving anonymization
- GraphQL API for data access and management
- Integration adapters for health, finance, and CRM modules

## Migration Steps

### 1. Update Cargo.toml Dependencies

Add the data lakehouse module to your project dependencies:

```toml
[dependencies]
cpc-core-data-lakehouse = { path = "../shared_packages/data_lakehouse" }
```

### 2. Database Schema Migration

The data lakehouse requires several new database tables:

1. `data_assets` - Stores metadata about data assets
2. `ingestion_jobs` - Tracks data ingestion jobs
3. `webm_columnar` - Stores WebM-encoded columnar data
4. `audit_logs` - Extended audit log entries
5. `data_lineage` - Tracks data transformation lineage

Run the provided SQL migration scripts to create these tables.

### 3. Configuration Updates

Update your application configuration to include data lakehouse settings:

```json
{
  "data_lakehouse": {
    "postgresql_connection": "postgresql://user:password@localhost:5432/lakehouse",
    "webm_storage_path": "/var/lib/lakehouse/webm",
    "audit_log_retention_days": 365,
    "cdc_replication_slot": "lakehouse_cdc"
  }
}
```

### 4. Service Integration

Integrate the data lakehouse services into your application:

```rust
use cpc_core_data_lakehouse::{
    application::ingestion_service::IngestionService,
    infrastructure::{
        cdc::postgres_cdc::PostgresCDCManager,
        storage::webm_columnar::WebMColumnarStorage,
        monitoring::audit_service::{DataAuditService, PostgresAuditLogRepository}
    }
};

// Initialize services
let cdc_manager = PostgresCDCManager::new(db_pool.clone());
let storage_manager = WebMColumnarStorage::new(db_pool.clone());
let audit_repo = PostgresAuditLogRepository::new(db_pool.clone());
let audit_service = DataAuditService::new(Arc::new(audit_repo), Default::default());

// Create ingestion service
let ingestion_service = IngestionService::new(
    Arc::new(repository),
    Arc::new(cdc_manager),
    Arc::new(storage_manager)
);
```

### 5. API Integration

Add the GraphQL schema to your API:

```rust
use cpc_core_data_lakehouse::infrastructure::api::graphql::{create_schema, LakehouseSchema};

let schema = create_schema();
// Mount the schema on your GraphQL endpoint
```

### 6. Module Integration

Integrate with existing modules using the provided adapters:

```rust
use cpc_core_data_lakehouse::infrastructure::integration::{
    health_adapter::HealthAdapter,
    finance_adapter::FinanceAdapter,
    crm_adapter::CRMAdapter
};

let health_adapter = HealthAdapter::new();
let finance_adapter = FinanceAdapter::new(cdc_manager);
let crm_adapter = CRMAdapter::new();
```

## Data Migration

### Health Module Data

Health data can be migrated using the HealthAdapter:

```rust
// Transform vital signs to data assets
let asset = health_adapter.transform_vital_sign(vital_sign);
// Apply research anonymization
health_adapter.apply_research_anonymization(&mut asset);
```

### Finance Module Data

Finance data can be migrated using CDC ingestion:

```rust
// Create ingestion job for finance data
let job = finance_adapter.create_finance_ingestion_job(
    "Finance Data Sync".to_string(),
    "postgresql://finance:pass@db:5432/finance".to_string(),
    "transactions".to_string()
).await?;
```

### CRM Module Data

CRM data can be processed using the CRMAdapter:

```rust
// Create customer journey analytics
let analytics_asset = crm_adapter.create_customer_journey_analytics(interactions);
// Perform cohort analysis
let cohort_result = crm_adapter.perform_cohort_analysis(&interactions);
```

## Testing

After migration, verify the following:

1. Data ingestion jobs can be created and executed
2. WebM columnar storage is working correctly
3. Audit logging is capturing access events
4. GraphQL API endpoints are functional
5. Module integrations are working as expected

## Rollback Procedure

If issues are encountered, you can rollback by:

1. Reverting the Cargo.toml changes
2. Dropping the new database tables
3. Restoring the previous application configuration
4. Reverting any code changes that depend on the data lakehouse

## Support

For assistance with migration, contact the data lakehouse module maintainers or refer to the documentation in `shared_packages/data_lakehouse/ARCHITECTURE.md`.