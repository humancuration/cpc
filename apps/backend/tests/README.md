# Media Post Workflow Integration Tests

This directory contains comprehensive integration tests for the media-post workflow in the CPC platform.

## Test Structure

### Backend Tests
- `media_post_integration.rs` - Core integration tests for media service and repository
- `e2e_media_post_workflow.rs` - End-to-end tests covering the full workflow
- `test_utils.rs` - Shared test utilities and helpers

### Frontend Tests
- `apps/cpc-platform/cypress/e2e/media_post_workflow.cy.js` - Cypress E2E tests
- `apps/cpc-platform/cypress/support/commands.js` - Custom Cypress commands
- `apps/cpc-platform/cypress/support/e2e.js` - Cypress configuration

## Running Tests

### Backend Tests
```bash
# Run all backend tests
cargo test --test media_post_integration
cargo test --test e2e_media_post_workflow

# Run specific test
cargo test --test media_post_integration test_happy_path_media_upload_to_post

# Run with test database
TEST_DATABASE_URL=postgres://localhost/cpc_test cargo test
```

### Frontend Tests
```bash
# Install dependencies
cd apps/cpc-platform
npm install

# Run E2E tests
npm run test:e2e

# Open Cypress UI
npm run test:e2e:open

# Run component tests
npm run test:component
```

## Test Scenarios

### Happy Path
- Upload single media file
- Upload multiple media files
- Create post with processed media
- Verify post appears in feed

### Error Handling
- Failed media upload
- Network interruption
- Processing failure
- Retry mechanisms

### Edge Cases
- Large file uploads
- Concurrent uploads
- Cancel processing
- File size limits

### Performance
- Upload time measurement
- Concurrent user testing
- Resource usage monitoring

## Test Data

Test data is automatically cleaned up after each test run. The test database is configured via the `TEST_DATABASE_URL` environment variable.

## Mock Services

- `mock_media_processor.rs` - Mock gRPC media processor service
- Cypress interceptors for GraphQL API mocking

## CI/CD Integration

Tests are designed to run in CI/CD pipelines. Environment variables:
- `TEST_DATABASE_URL` - Test database connection
- `CYPRESS_BASE_URL` - Frontend base URL
- `CYPRESS_API_URL` - Backend API URL