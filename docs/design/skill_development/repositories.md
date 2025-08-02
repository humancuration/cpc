# Skill Development Repositories

## Overview
Implements data access layer using SQLx with PostgreSQL. Follows repository pattern for database operations.

## Dependencies
- `sqlx` with Postgres
- `tokio` for async runtime
- Domain models

## Repository Interfaces

### SkillProgressRepository
```rust
#[async_trait]
pub trait SkillProgressRepository: Send + Sync {
    async fn create(&self, progress: &SkillProgress) -> Result<(), RepositoryError>;
    async fn update(&self, progress: &SkillProgress) -> Result<(), RepositoryError>;
    async fn find_by_user_and_skill(&self, user_id: &Uuid, skill_id: &Uuid) -> Result<Option<SkillProgress>, RepositoryError>;
    async fn list_by_user(&self, user_id: &Uuid) -> Result<Vec<SkillProgress>, RepositoryError>;
}
```

### LearningPathRepository
```rust
#[async_trait]
pub trait LearningPathRepository: Send + Sync {
    async fn create(&self, path: &LearningPath) -> Result<(), RepositoryError>;
    async fn update(&self, path: &LearningPath) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: &Uuid) -> Result<Option<LearningPath>, RepositoryError>;
    async fn list_by_creator(&self, creator_id: &Uuid) -> Result<Vec<LearningPath>, RepositoryError>;
}
```

### CertificationRepository
```rust
#[async_trait]
pub trait CertificationRepository: Send + Sync {
    async fn issue(&self, certification: &Certification) -> Result<(), RepositoryError>;
    async fn find_by_user(&self, user_id: &Uuid) -> Result<Vec<Certification>, RepositoryError>;
    async fn verify(&self, verification_code: &str) -> Result<Option<Certification>, RepositoryError>;
}
```

## Schema Updates Required
Update existing tables to match proto models:
```sql
-- Add learning_path table
CREATE TABLE learning_paths (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    creator_id UUID NOT NULL,
    difficulty_level SMALLINT NOT NULL CHECK (difficulty_level BETWEEN 1 AND 5),
    estimated_duration_hours INT,
    created_at TIMESTAMPTZ DEFAULT NOW()
);

-- Modify skill_progress table
ALTER TABLE skill_progress
ADD COLUMN current_level SMALLINT NOT NULL DEFAULT 1,
ADD COLUMN target_level SMALLINT NOT NULL DEFAULT 5,
ADD COLUMN last_practice_date TIMESTAMPTZ;

-- Add certification_type to certifications
ALTER TABLE certifications
ADD COLUMN certification_type SMALLINT NOT NULL,
ADD COLUMN level_achieved SMALLINT NOT NULL,
ADD COLUMN verification_code VARCHAR(64) NOT NULL UNIQUE;
```

## Error Handling
- Custom `RepositoryError` enum mapping SQLx errors to domain errors
- Transaction support for atomic operations
- Connection pooling via `PgPool`

## Testing Strategy
- Integration tests with testcontainers-rs
- Test all edge cases and constraint violations