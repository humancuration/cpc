# API Enhancements

## Unified Error Handling
```rust
pub enum ApiError {
    Unauthorized,
    Forbidden,
    NotFound,
    Validation(Vec<String>),
    Internal,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            ApiError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden").into_response(),
            ApiError::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            ApiError::Validation(errors) => (
                StatusCode::BAD_REQUEST,
                Json(json!({ "errors": errors })),
            ).into_response(),
            ApiError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error",
            ).into_response(),
        }
    }
}
```

## Request Validation Patterns
### JSON Validation:
```rust
#[derive(Deserialize, Validate)]
struct CreateProjectRequest {
    #[validate(length(min = 3, max = 50))]
    name: String,
    #[validate(range(min = 1))]
    version: u32,
}

async fn create_project(
    Json(payload): Json<CreateProjectRequest>
) -> Result<Json<Project>, ApiError> {
    payload.validate().map_err(|e| {
        let errors = e.field_errors()
            .values()
            .flat_map(|errs| errs.iter().map(|e| e.to_string()))
            .collect();
        ApiError::Validation(errors)
    })?;
    
    // ... implementation
}
```

### Path/Query Validation:
```rust
async fn get_project(
    Path(project_id): Path<Uuid>,
    Query(version): Query<Option<u32>>
) -> Result<Json<Project>, ApiError> {
    if !is_valid_uuid(&project_id) {
        return Err(ApiError::Validation(vec!["Invalid project ID".into()]));
    }
    // ... implementation
}
```

## OpenAPI Documentation
### Strategy:
1. Use `utoipa` crate for annotation-based documentation
2. Generate OpenAPI 3.0 spec at build time
3. Serve documentation at `/api-docs`
4. Integrate with Axum routes

### Example:
```rust
#[derive(OpenApi)]
#[openapi(
    paths(
        get_project,
        create_project
    ),
    components(
        schemas(Project, CreateProjectRequest)
    ),
    tags(
        (name = "projects", description = "Project management")
    )
)]
struct ApiDoc;

// Serve OpenAPI spec
Router::new().route("/api-docs", get(serve_openapi))

async fn serve_openapi() -> Json<utoipa::openapi::OpenApi> {
    Json(ApiDoc::openapi())
}
```

## Route Protection Matrix
| Route | Method | Auth Required | Permissions |
|-------|--------|---------------|-------------|
| /health | GET | No | None |
| /api/update/check | POST | No | None |
| /graphql | POST | Optional | Varies by query |
| /publish | POST | Yes | PublishContent |
| /projects | POST | Yes | CreateProject |
| /projects/{id} | GET | Optional | None (public) |
| /projects/{id} | PUT | Yes | EditProject |
| /projects/{id} | DELETE | Yes | DeleteProject |
| /auth/refresh | POST | Refresh token | None |

## Security Headers
```rust
use tower_http::set_header::SetResponseHeaderLayer;

// Add to Axum router
.layer(SetResponseHeaderLayer::overriding(
    CONTENT_SECURITY_POLICY,
    HeaderValue::from_static("default-src 'self'"),
))
.layer(SetResponseHeaderLayer::overriding(
    STRICT_TRANSPORT_SECURITY,
    HeaderValue::from_static("max-age=31536000; includeSubDomains"),
))
```

## Audit Logging
```rust
#[derive(Serialize)]
struct AuditLog {
    timestamp: DateTime<Utc>,
    user_id: Option<Uuid>,
    method: String,
    path: String,
    status: u16,
    duration: u128,
}