//! Logging utilities for the CPC platform
//!
//! This module provides a wrapper around the `tracing` crate for consistent
//! logging across all CPC applications. It follows the established patterns
//! and provides convenience functions.

use tracing::{event, span, Level, Span};
use std::fmt;

/// Log a message at the trace level
pub fn trace(message: impl fmt::Display) {
    event!(Level::TRACE, "{}", message);
}

/// Log a message at the debug level
pub fn debug(message: impl fmt::Display) {
    event!(Level::DEBUG, "{}", message);
}

/// Log a message at the info level
pub fn info(message: impl fmt::Display) {
    event!(Level::INFO, "{}", message);
}

/// Log a message at the warn level
pub fn warn(message: impl fmt::Display) {
    event!(Level::WARN, "{}", message);
}

/// Log a message at the error level
pub fn error(message: impl fmt::Display) {
    event!(Level::ERROR, "{}", message);
}

/// Create a new span at the trace level
pub fn trace_span(name: &str) -> Span {
    span!(Level::TRACE, "{}", name)
}

/// Create a new span at the debug level
pub fn debug_span(name: &str) -> Span {
    span!(Level::DEBUG, "{}", name)
}

/// Create a new span at the info level
pub fn info_span(name: &str) -> Span {
    span!(Level::INFO, "{}", name)
}

/// Log a message with structured fields at the info level
pub fn info_with_fields(message: &str, fields: &[(&str, &dyn fmt::Debug)]) {
    let mut msg = format!("{}", message);
    for (key, value) in fields {
        msg.push_str(&format!(" {}={:?}", key, value));
    }
    info(msg);
}

/// Log a message with structured fields at the error level
pub fn error_with_fields(message: &str, fields: &[(&str, &dyn fmt::Debug)]) {
    let mut msg = format!("{}", message);
    for (key, value) in fields {
        msg.push_str(&format!(" {}={:?}", key, value));
    }
    error(msg);
}

/// Log an error with context
pub fn log_error_with_context(error: &dyn std::error::Error, context: &str) {
    error!("{}: {}", context, error);
    let mut source = error.source();
    while let Some(e) = source {
        error!("Caused by: {}", e);
        source = e.source();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[test]
    #[traced_test]
    fn test_logging_functions() {
        trace("trace message");
        debug("debug message");
        info("info message");
        warn("warn message");
        error("error message");
        
        // These assertions check that the logs were recorded
        assert!(logs_contain("trace message"));
        assert!(logs_contain("debug message"));
        assert!(logs_contain("info message"));
        assert!(logs_contain("warn message"));
        assert!(logs_contain("error message"));
    }
    
    #[test]
    #[traced_test]
    fn test_spans() {
        let _span = info_span("test_span");
        info("message in span");
        assert!(logs_contain("message in span"));
    }
}