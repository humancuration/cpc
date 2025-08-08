# Cause Impact Tracker

## Overview

The Cause Impact Tracker is a comprehensive system for measuring the effectiveness of cause impact visualizations and refining them based on real community feedback and usage patterns. It provides tools for tracking engagement, analyzing correlations, and generating actionable insights to improve cause management within the CPC platform.

## Features

- **Engagement Tracking**: Monitor how community members interact with cause visualization components
- **Correlation Analysis**: Measure the relationship between visualization usage and cause engagement rates
- **Effectiveness Monitoring**: Track contribution effectiveness and community transformation metrics
- **Community Validation**: Record and analyze community feedback on cause impact
- **Privacy Compliance**: Collect data with full respect for user consent levels
- **Cross-platform Integration**: Connect with learning, volunteering, and financial impact metrics

## Modules

### Tracker
The core tracking functionality that collects metrics on:
- Visualization engagement (time spent, interactions, quality scores)
- Cause engagement correlation (participation rates, satisfaction)
- Contribution effectiveness (decision quality, impact measurement)
- Community validation (endorsements, critiques, suggestions)

### Analytics
Dashboard functionality for cause coordinators that provides:
- Engagement metrics and trends
- Cause effectiveness measurements
- Community impact analysis
- Feedback summaries and sentiment analysis
- Actionable recommendations

### Feedback
Feedback collection and processing systems that enable:
- Quick "Was this helpful?" feedback
- Detailed feedback with ratings and comments
- In-context suggestions for improvement
- Community voting on visualization effectiveness
- Qualitative feedback analysis

### Improvement
Continuous improvement mechanisms that include:
- A/B testing of different visualization approaches
- Automated suggestions for personalized engagement
- Community-curated visualization templates
- Impact scoring for visualization components

### Integration
Integration with the broader impact ecosystem that enables:
- Cross-platform correlation analysis
- Holistic community impact measurement
- Coordinated recommendations across platforms
- Unified data collection and analysis

## Usage

### Basic Setup

```rust
use cause_impact_tracker::CauseImpactTracker;
use consent_manager::domain::consent::DataSharingLevel;

// Create a tracker with the user's consent level
let mut tracker = CauseImpactTracker::new(DataSharingLevel::Standard);
```

### Tracking Visualization Engagement

```rust
use cause_impact_tracker::tracker::VisualizationType;

tracker.track_visualization_engagement(
    "user_123",
    "cause_impact_storytelling_v1",
    VisualizationType::Narrative,
    180.5, // Time spent in seconds
    25,    // Number of interactions
    0.85,  // Quality score (0.0 to 1.0)
    Some(0.92), // Decision confidence
)?;
```

### Tracking Cause Engagement Correlation

```rust
tracker.track_engagement_correlation(
    "user_123",
    vec!["cause_impact_storytelling_v1".to_string()],
    true,      // Currently engaged
    Some(4.5), // Months engaged
    Some(9),   // Satisfaction rating (1-10)
    Some(75.50), // Contribution amount
)?;
```

### Collecting Feedback

```rust
use cause_impact_tracker::FeedbackCollector;

let mut feedback_collector = FeedbackCollector::new(DataSharingLevel::Standard);

// Quick feedback
feedback_collector.collect_quick_feedback(
    "user_123",
    "cause_impact_storytelling_v1",
    true, // Helpful
)?;

// Detailed feedback
feedback_collector.collect_detailed_feedback(
    "user_456",
    "resource_allocation_tracker",
    4, // Rating (1-5 stars)
    Some("Very useful visualization".to_string()),
    true, // Helpful
    Some("Helped me decide to increase my contribution".to_string()),
    Some(8), // Understanding improvement (1-10)
    Some(9), // Confidence improvement (1-10)
)?;
```

### Generating Analytics Dashboard

```rust
use cause_impact_tracker::ImpactAnalyticsDashboard;
use skill_development::ml::CommunityData;

let metrics = tracker.get_metrics();
let dashboard = ImpactAnalyticsDashboard::new(metrics.clone());

// Create community data (in a real app, this would come from actual data)
let community_data = CommunityData {
    member_count: 1250,
    skill_distribution: HashMap::new(),
    activity_levels: HashMap::new(),
    resource_availability: HashMap::new(),
};

let summary = dashboard.generate_summary(&community_data);
println!("Engagement Quality Score: {:.2}", summary.engagement.quality_score);
```

## Privacy and Consent

The Cause Impact Tracker fully respects user privacy through a consent-based data collection system:

- **None**: No data collection
- **Minimal**: Basic engagement metrics only
- **Standard**: Full feature set (default)
- **Enhanced**: Additional detailed analytics

All data is collected with explicit user consent and can be configured at runtime.

## Integration with Other Systems

The Cause Impact Tracker integrates with:

- **Learning Impact Tracker**: Correlate cause engagement with learning activities
- **Volunteer Impact Tracker**: Connect cause participation with volunteering
- **Financial Impact Tracker**: Link cause visualization usage with financial contributions
- **Cause Management**: Direct integration with cause data and metadata

## Running the Demo

To see the tracker in action, run the provided demo:

```bash
cd shared_packages/cause_impact_tracker
cargo run --example cause_impact_demo
```

## Testing

Run the test suite with:

```bash
cd shared_packages/cause_impact_tracker
cargo test
```

## License

This project is part of the CPC platform and is licensed under the CPC license.