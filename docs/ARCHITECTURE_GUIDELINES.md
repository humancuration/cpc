## Offline-First Data Patterns

### Conflict Resolution Implementation
All offline-capable features must implement conflict resolution strategies following:
1. Vector clock or timestamp-based resolution
2. Explicit resolution interface implementation
3. Last-write-wins as minimum viable strategy

Example implementation:
```rust
trait ConflictResolution {
    fn resolve_conflict(&self, local: &DataType, remote: &DataType) -> Resolution;
}
```

### Adapter Selection Strategy
Implement composite adapters that automatically select between online/offline implementations:

```rust
enum UserPreferencesImpl {
    Online(GrpcUserPreferences),
    Offline(SledUserPreferences),
}

impl UserPreferences for UserPreferencesImpl {
    // Delegates to appropriate implementation
}
```

### Sync Queue Requirements
All offline features must:
1. Track sync state in storage model
2. Implement explicit sync queue
3. Provide background sync worker

## Web Application Architecture

### Core Principles
- Use `web_core` shared package for common functionality
- Implement responsive designs using Yew components
- Follow offline-first patterns with local storage fallback
- Implement comprehensive error handling with error boundaries
- Use consistent theming through the design system

### Web-Specific Adapters
```rust
enum StorageImpl {
    Online(IndexedDbStorage),
    Offline(LocalStorage),
}
```

### Component Patterns
All UI components should follow these patterns:
1. Implement the `BaseComponent` trait for consistency
2. Use `CommonProps` for common properties
3. Implement proper error boundaries for error handling
4. Follow the design system for consistent styling

Example implementation:
```rust
use web_core::components::{BaseComponent, CommonProps};

pub struct MyComponent {
    props: MyComponentProps,
}

#[derive(Properties, PartialEq, Clone)]
pub struct MyComponentProps {
    #[prop_or_default]
    pub common: CommonProps,
    // Component-specific props
}

impl BaseComponent for MyComponent {
    type Properties = MyComponentProps;
    
    fn create(props: &Self::Properties) -> Self {
        Self { props: props.clone() }
    }
    
    fn update_props(&mut self, props: Self::Properties) {
        self.props = props;
    }
    
    fn view(&self) -> Html {
        // Component rendering
    }
}
```

### API Client Patterns
Use the `ApiClient` for all API interactions:
1. Leverage request batching for improved performance
2. Implement rate limiting to prevent server overload
3. Use offline caching for resilient applications
4. Implement proper error handling and recovery

Example implementation:
```rust
use web_core::api_client::ApiClient;

let client = ApiClient::new("https://api.example.com".to_string());

// GraphQL query with error handling
match client.graphql_query::<MyDataType>(query, variables).await {
    Ok(response) => {
        if let Some(data) = response.data {
            // Handle data
        }
    }
    Err(error) => {
        // Handle error
    }
}
```