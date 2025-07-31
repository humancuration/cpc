# Allat Testing Guide

## Running Tests

To run the Allat test suite, use the standard Cargo test command:

```bash
cargo test
```

## Database Tests

Some tests require a PostgreSQL database to be available. For these tests, you need to set up a test database:

1. Create a test database in PostgreSQL:
   ```sql
   CREATE DATABASE allat_test;
   ```

2. Set the TEST_DATABASE_URL environment variable:
   ```bash
   export TEST_DATABASE_URL=postgresql://username:password@localhost/allat_test
   ```

3. Run the tests:
   ```bash
   cargo test
   ```

## Test Organization

Tests are organized as follows:

- Unit tests: Located within the source files using `#[cfg(test)]` modules
- Integration tests: Located in the `tests/` directory
- Database migration tests: `tests/database_migration_test.rs`

## Test Categories

### Database Migration Tests

These tests verify that the database migrations work correctly:

- `test_migrations` - Verifies that all migrations can be applied and tables are created correctly

### Repository Tests

Repository tests verify that the data access layer works correctly:

- Community repository tests
- Post repository tests
- Comment repository tests
- User repository tests

## Continuous Integration

For CI environments, make sure to:

1. Have PostgreSQL installed and running
2. Create the test database
3. Set the TEST_DATABASE_URL environment variable
4. Run `cargo test`

## Troubleshooting

If tests are failing due to database connection issues:

1. Verify PostgreSQL is running
2. Check that the database URL is correct
3. Ensure the test database exists
4. Verify that the user has appropriate permissions

If you get migration errors:

1. Make sure you're using a clean test database
2. Check that all migration files are present in the migrations directory
3. Verify that the migration files have correct SQL syntax