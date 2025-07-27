# Impact Report UI - Architectural Plan

## 1. Overview

This document outlines the architecture for the new Impact Report page in the `cpc-platform` application. This page provides users with a comprehensive overview of their positive impact using data from our backend GraphQL API. **The UI remains visually identical regardless of data source (mock or real), with all differences handled transparently through the feature flag system.**

## 2. File Structure

We will create a new `impact` module within `apps/cpc-platform/src/` to encapsulate all the logic and components related to the Impact Report page.

```
apps/cpc-platform/src/
├── impact/
│   ├── components/
│   │   ├── mod.rs
│   │   ├── impact_page.rs             # The main page component
│   │   ├── impact_timeline.rs         # Component for the timeline view
│   │   ├── impact_distribution_chart.rs # Component for the distribution chart
│   │   └── impact_breakdown_table.rs    # Component for the breakdown table
│   ├── routing.rs                     # Defines the routes for the impact module
│   └── mod.rs                         # Module declaration
├── api/
│   └── impact.rs                      # API calls for impact data
└── routes.rs                          # Main app router (updated)
```

## 3. Yew Components

### `ImpactPageComponent` (`impact_page.rs`)

This is the main container component for the Impact Report page.

-   **State:**
    -   `impact_report: UseStateHandle<Option<ImpactReport>>`
    -   `organization_impact_report: UseStateHandle<Option<OrganizationImpactReport>>`
    -   `loading: UseStateHandle<bool>`
    -   `error: UseStateHandle<Option<String>>`
-   **Functionality:**
    -   Fetches the `ImpactReport` and `OrganizationImpactReport` from the API on component mount.
    -   Manages the loading and error states.
    -   Renders the child components, passing them the relevant data.
-   **Layout:**
    -   A top-level summary section with key metrics (e.g., total impact, carbon footprint).
    -   A tabbed interface to switch between the user's personal impact and their organization's impact.
    -   Renders `ImpactTimelineComponent`, `ImpactDistributionChart`, and `ImpactBreakdownTable`.

### `ImpactTimelineComponent` (`impact_timeline.rs`)

Displays the user's impact over time.

-   **Props:**
    -   `timeline: Vec<ImpactTimelinePoint>`
-   **Functionality:**
    -   Renders a timeline of impact events using the `timeline` data.
    -   Each point on the timeline will show the date, a description of the event, and the impact value.
    -   We can use the `plotters` crate for a more visual representation.

### `ImpactDistributionChart` (`impact_distribution_chart.rs`)

Visualizes the distribution of impact across different categories.

-   **Props:**
    -   `distribution: Vec<ImpactDistribution>`
    -   `threshold: f64` - The UI degradation threshold value (default 0.15)
-   **Functionality:**
    -   Renders a pie chart or a bar chart showing the weight of each impact category.
    -   We will leverage the `plotters` crate for charting.
    -   Categories exceeding the threshold value are highlighted with special styling
    -   The threshold value is obtained from the feature flags subscription

### `ImpactBreakdownTable` (`impact_breakdown_table.rs`)

Provides a detailed breakdown of the user's impact.

-   **Props:**
    -   `breakdown: Vec<ImpactBreakdown>`
    -   `threshold: f64` - The UI degradation threshold value (default 0.15)
-   **Functionality:**
    -   Renders a table with the following columns: Category, Item Name, Contribution, and Impact Score.
    -   Allows for sorting and filtering of the data.
    -   Applies special styling to rows where the impact score exceeds the threshold
    -   Includes a threshold indicator in the table header

### Threshold-Based Highlighting

The UI degradation threshold is used to visually highlight significant impact categories:

1. **Chart Highlighting**:
   - Categories with weights exceeding the threshold are rendered with bolder outlines
   - The chart legend displays a special icon for highlighted categories
   - On hover, a tooltip shows "Significant Impact" for these categories

2. **Table Highlighting**:
   - Rows with impact scores above the threshold have a colored background
   - A threshold indicator appears in the table header showing the current value
   - Sorting by impact score automatically groups highlighted rows together

### Data Requirements and Validation

The ImpactDistribution data must adhere to the following requirements:

1. Each category must have a weight between 0 and 1
2. The sum of all weights must equal exactly 1.0 (within floating point precision)
3. There must be at least one category in the distribution

The UI component `ImpactDistributionChart` includes runtime validation that:
- Checks if the weights sum to 1.0 (within floating point precision)
- Displays an error message if the data is invalid
- Prevents rendering of the pie chart with invalid data

