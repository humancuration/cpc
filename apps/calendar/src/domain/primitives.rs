//! Primitive types and validation logic for the calendar domain

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use thiserror::Error;
use uuid::Uuid;

/// Error types for calendar domain operations
#[derive(Debug, Error)]
pub enum CalendarError {
    #[error("Invalid event duration: start time must be before end time")]
    InvalidEventDuration,
    
    #[error("Invalid recurrence rule: {0}")]
    InvalidRecurrence(String),
    
    #[error("Scheduling conflict detected")]
    SchedulingConflict(Vec<SchedulingConflict>),
    
    #[error("Invalid location data")]
    InvalidLocation,
    
    #[error("Invalid participant data")]
    InvalidParticipant,
    
    #[error("Event not found")]
    EventNotFound,
    
    #[error("Unauthorized access to event")]
    Unauthorized,
    
    #[error("Shift coverage requirement not met")]
    InsufficientCoverage,
    
    #[error("Invalid date range")]
    InvalidDateRange,
}

/// Represents a scheduling conflict
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulingConflict {
    pub event_id: Uuid,
    pub conflict_start: DateTime<Utc>,
    pub conflict_end: DateTime<Utc>,
    pub participants: Vec<Uuid>,
}

impl SchedulingConflict {
    pub fn new(
        event_id: Uuid,
        conflict_start: DateTime<Utc>,
        conflict_end: DateTime<Utc>,
        participants: Vec<Uuid>,
    ) -> Self {
        Self {
            event_id,
            conflict_start,
            conflict_end,
            participants,
        }
    }
}

/// Date range for querying events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl DateRange {
    /// Create a new date range
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, CalendarError> {
        if start >= end {
            return Err(CalendarError::InvalidDateRange);
        }
        
        Ok(Self { start, end })
    }

    /// Check if this date range overlaps with another
    pub fn overlaps(&self, other: &DateRange) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Get the duration of this date range
    pub fn duration(&self) -> chrono::Duration {
        self.end.signed_duration_since(self.start)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{Utc, Duration};

    #[test]
    fn test_date_range() {
        let start = Utc::now();
        let end = start + Duration::days(1);
        
        let date_range = DateRange::new(start, end).unwrap();
        assert_eq!(date_range.start, start);
        assert_eq!(date_range.end, end);
        
        // Test invalid date range
        let invalid_range = DateRange::new(end, start);
        assert!(invalid_range.is_err());
    }

    #[test]
    fn test_date_range_overlaps() {
        let start1 = Utc::now();
        let end1 = start1 + Duration::days(1);
        let range1 = DateRange::new(start1, end1).unwrap();
        
        // Overlapping range
        let start2 = start1 + Duration::hours(12);
        let end2 = start2 + Duration::days(1);
        let range2 = DateRange::new(start2, end2).unwrap();
        
        assert!(range1.overlaps(&range2));
        
        // Non-overlapping range
        let start3 = end1 + Duration::days(1);
        let end3 = start3 + Duration::days(1);
        let range3 = DateRange::new(start3, end3).unwrap();
        
        assert!(!range1.overlaps(&range3));
    }

    #[test]
    fn test_time_slot() {
        let start = Utc::now();
        let end = start + Duration::hours(1);
        
        let time_slot = TimeSlot::new(start, end).unwrap();
        assert_eq!(time_slot.start, start);
        assert_eq!(time_slot.end, end);
        
        // Test invalid time slot
        let invalid_slot = TimeSlot::new(end, start);
        assert!(invalid_slot.is_err());
    }

    #[test]
    fn test_geofence() {
        let geofence = Geofence::new(
            "Test Location".to_string(),
            37.7749, // San Francisco latitude
            -122.4194, // San Francisco longitude
            100.0, // 100 meters radius
        ).unwrap();
        
        assert_eq!(geofence.name, "Test Location");
        assert_eq!(geofence.latitude, 37.7749);
        assert_eq!(geofence.longitude, -122.4194);
        assert_eq!(geofence.radius, 100.0);
        
        // Test point within geofence
        assert!(geofence.contains_point(37.7749, -122.4194));
        
        // Test invalid geofence
        let invalid_geofence = Geofence::new(
            "Invalid Location".to_string(),
            100.0, // Invalid latitude
            -122.4194,
            100.0,
        );
        assert!(invalid_geofence.is_err());
    }
}

/// Represents a time slot for availability checking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimeSlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
}

impl TimeSlot {
    /// Create a new time slot
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Result<Self, CalendarError> {
        if start >= end {
            return Err(CalendarError::InvalidEventDuration);
        }
        
        Ok(Self { start, end })
    }

    /// Check if this time slot overlaps with another
    pub fn overlaps(&self, other: &TimeSlot) -> bool {
        self.start < other.end && other.start < self.end
    }

    /// Check if this time slot contains another time slot
    pub fn contains(&self, other: &TimeSlot) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

/// Geofence for location-based reminders
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Geofence {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub radius: f64, // in meters
}

impl Geofence {
    /// Create a new geofence
    pub fn new(name: String, latitude: f64, longitude: f64, radius: f64) -> Result<Self, CalendarError> {
        if radius <= 0.0 {
            return Err(CalendarError::InvalidLocation);
        }
        
        if latitude < -90.0 || latitude > 90.0 {
            return Err(CalendarError::InvalidLocation);
        }
        
        if longitude < -180.0 || longitude > 180.0 {
            return Err(CalendarError::InvalidLocation);
        }
        
        Ok(Self {
            name,
            latitude,
            longitude,
            radius,
        })
    }

    /// Check if a point is within this geofence
    pub fn contains_point(&self, lat: f64, lon: f64) -> bool {
        let distance = self.haversine_distance(lat, lon);
        distance <= self.radius
    }

    /// Calculate the distance between this geofence center and a point using haversine formula
    fn haversine_distance(&self, lat: f64, lon: f64) -> f64 {
        let r = 6371000.0; // Earth radius in meters
        let d_lat = (lat - self.latitude).to_radians();
        let d_lon = (lon - self.longitude).to_radians();
        let a = (d_lat / 2.0).sin().powi(2)
            + self.latitude.to_radians().cos()
            * lat.to_radians().cos()
            * (d_lon / 2.0).sin().powi(2);
        let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
        r * c
    }
}

/// Display implementation for Geofence
impl fmt::Display for Geofence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.name, self.latitude, self.longitude)
    }
}