<!-- DEPRECATED: Superseded by docs/architecture/impact-service.md -->
<!-- Last valid as of: 2025-07-26 -->
# Impact Report Implementation Notes

## Key Components

### Shared Models
- Location: `packages/cpc-core/src/models/impact.rs`
- Defines core data structures:
  - `ImpactCategory`: Enum (Environmental, Social, Economic)
  - `ImpactReport`: Main report structure
  - `ImpactTimelinePoint`: Historical data point
  - `ImpactBreakdownItem`: Detailed contribution item
- Used across backend and frontend

### Backend Implementation
- Location: `apps/backend/src/services/impact_service.rs`
- Main service: `ImpactService`
- Key method: `get_user_impact_report()`
  - Generates sample data (production would use real analytics)
  - Returns `ImpactReport` struct
- GraphQL integration in `apps/backend/src/graphql/impact.rs`

### Android Implementation
- Location: `apps/cpc-platform/android/app/src/main/kotlin/coop/cpc/platform/impact/`
- Key files:
  - `ImpactReportActivity.kt`: Entry activity
  - `ImpactReportScreen.kt`: Main composable UI
  - View models: Handle data loading and state
  - UI components:
    - `DistributionView.kt`: Shows category breakdown
    - `TimelineView.kt`: Shows historical trends
    - `BreakdownView.kt`: Shows itemized contributions

## Implementation Details

### Backend Service
The `ImpactService` currently returns sample data:
```rust
pub async fn get_user_impact_report(&self, user_id: Uuid) -> Result<ImpactReport, Error> {
    // Generate dummy data
    Ok(ImpactReport {
        user_id,
        generated_at: Utc::now(),
        overall_score: 82.4,
        // ... other fields
    })
}
```

### Android UI Structure
The impact report screen uses a tabbed interface:
```kotlin
HorizontalPager(state = pagerState) { page ->
    when (page) {
        0 -> DistributionView(report!!)
        1 -> TimelineView(report!!)
        2 -> BreakdownView(report!!)
    }
}
```

### Data Flow
1. Android app calls GraphQL API for impact report
2. Backend service generates report data
3. Data flows through ViewModel to UI components
4. UI renders three main views:
   - Distribution: Pie chart visualization
   - Timeline: Historical trend graph
   - Breakdown: Detailed contribution list

## Future Improvements
1. Replace sample data with real analytics
2. Add caching for report data
3. Implement data refresh mechanism
4. Add PDF export functionality
5. Enhance visualization with proper chart libraries