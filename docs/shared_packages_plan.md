# Shared Packages Plan for Redis, Diesel, and bb8

## Overview

This document outlines a plan for creating shared packages that utilize Redis, Diesel, and bb8 to enhance our applications. Based on our investigation, we found that Redis is already being used in some packages, but Diesel and bb8 are not currently utilized.

## Current Usage

### Redis (0.32.4)
- Used in `realtime_signaling` package for pub/sub messaging
- Used in `sheets` app for caching (with older version 0.24)

### Diesel (2.2.12)
- Declared in root Cargo.toml but not used in any packages

### bb8 (0.9.0)
- Declared in root Cargo.toml but not used in any packages

## Proposed Shared Packages

### 1. Database Connection Pool Package (`db_pool`)

This package will provide a shared database connection pool using bb8 with support for both PostgreSQL and SQLite.

#### Features:
- Connection pooling for PostgreSQL using `bb8` and `tokio-postgres`
- Connection pooling for SQLite using `bb8` and `rusqlite`
- Configuration management for connection parameters
- Health checks for database connections
- Metrics collection for connection pool usage

#### Dependencies:
- `bb8 = "0.9.0"`
- `tokio-postgres = "0.7"`
- `rusqlite = "0.30"`
- `bb8-rusqlite = "0.2"`

#### Usage in Apps:
- All apps that require database access can use this package instead of managing their own connections
- Provides consistent connection management across the platform
- Reduces resource usage by sharing connection pools

### 2. Database Abstraction Package (`db_abstraction`)

This package will provide a higher-level database abstraction using Diesel ORM.

#### Features:
- Database schema definitions using Diesel
- CRUD operations with type safety
- Query building utilities
- Migration support
- Support for both PostgreSQL and SQLite

#### Dependencies:
- `diesel = { version = "2.2.12", features = ["postgres", "sqlite", "r2d2"] }`
- `diesel_migrations = "2.2.0"`

#### Usage in Apps:
- Apps can define their domain models and let this package handle the database interactions
- Provides type safety and reduces boilerplate code
- Simplifies database operations with a clean API

### 3. Enhanced Redis Package (`redis_utils`)

This package will provide enhanced Redis functionality building on the existing usage.

#### Features:
- Connection management with connection pooling
- Serialization/deserialization utilities
- Distributed locking mechanisms
- Caching utilities with automatic expiration
- Pub/sub utilities with type safety
- Rate limiting implementations
- Session management

#### Dependencies:
- `redis = "0.32.4"`
- `bb8 = "0.9.0"`
- `bb8-redis = "0.15.0"`

#### Usage in Apps:
- All apps that need Redis functionality can use this package
- Provides consistent Redis usage patterns across the platform
- Adds higher-level utilities that simplify common Redis operations

## Implementation Plan

### Phase 1: Create `db_pool` Package
1. Create the package structure
2. Implement connection pooling for PostgreSQL
3. Implement connection pooling for SQLite
4. Add configuration management
5. Add health checks and metrics
6. Write documentation and examples

### Phase 2: Create `db_abstraction` Package
1. Create the package structure
2. Define database schema using Diesel
3. Implement CRUD operations
4. Add query building utilities
5. Implement migration support
6. Write documentation and examples

### Phase 3: Create `redis_utils` Package
1. Create the package structure
2. Implement connection management with pooling
3. Add serialization/deserialization utilities
4. Implement distributed locking
5. Add caching utilities
6. Add pub/sub utilities
7. Implement rate limiting
8. Add session management
9. Write documentation and examples

## Integration Plan

### Apps to Enhance with `db_pool`:
- `finance` - For transaction storage
- `crm` - For customer data storage
- `consent_manager` - For consent records
- `learning_core` - For learning progress tracking

### Apps to Enhance with `db_abstraction`:
- `finance` - For financial data modeling
- `crm` - For customer relationship modeling
- `consent_manager` - For consent data modeling
- `learning_core` - For learning data modeling

### Apps to Enhance with `redis_utils`:
- `sheets` - Upgrade to use the new package instead of direct Redis usage
- `realtime_signaling` - Enhance with additional Redis utilities
- `collaborative_docs` - Add caching and pub/sub enhancements
- `gallery` - Add job queue functionality

## Benefits

1. **Consistency**: Shared packages ensure consistent usage patterns across all apps
2. **Maintainability**: Centralized database and Redis logic makes maintenance easier
3. **Performance**: Connection pooling and caching improve performance
4. **Developer Experience**: Higher-level abstractions reduce boilerplate code
5. **Reliability**: Shared packages can include better error handling and monitoring

## Next Steps

1. Create the `db_pool` package
2. Create the `db_abstraction` package
3. Create the `redis_utils` package
4. Begin integrating these packages into existing apps
5. Update documentation for all shared packages