# Learning Platform Server Development Guide

## Overview

This document provides guidelines and instructions for developing the Learning Platform Server.

## Prerequisites

- Rust and Cargo (latest stable version)
- PostgreSQL (for database)
- Docker and Docker Compose (optional, for containerized development)
- Git (for version control)

## Project Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd apps/learning_platform_server
   ```

2. Install dependencies:
   ```bash
   cargo build
   ```

3. Set up the database:
   - Install PostgreSQL locally or use Docker Compose:
     ```bash
     docker-compose up -d db
     ```

4. Run database migrations:
   ```bash
   ./scripts/migrate.sh
   ```
   or
   ```bash
   cargo run --bin migrate
   ```

## Development Workflow

### Building the Project

```bash
# Build in debug mode
cargo build

# Build in release mode
cargo build --release
```

### Running the Server

```bash
# Run in debug mode
cargo run

# Run with Docker
docker-compose up
```

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run tests with output
cargo test -- --nocapture
```

### Running Benchmarks

```bash
# Run all benchmarks
cargo bench

# Run specific benchmark
cargo bench benchmark_name
```

### Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### Linting

```bash
# Run clippy
cargo clippy

# Run clippy with fixes
cargo clippy --fix
```

## Project Structure

```
apps/learning_platform_server/
├── src/
│   ├── database/          # Database models and repository
│   ├── middleware/        # Middleware components
│   ├── grpc/              # gRPC service implementations
│   ├── config.rs          # Configuration handling
│   ├── error.rs           # Error handling
│   ├── logging.rs         # Logging utilities
│   ├── utils.rs           # Utility functions
│   └── main.rs            # Application entry point
├── migrations/            # Database migration files
├── proto/                 # Protocol buffer definitions
├── scripts/               # Development scripts
├── examples/              # Example clients
├── tests/                 # Integration tests
├── benches/               # Benchmark tests
├── docs/                  # Documentation
├── Cargo.toml             # Rust package manifest
├── Cargo.lock             # Rust dependency lock file
├── build.rs               # Build script for gRPC code generation
├── Dockerfile             # Docker image definition
├── docker-compose.yml     # Docker Compose configuration
├── .env                   # Environment variables
├── .gitignore             # Git ignore file
├── Makefile               # Make targets for common tasks
├── API.md                 # API documentation
└── README.md              # Project README
```

## Adding New Features

1. **Create a new branch**:
   ```bash
   git checkout -b feature/new-feature-name
   ```

2. **Implement the feature**:
   - Add new modules/files as needed
   - Follow existing code patterns and conventions
   - Add appropriate error handling
   - Include unit tests

3. **Run tests**:
   ```bash
   cargo test
   ```

4. **Format and lint code**:
   ```bash
   cargo fmt
   cargo clippy
   ```

5. **Commit changes**:
   ```bash
   git add .
   git commit -m "Add new feature: description"
   ```

6. **Push and create pull request**:
   ```bash
   git push origin feature/new-feature-name
   ```

## Database Migrations

To add a new database migration:

1. Create a new migration file in the `migrations/` directory:
   ```bash
   touch migrations/YYYYMMDDHHMMSS_description.sql
   ```

2. Add your SQL migration code to the file

3. Run the migration:
   ```bash
   ./scripts/migrate.sh
   ```

## gRPC Service Development

To add a new gRPC service:

1. **Update the proto file** in `proto/` directory

2. **Regenerate gRPC code**:
   ```bash
   cargo build
   ```

3. **Create a new service implementation** in `src/grpc/`

4. **Register the service** in `src/main.rs`

5. **Add tests** in `tests/` directory

## Error Handling

When adding new code:

1. Use the `AppError` enum for application errors
2. Convert to `tonic::Status` for gRPC responses
3. Log errors appropriately with context
4. Provide meaningful error messages to clients

## Testing Guidelines

1. **Unit Tests**: Test individual functions and modules
2. **Integration Tests**: Test service integration with database
3. **Test Coverage**: Aim for high test coverage, especially for critical paths
4. **Test Data**: Use realistic test data
5. **Test Isolation**: Ensure tests can run independently

## Performance Considerations

1. **Async Operations**: Use async/await for I/O operations
2. **Database Indexes**: Add indexes for frequently queried columns
3. **Connection Pooling**: Use connection pooling for database access
4. **Caching**: Consider caching for frequently accessed data
5. **Benchmarking**: Regularly benchmark critical paths

## Security Considerations

1. **Input Validation**: Validate all inputs
2. **Authentication**: Use JWT for authentication
3. **Authorization**: Implement proper authorization checks
4. **Password Security**: Use bcrypt for password hashing
5. **SQL Injection**: Use parameterized queries

## Documentation

Keep documentation up to date:

1. **API Documentation**: Update `API.md` when adding new endpoints
2. **Database Schema**: Update `docs/database_schema.md` when modifying schema
3. **Architecture**: Update `docs/architecture.md` when making architectural changes
4. **Development Guide**: Update this document when adding new development practices

## Deployment

1. **Build Release Binary**:
   ```bash
   cargo build --release
   ```

2. **Docker Image**:
   ```bash
   docker build -t learning-platform-server .
   ```

3. **Docker Compose**:
   ```bash
   docker-compose up -d
   ```

## Troubleshooting

### Common Issues

1. **Database Connection Failed**:
   - Check if PostgreSQL is running
   - Verify `DATABASE_URL` environment variable
   - Check database credentials

2. **Migration Errors**:
   - Ensure database is accessible
   - Check migration file syntax
   - Verify migration order

3. **gRPC Code Generation Failed**:
   - Check proto file syntax
   - Verify `tonic-build` dependency
   - Ensure proto files are in correct location

### Getting Help

1. **Check Logs**: Look at application logs for error messages
2. **Run Tests**: Run specific tests to isolate issues
3. **Consult Documentation**: Review relevant documentation
4. **Ask Team**: Reach out to team members for assistance