This ensures that the visualization accurately represents the impact distribution without misleading proportions.

### Error State Mapping

The UI handles backend errors through these mappings:

| GraphQL Error Code | UI Component | User Message | Developer Action |
|--------------------|--------------|--------------|------------------|
| `IMPACT_DATA_MISSING` | ImpactPage | "Impact data unavailable. [Retry]" | Verify user data exists |
| `IMPACT_CALCULATION_ERROR` | ErrorBanner | "Calculation failed. Contact support." | Check calculation logs |
| `IMPACT_VALIDATION_FAILED` | DistributionChart | "Invalid distribution data" (tooltips on invalid sections) | Validate data pipeline |

**Feature Flag Visibility**:
When testing, the current feature flag state (`impact_real_data_enabled`) and UI degradation threshold (`cpc_ui_degradation_threshold`) are visible in browser developer tools under:
```
Network → GraphQL Request → Response Headers → X-Feature-Flags
```

## Performance Considerations

Real data calculations introduce these performance considerations:

- **Maximum response time**: 850ms (p95) for impact report generation
- **Loading states**:
  - 0-300ms: No loading indicator
  - 300-1500ms: Skeleton UI with progress bar
  - >1500ms: "Data taking longer than expected" message
- **Degraded mode**: When response time exceeds 1500ms:
  - Show cached mock data with "Live data delayed" banner
  - Auto-refresh every 30 seconds
- **Real data validation**: All real data responses include `x-data-source: real` header for verification

## 4. API Integration (`api/impact.rs`)

This new module will handle all GraphQL API calls related to impact data.

-   **Functions:**
    -   `get_impact_report(user_id: String) -> Result<ImpactReport, String>`: Fetches the `ImpactReport` for a given user.
    -   `get_organization_impact_report(org_id: String, year: i32) -> Result<OrganizationImpactReport, String>`: Fetches the `OrganizationImpactReport` for a given organization.
-   **GraphQL Queries:**
    -   We will define the `getImpactReport` and `getOrganizationImpactReport` queries in new `.graphql` files under `apps/cpc-platform/src/graphql/queries/`.
    -   These files will be processed by our build script to generate the necessary Rust types.

### `graphql/queries/impact.graphql`

```graphql
query GetImpactReport($userId: ID!) {
  getImpactReport(userId: $userId) {
    userId
    totalImpact
    breakdown {
      category
      amount
      itemName
      contribution
      impactScore
    }
    distribution {
      category
      weight
    }
    timeline {
      date
      description
      impactValue
      timestamp
      score
    }
    generatedAt
  }
}

query GetOrganizationImpactReport($orgId: UUID!, $year: Int!) {
  getOrganizationImpactReport(orgId: $orgId, year: $year) {
    organizationId
    year
    carbonFootprint
    communityInvestment
    diversityMetrics {
      genderDiversity
      ethnicDiversity
    }
    supplyChainScore
  }
}
```

## 5. Routing

We will add a new route to the main application router to handle the Impact Report page.

### `impact/routing.rs`

```rust
use yew::prelude::*;
use yew_router::prelude::*;
use crate::impact::components::impact_page::ImpactPage;

#[derive(Clone, Routable, PartialEq)]
pub enum ImpactRoute {
    #[at("/impact")]
    Report,
}

pub fn switch_impact(routes: ImpactRoute) -> Html {
    match routes {
        ImpactRoute::Report => html! { <ImpactPage /> },
    }
}
```

### `routes.rs` Changes

The main `AppRoute` enum in `apps/cpc-platform/src/routes.rs` will be updated to include the new `Impact` route.

```rust
// In apps/cpc-platform/src/routes.rs (or equivalent)

use yew_router::prelude::*;
use crate::impact::routing::{switch_impact, ImpactRoute};
// ... other imports

#[derive(Clone, Routable, PartialEq)]
pub enum AppRoute {
    // ... other routes
    #[at("/impact")]
    #[nest("/impact")] // Add this line
    Impact,
}

pub fn switch(routes: AppRoute) -> Html {
    match routes {
        // ... other route arms
        AppRoute::Impact => html! {
            <Switch<ImpactRoute> render={switch_impact} />
        },
    }
}
```

This architecture provides a solid foundation for building the Impact Report page. It is modular, scalable, and follows the existing patterns in the codebase.