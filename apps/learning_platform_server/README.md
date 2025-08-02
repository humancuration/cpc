# Learning Platform Server

This is the backend server for the CPC Learning Platform. It provides gRPC services for course management, enrollment, credential issuance, and tipping functionality.

## Features

- Course management (create, update, list)
- User enrollment in courses
- Progress tracking
- Academic credential issuance
- Tipping system for educators
- User authentication and registration

## Technologies

- Rust
- gRPC (Tonic)
- PostgreSQL (SQLx)
- JWT for authentication
- Docker for containerization

## Setup

### Option 1: Manual Setup

1. Install Rust and Cargo
2. Install PostgreSQL
3. Create a database named `learning_platform`
4. Set up the environment variables in `.env`
5. Run the server: `cargo run`

### Option 2: Docker Setup

1. Install Docker and Docker Compose
2. Run `docker-compose up` to start the server and database

### Option 3: Using Scripts

1. Make sure you have Rust and Cargo installed
2. Run `./scripts/start.sh` to start the server with database and migrations

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
- `UserService`: User registration
- `HealthService`: Service health checks

See [API.md](API.md) for detailed API documentation.

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
```

To run with Docker:
```bash
docker-compose up
```

To run the example client:
```bash
cargo run --example client
```

To run migrations:
```bash
./scripts/migrate.sh
```

To start the server with database and migrations:
```bash
./scripts/start.sh
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
├── Cargo.toml             # Rust package manifest
├── Cargo.lock             # Rust dependency lock file
├── build.rs               # Build script for gRPC code generation
├── Dockerfile             # Docker image definition
├── docker-compose.yml     # Docker Compose configuration
├── .env                   # Environment variables
├── .gitignore             # Git ignore file
├── Makefile               # Make targets for common tasks
├── API.md                 # API documentation
└── README.md              # This file
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Write tests if applicable
5. Run tests to ensure nothing is broken
6. Submit a pull request

## License

This project is proprietary and confidential. All rights reserved.