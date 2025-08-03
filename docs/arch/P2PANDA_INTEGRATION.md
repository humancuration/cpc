# p2panda Integration Architecture

## Trait Interface
```rust
pub trait FederationService {
    async fn share_visualization(
        data: Vec<Review<Product>>, 
        vis_type: &str
    ) -> Result<String, FederationError>;
    
    async fn get_shared_visualization(
        share_id: &str
    ) -> Result<SharedVisualization, FederationError>;
}
```

## Mock Implementation
```rust
pub struct MockFederationService;

impl FederationService for MockFederationService {
    async fn share_visualization(
        data: Vec<Review<Product>>, 
        vis_type: &str
    ) -> Result<String, FederationError> {
        // Return mock share_id
    }
    
    async fn get_shared_visualization(
        share_id: &str
    ) -> Result<SharedVisualization, FederationError> {
        // Return mock visualization data
    }
}
```

## Document Schema
```yaml
SharedVisualization:
  fields:
    id: string
    visualization_type: string
    data: 
      type: array
      items:
        $ref: '#/components/schemas/Review'
    timestamp: datetime
    creator: string
```

## Error Handling
- Add specific p2panda error variants
- Implement retry logic for network errors
- Use timeouts for federation calls