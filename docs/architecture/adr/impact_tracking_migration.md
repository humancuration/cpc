# Impact Tracking Migration - Architecture Decision Record

## Context
We're migrating impact tracking functionality from the old Android implementation to our shared Rust backend. The current implementation is music-specific and needs to be generalized to support environmental, social, and economic impact tracking.

## Requirements
1. Support multiple impact categories
2. Efficient data aggregation
3. Privacy-preserving data collection
4. Historical data tracking
5. Verifiable impact claims
6. Desktop and mobile client compatibility

## Proposed Architecture

### Data Models
```rust
// packages/cpc-core/src/models/impact.rs

pub enum ImpactCategory {
    Environmental,
    Social,
    Economic,
}

pub struct ImpactMetric {
    pub id: Uuid,
    pub name: String,
    pub category: ImpactCategory,
    pub unit: String,
    pub calculation_formula: String,
}

pub struct ImpactReport {
    pub id: Uuid,
    pub user_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub overall_score: f64,
    pub category_distribution: HashMap<ImpactCategory, f64>,
}

pub struct ImpactTimelinePoint {
    pub timestamp: DateTime<Utc>,
    pub value: f64,
    pub category: ImpactCategory,
}

pub struct ImpactBreakdownItem {
    pub id: Uuid,
    pub source: String,
    pub description: String,
    pub category: ImpactCategory,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}
```

### Services
```rust
// packages/cpc-core/src/services/impact.rs

pub struct ImpactCalculator;

impl ImpactCalculator {
    pub fn calculate_impact(&self, raw_data: Vec<RawImpactData>) -> ImpactReport {
        // Implementation logic
    }
}

pub struct ImpactAggregator {
    repository: Arc<dyn ImpactRepository>,
}

impl ImpactAggregator {
    pub async fn aggregate_user_impact(&self, user_id: Uuid) -> ImpactReport {
        // Aggregate data from multiple sources
    }
}
```

### gRPC Service Definition
```proto
// packages/cpc-protos/impact.proto

service ImpactService {
    rpc ComputeImpactReport (ComputeImpactReportRequest) 
        returns (stream ComputeImpactReportResponse);
}

message ComputeImpactReportRequest {
    string user_id = 1;
    google.protobuf.Timestamp start_date = 2;
    google.protobuf.Timestamp end_date = 3;
}

message ComputeImpactReportResponse {
    oneof result {
        ProgressUpdate progress = 1;
        ImpactReport report = 2;
    }
}

message ProgressUpdate {
    string message = 1;
    float percent_complete = 2;
}
```

### Database Schema
```sql
CREATE TABLE impact_metrics (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    category TEXT NOT NULL,
    unit TEXT NOT NULL,
    calculation_formula TEXT NOT NULL
);

CREATE TABLE impact_reports (
    id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    period_start INTEGER NOT NULL,
    period_end INTEGER NOT NULL,
    overall_score REAL NOT NULL,
    category_distribution TEXT NOT NULL
);

CREATE TABLE impact_data_points (
    id TEXT PRIMARY KEY,
    report_id TEXT NOT NULL,
    source TEXT NOT NULL,
    value REAL NOT NULL,
    timestamp INTEGER NOT NULL,
    category TEXT NOT NULL,
    metadata TEXT NOT NULL
);
```

## Migration Plan
1. Implement core models and services in `cpc-core`
2. Add gRPC service definition to `cpc-protos`
3. Create GraphQL API in backend
4. Integrate with Axum server
5. Update Android client to use new API
6. Implement privacy-preserving data collection
7. Add verification mechanisms for impact claims

## Consequences
- Unified impact tracking across all platforms
- Support for multiple impact categories
- Scalable architecture for future expansion
- Verifiable impact claims through cryptographic proofs
- Efficient data processing through worker nodes