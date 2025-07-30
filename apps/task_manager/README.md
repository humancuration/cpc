# Task Manager Module

A comprehensive task management system built with Rust, featuring real-time collaboration, notifications, and P2P synchronization.

## Features

- 📋 **Task Management**: Create, update, complete, and delete tasks with rich metadata
- 🗂️ **Project Organization**: Organize tasks into projects with custom colors and descriptions
- ⏰ **Reminders**: Set reminders for due tasks with push notifications
- 🔄 **Real-time Sync**: P2P synchronization using CRDTs for conflict-free collaboration
- 📱 **Cross-platform**: Works on desktop, mobile, and web with native notifications
- 🔍 **Search & Filter**: Advanced filtering by status, priority, assignee, and date
- 📊 **Analytics**: Track productivity with completion statistics and trends

## Architecture

This module follows **hexagonal architecture** with clear separation of concerns:

- **Domain**: Core business logic and entities
- **Application**: Use cases and service layer
- **Infrastructure**: Database, P2P, and notification adapters
- **Web**: HTTP API and GraphQL interface

## Quick Start

### Prerequisites

- Rust 1.75+
- PostgreSQL 14+
- Docker (optional, for development)

### Installation

1. Clone the repository:
```bash
git clone <repo-url>
cd apps/task_manager
```

2. Copy environment variables:
```bash
cp .env.example .env
# Edit .env with your database credentials
```

3. Run database migrations:
```bash
sqlx migrate run
```

4. Start the server:
```bash
cargo run
```

### Development Setup with Docker

```bash
# Start PostgreSQL
docker-compose up -d postgres

# Run migrations
sqlx migrate run

# Start development server
cargo run
```

## API Usage

### GraphQL Endpoint

The API is available at `http://localhost:8080/graphql` with a GraphQL playground.

#### Example Queries

```graphql
# Create a task
mutation CreateTask {
  createTask(input: {
    title: "Implement user authentication"
    description: "Add OAuth2 integration"
    dueDate: "2024-08-01T12:00:00Z"
    priority: HIGH
    tags: ["backend", "security"]
  }) {
    id
    title
    status
    createdAt
  }
}

# List tasks with filtering
query ListTasks {
  tasks(
    status: "pending"
    priority: HIGH
    limit: 10
  ) {
    id
    title
    status
    priority
    dueDate
    project {
      name
      color
    }
  }
}

# Subscribe to reminders
subscription ReminderAlerts {
  reminderAlert {
    id
    task {
      title
    }
    message
    remindAt
  }
}
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | - |
| `SERVER_HOST` | Server bind address | `127.0.0.1` |
| `SERVER_PORT` | Server port | `8080` |
| `RUST_LOG` | Logging level | `info` |

### Notification Services

Optional services for push notifications:

- **Firebase Cloud Messaging**: Set `FIREBASE_SERVICE_ACCOUNT_PATH`
- **Apple Push Notification Service**: Configure `APNS_*` variables

### P2P Configuration

- **P2P Bootstrap Nodes**: Set `P2P_BOOTSTRAP_NODES` for discovery

## Testing

Run the test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# With coverage
cargo tarpaulin --out Html
```

## Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m 'Add amazing feature'`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Support

- 📖 [Documentation](https://docs.cpc.dev/task-manager)
- 🐛 [Issue Tracker](https://github.com/cpc-cooperative/cpc/issues)
- 💬 [Discord Community](https://discord.gg/cpc)