fn main() {
    // Keep this binary as a simple smoke test for the core crate.
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("CPC_LOG")
                .ok()
                .or_else(|| std::env::var("RUST_LOG").ok())
                .unwrap_or_else(|| "info".to_string()),
        )
        .init();

    println!("CPC Video Editor core smoke test starting...");
    video_editor_core::bootstrap::run_headless();
}