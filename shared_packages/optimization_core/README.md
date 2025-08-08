# Optimization Core

A comprehensive optimization framework for the CPC ecosystem using `argmin` to enable sophisticated resource allocation, volunteer matching, and cooperative planning.

## Overview

The Optimization Core provides a robust foundation for solving complex optimization problems across the cooperative platform. It's designed with cooperative values at its core, ensuring that mathematical optimization serves community benefit and fairness.

## Features

### Core Optimization Engine
- Uses `argmin` as the underlying optimization framework
- Implements multiple optimization algorithms (gradient descent, evolutionary algorithms, etc.)
- Provides cooperative values-aware objective functions
- Supports both single-objective and multi-objective optimization
- Includes robust convergence detection and error handling

### Application-Specific Optimizers

#### Volunteer-Task Matching (`volunteer_coordination`)
- Matching volunteers to tasks based on skills, availability, and community impact
- Balancing workload across volunteers
- Maximizing skill development opportunities

#### Resource Allocation (`cooperative_fundraising`)
- Optimizing resource distribution across causes
- Balancing immediate needs with long-term sustainability
- Incorporating community voting results

#### Route Optimization (`community_connect`)
- Optimizing physical meeting routes for community events
- Minimizing transportation costs and environmental impact
- Maximizing accessibility for community members

#### Skill Development Pathways (`skill_development`)
- Creating personalized learning paths
- Optimizing skill acquisition sequences
- Balancing individual growth with community needs

### Performance Considerations
- Progressive solution refinement (important for Web/WASM)
- Parallel evaluation of candidate solutions
- Caching of common optimization patterns
- Memory-efficient data structures for large-scale problems
- Time-bound optimization with best-available results

### Cooperative Values Integration
- Design objective functions that prioritize community benefit over individual optimization
- Implement "fairness constraints" to prevent optimization from creating inequitable outcomes
- Create mechanisms for community input into optimization parameters

## Architecture

```
optimization_core/
├── src/
│   ├── engine.rs          # Core optimization engine
│   ├── problem.rs         # Optimization problem definitions
│   ├── solvers.rs         # Optimization algorithm implementations
│   ├── progressive.rs     # Progressive optimization for Web/WASM
│   ├── cooperative_values.rs # Cooperative values integration
│   ├── error.rs           # Error types
│   └── lib.rs             # Public API
├── examples/
│   └── community_impact_optimization.rs # Comprehensive usage examples
└── Cargo.toml             # Package manifest
```

## Usage Examples

### Basic Optimization Engine Usage

```rust
use optimization_core::engine::OptimizationEngine;
use optimization_core::problem::VolunteerMatchingObjective;
use ndarray::Array1;

// Create optimization engine
let mut engine = OptimizationEngine::new();

// Define volunteer skills (2 volunteers, 3 tasks)
let skill_matrix = ndarray::arr2(&[
    [0.9, 0.2, 0.7], // Volunteer 1 skills
    [0.3, 0.8, 0.6], // Volunteer 2 skills
]);

let availability = ndarray::arr1(&[1.0, 1.0]);
let task_priority = ndarray::arr1(&[1.0, 1.0, 1.0]);
let community_impact = ndarray::arr1(&[0.8, 0.9, 0.7]);

// Create optimization problem
let problem = engine.create_volunteer_matching_problem(
    skill_matrix,
    availability,
    task_priority,
    community_impact,
);

// Solve the problem
let result = engine.solve(&problem, Some("volunteer_matching".to_string()))?;

// Use the solution
println!("Optimal assignment: {:?}", result.solution);
```

### Cooperative Values Integration

```rust
use optimization_core::engine::{OptimizationEngine, EngineConfig};
use optimization_core::cooperative_values::CooperativeValues;

// Create engine with cooperative values
let cooperative_values = CooperativeValues {
    prioritize_community_benefit: true,
    community_impact_weight: 2.0,
    show_transparency: true,
    enable_community_validation: true,
    fairness_threshold: 0.7,
    max_inequality: 0.4,
};

let config = EngineConfig {
    cooperative_values,
    ..Default::default()
};

let mut engine = OptimizationEngine::with_config(config);
```

### Progressive Optimization for Web/WASM

```rust
use optimization_core::progressive::ProgressiveOptimizer;

// Create progressive optimizer for Web/WASM environments
let optimizer = ProgressiveOptimizer::new();

// Solve with time constraints and fallback mechanisms
let result = optimizer.solve_progressive(&problem)?;
```

## Integration with CPC Ecosystem

The optimization framework integrates seamlessly with other CPC packages:

- **bi_analytics**: Use optimization results for advanced analytics
- **common_utils**: Leverage high-precision calculations
- **consent_manager**: Ensure privacy-aware optimization
- **Application packages**: Specialized optimizers for each domain

## Performance Considerations

### Web/WASM Performance
- Implements time-sliced optimization to prevent browser UI blocking
- Creates fallback to simpler algorithms when complex optimizations exceed WASM limits
- Designs progressive disclosure of results (show partial results while optimizing)

### Memory Efficiency
- Uses efficient data structures from `ndarray`
- Implements caching strategies to avoid recomputation
- Supports streaming optimization for large datasets

## Testing

The framework includes comprehensive tests for all components:

```bash
cargo test
```

## Examples

Run the comprehensive examples:

```bash
cargo run --example community_impact_optimization
```

## License

This package is part of the CPC software ecosystem and is licensed under the CPC license.

## Contributing

Contributions are welcome! Please read our contributing guidelines and code of conduct.