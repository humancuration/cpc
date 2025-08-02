# Learning Core Usage Guide

This guide explains how to use the Learning Core crate in your applications.

## Installation

Add the following to your `Cargo.toml`:

```toml
[dependencies]
learning_core = { path = "../shared_packages/learning_core" }
```

## Running the gRPC Server

To run the gRPC server:

1. Set up a PostgreSQL database
2. Run migrations:
   ```bash
   DATABASE_URL=postgres://user:password@localhost/learning_db cargo run --bin migrate
   ```
3. Start the server:
   ```bash
   DATABASE_URL=postgres://user:password@localhost/learning_db SERVER_ADDRESS=127.0.0.1:50052 cargo run --bin grpc_server
   ```

## Using the gRPC Client

To use the gRPC client in your application:

```rust
use learning_core::learning_platform_client::LearningPlatformClient;
use learning_core::{CourseRequest, EnrollmentRequest};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = LearningPlatformClient::connect("http://[::1]:50052").await?;

    // Create a course
    let request = tonic::Request::new(CourseRequest {
        title: "Rust Programming".to_string(),
        description: "Learn Rust programming language".to_string(),
        creator_id: "user-uuid-here".to_string(),
    });

    let response = client.create_course(request).await?;
    let course = response.into_inner().course.unwrap();
    println!("Created course: {}", course.title);

    Ok(())
}
```

## Using the Domain Models Directly

You can also use the domain models directly in your application:

```rust
use learning_core::domain::{Course, Enrollment, AcademicCredential, Tip};
use uuid::Uuid;

// Create a course
let course = Course::new(
    "Rust Programming".to_string(),
    "Learn Rust programming language".to_string(),
    Uuid::new_v4() // creator_id
);

// Create an enrollment
let enrollment = Enrollment::new(
    Uuid::new_v4(), // user_id
    course.id
);

// Create a credential
let credential = AcademicCredential::new(
    Uuid::new_v4(), // user_id
    course.id,
    learning_core::domain::CredentialType::Certificate
);

// Create a tip
let tip = Tip::new(
    Uuid::new_v4(), // from_user_id
    Uuid::new_v4(), // to_user_id
    10.0, // amount
    "USD".to_string(), // currency
    Some(course.id) // course_id
);
```

## Running Tests

To run unit tests:

```bash
cargo test
```

To run integration tests (requires a test database):

```bash
DATABASE_URL=postgres://user:password@localhost/learning_test_db cargo test
```

## Database Schema

The crate uses the following tables:

- `courses` - Course metadata
- `modules` - Course modules
- `lessons` - Individual lessons
- `enrollments` - User course enrollments
- `academic_credentials` - Issued credentials
- `tips` - Tipping transactions

See `migrations/0001_learning_platform_tables.sql` for the full schema.