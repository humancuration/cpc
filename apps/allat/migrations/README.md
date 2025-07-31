# Allat Database Migrations

This directory contains the SQL migration files for the Allat application.

## Running Migrations

To run migrations manually, you can use the SQLx CLI:

```bash
# Install SQLx CLI if you haven't already
cargo install sqlx-cli

# Run migrations
DATABASE_URL=your_database_url cargo sqlx migrate run
```

Or you can run the application which will automatically apply migrations on startup:

```bash
DATABASE_URL=your_database_url cargo run
```

## Migration Files

- `0001_initial_schema.up.sql` - Creates the initial database schema
- `0001_initial_schema.down.sql` - Drops the initial database schema

## Environment Variables

- `DATABASE_URL` - PostgreSQL connection string (default: `postgresql://localhost/allat_dev`)

Example:
```
DATABASE_URL=postgresql://username:password@localhost/allat_dev