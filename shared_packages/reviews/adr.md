# ADR: Shared Reviews Package Design

## Status
Proposed

## Context
We need a generic review system that can be attached to any entity (products, services, apps) across our app ecosystem. The system must support:
- Advanced filtering capabilities
- Scientific/analytical features
- Data visualization
- Integration with data lakehouses
- Federation support

## Decision
We'll implement a shared `reviews` package using:
1. **Hexagonal architecture** with vertical slices
2. **Core domain models** that are entity-agnostic
3. **Plotters integration** for visualization
4. **Federation hooks** using existing consent manager
5. **Data lakehouse adapters** for large-scale analysis

### Core Domain Models
```rust
// Generic review model
pub struct Review<T: Entity> {
    pub id: Uuid,
    pub entity_id: Uuid,      // ID of the entity being reviewed
    pub entity_type: String,  // e.g. "product", "service"
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub ratings: Vec<Rating>,
    pub attributes: Vec<Attribute>,
    pub demographics: Option<Demographics>,
    pub survey_response: Option<SurveyResponse>, // Added for survey integration
    pub created_at: DateTime<Utc>,
}

// Rating with scientific metrics
pub struct Rating {
    pub metric: String,       // e.g. "effectiveness", "safety"
    pub value: f32,           // 0.0 - 1.0 scale
    pub unit: Option<String>, // e.g. "%", "mg"
}

// Flexible attribute system
pub struct Attribute {
    pub key: String,          // e.g. "side_effects", "durability"
    pub value: String,        // e.g. "headache", "long-lasting"
}

// User demographics (optional)
pub struct Demographics {
    pub age_group: String,    // e.g. "18-25", "65+"
    pub gender: String,
    pub location: String,
}
```

### Analytics Engine Design
```rust
pub struct AnalyticsEngine {
    // Statistical analysis capabilities
    pub fn average_rating(&self, filters: ReviewFilters) -> f32 { ... }
    pub fn rating_distribution(&self, filters: ReviewFilters) -> HashMap<f32, u32> { ... }
    
    // Visualization integration
    pub fn plot_rating_trends(&self, filters: ReviewFilters) -> PlotResult { ... }
    pub fn compare_entities(&self, entity_ids: Vec<Uuid>) -> PlotResult { ... }
    
    // Data lakehouse integration
    pub fn export_to_lakehouse(&self, filters: ReviewFilters) { ... }
    pub fn analyze_in_lakehouse(&self, query: String) -> AnalysisResult { ... }
}
```

### Filtering System
```rust
pub struct ReviewFilters {
    pub entity_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub metrics: Vec<MetricFilter>,
    pub attributes: Vec<AttributeFilter>,
    pub demographics: Option<DemographicFilter>,
    pub date_range: Option<(DateTime<Utc>, DateTime<Utc>)>,
}

// Visitor pattern implementation
pub trait FilterVisitor {
    fn visit_metric(&mut self, filter: &MetricFilter);
    fn visit_attribute(&mut self, filter: &AttributeFilter);
    fn visit_demographic(&mut self, filter: &DemographicFilter);
}
```

### Integration Points
1. **Ethical Scanner**
   - Attach reviews to `Product` model
   - Use health metrics from `health_engine`
   - Share reviews via federation

2. **Data Lakehouse**
   - Export reviews as Parquet files
   - Run large-scale SQL analysis
   - Visualize trends over time

3. **Federation**
   - Opt-in data sharing
   - Consent-manager integration
   - Cross-instance review aggregation

## Consequences
- Positive: Reusable review system across all apps
- Positive: Scientific approach to user reviews
- Negative: Added complexity for entity binding
- Risk: Performance with large review datasets

## Integration with Survey System
We've added optional survey responses to reviews:
```rust
pub struct Review<T: Entity> {
    // ... existing fields ...
    pub survey_response: Option<SurveyResponse>,
}
```
- Survey responses allow detailed feedback beyond star ratings
- Validation ensures survey responses match survey templates
- Analysis can correlate survey data with review metrics

## Next Steps
1. Implement core models and repository
2. Build analytics engine with plotters integration
3. Create federation adapters
4. Add data lakehouse export capabilities
5. Integrate survey module with review system