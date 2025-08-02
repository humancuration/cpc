# Skill Development Package

This package provides core functionality for skill development tracking, including skill management, progress tracking, learning paths, and certification management.

## Features

- **Skill Management**: Create and manage skills
- **Progress Tracking**: Track user progress for each skill (0-100%)
- **Learning Paths**: Create structured learning paths with milestones
- **Certification Management**: Store and retrieve user certifications
- **User Skill Profiles**: Comprehensive view of user skills and progress
- **gRPC API**: Full gRPC service for all functionality
- **Multiple Data Stores**: PostgreSQL and Sled support

## Database Schema

The package uses PostgreSQL with the following tables:

- `skills`: Stores skill definitions
- `skill_progress`: Tracks user progress for skills
- `certifications`: Stores user certifications
- `learning_paths`: Stores learning path definitions
- `milestones`: Stores milestones within learning paths

## Usage

### Setting up the database

1. Follow the instructions in `docs/skill_development/DATABASE_SETUP.md`
2. Run migrations: `cargo run --bin migrate up`

### Running the gRPC Server

```bash
# Start the gRPC server
cargo run --bin grpc_server
```

### Basic usage

```rust
use skill_development::*;
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = "postgres://skill_dev_user:secure_password@localhost/skill_dev_db";
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    
    // Run migrations
    sqlx::migrate!("./migrations").run(&pool).await?;
    
    // Create services
    let skill_tracking_service = SkillTrackingService::new(pool.clone());
    let learning_path_service = LearningPathService::new(pool.clone());
    let certification_service = CertificationService::new(pool.clone());
    
    // Add a skill
    let skill = skill_tracking_service.skill_repo.add_skill(
        "Rust Programming".to_string(),
        Some("Master Rust programming language".to_string())
    ).await?;
    
    // Track progress
    let user_id = uuid::Uuid::new_v4();
    let progress = skill_tracking_service.track_skill_progress(
        skill.id, 
        user_id, 
        75
    ).await?;
    
    // Create learning path
    let path = learning_path_service.create_learning_path(
        user_id,
        skill.id,
        "Rust Programming Path".to_string(),
        Some("Learn Rust from basics to advanced".to_string())
    ).await?;
    
    // Add milestone to learning path
    let milestone = learning_path_service.add_milestone_to_path(
        path.id,
        "Variables and Data Types".to_string(),
        Some("Learn about variables and data types in Rust".to_string()),
        1,
        Some(2)
    ).await?;
    
    // Complete milestone
    learning_path_service.complete_milestone(milestone.id).await?;
    
    // Issue certification
    let certification = certification_service.issue_certification(
        "Rust Programming Certification".to_string(),
        "CPC Cooperative".to_string(),
        chrono::Utc::now().date_naive(),
        user_id,
        Some(skill.id),
        domain::CertificationType::SkillAssessment,
        Some(2) // Advanced
    ).await?;
    
    // Get user profile
    let progress_list = skill_tracking_service.get_user_skill_progress(user_id).await?;
    let certs = certification_service.get_user_certifications(user_id).await?;
    let paths = learning_path_service.get_user_learning_paths(user_id).await?;
    
    Ok(())
}
```

## Running Tests

```bash
# Set up test database
export DATABASE_URL=postgres://skill_dev_user:secure_password@localhost/skill_dev_test_db

# Run tests
cargo test
```

## Migration Commands

```bash
# Apply all pending migrations
cargo run --bin migrate up

# Check migration status
cargo run --bin migrate status

# Rollback the latest migration
cargo run --bin migrate down
```

## gRPC Service

The package includes a full gRPC service implementation with the following methods:

- `TrackSkillProgress`: Track/update skill progress for a user
- `CreateLearningPath`: Create a new learning path
- `IssueCertification`: Issue a certification to a user
- `GetUserSkillProgress`: Get all skill progress for a user
- `GetUserCertifications`: Get all certifications for a user

## Dependencies

- `sqlx` - PostgreSQL client with compile-time checked queries
- `uuid` - UUID generation and handling
- `chrono` - Date/time handling
- `serde` - Serialization/deserialization
- `tonic` - gRPC implementation
- `async-trait` - Async trait support
- `validator` - Data validation
- `sled` - Embedded database for edge cases
- `tracing` - Logging and instrumentation

## Modules

### Domain
- `Skill`: Skill definition
- `SkillProgress`: User progress for a skill
- `LearningPath`: Structured learning path with milestones
- `Certification`: User certification with verification

### Application
- `SkillTrackingService`: Service for tracking skill progress
- `LearningPathService`: Service for managing learning paths
- `CertificationService`: Service for managing certifications

### Infrastructure
- `PostgresRepository`: PostgreSQL implementations for all repositories
- `SledRepository`: Sled embedded database implementations
- `gRPC`: Full gRPC service implementation