# Learning Platform Server Deployment Guide

## Overview

This document provides instructions for deploying the Learning Platform Server to various environments.

## Deployment Options

### 1. Docker Deployment

The recommended deployment method uses Docker and Docker Compose for containerized deployment.

#### Prerequisites

- Docker Engine 20.10+
- Docker Compose 1.29+

#### Deployment Steps

1. **Clone the repository**:
   ```bash
   git clone <repository-url>
   cd apps/learning_platform_server
   ```

2. **Configure environment variables**:
   Update the `.env` file with production values:
   ```bash
   # Database configuration
   DATABASE_URL=postgresql://user:password@db:5432/learning_platform
   
   # Server configuration
   SERVER_ADDR=0.0.0.0:50051
   
   # JWT secret (use a strong secret in production)
   JWT_SECRET=your_production_secret_here
   ```

3. **Build and start services**:
   ```bash
   docker-compose up -d
   ```

4. **Run database migrations**:
   ```bash
   docker-compose exec learning_platform_server cargo run --bin migrate
   ```

5. **Verify deployment**:
   ```bash
   docker-compose logs learning_platform_server
   ```

#### Docker Compose Configuration

The `docker-compose.yml` file defines two services:
- `db`: PostgreSQL database
- `learning_platform_server`: The application server

### 2. Manual Deployment

For environments where Docker is not available, you can deploy manually.

#### Prerequisites

- Rust and Cargo (latest stable version)
- PostgreSQL 12+
- Systemd or similar process manager (optional)

#### Deployment Steps

1. **Install dependencies**:
   ```bash
   # Ubuntu/Debian
   sudo apt update
   sudo apt install postgresql postgresql-contrib build-essential curl
   
   # Install Rust
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Set up PostgreSQL**:
   ```bash
   # Create database and user
   sudo -u postgres psql
   CREATE DATABASE learning_platform;
   CREATE USER learning_user WITH PASSWORD 'your_password';
   GRANT ALL PRIVILEGES ON DATABASE learning_platform TO learning_user;
   \q
   ```

3. **Clone and build the application**:
   ```bash
   git clone <repository-url>
   cd apps/learning_platform_server
   cargo build --release
   ```

4. **Configure environment variables**:
   Create a `.env` file or export environment variables:
   ```bash
   export DATABASE_URL=postgresql://learning_user:your_password@localhost:5432/learning_platform
   export SERVER_ADDR=127.0.0.1:50051
   export JWT_SECRET=your_production_secret_here
   ```

5. **Run database migrations**:
   ```bash
   ./target/release/migrate
   ```

6. **Start the server**:
   ```bash
   ./target/release/learning_platform_server
   ```

7. **Set up as a service (optional)**:
   Create a systemd service file `/etc/systemd/system/learning-platform-server.service`:
   ```ini
   [Unit]
   Description=Learning Platform Server
   After=network.target

   [Service]
   Type simple
   User=learning-user
   WorkingDirectory=/path/to/apps/learning_platform_server
   Environment=DATABASE_URL=postgresql://learning_user:your_password@localhost:5432/learning_platform
   Environment=SERVER_ADDR=127.0.0.1:50051
   Environment=JWT_SECRET=your_production_secret_here
   ExecStart=/path/to/apps/learning_platform_server/target/release/learning_platform_server
   Restart=always

   [Install]
   WantedBy=multi-user.target
   ```

   Enable and start the service:
   ```bash
   sudo systemctl enable learning-platform-server
   sudo systemctl start learning-platform-server
   ```

## Environment Configuration

### Required Environment Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `DATABASE_URL` | PostgreSQL connection string | `postgresql://user:pass@host:5432/db` |
| `SERVER_ADDR` | Server bind address | `0.0.0.0:50051` |
| `JWT_SECRET` | Secret key for JWT signing | `your_secret_key_here` |

### Optional Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Log level | `info` |
| `DATABASE_MAX_CONNECTIONS` | Max database connections | `10` |

