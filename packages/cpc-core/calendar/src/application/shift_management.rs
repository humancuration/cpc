//! Shift management service for cooperative work scheduling

use crate::domain::{
    WorkShift, ShiftSchedule, RotationPattern, StaffMember, AvailabilitySlot, 
    CoverageStatus, CalendarError
};
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;
use chrono::{Date, Utc, Duration, Weekday as ChronoWeekday};

/// Repository trait for work shifts
#[async_trait]
pub trait ShiftRepository: Send + Sync {
    async fn save(&self, shift: &WorkShift) -> Result<(), CalendarError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<WorkShift>, CalendarError>;
    async fn find_by_cooperative_id(&self, cooperative_id: Uuid) -> Result<Vec<WorkShift>, CalendarError>;
    async fn find_by_date_range(&self, cooperative_id: Uuid, start: Date<Utc>, end: Date<Utc>) -> Result<Vec<WorkShift>, CalendarError>;
    async fn update(&self, shift: &WorkShift) -> Result<(), CalendarError>;
    async fn delete(&self, id: Uuid) -> Result<(), CalendarError>;
}

/// Availability service trait
#[async_trait]
pub trait AvailabilityService: Send + Sync {
    async fn get_staff_availability(
        &self,
        staff_ids: &[Uuid],
        start_date: Date<Utc>,
        end_date: Date<Utc>,
    ) -> Result<Vec<StaffMember>, CalendarError>;
}

/// Shift management service
pub struct ShiftManagementService {
    repository: Arc<dyn ShiftRepository>,
    availability_service: Arc<dyn AvailabilityService>,
}

impl ShiftManagementService {
    /// Create a new shift management service
    pub fn new(
        repository: Arc<dyn ShiftRepository>,
        availability_service: Arc<dyn AvailabilityService>,
    ) -> Self {
        Self {
            repository,
            availability_service,
        }
    }

    /// Generate shift rotation based on schedule pattern
    pub async fn generate_shift_rotation(
        &self,
        cooperative_id: Uuid,
        schedule: &ShiftSchedule,
        staff: &[Uuid], // Just user IDs, we'll fetch full details
        start_date: Date<Utc>,
        end_date: Date<Utc>,
        position: String,
    ) -> Result<Vec<WorkShift>, CalendarError> {
        // Get full staff details with availability
        let staff_members = self.availability_service.get_staff_availability(
            staff,
            start_date,
            end_date,
        ).await?;

        match schedule {
            ShiftSchedule::Rotating { pattern, rotation_period, .. } => {
                Ok(self.apply_rotation_pattern(
                    cooperative_id,
                    pattern,
                    &staff_members,
                    start_date,
                    end_date,
                    *rotation_period,
                    position,
                ))
            },
            ShiftSchedule::Fixed { .. } => {
                // For fixed shifts, we create one shift per staff member
                let mut shifts = Vec::new();
                for staff_member in &staff_members {
                    let shift = WorkShift::new(
                        cooperative_id,
                        position.clone(),
                        schedule.clone(),
                    );
                    shifts.push(shift);
                }
                Ok(shifts)
            }
        }
    }

    /// Apply rotation pattern to generate shifts
    fn apply_rotation_pattern(
        &self,
        cooperative_id: Uuid,
        pattern: &RotationPattern,
        staff: &[StaffMember],
        start_date: Date<Utc>,
        end_date: Date<Utc>,
        rotation_period: Duration,
        position: String,
    ) -> Vec<WorkShift> {
        let mut shifts = Vec::new();
        
        match pattern {
            RotationPattern::DayOff { days } => {
                self.generate_day_off_rotation(
                    &mut shifts,
                    cooperative_id,
                    staff,
                    start_date,
                    end_date,
                    days,
                    position,
                );
            },
            RotationPattern::WeekOnWeekOff => {
                self.generate_week_on_week_off_rotation(
                    &mut shifts,
                    cooperative_id,
                    staff,
                    start_date,
                    end_date,
                    position,
                );
            },
            RotationPattern::Custom(rotations) => {
                self.generate_custom_rotation(
                    &mut shifts,
                    cooperative_id,
                    staff,
                    start_date,
                    end_date,
                    rotations,
                    position,
                );
            },
        }
        
        shifts
    }

