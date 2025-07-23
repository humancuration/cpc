# Impact Reporting Architecture Plan

## Overview
This document outlines the architecture for implementing the Impact Reporting feature in the desktop app (PDS). The design is based on the Android implementation while adapting to our tech stack (Svelte frontend + Rust backend + GraphQL).

## Android Implementation Reference
From the available `ImpactReportActivity.kt` we know:
- Main activity sets up the ImpactReportScreen with a user ID
- Uses Android's Jetpack Compose for UI
- Contains placeholder user ID logic

Other key files (based on file names):
- `ImpactBreakdownItem.kt`: Likely a data model for impact breakdown items
- `ImpactCategory.kt`: Data model for impact categories
- `ImpactTimelinePoint.kt`: Data model for timeline points
- `ImpactRepository.kt`: Handles data operations
- `ImpactReportViewModel.kt`: Manages UI state and business logic

## Desktop Implementation Components

### Frontend (Svelte)
Location: `apps/pds/frontend/src/lib/impact/`
1. **ImpactReportPage.svelte** (routes/impact/+page.svelte)
   - Main page component
   - Fetches data via GraphQL
   - Composes other components

2. **BreakdownView.svelte**
   - Visualizes impact breakdown data
   - Accepts `breakdownItems` prop

3. **DistributionView.svelte**
   - Shows impact distribution
   - Accepts `distributionData` prop

4. **TimelineView.svelte**
   - Displays timeline of impact events
   - Accepts `timelinePoints` prop

5. **ImpactStore.js**
   - Svelte store for managing impact state
   - Handles loading states and errors

### Backend (Rust)
Location: `apps/pds/src/impact.rs`
1. **ImpactService struct**
   - Handles business logic
   - Interfaces with database
   - Processes impact calculations

2. **Data Models**
   - `ImpactBreakdownItem`
   - `ImpactCategory`
   - `ImpactTimelinePoint`

### Shared (cpc-core)
Location: `packages/cpc-core/src/models/impact.rs`
- Shared data models (used by both frontend and backend)
- `ImpactBreakdownItem`
- `ImpactCategory`
- `ImpactTimelinePoint`
- `ImpactReport`

### GraphQL Operations
Location: `apps/pds/frontend/src/lib/graphql/impact.ts`
```graphql
# Queries
query GetImpactReport($userId: ID!) {
  impactReport(userId: $userId) {
    userId
    totalImpact
    breakdown {
      category
      amount
    }
    timeline {
      date
      description
      impactValue
    }
  }
}

# Mutations
mutation GenerateImpactReport($userId: ID!) {
  generateImpactReport(userId: $userId) {
    success
    message
    reportId
  }
}
```

## Data Flow
1. User navigates to /impact route
2. Frontend:
   - Calls `GetImpactReport` query
   - Shows loading state
   - On success:
     - Stores data in ImpactStore
     - Renders BreakdownView, DistributionView, TimelineView
   - On error: Shows error message

3. Backend (impact.rs):
   - Receives GraphQL request
   - Fetches data from database
   - Performs impact calculations
   - Returns structured ImpactReport

4. Shared Models:
   - Ensure consistent data structures
   - Serialize/deserialize between Rust and TS

## File Structure
```
apps/pds/
├── frontend/
│   ├── src/
│   │   ├── routes/
│   │   │   └── impact/
│   │   │       └── +page.svelte (main page)
│   │   ├── lib/
│   │   │   ├── impact/
│   │   │   │   ├── BreakdownView.svelte
│   │   │   │   ├── DistributionView.svelte
│   │   │   │   ├── TimelineView.svelte
│   │   │   │   └── ImpactStore.js
│   │   │   └── graphql/
│   │   │       └── impact.ts (GraphQL operations)
│   ├── src-tauri/
│   │   └── ... (Tauri config)
├── src/
│   ├── impact.rs (backend service)
│   └── ... (other backend modules)

packages/
├── cpc-core/
│   └── src/
│       ├── models/
│       │   └── impact.rs (shared models)
│       └── ... (other core logic)
```

## Next Steps
1. Implement shared models in cpc-core
2. Create backend impact service
3. Develop frontend components
4. Set up GraphQL operations