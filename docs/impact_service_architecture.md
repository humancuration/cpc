# Impact Service Architecture

## Overview
The impact service provides carbon footprint, community investment, diversity metrics, and supply chain scoring for users and organizations. It follows hexagonal architecture with core domain logic in `cpc-core` and adapters for GraphQL, Tauri, and database.

## Core Components

### 1. Domain Layer (cpc-core)
```rust
// packages/cpc-core/src/impact.rs

pub struct ImpactCalculator {
    // Configuration and dependencies
}

impl ImpactCalculator {
    pub async fn calculate_carbon_footprint(org_id: Uuid, year: i32) -> Result<f64> {
        // Actual implementation using sustainability data
    }
    
    // Similar methods for other calculations
}

pub struct DiversityMetrics {
    pub gender_diversity: f64,
    pub ethnic_diversity: f64,
}

pub struct OrganizationImpactReport {
    // Fields matching GraphQL schema
}
```

### 2. Service Layer (backend)
```rust
// apps/backend/src/services/impact.rs

pub struct ImpactService {
    db: DbPool,
    calculator: ImpactCalculator,
}

impl ImpactService {
    pub async fn get_organization_impact_report(
        &self, 
        org_id: Uuid, 
        year: i32
    ) -> Result<Option<OrganizationImpactReport>> {
        // Database lookup + calculation
    }
    
    // Implement other methods
}
```

### 3. Database Schema
```sql
-- apps/backend/migrations/202407241300_create_impact_tables.sql

CREATE TABLE organization_impact_reports (
    id UUID PRIMARY KEY,
    organization_id UUID NOT NULL,
    year INT NOT NULL,
    carbon_footprint FLOAT NOT NULL,
    community_investment FLOAT NOT NULL,
    gender_diversity FLOAT NOT NULL,
    ethnic_diversity FLOAT NOT NULL,
    supply_chain_score FLOAT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_org_impact ON organization_impact_reports(organization_id, year);
```

### 4. GraphQL Layer
```rust
// apps/backend/src/graphql/impact.rs

#[derive(Default)]
pub struct ImpactQuery;

#[Object]
impl ImpactQuery {
    async fn get_organization_impact_report(
        &self,
        ctx: &Context<'_>,
        org_id: Uuid,
        year: i32
    ) -> Result<Option<OrganizationImpactReport>> {
        let service = ctx.data_unchecked::<ImpactService>();
        service.get_organization_impact_report(org_id, year).await
    }
    
    // Other resolvers
}

pub struct ImpactSubscription;

#[Subscription]
impl ImpactSubscription {
    async fn impact_report_updated(
        &self,
        user_id: ID
    ) -> impl Stream<Item = ImpactReport> {
        // Implementation using async streams
    }
}
```

### 5. Tauri Command Layer
```rust
// apps/cpc-platform/src-tauri/src/impact_commands.rs

#[tauri::command]
async fn get_organization_impact_report(
    org_id: String,
    year: i32,
    state: State<'_, AppState>
) -> Result<OrganizationImpactReport, Error> {
    let uuid = Uuid::parse_str(&org_id)?;
    state.impact_service.get_organization_impact_report(uuid, year).await?
}
```

## Workflow

1. **Report Generation:**
   - Client triggers report generation via GraphQL mutation or Tauri command
   - ImpactService orchestrates data collection from DB
   - ImpactCalculator performs domain calculations
   - Results stored in DB and pushed via subscription

2. **Data Flow:**
   ```mermaid
   graph LR
   Client-->|Request|GraphQL/Tauri
   GraphQL/Tauri-->|Call|ImpactService
   ImpactService-->|Use|ImpactCalculator
   ImpactService-->|Query|Database
   ImpactService-->|Push|Subscription
   ```

```

3. **Error Handling:**
- Domain errors mapped to GraphQL errors
- Tauri commands return Result types
- Comprehensive logging via tracing

## Frontend Integration

The frontend integrates with the impact service through the following components:

### Impact Service (Rust)
Located at `apps/cpc-platform/src-yew/src/services/impact.rs`
- Provides async methods to call Tauri commands
- Handles response parsing and error conversion
- Includes:
- `get_impact_report`: Fetches complete impact data
- `recalculate_impact`: Triggers report recalculation
- `subscribe_impact_updates`: Subscribes to real-time updates

### Impact Dashboard
Located at `apps/cpc-platform/src-yew/src/components/impact/impact_dashboard.rs`
- Fetches impact data on mount and when organization changes
- Uses Yewdux for global state management
- Implements:
- Loading states during data fetch
- Error handling for failed requests
- Real-time updates via subscriptions
- Manual refresh button to trigger recalculation

### Card Components
Located in `apps/cpc-platform/src-yew/src/components/impact/`
- Receive data objects from dashboard
- Render specific impact metrics:
- `carbon_footprint_card.rs`: Carbon footprint data
- `diversity_metrics_card.rs`: Diversity metrics
- `community_investment_card.rs`: Community investments
- `supply_chain_ethics_chart.rs`: Supply chain scores

### State Management
Located at `apps/cpc-platform/src-yew/src/store.rs`
- Stores impact data in global state
- Provides reducers for updating impact data
- Handles loading states and errors

## Next Steps

1. Implement core calculation logic in ImpactCalculator
2. Create database migrations
3. Complete GraphQL resolvers
4. Implement Tauri command layer
5. Add integration tests
6. Enhance frontend with additional visualization options