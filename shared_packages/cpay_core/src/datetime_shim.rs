// DEPRECATED: This file will be removed on 2025-10-01
// Migrate to common_utils equivalents instead
// DEPRECATED: This file has been migrated to use common_utils directly.
// Date: 2025-08-03
// All functionality has been replaced with direct calls to common_utils::datetime
//! DateTime compatibility shim for integrating common_utils
//!
//! This module provides compatibility between the CPay Core's datetime functions
//! and the common_utils::datetime functions for backward compatibility
//! during the migration process.

#[cfg(feature = "common-utils-integration")]
use common_utils::datetime;
#[cfg(feature = "common-utils-integration")]
use common_utils::error::CommonError;

#[cfg(feature = "common-utils-integration")]
use chrono::{DateTime, Utc};

/// Format a DateTime for display
///
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.3.0", note = "Use common_utils::datetime::format_datetime instead")]
pub fn format_datetime(dt: &DateTime<Utc>) -> String {
    datetime::format_datetime(dt)
}

/// Format a DateTime for ISO 8601 format
///
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.3.0", note = "Use common_utils::datetime::format_iso8601 instead")]
pub fn format_iso8601(dt: &DateTime<Utc>) -> String {
    datetime::format_iso8601(dt)
}

/// Parse an ISO 8601 datetime string
///
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.3.0", note = "Use common_utils::datetime::parse_iso8601 instead")]
pub fn parse_iso8601(s: &str) -> Result<DateTime<Utc>, CommonError> {
    datetime::parse_iso8601(s)
}

/// Get the current UTC time
///
/// This function provides backward compatibility by forwarding calls
/// to the common_utils implementation.
#[cfg(feature = "common-utils-integration")]
#[deprecated(since = "0.3.0", note = "Use common_utils::datetime::now_utc instead")]
pub fn now_utc() -> DateTime<Utc> {
    datetime::now_utc()
}

/// Fallback implementations when common-utils-integration feature is disabled
#[cfg(not(feature = "common-utils-integration"))]
mod fallback {
    use chrono::{DateTime, Utc, TimeZone};
    use std::str::FromStr;

    /// Format a DateTime for display
    pub fn format_datetime(dt: &DateTime<Utc>) -> String {
        dt.format("%Y-%m-%d %H:%M:%S UTC").to_string()
    }

    /// Format a DateTime for ISO 8601 format
    pub fn format_iso8601(dt: &DateTime<Utc>) -> String {
        dt.to_rfc3339()
    }

    /// Parse an ISO 8601 datetime string
    pub fn parse_iso8601(s: &str) -> Result<DateTime<Utc>, String> {
        DateTime::parse_from_rfc3339(s)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| e.to_string())
    }

    /// Get the current UTC time
    pub fn now_utc() -> DateTime<Utc> {
        Utc::now()
    }
}

#[cfg(not(feature = "common-utils-integration"))]
pub use fallback::*;

#[cfg(test)]
#[cfg(feature = "common-utils-integration")]
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
    fn test_now_utc() {
        let dt1 = now_utc();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let dt2 = now_utc();
        assert!(dt2 > dt1);
    }
}

#[cfg(test)]
#[cfg(not(feature = "common-utils-integration"))]
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
    fn test_now_utc() {
        let dt1 = now_utc();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let dt2 = now_utc();
        assert!(dt2 > dt1);
    }
}