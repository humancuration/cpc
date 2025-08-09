# CPC Statistics Core

A statistical framework for the CPC platform that provides standardized statistical analysis capabilities with cooperative values-aligned result presentation.

## Features

- **Standardized Error Handling**: User-friendly error messages with actionable resolution steps
- **Confidence Intervals**: Multiple methods including bootstrapping, parametric, and Bayesian approaches
- **Statistical Significance Testing**: Color-coded significance levels with plain-language explanations
- **Effect Size Calculations**: Cohen's d and other effect size measures
- **Cooperative Values Alignment**: All results include methodology sources and audit trails

## Architecture

The library is organized into three main modules:

### Error Handling (`error.rs`)
Provides `StatisticalError` enum with:
- User-friendly messages with actionable steps
- Methodology source information
- Conversion from library errors (statrs, polars)

### Confidence Intervals (`confidence.rs`)
Implements:
- Bootstrap method for small datasets (<1000 samples)
- Parametric method for known distributions
- Bayesian approach for fundraising impact analysis
- Effect size calculations

### Significance Testing (`significance.rs`)
Implements:
- Color coding system for statistical significance:
  - Green: p < 0.01 (highly significant)
  - Yellow: 0.01 ≤ p < 0.05 (moderately significant)
  - Red: p ≥ 0.05 (not significant)
- Plain-language explanation generator

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
cpc_statistics_core = { path = "../../shared_packages/cpc_statistics_core" }
```

Enable the statistics feature in your application:

```toml
[features]
default = ["statistics"]
statistics = ["cpc_statistics_core"]
```

### Example Usage

```rust
use cpc_statistics_core::{
    ConfidenceCalculator,
    SignificanceTester,
    StatisticalError,
};

// Calculate confidence interval
let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
let ci = ConfidenceCalculator::parametric_interval(&data, 0.95)?;

// Perform significance test
let significance = SignificanceTester::one_sample_t_test(&data, 3.0)?;

println!("Confidence interval: [{}, {}]", ci.lower, ci.upper);
println!("Significance: p = {}", significance.p_value);
```

## Cooperative Values Integration

Every statistical result includes:
- **"Explain This"** functionality for detailed methodology
- **Methodology source** attribution for transparency
- **Audit trail** accessibility for all cooperative members
- **Plain-language explanations** instead of technical jargon

## Platform-Specific Optimizations

- **Web**: Implements Web Worker boundaries with message passing
- **Win64**: Uses rayon for parallel processing where appropriate
- **Memory Management**: Respects 5MB memory limits for web context

## Testing

The library includes comprehensive tests:
- Validation against NIST datasets for statistical correctness
- Cross-platform consistency testing (Web vs Win64)
- Memory usage verification within platform limits
- User testing of statistical explanations with cooperative members

Run tests with:

```bash
cargo test
```

## Dependencies

- `statrs` - Statistical computing library
- `polars` - DataFrame library for data processing
- `ndarray` - N-dimensional array library
- `ndarray-stats` - Statistical extensions for ndarray
- `thiserror` - Error handling utilities

## License

This library is part of the CPC software ecosystem and is licensed under the CPC license to promote sharing within the federation.