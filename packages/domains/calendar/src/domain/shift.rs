//! Work shift domain model for cooperative scheduling

use chrono::{DateTime, Utc, NaiveTime, Duration, Weekday as ChronoWeekday};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents a work shift in a cooperative
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkShift {
    pub id: Uuid,
    pub cooperative_id: Uuid, // For co-op shift management
    pub position: String,
    pub schedule: ShiftSchedule,
    pub coverage: ShiftCoverage,
    pub created_at: DateTime<Utc>,
}

impl WorkShift {
    /// Create a new work shift
    pub fn new(
        cooperative_id: Uuid,
        position: String,
        schedule: ShiftSchedule,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            cooperative_id,
            position,
            schedule,
            coverage: ShiftCoverage::new(1), // Default to 1 minimum staff
            created_at: Utc::now(),
        }
    }

    /// Assign a staff member to this shift
    pub fn assign_staff(&mut self, user_id: Uuid) {
        if !self.coverage.current_staff.contains(&user_id) {
            self.coverage.current_staff.push(user_id);
            self.coverage.update_status();
        }
    }

    /// Remove a staff member from this shift
    pub fn remove_staff(&mut self, user_id: &Uuid) {
        self.coverage.current_staff.retain(|id| id != user_id);
        self.coverage.update_status();
    }

    /// Check if this shift is fully covered
    pub fn is_fully_covered(&self) -> bool {
        matches!(self.coverage.coverage_status, CoverageStatus::FullyCovered)
    }
}

/// Schedule for a work shift
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShiftSchedule {
    Fixed { 
        start_time: NaiveTime, 
        duration: Duration 
    },
    Rotating { 
        pattern: RotationPattern,
        start_date: chrono::NaiveDate,
        rotation_period: Duration,
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use uuid::Uuid;

    #[test]
    fn test_create_work_shift() {
        let cooperative_id = Uuid::new_v4();
        let schedule = ShiftSchedule::Fixed {
            start_time: chrono::NaiveTime::from_hms(9, 0, 0),
            duration: Duration::hours(8),
        };
        
        let shift = WorkShift::new(
            cooperative_id,
            "Cashier".to_string(),
            schedule.clone(),
        );
        
        assert_eq!(shift.cooperative_id, cooperative_id);
        assert_eq!(shift.position, "Cashier");
        assert_eq!(shift.schedule, schedule);
        assert_eq!(shift.coverage.minimum_staff, 1);
        assert!(shift.coverage.current_staff.is_empty());
        assert_eq!(shift.coverage.coverage_status, CoverageStatus::Understaffed);
    }

    #[test]
    fn test_assign_staff() {
        let cooperative_id = Uuid::new_v4();
        let schedule = ShiftSchedule::Fixed {
            start_time: chrono::NaiveTime::from_hms(9, 0, 0),
            duration: Duration::hours(8),
        };
        
        let mut shift = WorkShift::new(
            cooperative_id,
            "Cashier".to_string(),
            schedule,
        );
        
        let staff_id = Uuid::new_v4();
        shift.assign_staff(staff_id);
        
        assert_eq!(shift.coverage.current_staff.len(), 1);
        assert_eq!(shift.coverage.current_staff[0], staff_id);
        assert_eq!(shift.coverage.coverage_status, CoverageStatus::FullyCovered);
    }

    #[test]
    fn test_remove_staff() {
        let cooperative_id = Uuid::new_v4();
        let schedule = ShiftSchedule::Fixed {
            start_time: chrono::NaiveTime::from_hms(9, 0, 0),
            duration: Duration::hours(8),
        };
        
        let mut shift = WorkShift::new(
            cooperative_id,
            "Cashier".to_string(),
            schedule,
        );
        
        let staff_id = Uuid::new_v4();
        shift.assign_staff(staff_id);
        assert_eq!(shift.coverage.current_staff.len(), 1);
        
        shift.remove_staff(&staff_id);
        assert!(shift.coverage.current_staff.is_empty());
        assert_eq!(shift.coverage.coverage_status, CoverageStatus::Understaffed);
    }
}

/// Patterns for rotating shifts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RotationPattern {
    DayOff { 
        days: Vec<ChronoWeekday> 
    },
    WeekOnWeekOff,
    Custom(Vec<ShiftRotation>),
}

/// Custom shift rotation definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftRotation {
    pub day_offset: i64, // Days from start date
    pub staff_members: Vec<Uuid>, // User IDs assigned to this rotation
}

/// Coverage information for a shift
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShiftCoverage {
    pub minimum_staff: u8,
    pub current_staff: Vec<Uuid>, // User IDs scheduled
    pub coverage_status: CoverageStatus,
}

impl ShiftCoverage {
    /// Create new shift coverage with minimum staff requirement
    pub fn new(minimum_staff: u8) -> Self {
        Self {
            minimum_staff,
            current_staff: Vec::new(),
            coverage_status: CoverageStatus::Understaffed,
        }
    }

    /// Update the coverage status based on current staffing
    pub fn update_status(&mut self) {
        self.coverage_status = if self.current_staff.len() < self.minimum_staff as usize {
            CoverageStatus::Understaffed
        } else if self.current_staff.len() == self.minimum_staff as usize {
            CoverageStatus::FullyCovered
        } else {
            CoverageStatus::Overstaffed
        };
    }

    /// Check if the shift is understaffed
    pub fn is_understaffed(&self) -> bool {
        matches!(self.coverage_status, CoverageStatus::Understaffed)
    }
}

/// Status of shift coverage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CoverageStatus {
    Understaffed,
    FullyCovered,
    Overstaffed,
}

/// Staff member information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaffMember {
    pub id: Uuid,
    pub name: String,
    pub availability: Vec<AvailabilitySlot>,
}

/// Availability slot for a staff member
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailabilitySlot {
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub availability_type: AvailabilityType,
}

/// Types of availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AvailabilityType {
    Preferred,
    Available,
    Unavailable,
}