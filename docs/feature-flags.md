# Feature Flags Implementation Guide

## Core Implementation

Our feature flag system follows the **Safe Rollout Pattern** to enable controlled feature deployment while maintaining backward compatibility. The implementation centers around the `FeatureFlags` struct:

```rust
// apps/backend/src/main.rs
#[derive(Clone)]
struct FeatureFlags {
    impact_real_data_enabled: bool,
}

impl FeatureFlags {
    fn new() -> Self {
        Self { impact_real_data_enabled: false } // Safe default
    }
}
```

### Key Characteristics
- **Immutable State**: Flags are set at startup and never modified
- **Zero Overhead**: Simple boolean checks with no runtime configuration
- **Explicit Injection**: Flags are explicitly passed to components that need them
- **Context-Aware**: Available through Axum/GraphQL request context

## Backward Compatibility Strategy

When introducing new features that might affect existing clients:

```rust
// apps/backend/src/graphql/impact.rs
let distribution = if feature_flags.impact_real_data_enabled {
    impact_calculator.calculate(&user_id).await?
} else {
    vec![
        ImpactDistribution { category: "Community".into(), weight: 0.45 },
        ImpactDistribution { category: "Environment".into(), weight: 0.30 },
        ImpactDistribution { category: "Workers".into(), weight: 0.25 },
    ]
};
```

### Compatibility Approach
1. **Graceful Degradation**: New code includes fallback paths for disabled features
2. **Data Consistency**: Both paths produce identical data structures
3. **Validation Parity**: Same business rules apply regardless of flag state
4. **Seamless Transition**: Clients experience no disruption during rollout

> **Why This Works**: By maintaining identical output structures between implementations, we avoid client-side compatibility issues. The frontend receives consistent data shapes whether the feature is enabled or not.

## When to Use Feature Flags

### ✅ Recommended Use Cases

#### 1. Business Logic Evolution
When introducing new calculation algorithms that must coexist with legacy implementations:
```rust
// Before full rollout
let result = if flags.new_calculator_enabled {
    NewCalculator::calculate(...)
} else {
    LegacyCalculator::calculate(...)
};
```

#### 2. Data Migration Strategies
During phased data migrations where both old and new data formats exist:
```rust
if flags.use_new_data_model {
    process_new_format(data)
} else {
    convert_and_process_old_format(data)
}
```

#### 3. Controlled Experimentation
For A/B testing of different impact distribution models:
```rust
let model = match flags.experiment_group {
    ExperimentGroup::A => DistributionModel::Conservative,
    ExperimentGroup::B => DistributionModel::Aggressive,
};
```

### 🚫 Anti-Patterns to Avoid

#### 1. Configuration Creep
```rust
// ❌ Dangerous: Using flags for permanent configuration
if flags.show_community_impact {
    // ...community logic
}
if flags.show_environment_impact {
    // ...environment logic
}
// Leads to combinatorial explosion of states
```

**Instead**: Use proper user preferences stored in the database.

#### 2. Security Bypass
```rust
// ❌ Critical flaw: Using flags for access control
if flags.is_admin_override {
    grant_admin_access();
}
```

**Instead**: Use proper authentication and authorization mechanisms.

#### 3. Temporary Technical Debt
```rust
// ❌ Debt trap: Flags that live forever
if flags.temp_fix_for_issue_123 {
    // Special case handling
}
```

**Mitigation**: Every flag must have an expiration strategy.

## Best Practices

### 1. Safe Default Policy
Always initialize new flags in the **disabled state**:
```rust
fn new() -> Self {
    Self {
        impact_real_data_enabled: false, // Default to safe behavior
    }
}
```

### 2. Explicit Expiration Strategy
Document removal criteria when adding a flag:
```rust
/// impact_real_data_enabled
/// - Enabled: July 1, 2025
/// - Target removal: August 15, 2025
/// - Criteria: 100% traffic on new implementation for 7 days
```

### 3. Monitoring Requirements
Every flag must have corresponding monitoring:
- Percentage of requests using each path
- Error rates by flag state
- Performance comparison between implementations

### 4. Test Coverage Mandate
All flag combinations must be tested:
```rust
#[test]
fn test_with_flags_disabled() {
    let flags = FeatureFlags { impact_real_data_enabled: false };
    // Test legacy behavior
}

#[test]
fn test_with_flags_enabled() {
    let flags = FeatureFlags { impact_real_data_enabled: true };
    // Test new behavior
}
```

## Impact Real Data Rollout

### Flag Configuration
- **Flag name**: `impact_real_data_enabled`
- **Purpose**: Controls transition from mock data to real impact distribution calculations
- **Default state**: `false` (mock data)
- **Configuration source**: Environment variable `IMPACT_REAL_DATA_ENABLED`

### Rollout Strategy
1. **Initial deployment**: 0% activation (mock data only)
2. **Validation phase**: 25% activation after verifying:
   - Database migration completion
   - Successful dry-run calculations
3. **Expansion phase**: 50% activation after 24h with:
   - <1% calculation error rate
   - p95 response time <850ms
4. **Full activation**: 100% activation after 72h stable performance

### Critical Dependencies
- Must complete database migration [`20250726_impact_weights_table.sql`](/apps/backend/migrations/20250726_impact_weights_table.sql)
- Requires monitoring pipeline for:
  ```text
  impact_calculation_success_rate
  distribution_validation_errors
  real_data_response_time
  ```

### Safety Mechanisms
- **Automatic fallback**: Reverts to mock data when:
  ```rust
  if error_rate > 0.05 && error_duration > Duration::minutes(5) {
      use_mock_data = true;
  }
  ```
- **Circuit breaker**: Temporarily disables real data calculations after 3 consecutive failures

### Monitoring Requirements
| Metric | Target | Alert Threshold |
|--------|--------|-----------------|
| Calculation Success Rate | ≥99.5% | <95% for 5min |
| Validation Error Rate | 0% | >1% for 10min |
| p95 Response Time | ≤850ms | ≥1500ms for 5min |

**Configuration Example**:
```rust
// apps/backend/src/main.rs
fn new() -> Self {
    Self {
        impact_real_data_enabled: env::var("IMPACT_REAL_DATA_ENABLED")
            .map(|v| v.parse().unwrap_or(false))
            .unwrap_or(false),
    }
}
```

> **Operational Note**: This flag must be removed within 30 days of full activation. Create GitHub issue `CLEANUP-impact_real_data_enabled` with target date.

### Impact Distribution Highlight Threshold

This feature flag controls the visual highlighting of significant impact categories in the distribution chart and breakdown table.

| Parameter | Environment Variable | Default | Description | Effect |
|-----------|----------------------|---------|-------------|--------|
| Impact Distribution Highlight Threshold | `CPC_UI_DEGRADATION_THRESHOLD` | 0.15 | Controls when impact categories are visually highlighted | Categories exceeding this threshold receive special styling in charts and tables |

### UI Degradation Thresholds
| Threshold Parameter | Default | Description | Alert Threshold |
|---------------------|---------|-------------|-----------------|
| `impact_ui_fresh_threshold` | 300ms | Maximum latency before showing live indicator | N/A |
| `impact_ui_degraded_threshold` | 1500ms | Maximum latency before showing skeleton loader | ≥2000ms for 5min |
| `impact_ui_stale_threshold` | 1500ms | Minimum latency to show warning banner | ≥1800ms for 10min |