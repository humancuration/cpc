# Impact Visualization Framework

The Impact Visualization Framework translates complex mathematical results into clear community impact metrics that all cooperative members can understand and act upon.

## Overview

This framework bridges the gap between technical analysis and community understanding by:
- Translating complex metrics into community impact language
- Showing how individual actions contribute to collective goals
- Making cooperative values tangible and measurable
- Enabling data-informed decision making at all levels of the cooperative

## Modules

### Core (`impact_viz::core`)
The foundation of the visualization framework providing:
- `ImpactVisualization` trait for implementing visualization components
- `ImpactMetric` for representing community impact metrics
- `ValuesTranslator` for connecting metrics to cooperative values
- `AccessibilityManager` for ensuring inclusive design

### Volunteer Impact (`impact_viz::volunteer`)
Visualizations for volunteer coordination:
- Individual and collective volunteer impact dashboards
- Skill development progression with community benefit context
- Retention predictions with actionable insights
- Impact stories connecting activities to outcomes

### Financial Health (`impact_viz::financial`)
Visualizations for financial transparency:
- Community wellbeing indicators from financial metrics
- Resource flows with community impact attribution
- Sustainability metrics with future projections
- "What if" scenarios for different allocation strategies

### Skill Development (`impact_viz::skill`)
Visualizations for learning and growth:
- Community skill landscapes with gap analysis
- Individual skill growth within community context
- Skill development mapping to community needs
- Pathway visualizations connecting learning to impact

### Cause Impact (`impact_viz::cause`)
Visualizations for cause effectiveness:
- Cause effectiveness with narrative elements
- Resource impact with community stories
- Prediction confidence with transparent explanations
- Comparative visualizations across different causes

## Features

### Values-Aligned Presentation
- Narrative-driven visualization that tells community stories
- Comparative metrics showing community progress over time
- Transparent methodology explanations ("how we measure impact")
- Community validation features for impact metrics
- Multilingual and culturally appropriate representations

### Accessibility First
- Text alternatives for all visual elements
- Screen reader support
- High contrast modes
- Simplified views for low-bandwidth environments
- Configurable font sizes

### Community Co-Creation
- Features for community input on visualization design
- Mechanisms for community validation of impact metrics
- Customization based on community priorities
- Collaborative interpretation features

## Examples

The framework includes several examples demonstrating different visualization types:

1. **Volunteer Impact Dashboard** - `examples/volunteer_impact_dashboard.rs`
2. **Financial Health Visualization** - `examples/financial_health_visualization.rs`
3. **Skill Development Mapping** - `examples/skill_development_mapping.rs`
4. **Cause Impact Storytelling** - `examples/cause_impact_storytelling.rs`

## Integration

The framework integrates with:
- Statistics from `cause_management`
- Financial precision from `common_utils`
- BI analytics from `bi_analytics`
- Optimization results from `optimization_core`
- ML predictions from `ml_core`
- Consent management system

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
impact_viz = { path = "../shared_packages/impact_viz" }
```

Basic usage example:

```rust
use impact_viz::core::ImpactVizCore;
use impact_viz::volunteer::VolunteerImpactDashboard;

// Create the core visualization engine
let core_viz = ImpactVizCore::new();
let dashboard = VolunteerImpactDashboard::new(Box::new(core_viz));

// Visualize individual volunteer impact
let engagement_data = get_volunteer_engagement_data();
let viz_result = dashboard.visualize_individual_impact(&engagement_data);
```

## License

This project is licensed under the CPC License - see the LICENSE file for details.

## Contributing

We welcome contributions from the community! Please see our contributing guidelines for more information.