# Allat Database Implementation Summary

## Overview

This document summarizes the database implementation for the Allat application, including schema design, migrations, and repository patterns.

## Database Schema

The database schema is defined in `apps/allat/docs/database_schema.md` and includes the following tables:

1. **communities** - Stores forum communities
2. **users** - Stores user accounts
3. **posts** - Stores both posts and comments (comments have parent_id references)
4. **media_assets** - Stores media associated with posts
5. **votes** - Stores upvotes/downvotes on posts

## Migration System

Migrations are implemented using SQLx migration system:

- Migration files are located in `apps/allat/migrations/`
- Up migration: `0001_initial_schema.up.sql` - Creates all tables and indexes
- Down migration: `0001_initial_schema.down.sql` - Drops all tables and indexes
- Migrations are automatically applied when the application starts

## Database Connection

Database connection is configured in `apps/allat/src/main.rs`:

- Uses `DATABASE_URL` environment variable (defaults to `postgresql://localhost/allat_dev`)
- Creates a connection pool using `sqlx::PgPool`
- Automatically runs migrations on startup

## Repository Pattern

Repositories follow the repository pattern for data access:

1. **PgCommunityRepository** - Handles community data operations
2. **PgPostRepository** - Handles post data operations
3. **PgCommentRepository** - Handles comment data operations (uses posts table with parent_id)
4. **PgUserRepository** - Handles user data operations

## Key Implementation Details

### Comments Implementation

Comments are stored in the same `posts` table as regular posts, with the `parent_id` field indicating:
- `NULL` for top-level posts
- Reference to parent post ID for comments
- Reference to parent comment ID for nested replies

### Vote System

Votes are stored in a separate `votes` table with:
- Unique constraint on (user_id, post_id) to prevent duplicate voting
- Vote type field to distinguish between upvotes and downvotes

### Media Assets

Media assets are stored in a separate `media_assets` table with:
- Foreign key reference to the post they belong to
- Support for both images and videos
- Optional thumbnail URLs and alt text

## Testing

Database migrations can be tested with the test suite in `apps/allat/tests/database_migration_test.rs`.

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string
- `TEST_DATABASE_URL` - PostgreSQL connection string for tests

## Manual Migration Management

SQLx CLI can be used for manual migration management:

```bash
# Install SQLx CLI
cargo install sqlx-cli

# Run migrations
cargo sqlx migrate run

# Revert last migration
cargo sqlx migrate revert

# Add a new migration
cargo sqlx migrate add migration_name