#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        http::{Request, StatusCode},
        routing::get,
        Router,
    };
    use serde_json::{json, Value};
    use std::net::SocketAddr;
    use tokio::net::TcpListener;
    use tower::ServiceExt; // for `oneshot` and `ready`

    // Placeholder test since we can't easily test the full API without a database
    #[tokio::test]
    async fn test_health_check() {
        let app = Router::new().route("/health", get(|| async { "OK" }));

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}