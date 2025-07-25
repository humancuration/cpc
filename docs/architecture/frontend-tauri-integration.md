# Frontend-Tauri Integration Specification

## 1. Service Module Structure

### Core Principles:
- **Vertical slices**: Each domain (auth, social, feed, governance) gets its own service module
- **Hexagonal architecture**: Tauri commands as primary ports, Yew components as adapters
- **Screaming architecture**: Module names and structures clearly reveal their purpose

```rust
// apps/cpc-platform/src-yew/src/services/
├── mod.rs                // Service registry
├── auth.rs               // Authentication service
├── social.rs             // Social interactions service
├── feed.rs               // Feed management service
├── governance.rs         // Governance operations service
└── tauri.rs              // Shared Tauri invocation utils
```

### Service Module Template (auth.rs example):
```rust
use wasm_bindgen::prelude::*;
use crate::types::{User, ApiError};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub struct AuthService;

impl AuthService {
    pub async fn login(email: String, password: String) -> Result<User, ApiError> {
        let args = JsValue::from_serde(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .unwrap();
        
        let response = invoke("login", args).await;
        // Add proper error mapping
    }
    
    // Similar patterns for logout, register
}
```

## 2. GraphQL-like Service Interfaces

### Service Method Signature Pattern:
```rust
pub async fn operation_name(params: Type) -> Result<OutputType, ServiceError>
```

### Example for Social Service:
```rust
pub struct SocialService;

impl SocialService {
    pub async fn create_post(content: String) -> Result<Post, ServiceError> { /* ... */ }
    pub async fn like_post(post_id: String) -> Result<LikeResponse, ServiceError> { /* ... */ }
    pub async fn comment_post(post_id: String, content: String) -> Result<Comment, ServiceError> { /* ... */ }
}
```

## 3. Component Integration Patterns

### ProductHeader Component (apps/cpc-platform/src-yew/src/components/product/ProductHeader.rs)
```rust
use crate::services::auth::AuthService;

pub struct ProductHeader {
    auth_state: UseStateHandle<AuthState>,
}

impl Component for ProductHeader {
    fn create(ctx: &Context<Self>) -> Self {
        let auth_state = ctx.link().use_state(|| AuthState::Unknown);
        
        // Check auth state on component mount
        ctx.link().send_future(async {
            match AuthService::check_session().await {
                Ok(user) => AuthState::Authenticated(user),
                Err(_) => AuthState::Unauthenticated,
            }
        });
        
        Self { auth_state }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        match &*self.auth_state {
            AuthState::Authenticated(user) => self.render_authenticated(user),
            _ => self.render_login_button(ctx),
        }
    }
}

impl ProductHeader {
    fn render_login_button(&self, ctx: &Context<Self>) -> Html {
        let on_login = ctx.link().callback_future(|_| async {
            match AuthService::login("test@example.com", "password").await {
                Ok(user) => Msg::LoginSuccess(user),
                Err(e) => Msg::LoginError(e),
            }
        });
        
        html! {
            <button onclick={on_login}>{ "Login" }</button>
        }
    }
}
```

## 4. State Management Approach

### Authentication State Machine:
```rust
pub enum AuthState {
    Unknown,          // Initial state
    Authenticated(User),
    Unauthenticated,
    Loading,
    Error(ApiError),
}
```

### Feed State Management:
```rust
pub struct FeedState {
    pub posts: Vec<Post>,
    pub loading: bool,
    pub error: Option<ApiError>,
    pub last_updated: Option<DateTime<Utc>>,
}
```

### Governance Voting Status:
```rust
pub struct ProposalState {
    pub proposals: Vec<Proposal>,
    pub user_votes: HashMap<String, VoteDirection>, // proposal_id -> vote
    pub loading: bool,
}
```

## 5. Error Handling Strategy

### Unified Error Type:
```rust
#[derive(Debug, Serialize, Deserialize)]
pub enum ApiError {
    AuthError(String),
    NetworkError(String),
    ValidationError(Vec<String>),
    ServerError(String),
    PermissionDenied,
}
```

### Error Handling in Components:
```rust
fn view_error(error: &ApiError) -> Html {
    match error {
        ApiError::AuthError(msg) => html! { <div class="error">{ "Auth Error: " }{msg}</div> },
        ApiError::NetworkError(_) => html! { <div class="error">{"Network error - please try again"}</div> },
        // ... other cases
    }
}
```

## 6. Subscription Model for Real-time Updates

### Feed Subscription Service:
```rust
pub struct FeedService;

impl FeedService {
    pub async fn subscribe_to_feed() -> Result<impl Stream<Item = FeedEvent>, ApiError> {
        // Setup Tauri event subscription
    }
}

pub enum FeedEvent {
    NewPost(Post),
    UpdatedPost(Post),
    DeletedPost(String),
}
```

### Component Integration:
```rust
ctx.link().send_future(async move {
    let mut stream = FeedService::subscribe_to_feed().await.unwrap();
    while let Some(event) = stream.next().await {
        match event {
            FeedEvent::NewPost(post) => Msg::AddPost(post),
            // ... other events
        }
    }
});
```

## 7. Implementation Roadmap

1. **Phase 1**: Implement auth service and integrate with ProductHeader
2. **Phase 2**: Build social service with create_post/like_post/comment_post
3. **Phase 3**: Implement feed service with real-time subscriptions
4. **Phase 4**: Add governance service with voting capabilities
5. **Phase 5**: Refactor camera service to use shared Tauri invocation pattern

## 8. Error Handling Improvements for Tauri Commands

Update auth command to use proper error types:

```rust
// apps/cpc-platform/src-tauri/src/auth.rs
#[command]
pub async fn login(email: String, password: String) -> Result<User, ApiError> {
    // Validate inputs
    if email.is_empty() || password.is_empty() {
        return Err(ApiError::ValidationError(vec!["Email and password required".into()]));
    }
    
    // Actual auth logic
}