# Allat Database Setup

## Prerequisites

- PostgreSQL 17.5 or later
- SQLx CLI (optional, for manual migration management)

## Database Configuration

The application uses the following environment variables for database configuration:

- `DATABASE_URL` - PostgreSQL connection string
  - Default: `postgresql://localhost/allat_dev`
  - Format: `postgresql://[user[:password]@][netloc][:port][,...][/dbname][?param1=value1&...]`

## Setting up the Database

1. Create a PostgreSQL database:
   ```sql
   CREATE DATABASE allat_dev;
   ```

2. Set the DATABASE_URL environment variable:
   ```bash
   export DATABASE_URL=postgresql://username:password@localhost/allat_dev
   ```

3. Run the application - migrations will be applied automatically:
   ```bash
   cargo run
   ```

## Manual Migration Management

If you need to manage migrations manually, you can use the SQLx CLI:

```bash
# Install SQLx CLI
cargo install sqlx-cli

# Run migrations
cargo sqlx migrate run

# Revert last migration
cargo sqlx migrate revert

# Add a new migration
cargo sqlx migrate add migration_name
```

## Testing

For running tests, you can set up a separate test database:

```sql
CREATE DATABASE allat_test;
```

Then run tests with:
```bash
TEST_DATABASE_URL=postgresql://username:password@localhost/allat_test cargo test