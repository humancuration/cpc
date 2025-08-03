# Shared Reviews Package Design

## Module Structure
```
reviews/
├── src/
│   ├── lib.rs          // Main exports
│   ├── models.rs       // Core domain models
│   ├── analytics.rs    // Statistical analysis and visualization
│   ├── filters.rs      // Filtering system implementation
│   ├── federation.rs   // Federation integration
│   ├── lakehouse.rs    // Data lakehouse integration
│   └── repository.rs   // Data access layer
├── Cargo.toml
└── README.md
```

## Data Models
```rust
// models.rs
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

/// Trait for entities that can be reviewed
pub trait Entity: Send + Sync {
    fn id(&self) -> Uuid;
    fn entity_type(&self) -> String;
}

/// Generic review model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review<T: Entity> {
    pub id: Uuid,
    pub entity: T,
    pub user_id: Uuid,
    pub title: String,
    pub content: String,
    pub ratings: Vec<Rating>,
    pub attributes: Vec<Attribute>,
    pub demographics: Option<Demographics>,
    pub created_at: DateTime<Utc>,
}

/// Scientific rating metric
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rating {
    /// Metric name (e.g., "effectiveness", "safety")
    pub metric: String,
    
    /// Value on 0.0-1.0 scale
    pub value: f32,
    
    /// Unit of measurement (e.g., "%", "mg")
    pub unit: Option<String>,
    
    /// Method used for measurement
    pub method: RatingMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RatingMethod {
    UserReported,
    ExpertAnalysis,
    ClinicalTrial,
}

/// Flexible attribute system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    /// Attribute key (e.g., "side_effects", "durability")
    pub key: String,
    
    /// Attribute value (e.g., "headache", "long-lasting")
    pub value: String,
}

/// User demographics (optional)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Demographics {
    pub age_group: String,
    pub gender: String,
    pub location: String,
    pub occupation: Option<String>,
}
```

## Analytics Engine Design
### Core Capabilities
1. **Statistical Analysis**
   - Rating distributions
   - Sentiment analysis using NLP
   - Correlation analysis
   - Time-based trends

2. **Visualization**
   - Use plotters crate for charts
   - Generate histograms for rating distributions
   - Create trend lines over time
   - Comparison charts between entities

### Implementation
```rust
// analytics.rs
use plotters::prelude::*;
use crate::models::{Review, Rating};
use crate::filters::ReviewFilters;

pub struct AnalyticsEngine;

impl AnalyticsEngine {
    /// Calculate average rating with filters
    pub fn average_rating(&self, filters: &ReviewFilters) -> f32 {
        // Implementation
    }
    
    /// Generate rating distribution chart
    pub fn plot_rating_distribution(
        &self,
        filters: &ReviewFilters
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Use plotters to create histogram
        Ok(())
    }
    
    /// Analyze sentiment using NLP
    pub fn analyze_sentiment(&self, text: &str) -> f32 {
        // Sentiment score implementation
    }
}
```

## Integration Points
### Ethical Scanner
```rust
// In ethical_scanner product implementation
impl Entity for Product {
    fn id(&self) -> Uuid { self.id }
    fn entity_type(&self) -> String { "product".to_string() }
}

// Usage example
let review = Review {
    entity: product.clone(),
    ratings: vec![
        Rating {
            metric: "safety".to_string(),
            value: 0.85,
            unit: None,
            method: RatingMethod::ExpertAnalysis,
        }
    ],
    // ... other fields
};
```

### Data Lakehouse
- Export reviews as Parquet files
- Use DuckDB for analysis
- Federated query support across instances

## Next Steps
1. Implement core repository with PostgreSQL backend
2. Add federation hooks using consent manager
3. Develop visualization components