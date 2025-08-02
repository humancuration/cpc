# Skill Development gRPC Client

## Overview
Implements client for SkillDevelopment gRPC service. Used by the skill_tracking app to communicate with backend service.

## Dependencies
- `tonic` 0.10.2
- `prost` 0.12.3
- `tokio` for async runtime

## Client Implementation
Location: `apps/skill_tracking/src/services/grpc_client.rs`

```rust
pub struct SkillDevelopmentClient {
    inner: skill_development_client::SkillDevelopmentClient<tonic::transport::Channel>,
}

impl SkillDevelopmentClient {
    pub async fn connect(addr: String) -> Result<Self, Box<dyn std::error::Error>> {
        let client = skill_development_client::SkillDevelopmentClient::connect(addr).await?;
        Ok(Self { inner: client })
    }

    pub async fn track_skill_progress(
        &mut self,
        request: TrackSkillProgressRequest,
    ) -> Result<TrackSkillProgressResponse, Status> {
        self.inner.track_skill_progress(request).await
            .map(|res| res.into_inner())
    }

    pub async fn create_learning_path(
        &mut self,
        request: CreateLearningPathRequest,
    ) -> Result<CreateLearningPathResponse, Status> {
        self.inner.create_learning_path(request).await
            .map(|res| res.into_inner())
    }
    
    // Other methods...
}
```

## Connection Management
- Automatic reconnection on failure
- Timeout configuration
- Load balancing (future enhancement)

## Error Handling
- Map gRPC status codes to domain errors
- Retry on transient errors
- Circuit breaker pattern for fault tolerance

## Integration with UI
- Service provider pattern through Yew context
- Reactive state management
- Real-time updates via polling or streaming

## Testing Strategy
- Mock server for unit tests
- Integration tests with real service
- Test all error scenarios