## Health Checks

The server provides a health check endpoint via gRPC HealthService.

To check health using grpcurl:
```bash
grpcurl -plaintext localhost:50051 grpc.health.v1.Health/Check
```

## Monitoring and Logging

### Logging

The server uses structured logging with the following levels:
- `ERROR`: Critical errors
- `WARN`: Warning conditions
- `INFO`: General information
- `DEBUG`: Debug information (in development)

### Metrics

Currently, the server does not expose metrics directly. For production deployments, consider:

1. **Using a service mesh** like Istio for automatic metrics collection
2. **Adding Prometheus metrics** to the application
3. **Using application performance monitoring** tools

## Backup and Recovery

### Database Backup

For PostgreSQL, use `pg_dump`:
```bash
pg_dump -h localhost -U learning_user learning_platform > backup.sql
```

### Recovery

Restore using `psql`:
```bash
psql -h localhost -U learning_user learning_platform < backup.sql
```

## Security Considerations

### Network Security

- Use a reverse proxy (nginx, HAProxy) for TLS termination
- Restrict access to the database port
- Use firewall rules to limit access

### Application Security

- Use strong, randomly generated secrets for `JWT_SECRET`
- Keep dependencies up to date
- Regularly scan for vulnerabilities

### Data Security

- Use encrypted connections to the database (TLS)
- Regularly rotate secrets
- Implement proper access controls

## Scaling

### Horizontal Scaling

The application is stateless and can be scaled horizontally by:

1. Running multiple instances behind a load balancer
2. Using a shared database
3. Ensuring sticky sessions if needed for websocket connections (not currently used)

### Database Scaling

For high-load scenarios:

1. Use database connection pooling
2. Implement read replicas for read-heavy workloads
3. Consider database sharding for very large datasets

## Troubleshooting

### Common Issues

1. **Database Connection Failed**:
   - Check if PostgreSQL is running
   - Verify `DATABASE_URL` environment variable
   - Check database credentials and permissions

2. **Port Already in Use**:
   - Change `SERVER_ADDR` to use a different port
   - Check for other processes using the port

3. **Migration Errors**:
   - Ensure database is accessible
   - Check migration file syntax
   - Verify migration order

### Log Analysis

Check logs for error messages:
```bash
# Docker
docker-compose logs learning_platform_server

# Manual deployment
tail -f /var/log/learning-platform-server.log
```

## Rollback Procedure

To rollback to a previous version:

1. **Docker Deployment**:
   ```bash
   docker-compose down
   git checkout <previous-tag-or-commit>
   docker-compose up -d
   ```

2. **Manual Deployment**:
   ```bash
   systemctl stop learning-platform-server
   # Restore previous binary
   systemctl start learning-platform-server
   ```

## Maintenance

### Regular Tasks

1. **Update Dependencies**:
   ```bash
   cargo update
   ```

2. **Backup Database**:
   Regular database backups as per your backup policy

3. **Monitor Logs**:
   Regular log monitoring for errors and anomalies

### Updates

To update to a new version:

1. **Docker Deployment**:
   ```bash
   docker-compose down
   git pull
   docker-compose up -d
   docker-compose exec learning_platform_server cargo run --bin migrate
   ```

2. **Manual Deployment**:
   ```bash
   systemctl stop learning-platform-server
   git pull
   cargo build --release
   ./target/release/migrate
   systemctl start learning-platform-server
   ```

## Disaster Recovery

### Recovery Plan

1. **Restore Database from Backup**:
   ```bash
   psql -h localhost -U learning_user learning_platform < backup.sql
   ```

2. **Deploy Latest Application Code**:
   Follow deployment steps above

3. **Run Migrations**:
   ```bash
   cargo run --bin migrate
   ```

4. **Start Services**:
   ```bash
   docker-compose up -d
   # or
   systemctl start learning-platform-server
   ```

## Contact

For deployment issues, contact the development team at [team-email].