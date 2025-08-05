# Collaboration Tests Documentation

## Overview

This document describes the testing strategy for collaboration features in CPC applications, including unit tests, integration tests, and stress tests.

## Test Categories

### 1. Unit Tests
Unit tests focus on individual components:
- CRDT operations
- Document service methods
- Signaling message handling
- Conflict detection algorithms

### 2. Integration Tests
Integration tests verify interactions between components:
- Document editing workflows
- Conflict resolution scenarios
- Presence tracking
- Access control

### 3. Stress Tests
Stress tests evaluate system performance under load:
- Concurrent document updates
- High user count scenarios
- Network latency simulation
- Resource utilization

## Unit Tests

### CRDT Operations
```rust
#[test]
fn test_crdt_document_creation() {
    let doc = CrdtDocument::new("actor1".to_string());
    assert!(doc.get_heads().is_empty());
}

#[test]
fn test_put_and_get() {
    let mut doc = CrdtDocument::new("actor1".to_string());
    let root = doc.doc.get_object_root();
    
    doc.put(&root, "key1", serde_json::Value::String("value1".to_string())).unwrap();
    
    let value = doc.get(&root, "key1").unwrap();
    assert_eq!(value, Some(serde_json::Value::String("value1".to_string())));
}
```

### Document Service
```rust
#[tokio::test]
async fn test_document_creation() -> Result<(), Box<dyn std::error::Error>> {
    let provider = Arc::new(MockDocProvider::new());
    let access_checker = DocumentAccessChecker::new(None);
    let service = CollaborativeDocService::new(provider, access_checker);
    
    let owner_id = Uuid::new_v4();
    let content = DocumentContent {
        data: json!({"text": "Hello, world!"}),
        format: "json".to_string(),
    };
    
    let metadata = service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;
    
    assert_eq!(metadata.title, "Test Document");
    assert_eq!(metadata.owner_id, owner_id);
    
    Ok(())
}
```

## Integration Tests

### Concurrent Document Updates
```rust
#[tokio::test]
async fn test_concurrent_document_updates() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize service
    let provider = Arc::new(PostgresDocStore::new("postgresql://user:pass@localhost/test_db").await?);
    let access_checker = DocumentAccessChecker::new(None);
    let service = CollaborativeDocService::new(provider, access_checker);
    
    // Create a document
    let owner_id = Uuid::new_v4();
    let content = DocumentContent {
        data: json!({"text": "Initial content"}),
        format: "json".to_string(),
    };
    
    let metadata = service.create_document(
        owner_id,
        "Test Document".to_string(),
        content,
        vec!["test".to_string()],
    ).await?;
    
    // Simulate concurrent updates from different users
    let document_id = metadata.id;
    let user1_id = Uuid::new_v4();
    let user2_id = Uuid::new_v4();
    
    let content1 = DocumentContent {
        data: json!({"text": "Updated by user 1"}),
        format: "json".to_string(),
    };
    
    let content2 = DocumentContent {
        data: json!({"text": "Updated by user 2"}),
        format: "json".to_string(),
    };
    
    // Perform concurrent updates
    let service_clone1 = service.clone();
    let service_clone2 = service.clone();
    let doc_id1 = document_id;
    let doc_id2 = document_id;
    
    let update1 = tokio::spawn(async move {
        service_clone1.update_document(doc_id1, user1_id, content1).await
    });
    
    let update2 = tokio::spawn(async move {
        service_clone2.update_document(doc_id2, user2_id, content2).await
    });
    
    let result1 = update1.await??;
    let result2 = update2.await??;
    
    // Both updates should succeed
    assert!(result1.version > 0);
    assert!(result2.version > 0);
    
    Ok(())
}
```

### CRDT Document Merge
```rust
#[tokio::test]
async fn test_crdt_document_merge() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize service
    let provider = Arc::new(PostgresDocStore::new("postgresql://user:pass@localhost/test_db").await?);
    let access_checker = DocumentAccessChecker::new(None);
    let service = CollaborativeDocService::new(provider, access_checker);
    
    // Create a CRDT document
    let owner_id = Uuid::new_v4();
    let initial_content = json!({"content": "Hello, world!"});
    
    let metadata = service.create_crdt_document(
        owner_id,
        "CRDT Test Document".to_string(),
        initial_content,
        vec!["crdt".to_string(), "test".to_string()],
    ).await?;
    
    // Get the current document content
    let current_content = service.get_document_content(metadata.id, owner_id).await?;
    
    // Create another version with different changes
    let mut crdt_doc = collaborative_docs::CrdtDocument::new(owner_id.to_string());
    let root = crdt_doc.doc.get_object_root();
    crdt_doc.put(&root, "content", json!("Hello, CRDT world!"))
        .map_err(|e| DocumentError::InvalidFormat(e.to_string()))?;
    
    let crdt_data = crdt_doc.save()
        .map_err(|e| DocumentError::SerializationError(e.to_string()))?;
    
    // Merge the documents
    let merged_metadata = service.merge_crdt_document(
        metadata.id,
        owner_id,
        crdt_data,
    ).await?;
    
    // Check that the merge was successful
    assert!(merged_metadata.version > metadata.version);
    
    Ok(())
}
```

