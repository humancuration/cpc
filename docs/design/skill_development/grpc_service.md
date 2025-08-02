# Skill Development gRPC Service

## Overview
Implements the gRPC server from proto definition. Maps between protobuf messages and domain models.

## Dependencies
- Application services
- `tonic` 0.10.2
- `prost` 0.12.3
- `tracing` for logging

## Service Implementation
```rust
pub struct SkillDevelopmentService {
    skill_tracker: Arc<dyn SkillTrackingService>,
    path_creator: Arc<dyn LearningPathService>,
    cert_issuer: Arc<dyn CertificationService>,
}

#[tonic::async_trait]
impl SkillDevelopment for SkillDevelopmentService {
    async fn track_skill_progress(&self, request: Request<TrackSkillProgressRequest>) -> Result<Response<TrackSkillProgressResponse>, Status> {
        // Convert request to domain model
        // Call application service
        // Convert result to proto response
    }
    
    async fn create_learning_path(&self, request: Request<CreateLearningPathRequest>) -> Result<Response<CreateLearningPathResponse>, Status> {
        // Implementation
    }
    
    // Other methods...
}
```

## Server Setup
In `src/infrastructure/grpc/mod.rs`:
```rust
pub async fn run_grpc_server(
    addr: SocketAddr,
    skill_tracker: Arc<dyn SkillTrackingService>,
    path_creator: Arc<dyn LearningPathService>,
    cert_issuer: Arc<dyn CertificationService>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = SkillDevelopmentService { skill_tracker, path_creator, cert_issuer };
    Server::builder()
        .add_service(SkillDevelopmentServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
```

## Error Mapping
- Map domain errors to gRPC status codes:
  - `NotFound` → `Status::not_found()`
  - `ValidationError` → `Status::invalid_argument()`
  - `PermissionDenied` → `Status::permission_denied()`
- Use `tonic::Status` for all error responses

## Integration Points
- SocialIntegration event bus for real-time updates
- Offline sync service for Sled storage
- Authentication middleware

## Testing Strategy
- Mock application services
- Integration tests with in-memory gRPC client
- Test all edge cases and error mappings