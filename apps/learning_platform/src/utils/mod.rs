/// Utility functions for the learning platform frontend

/// Format a timestamp for display
pub fn format_timestamp(timestamp: &prost_types::Timestamp) -> String {
    use chrono::{DateTime, Utc, TimeZone};
    
    let dt = Utc.timestamp_opt(timestamp.seconds, timestamp.nanos as u32).single();
    match dt {
        Some(datetime) => datetime.format("%Y-%m-%d").to_string(),
        None => "Invalid Date".to_string(),
    }
}

/// Truncate text to a specified length
pub fn truncate_text(text: &str, max_length: usize) -> String {
    if text.len() <= max_length {
        text.to_string()
    } else {
        format!("{}...", &text[..max_length - 3])
    }
}

/// Convert enrollment status number to string
pub fn enrollment_status_to_string(status: i32) -> &'static str {
    match status {
        0 => "Enrolled",
        1 => "In Progress",
        2 => "Completed",
        3 => "Dropped",
        _ => "Unknown",
    }
}

/// Convert credential type number to string
pub fn credential_type_to_string(cred_type: i32) -> &'static str {
    match cred_type {
        0 => "Certificate",
        1 => "Micro Degree",
        2 => "Degree",
        3 => "Badge",
        _ => "Unknown",
    }
}