    /// Generate day off rotation pattern
    fn generate_day_off_rotation(
        &self,
        shifts: &mut Vec<WorkShift>,
        cooperative_id: Uuid,
        staff: &[StaffMember],
        start_date: Date<Utc>,
        end_date: Date<Utc>,
        days_off: &[ChronoWeekday],
        position: String,
    ) {
        let mut current_date = start_date;
        let mut staff_index = 0;
        
        while current_date <= end_date {
            // Skip days that are designated as days off
            if !days_off.contains(&current_date.weekday()) {
                // Assign staff in round-robin fashion
                let assigned_staff = &staff[staff_index % staff.len()];
                
                // Create a fixed shift for this day
                let schedule = ShiftSchedule::Fixed {
                    start_time: chrono::NaiveTime::from_hms(9, 0, 0), // Default start time
                    duration: Duration::hours(8), // Default 8-hour shift
                };
                
                let mut shift = WorkShift::new(
                    cooperative_id,
                    position.clone(),
                    schedule,
                );
                
                // Assign the staff member
                shift.assign_staff(assigned_staff.id);
                
                shifts.push(shift);
                
                // Move to next staff member for next working day
                staff_index += 1;
            }
            
            current_date = current_date.succ();
        }
    }

    /// Generate week on, week off rotation pattern
    fn generate_week_on_week_off_rotation(
        &self,
        shifts: &mut Vec<WorkShift>,
        cooperative_id: Uuid,
        staff: &[StaffMember],
        start_date: Date<Utc>,
        end_date: Date<Utc>,
        position: String,
    ) {
        let mut current_date = start_date;
        let mut staff_index = 0;
        let mut is_working_week = true;
        
        while current_date <= end_date {
            // If it's a working week, assign shifts for the whole week
            if is_working_week {
                // Assign shifts for each day of the working week (Mon-Fri)
                let week_start = current_date;
                let week_end = week_start + Duration::days(4); // Friday
                
                for day_offset in 0..5 {
                    let day = week_start + Duration::days(day_offset);
                    
                    // Skip weekends
                    if day.weekday() == ChronoWeekday::Sat || day.weekday() == ChronoWeekday::Sun {
                        continue;
                    }
                    
                    // Assign staff in round-robin fashion
                    let assigned_staff = &staff[staff_index % staff.len()];
                    
                    // Create a fixed shift for this day
                    let schedule = ShiftSchedule::Fixed {
                        start_time: chrono::NaiveTime::from_hms(9, 0, 0), // Default start time
                        duration: Duration::hours(8), // Default 8-hour shift
                    };
                    
                    let mut shift = WorkShift::new(
                        cooperative_id,
                        position.clone(),
                        schedule,
                    );
                    
                    // Assign the staff member
                    shift.assign_staff(assigned_staff.id);
                    
                    shifts.push(shift);
                    
                    // Move to next staff member for next day
                    staff_index += 1;
                }
            }
            
            // Move to next week
            current_date = current_date + Duration::weeks(1);
            is_working_week = !is_working_week;
        }
    }

    /// Generate custom rotation pattern
    fn generate_custom_rotation(
        &self,
        shifts: &mut Vec<WorkShift>,
        cooperative_id: Uuid,
        staff: &[StaffMember],
        start_date: Date<Utc>,
        end_date: Date<Utc>,
        rotations: &[crate::domain::ShiftRotation],
        position: String,
    ) {
        for rotation in rotations {
            let rotation_date = start_date + Duration::days(rotation.day_offset);
            
            // Skip if outside the requested date range
            if rotation_date < start_date || rotation_date > end_date {
                continue;
            }
            
            // Create a fixed shift for this rotation
            let schedule = ShiftSchedule::Fixed {
                start_time: chrono::NaiveTime::from_hms(9, 0, 0), // Default start time
                duration: Duration::hours(8), // Default 8-hour shift
            };
            
            let mut shift = WorkShift::new(
                cooperative_id,
                position.clone(),
                schedule,
            );
            
            // Assign all staff members for this rotation
            for user_id in &rotation.staff_members {
                shift.assign_staff(*user_id);
            }
            
            shifts.push(shift);
        }
    }

