# Skill Development Database Setup

## Prerequisites
- PostgreSQL 17.5 installed
- `psql` command-line tool
- Rust and Cargo installed

## Setup Steps

### 1. Create database and user
```bash
sudo -u postgres createuser skill_dev_user
sudo -u postgres createdb skill_dev_db
sudo -u postgres psql -c "ALTER USER skill_dev_user WITH PASSWORD 'secure_password';"
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE skill_dev_db TO skill_dev_user;"
```

### 2. Set environment variable
```bash
export DATABASE_URL=postgres://skill_dev_user:secure_password@localhost/skill_dev_db
```

You can also add this to your `.env` file:
```bash
echo "DATABASE_URL=postgres://skill_dev_user:secure_password@localhost/skill_dev_db" > .env
```

### 3. Run migrations
```bash
cd shared_packages/skill_development
cargo run --bin migrate up
```

### 4. Verify schema
```bash
psql $DATABASE_URL -c "\dt"
```

You should see the following tables:
- `skills` - Stores information about different skills
- `skill_progress` - Tracks user progress for each skill
- `certifications` - Stores user certifications
- `_sqlx_migrations` - Migration history (automatically created)

## Test Database Setup

Create a separate test database:

```bash
sudo -u postgres createdb skill_dev_test_db
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE skill_dev_test_db TO skill_dev_user;"
```

Set test database URL:
```bash
export DATABASE_URL=postgres://skill_dev_user:secure_password@localhost/skill_dev_test_db
```

## Development Workflow

### Running migrations during development
```bash
# Apply all pending migrations
cargo run --bin migrate up

# Check migration status
cargo run --bin migrate status

# Rollback the latest migration (use with caution)
cargo run --bin migrate down
```

### Creating new migrations
1. Create a new SQL file in `migrations/` with the next sequential number:
   ```
   migrations/0002_add_skill_categories.sql
   ```
2. Add your SQL statements
3. Run `cargo run --bin migrate up` to apply

## CI/CD Integration

In your CI pipeline:

```yaml
# Example GitHub Actions workflow
- name: Setup test database
  run: |
    sudo systemctl start postgresql
    sudo -u postgres createdb skill_dev_test_db
    sudo -u postgres psql -c "CREATE USER test_user WITH PASSWORD 'test_pass';"
    sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE skill_dev_test_db TO test_user;"

- name: Run migrations
  run: |
    export DATABASE_URL=postgres://test_user:test_pass@localhost/skill_dev_test_db
    cd shared_packages/skill_development
    cargo run --bin migrate up

- name: Run tests
  run: |
    export DATABASE_URL=postgres://test_user:test_pass@localhost/skill_dev_test_db
    cargo test
```

## Troubleshooting

### Connection issues
- Ensure PostgreSQL is running: `sudo systemctl start postgresql`
- Check connection string format
- Verify user permissions: `\du` in psql

### Migration issues
- Check migration file naming: must be `NNNN_description.sql`
- Ensure SQL syntax is valid
- Check for conflicts with existing schema

### Resetting database
```bash
# Drop and recreate database
sudo -u postgres dropdb skill_dev_db
sudo -u postgres createdb skill_dev_db
sudo -u postgres psql -c "GRANT ALL PRIVILEGES ON DATABASE skill_dev_db TO skill_dev_user;"

# Re-run migrations
cd shared_packages/skill_development
cargo run --bin migrate up
```

## Database Schema

### skills
- `id` (UUID): Primary key
- `name` (VARCHAR): Skill name
- `description` (TEXT): Detailed description
- `created_at` (TIMESTAMP): Creation timestamp

### skill_progress
- `id` (UUID): Primary key
- `skill_id` (UUID): Foreign key to skills
- `user_id` (UUID): User identifier
- `progress` (SMALLINT): Progress percentage (0-100)
- `updated_at` (TIMESTAMP): Last update timestamp

### certifications
- `id` (UUID): Primary key
- `name` (VARCHAR): Certification name
- `issuing_organization` (VARCHAR): Organization that issued the certification
- `issue_date` (DATE): When the certification was issued
- `user_id` (UUID): User identifier