## Stress Tests

### Locust Test Script
```python
# locustfile.py
from locust import HttpUser, task, between
import uuid
import json

class CollaborativeEditorUser(HttpUser):
    wait_time = between(1, 5)
    
    def on_start(self):
        # Connect to WebSocket
        self.document_id = str(uuid.uuid4())
        self.user_id = str(uuid.uuid4())
        
        # Join document
        self.client.post("/api/join", json={
            "document_id": self.document_id,
            "user_id": self.user_id
        })
    
    @task(3)
    def edit_document(self):
        # Simulate document editing
        self.client.post("/api/edit", json={
            "document_id": self.document_id,
            "user_id": self.user_id,
            "content": "Some edited content",
            "position": {"line": 0, "column": 10}
        })
    
    @task(1)
    def add_comment(self):
        # Simulate adding a comment
        self.client.post("/api/comment", json={
            "document_id": self.document_id,
            "user_id": self.user_id,
            "content": "This is a comment",
            "position": {"line": 0, "column": 5}
        })
    
    @task(2)
    def move_cursor(self):
        # Simulate cursor movement
        self.client.post("/api/cursor", json={
            "document_id": self.document_id,
            "user_id": self.user_id,
            "position": {"line": 0, "column": 15}
        })
    
    def on_stop(self):
        # Leave document
        self.client.post("/api/leave", json={
            "document_id": self.document_id,
            "user_id": self.user_id
        })
```

### GitHub Actions Workflow
```yaml
name: Stress Tests

on:
  schedule:
    - cron: '0 2 * * *'  # Run daily at 2 AM UTC
  workflow_dispatch:     # Allow manual triggering

jobs:
  stress-test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:13
        env:
          POSTGRES_PASSWORD: postgres
          POSTGRES_DB: test_db
        options: >-
          --health-cmd pg_isready
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 5432:5432
      redis:
        image: redis:6
        options: >-
          --health-cmd "redis-cli ping"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
        ports:
          - 6379:6379

    steps:
    - name: Checkout code
      uses: actions/checkout@v3

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        profile: minimal
        override: true

    - name: Cache Cargo registry
      uses: actions/cache@v3
      with:
        path: ~/.cargo/registry
        key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache Cargo index
      uses: actions/cache@v3
      with:
        path: ~/.cargo/git
        key: ${{ runner.os }}-cargo-index-${{ hashFiles('**/Cargo.lock') }}

    - name: Cache target directory
      uses: actions/cache@v3
      with:
        path: target
        key: ${{ runner.os }}-cargo-build-target-${{ hashFiles('**/Cargo.lock') }}

    - name: Install dependencies
      run: |
        sudo apt-get update
        sudo apt-get install -y postgresql-client

    - name: Run database migrations
      run: |
        # Add database migration commands here if needed
        echo "Running database migrations..."

    - name: Build project
      run: |
        cargo build --release --all-features

    - name: Install Locust
      run: |
        pip install locust

    - name: Run stress tests
      run: |
        # Start the application in background
        cargo run --bin signaling_server &
        SERVER_PID=$!
        
        # Wait for server to start
        sleep 10
        
        # Run Locust stress tests
        locust -f tests/locustfile.py --headless -u 100 -r 10 --run-time 5m --host http://localhost:8080
        
        # Stop the server
        kill $SERVER_PID

    - name: Upload test results
      uses: actions/upload-artifact@v3
      if: always()
      with:
        name: stress-test-results
        path: |
          locust_report.html
          stress_test_logs.txt
```

## Test Execution

### Running Unit Tests
```bash
cargo test
```

### Running Integration Tests
```bash
cargo test --test collaboration_tests
```

### Running Stress Tests Locally
```bash
# Start dependencies
docker-compose up -d postgres redis

# Run stress tests
locust -f tests/locustfile.py --host http://localhost:8080
```

## Test Metrics

### Performance Benchmarks
- Response time under 100ms for 95% of requests
- Support for 1000+ concurrent users
- Document update latency < 200ms
- Presence update latency < 50ms

### Resource Utilization
- CPU usage < 70% under normal load
- Memory usage < 500MB for 100 concurrent users
- Database connections < 50 for 1000 concurrent users
- Network bandwidth < 1MB/s per user

## Test Maintenance

### Updating Tests
1. When adding new features, add corresponding tests
2. Update test data when data models change
3. Review and update test scripts定期
4. Monitor test coverage metrics

### Test Coverage
- Unit test coverage: > 80%
- Integration test coverage: > 70%
- Stress test coverage: > 60% of critical paths

### Continuous Integration
- All tests run on every pull request
- Stress tests run daily
- Performance benchmarks tracked over time
- Test results published to dashboard