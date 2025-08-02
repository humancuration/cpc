# Learning Platform Server

This is the backend server for the CPC Learning Platform. It provides gRPC services for course management, enrollment, credential issuance, and tipping functionality.

## Features

- Course management (create, update, list)
- User enrollment in courses
- Progress tracking
- Academic credential issuance
- Tipping system for educators
- User authentication

## Technologies

- Rust
- gRPC (Tonic)
- PostgreSQL (SQLx)
- JWT for authentication

## Setup

1. Install Rust and Cargo
2. Install PostgreSQL
3. Create a database named `learning_platform`
4. Set up the environment variables in `.env`
5. Run the server: `cargo run`

## Configuration

The server can be configured using environment variables:

- `DATABASE_URL`: PostgreSQL connection string
- `SERVER_ADDR`: Address to bind the gRPC server to
- `JWT_SECRET`: Secret key for JWT token signing

## API

The server exposes the following gRPC services:

- `CourseService`: Manage courses
- `EnrollmentService`: Handle user enrollments
- `CredentialService`: Issue academic credentials
- `TipService`: Process tips to educators
- `AuthService`: User authentication

## Development

To build the project:
```bash
cargo build
```

To run tests:
```bash
cargo test
```

To run the server:
```bash
cargo run