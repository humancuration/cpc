# Skill Development Application Services

## Overview
Implements business logic for skill tracking operations. Mediates between domain models and infrastructure.

## Dependencies
- Domain models
- Repository traits
- `thiserror` for error handling
- `async_trait` for async methods

## Service Interfaces

### SkillTrackingService
```rust
#[async_trait]
pub trait SkillTrackingService {
    async fn track_progress(&self, user_id: Uuid, skill_id: Uuid, new_level: u8) -> Result<SkillProgress, ServiceError>;
    async fn get_progress(&self, user_id: Uuid) -> Result<Vec<SkillProgress>, ServiceError>;
    async fn calculate_progress_percentage(&self, progress: &SkillProgress) -> f32;
}

pub struct SkillTrackingServiceImpl {
    repo: Arc<dyn SkillProgressRepository>,
}
```

### LearningPathService
```rust
#[async_trait]
pub trait LearningPathService {
    async fn create_learning_path(&self, path_data: CreateLearningPathRequest) -> Result<LearningPath, ServiceError>;
    async fn recommend_paths(&self, user_id: Uuid) -> Result<Vec<LearningPath>, ServiceError>;
}
```

### CertificationService
```rust
#[async_trait]
pub trait CertificationService {
    async fn issue_certification(&self, request: IssueCertificationRequest) -> Result<Certification, ServiceError>;
    async fn verify_certification(&self, code: &str) -> Result<Certification, ServiceError>;
}
```

## Business Rules
- Progress percentage calculated as: `(current_level / target_level) * 100`
- Certification issuance requires minimum skill level 80%
- Learning path recommendations based on user's skill gaps
- Validation of all inputs before persistence

## Error Handling
- `ServiceError` enum with variants:
  - `ValidationError(String)`
  - `RepositoryError(RepositoryError)`
  - `NotFound`
  - `PermissionDenied`
- Comprehensive error mapping

## Testing Strategy
- Mock repositories for unit tests
- Test all business rules and edge cases
- Property-based testing for validation logic