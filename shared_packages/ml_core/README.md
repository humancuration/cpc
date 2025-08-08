# ML Core for CPC Platform

Machine learning framework built on top of the `linfa` library, designed specifically for cooperative values and community impact.

## Overview

The ML Core provides a comprehensive machine learning framework that integrates seamlessly with the CPC ecosystem while maintaining our cooperative values and community focus. It offers:

- Cooperative values-aware model training
- Privacy-preserving machine learning
- Explainable AI with community-focused interpretations
- Bias detection and mitigation
- Integration with CPC's mathematical ecosystem

## Features

### Core ML Engine
- Unified interface for all ML operations
- Caching for improved performance
- Progressive ML support for Web/WASM environments
- Cooperative values integration

### Models
Pre-built models for cooperative applications:
- **Volunteer Impact Prediction**: Predict volunteer retention, identify skill opportunities, forecast community impact
- **Financial Trend Analysis**: Predict financial sustainability, detect anomalous transactions, forecast economic trends
- **Skill Development Forecasting**: Predict skill acquisition timelines, identify optimal learning pathways
- **Cause Impact Modeling**: Predict cause effectiveness, identify success factors, forecast resource needs

### Evaluation
Comprehensive model evaluation with:
- Traditional ML metrics (accuracy, precision, recall, etc.)
- Community impact metrics (benefit, equity, accessibility)
- Cooperative values alignment metrics (transparency, fairness, participation)
- Bias metrics (demographic parity, equalized odds)
- Privacy compliance metrics

### Explainability
Tools for explaining ML predictions in accessible terms:
- Natural language explanations
- Feature importance visualization
- "What would change this outcome?" exploration
- Community validation workflows

### Privacy
Privacy-preserving techniques:
- Differential privacy
- Federated learning support
- Data anonymization
- Consent management integration

### Bias Detection & Mitigation
Fairness-focused tools:
- Bias detection across protected attributes
- Bias mitigation techniques
- Fairness constraint enforcement
- Demographic parity checking

## Architecture

```
ml_core/
├── engine/          # Core ML engine with cooperative values integration
├── models/          # Pre-built models for cooperative applications
├── evaluation/      # Model evaluation with community impact metrics
├── explainability/  # Tools for explaining ML predictions in accessible terms
├── privacy/         # Privacy-preserving techniques for ML training
├── bias/            # Bias detection and mitigation tools
├── cooperative_values/ # Cooperative values integration
└── error/           # Common error types
```

## Usage

### Basic Setup

```rust
use ml_core::MLEngine;
use ml_core::cooperative_values::CooperativeValues;

// Create ML engine with default cooperative values
let mut engine = MLEngine::new();

// Or with custom cooperative values
let values = CooperativeValues {
    enable_bias_detection: true,
    enable_privacy_preserving: true,
    enable_explainability: true,
    enable_community_validation: true,
    community_impact_weight: 0.8,
    ..Default::default()
};

let mut engine = MLEngine::with_config(values.into());
```

### Training a Model

```rust
use ml_core::models::ModelType;

// Prepare your training data
let training_data = /* your data */;
let labels = /* your labels */;

// Train a model
let model_id = engine.train_model(
    ModelType::VolunteerImpact,
    &training_data,
    &labels
)?;
```

### Making Predictions

```rust
// Make predictions with a trained model
let input_data = /* your input data */;
let predictions = engine.predict(&model_id, &input_data)?;
```

### Evaluating Models

```rust
// Evaluate model performance
let test_data = /* your test data */;
let test_labels = /* your test labels */;
let evaluation = engine.evaluate_model(&model_id, &test_data, &test_labels)?;
```

### Explainability

```rust
use std::collections::HashMap;

// Explain a prediction
let features = HashMap::from([
    ("feature1".to_string(), 0.8),
    ("feature2".to_string(), 0.5),
]);
let explanation = engine.explain_prediction(&predictions, &features)?;
```

## Integration with CPC Ecosystem

### Volunteer Coordination
```rust
use volunteer_coordination::ml::VolunteerML;

let volunteer_ml = VolunteerML::new();
let retention_score = volunteer_ml.predict_volunteer_retention(&engagement_data)?;
```

### Financial Systems (CPay)
```rust
use cpay_core::ml::FinancialML;

let financial_ml = FinancialML::new();
let sustainability_score = financial_ml.predict_sustainability(&financial_data)?;
```

### Skill Development
```rust
use skill_development::ml::SkillDevelopmentML;

let skill_ml = SkillDevelopmentML::new();
let timeline = skill_ml.predict_acquisition_timeline(&skill_data, &learner_profile)?;
```

### Cause Management
```rust
use cause_management::ml::CauseImpactML;

let cause_ml = CauseImpactML::new();
let effectiveness = cause_ml.predict_cause_effectiveness(&cause_data)?;
```

## Cooperative Values Integration

All ML operations are designed with cooperative values in mind:

- **Bias Detection**: Automatic detection and mitigation of bias across protected attributes
- **Privacy Preservation**: Differential privacy and federated learning support
- **Explainability**: Accessible explanations for all predictions
- **Community Validation**: Workflows for community feedback on model outputs
- **Fairness Constraints**: Enforcement of fairness in model training and predictions

## Examples

See the `examples/` directory for detailed usage examples:

- `volunteer_impact.rs`: Volunteer retention prediction and pathway recommendation
- `financial_trend.rs`: Financial sustainability prediction and fraud detection
- `skill_forecast.rs`: Skill acquisition timeline prediction and learning pathway optimization
- `cause_impact.rs`: Cause effectiveness prediction and resource allocation optimization

## Examples

Run the examples to see the ML Core in action:

```bash
# Volunteer impact prediction example
cargo run --example volunteer_impact

# Financial trend analysis example
cargo run --example financial_trend

# Skill development forecasting example
cargo run --example skill_forecast

# Cause impact modeling example
cargo run --example cause_impact

# Cooperative values integration example
cargo run --example cooperative_values
```

## Testing

Run tests with:

```bash
cargo test
```

## License

This crate is part of the Cooperative Platform Cooperative (CPC) ecosystem and is licensed under the CPC License, which promotes sharing within the federation while respecting the dignity of all participants.