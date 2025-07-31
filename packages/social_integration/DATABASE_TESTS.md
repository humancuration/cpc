# Database Integration Tests

This document explains how to run and set up the database integration tests for the social integration repositories.

## Overview

The database integration tests use the `sqlx::test` attribute which automatically sets up a temporary PostgreSQL database for testing. These tests verify that the PostgreSQL repositories work correctly with a real database.

## Prerequisites

1. **PostgreSQL**: A PostgreSQL server must be running and accessible.
2. **Environment Variables**: The tests use standard PostgreSQL environment variables:
   - `DATABASE_URL`: Connection string for the test database

## Running the Tests

To run the database integration tests, use the standard cargo test command:

```bash
cd packages/social_integration
cargo test
```

The `sqlx::test` attribute will automatically:
1. Create a temporary database for testing
2. Run the migrations
3. Execute the test
4. Clean up the database afterward

## Test Structure

The tests are organized in modules next to their corresponding repositories:
- `postgres_unified_post_repository_test.rs` - Tests for unified post repository
- `postgres_user_following_repository_test.rs` - Tests for user following repository
- `postgres_tip_transaction_repository_test.rs` - Tests for tip transaction repository

## What the Tests Cover

### PostgresUnifiedPostRepository Tests
- Save and retrieve posts by ID
- Update existing posts
- Find posts by author
- Find posts by source
- JSON serialization/deserialization for post properties
- Timestamp preservation
- Error handling for non-existent posts
- Enhanced post handling with original_id and content fields

### PostgresUserFollowingRepository Tests
- Follow and unfollow users
- Get following list
- Multiple follows/unfollows
- Idempotent operations
- Order preservation
- Circular relationships

### PostgresTipTransactionRepository Tests
- Record tip transactions
- Currency validation (supports all currencies)
- Multiple transactions
- Edge cases (zero amounts, large amounts)
- Special characters in descriptions
- Multiple users

## Detailed Test Case Descriptions

### Unified Post Repository Tests

1. `test_save_and_find_by_id`: Verifies that posts can be saved and retrieved by ID
2. `test_save_updates_existing_post`: Ensures that existing posts can be updated
3. `test_find_by_author`: Tests finding posts by author ID
4. `test_find_by_source`: Tests finding posts by source application
5. `test_find_nonexistent_post_returns_none`: Verifies correct handling of non-existent posts
6. `test_find_by_author_returns_empty_for_no_posts`: Tests behavior when no posts exist for an author
7. `test_find_by_source_returns_empty_for_no_posts`: Tests behavior when no posts exist for a source
8. `test_json_serialization_deserialization`: Verifies complex JSON properties are handled correctly
9. `test_timestamps_are_preserved`: Ensures timestamps are correctly stored and retrieved

### User Following Repository Tests

1. `test_follow_and_get_following`: Basic follow and retrieve functionality
2. `test_unfollow`: Tests unfollowing functionality
3. `test_follow_multiple_users`: Verifies following multiple users
4. `test_unfollow_one_of_multiple`: Tests unfollowing one of multiple followed users
5. `test_get_following_returns_empty_for_nonexistent_user`: Tests behavior for users with no follows
6. `test_follow_same_user_twice_is_idempotent`: Ensures idempotent follow operations
7. `test_unfollow_nonexistent_relationship`: Tests unfollowing non-existent relationships
8. `test_following_order_is_preserved`: Verifies the order of following is preserved
9. `test_circular_following`: Tests circular following relationships

### Tip Transaction Repository Tests

1. `test_record_transaction`: Basic transaction recording
2. `test_record_transaction_with_different_currencies`: Tests recording transactions with different currencies
3. `test_record_multiple_transactions`: Verifies multiple transactions can be recorded
4. `test_record_transaction_with_zero_amount`: Tests zero amount transactions
5. `test_record_transaction_with_large_amount`: Tests large amount transactions
6. `test_record_transaction_with_special_characters_in_description`: Tests special characters in descriptions
7. `test_record_transaction_for_multiple_users`: Verifies transactions for multiple users
8. `test_record_transaction_with_same_user_multiple_times`: Tests multiple transactions for the same user
9. `test_record_transaction_with_empty_description`: Tests empty description handling
10. `test_record_transaction_with_very_long_transaction_type`: Tests long transaction type strings

## Test Execution Examples

### Running All Tests
```bash
cd packages/social_integration
cargo test
```

### Running Specific Test Modules
```bash
# Run only unified post repository tests
cargo test postgres_unified_post_repository_test

# Run only user following repository tests
cargo test postgres_user_following_repository_test

# Run only tip transaction repository tests
cargo test postgres_tip_transaction_repository_test
```

### Running Specific Tests
```bash
# Run a specific test function
cargo test test_save_and_find_by_id

# Run tests matching a pattern
cargo test test_record_transaction
```

## Troubleshooting

If tests fail with connection errors:
1. Ensure PostgreSQL is running
2. Check that `DATABASE_URL` is set correctly
3. Verify the database user has appropriate permissions

Example `DATABASE_URL`:
```
DATABASE_URL=postgresql://username:password@localhost/database_name
```

### Common Issues

1. **Permission Denied**: Ensure the database user has CREATEDB permissions
2. **Connection Refused**: Verify PostgreSQL is running and accessible
3. **Migration Failures**: Check that all migration files are valid SQL
4. **Test Database Cleanup**: The `sqlx::test` attribute should handle cleanup automatically

## CI/CD Integration

These tests are designed to run in CI/CD pipelines. The `sqlx::test` attribute handles all database setup automatically, making it suitable for automated testing environments.