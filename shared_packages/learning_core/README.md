# Learning Core

Learning Core is the backend library for the CPC Learning Platform, providing domain models, application services, and gRPC infrastructure for course management, enrollment tracking, credential issuance, and tipping functionality.

## Features

- Course creation and management
- User enrollment tracking
- Progress monitoring
- Academic credential issuance
- Educator tipping system
- gRPC API for integration
- PostgreSQL database integration

## Architecture

The crate follows a hexagonal architecture with the following layers:

- **Domain**: Core business entities (Course, Enrollment, Credential, Tip)
- **Application**: Service layer coordinating business logic
- **Infrastructure**: Database repositories and gRPC server/client

## Getting Started

### Prerequisites

- Rust 1.70+
- PostgreSQL 17.5+
- Tonic for gRPC

### Building

```bash
cd shared_packages/learning_core
cargo build
```

### Running Migrations

```bash
DATABASE_URL=postgres://user:password@localhost/learning_db cargo run --bin migrate
```

### Running the gRPC Server

```bash
DATABASE_URL=postgres://user:password@localhost/learning_db SERVER_ADDRESS=127.0.0.1:50052 cargo run --bin grpc_server
```

## Database Schema

The crate uses the following tables:

- `courses` - Course metadata
- `modules` - Course modules
- `lessons` - Individual lessons
- `enrollments` - User course enrollments
- `academic_credentials` - Issued credentials
- `tips` - Tipping transactions

## gRPC API

The service exposes the following gRPC methods:

- `CreateCourse` - Create a new course
- `EnrollUser` - Enroll a user in a course
- `UpdateProgress` - Update user progress in a course
- `IssueCredential` - Issue an academic credential
- `TipEducator` - Send a tip to an educator

See `proto/learning_platform.proto` for full API definitions.

## Integration with Skill System

The learning platform integrates with the skill development system by:

1. Updating skill progress when course progress is updated
2. Issuing skill-based certifications
3. Sharing user learning paths with skill development

## License

This software is part of the CPC ecosystem and is licensed under the CPC License.