    /// Request a shift swap between two staff members
    pub async fn request_shift_swap(
        &self,
        shift_id: Uuid,
        requesting_staff_id: Uuid,
        target_staff_id: Uuid,
    ) -> Result<ShiftSwapRequest, CalendarError> {
        let mut shift = self.repository.find_by_id(shift_id).await?
            .ok_or(CalendarError::EventNotFound)?;

        // Verify requesting staff is assigned to this shift
        if !shift.coverage.current_staff.contains(&requesting_staff_id) {
            return Err(CalendarError::InvalidParticipant);
        }

        // Create shift swap request
        let request = ShiftSwapRequest::new(
            shift_id,
            requesting_staff_id,
            target_staff_id,
        );

        // In a real implementation, we would notify the target staff member
        // and wait for their approval before making the change

        Ok(request)
    }

    /// Approve a shift swap request
    pub async fn approve_shift_swap(
        &self,
        request_id: Uuid,
    ) -> Result<(), CalendarError> {
        // In a real implementation, we would:
        // 1. Find the shift swap request
        // 2. Verify the target staff member is approving
        // 3. Update the shift assignments
        // 4. Notify all relevant parties
        // 5. Mark the request as completed

        // For now, we'll just return Ok
        Ok(())
    }

    /// Get work shifts for a cooperative within a date range
    pub async fn list_work_shifts(
        &self,
        cooperative_id: Uuid,
        start_date: Option<Date<Utc>>,
        end_date: Option<Date<Utc>>,
        position: Option<String>,
    ) -> Result<Vec<WorkShift>, CalendarError> {
        let shifts = self.repository.find_by_cooperative_id(cooperative_id).await?;

        let filtered_shifts = shifts.into_iter()
            .filter(|shift| {
                // Filter by date range if provided
                let in_date_range = match (&start_date, &end_date) {
                    (Some(start), Some(end)) => {
                        // For simplicity, we'll check if the shift's creation date is in range
                        // A more sophisticated implementation would check the actual shift dates
                        shift.created_at.date() <= *end && shift.created_at.date() >= *start
                    },
                    (Some(start), None) => shift.created_at.date() >= *start,
                    (None, Some(end)) => shift.created_at.date() <= *end,
                    (None, None) => true,
                };

                // Filter by position if provided
                let correct_position = if let Some(ref filter_position) = position {
                    &shift.position == filter_position
                } else {
                    true
                };

                in_date_range && correct_position
            })
            .collect();

        Ok(filtered_shifts)
    }

    /// Check shift coverage and identify understaffed shifts
    pub async fn check_coverage(
        &self,
        cooperative_id: Uuid,
        date: Date<Utc>,
    ) -> Result<Vec<WorkShift>, CalendarError> {
        let shifts = self.repository.find_by_date_range(
            cooperative_id,
            date,
            date,
        ).await?;

        let understaffed_shifts = shifts.into_iter()
            .filter(|shift| shift.coverage.is_understaffed())
            .collect();

        Ok(understaffed_shifts)
    }
}

/// Represents a shift swap request
#[derive(Debug, Clone)]
pub struct ShiftSwapRequest {
    pub id: Uuid,
    pub shift_id: Uuid,
    pub requesting_staff_id: Uuid,
    pub target_staff_id: Uuid,
    pub status: ShiftSwapStatus,
    pub created_at: chrono::DateTime<Utc>,
}

impl ShiftSwapRequest {
    /// Create a new shift swap request
    pub fn new(
        shift_id: Uuid,
        requesting_staff_id: Uuid,
        target_staff_id: Uuid,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            shift_id,
            requesting_staff_id,
            target_staff_id,
            status: ShiftSwapStatus::Pending,
            created_at: Utc::now(),
        }
    }

    /// Approve the shift swap request
    pub fn approve(&mut self) {
        self.status = ShiftSwapStatus::Approved;
    }

    /// Reject the shift swap request
    pub fn reject(&mut self) {
        self.status = ShiftSwapStatus::Rejected;
    }
}

/// Status of a shift swap request
#[derive(Debug, Clone, PartialEq)]
pub enum ShiftSwapStatus {
    Pending,
    Approved,
    Rejected,
}