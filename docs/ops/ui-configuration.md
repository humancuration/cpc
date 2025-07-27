# UI Configuration Guide

## Degradation Threshold
Configures when UI elements should be visually highlighted based on impact contribution
- **Environment Variable**: `CPC_UI_DEGRADATION_THRESHOLD`
- **Default**: 0.15 (15%)
- **Range**: 0.0-1.0

```bash
# Example: Set threshold to 20%
export CPC_UI_DEGRADATION_THRESHOLD=0.2
```

Implementation details:
- Backend: `UiThresholds` struct in [config.rs](../apps/backend/src/config.rs)
- Frontend: Prop-drilled to impact visualization components

## Performance Thresholds
### Impact Distribution Chart Parameters
```rust
// Default configuration (apps/backend/src/config.rs)
Config {
    impact_ui_thresholds: UiThresholds {
        degradation: 0.15,
        fresh: Duration::from_millis(300),
        degraded: Duration::from_millis(1500),
        stale: Duration::from_millis(1500),
    }
}
```

**Adjustment Procedure**:
1. Update environment variables:
   - `CPC_UI_DEGRADATION_THRESHOLD=0.15`
   - `IMPACT_UI_FRESH_THRESHOLD_MS=300`
   - `IMPACT_UI_DEGRADED_THRESHOLD_MS=1500`
   - `IMPACT_UI_STALE_THRESHOLD_MS=1500`
2. Restart backend service
3. Verify with synthetic monitoring

**Monitoring Recommendations**:
- Track `impact_ui_response_time` metric
- Set alerts for when p95 latency exceeds 1500ms for >5min
- Use canary deployments for threshold changes