//! DateTime utilities for the CPC platform
//!
//! This module provides common datetime functions and formatting utilities.

use chrono::{DateTime, Utc, NaiveDateTime, Duration, TimeZone};
use crate::error::{CommonError, Result};

/// Format a DateTime for display
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
}

/// Format a DateTime for ISO 8601 format
pub fn format_iso8601(dt: &DateTime<Utc>) -> String {
    dt.to_rfc3339()
}

/// Parse an ISO 8601 datetime string
pub fn parse_iso8601(s: &str) -> Result<DateTime<Utc>> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(CommonError::from)
}

/// Get the current UTC time
pub fn now_utc() -> DateTime<Utc> {
    Utc::now()
}

/// Add duration to a datetime
pub fn add_duration(dt: &DateTime<Utc>, duration: Duration) -> DateTime<Utc> {
    *dt + duration
}

/// Subtract duration from a datetime
pub fn subtract_duration(dt: &DateTime<Utc>, duration: Duration) -> DateTime<Utc> {
    *dt - duration
}

/// Calculate the duration between two datetimes
pub fn duration_between(start: &DateTime<Utc>, end: &DateTime<Utc>) -> Duration {
    *end - *start
}

/// Check if a datetime is in the past
pub fn is_past(dt: &DateTime<Utc>) -> bool {
    *dt < now_utc()
}

/// Check if a datetime is in the future
pub fn is_future(dt: &DateTime<Utc>) -> bool {
    *dt > now_utc()
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_format_datetime() {
        let dt = Utc.with_ymd_and_hms(2023, 10, 15, 14, 30, 0).unwrap();
        let formatted = format_datetime(&dt);
        assert_eq!(formatted, "2023-10-15 14:30:00 UTC");
    }
    
    #[test]
    fn test_format_iso8601() {
        let dt = Utc.with_ymd_and_hms(2023, 10, 15, 14, 30, 0).unwrap();
        let formatted = format_iso8601(&dt);
        assert!(formatted.starts_with("2023-10-15T14:30:00"));
    }
    
    #[test]
    fn test_parse_iso8601() {
        let iso_string = "2023-10-15T14:30:00Z";
        let dt = parse_iso8601(iso_string).unwrap();
        assert_eq!(dt.year(), 2023);
        assert_eq!(dt.month(), 10);
        assert_eq!(dt.day(), 15);
        assert_eq!(dt.hour(), 14);
        assert_eq!(dt.minute(), 30);
    }
    
    #[test]
    fn test_parse_invalid_iso8601() {
        let invalid_string = "invalid datetime";
        let result = parse_iso8601(invalid_string);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_add_subtract_duration() {
        let dt = Utc.with_ymd_and_hms(2023, 10, 15, 14, 30, 0).unwrap();
        let duration = Duration::hours(2);
        
        let later = add_duration(&dt, duration);
        assert_eq!(later.hour(), 16);
        
        let earlier = subtract_duration(&dt, duration);
        assert_eq!(earlier.hour(), 12);
    }
    
    #[test]
    fn test_duration_between() {
        let start = Utc.with_ymd_and_hms(2023, 10, 15, 14, 30, 0).unwrap();
        let end = Utc.with_ymd_and_hms(2023, 10, 15, 16, 30, 0).unwrap();
        let duration = duration_between(&start, &end);
        assert_eq!(duration.num_hours(), 2);
    }
}