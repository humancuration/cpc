#[cfg(test)]
mod tests {
    use learning_platform_server::logging;
    
    #[test]
    fn test_logging_init() {
        // This test just ensures the function can be called without panicking
        // In a real test, we might capture log output, but that's complex
        logging::init_logging();
    }
    
    #[test]
    fn test_log_macros() {
        // These tests just ensure the macros can be called without panicking
        // In a real test, we might capture log output, but that's complex
        logging::init_logging();
        
        log_info!("This is an info message");
        log_warn!("This is a warning message");
        log_error!("This is an error message");
        log_debug!("This is a debug message");
    }
}