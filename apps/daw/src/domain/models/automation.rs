use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::cmp::Ordering;

/// Represents the interpolation type between automation points
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum InterpolationType {
    Linear,
    Bezier {
        handle_left: (f32, f32),
        handle_right: (f32, f32),
    },
    Hold,
}

/// Represents a single automation point
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct AutomationPoint {
    /// Position in timeline (samples from project start)
    pub position: u64,
    /// Value at this point (normalized 0.0 to 1.0)
    pub value: f32,
    /// Interpolation type to the next point
    pub interpolation: InterpolationType,
}

impl AutomationPoint {
    /// Create a new automation point
    pub fn new(position: u64, value: f32) -> Self {
        Self {
            position,
            value: value.clamp(0.0, 1.0),
            interpolation: InterpolationType::Linear,
        }
    }

    /// Create a new automation point with specific interpolation
    pub fn with_interpolation(position: u64, value: f32, interpolation: InterpolationType) -> Self {
        Self {
            position,
            value: value.clamp(0.0, 1.0),
            interpolation,
        }
    }

    /// Create a bezier point with control handles
    pub fn bezier(position: u64, value: f32, handle_left: (f32, f32), handle_right: (f32, f32)) -> Self {
        Self {
            position,
            value: value.clamp(0.0, 1.0),
            interpolation: InterpolationType::Bezier { handle_left, handle_right },
        }
    }

    /// Create a hold/step point
    pub fn hold(position: u64, value: f32) -> Self {
        Self {
            position,
            value: value.clamp(0.0, 1.0),
            interpolation: InterpolationType::Hold,
        }
    }
}

/// Represents an automation lane for a parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationLane {
    /// Unique identifier for this automation lane
    pub lane_id: Uuid,
    /// Unique identifier for the parameter
    pub parameter_id: String,
    /// Optional track ID (for track parameters)
    pub track_id: Option<Uuid>,
    /// Optional effect ID (for effect parameters)
    pub effect_id: Option<Uuid>,
    /// Automation points sorted by position
    pub points: Vec<AutomationPoint>,
}

impl AutomationLane {
    /// Create a new automation lane for a parameter
    pub fn new(parameter_id: String) -> Self {
        Self {
            parameter_id,
            track_id: None,
            effect_id: None,
            points: Vec::new(),
        }
    }

    /// Create a new track parameter automation lane
    pub fn for_track(parameter_id: String, track_id: Uuid) -> Self {
        Self {
            parameter_id,
            track_id: Some(track_id),
            effect_id: None,
            points: Vec::new(),
        }
    }

    /// Create a new effect parameter automation lane
    pub fn for_effect(parameter_id: String, effect_id: Uuid) -> Self {
        Self {
            parameter_id,
            track_id: None,
            effect_id: Some(effect_id),
            points: Vec::new(),
        }
    }

    /// Add an automation point
    pub fn add_point(&mut self, point: AutomationPoint) -> Result<(), String> {
        // Check if point already exists at this position
        if let Some(pos) = self.points.iter().position(|p| p.position == point.position) {
            self.points[pos] = point;
        } else {
            self.points.push(point);
            self.points.sort_by_key(|p| p.position);
        }
        Ok(())
    }

    /// Remove an automation point at a specific position
    pub fn remove_point(&mut self, position: u64) -> bool {
        if let Some(pos) = self.points.iter().position(|p| p.position == position) {
            self.points.remove(pos);
            true
        } else {
            false
        }
    }

    /// Move an automation point to a new position
    pub fn move_point(&mut self, old_position: u64, new_position: u64, new_value: Option<f32>) -> Result<(), String> {
        if let Some(pos) = self.points.iter().position(|p| p.position == old_position) {
            let mut point = self.points[pos];
            point.position = new_position;
            if let Some(val) = new_value {
                point.value = val.clamp(0.0, 1.0);
            }
            
            self.points.remove(pos);
            self.add_point(point)?;
            Ok(())
        } else {
            Err("Point not found".to_string())
        }
    }

    /// Get the value at a specific time using interpolation
    pub fn evaluate_at(&self, time: u64) -> f32 {
        if self.points.is_empty() {
            return 0.5; // Default value when no points exist
        }

        // Binary search for the correct segment
        let pos = self.points.binary_search_by(|probe| probe.position.cmp(&time));
        
        match pos {
            Ok(index) => self.points[index].value, // Exact match
            Err(index) => {
                if index == 0 {
                    // Before first point
                    self.points[0].value
                } else if index >= self.points.len() {
                    // After last point
                    self.points.last().unwrap().value
                } else {
                    // Between two points - interpolate
                    let prev = &self.points[index - 1];
                    let next = &self.points[index];
                    self.interpolate(prev, next, time)
                }
            }
        }
    }

    /// Interpolate between two points based on their interpolation type
    fn interpolate(&self, prev: &AutomationPoint, next: &AutomationPoint, time: u64) -> f32 {
        if prev.position == next.position {
            return prev.value;
        }

        let t = (time - prev.position) as f32 / (next.position - prev.position) as f32;
        let t = t.clamp(0.0, 1.0);

        match prev.interpolation {
            InterpolationType::Linear => {
                prev.value + (next.value - prev.value) * t
            }
            InterpolationType::Hold => {
                prev.value
            }
            InterpolationType::Bezier { handle_left, handle_right } => {
                // Cubic Bezier interpolation
                // Using De Casteljau's algorithm for cubic Bezier curves
                let p0 = (0.0, prev.value);
                let p1 = handle_right;
                let p2 = handle_left;
                let p3 = (1.0, next.value);

                // Cubic Bezier formula: B(t) = (1-t)³P0 + 3(1-t)²tP1 + 3(1-t)t²P2 + t³P3
                let mt = 1.0 - t;
                let mt2 = mt * mt;
                let t2 = t * t;
                
                let y = mt2 * mt * p0.1 + 
                       3.0 * mt2 * t * p1.1 + 
                       3.0 * mt * t2 * p2.1 + 
                       t2 * t * p3.1;

                y.clamp(0.0, 1.0)
            }
        }
    }

    /// Get all points within a time range
    pub fn get_points_in_range(&self, start: u64, end: u64) -> Vec<&AutomationPoint> {
        self.points
            .iter()
            .filter(|p| p.position >= start && p.position <= end)
            .collect()
    }

    /// Get the total duration covered by automation points
    pub fn duration(&self) -> u64 {
        if self.points.is_empty() {
            0
        } else {
            self.points.last().unwrap().position
        }
    }

    /// Clear all automation points
    pub fn clear(&mut self) {
        self.points.clear();
    }

    /// Check if the lane has any automation points
    pub fn is_empty(&self) -> bool {
        self.points.is_empty()
    }

    /// Get the number of automation points
    pub fn len(&self) -> usize {
        self.points.len()
    }
}

/// Helper functions for common automation operations
pub mod automation_utils {
    use super::*;
    use std::collections::HashMap;

    /// Map parameter names to IDs
    pub fn track_parameter_id(parameter_name: &str) -> String {
        format!("track:{}", parameter_name)
    }

    pub fn effect_parameter_id(effect_id: Uuid, parameter_name: &str) -> String {
        format!("effect:{}:{}", effect_id, parameter_name)
    }

    /// Scale automation value from normalized to actual range
    pub fn scale_value(normalized: f32, min: f32, max: f32) -> f32 {
        min + normalized * (max - min)
    }

    /// Convert actual value to normalized
    pub fn normalize_value(value: f32, min: f32, max: f32) -> f32 {
        ((value - min) / (max - min)).clamp(0.0, 1.0)
    }

    /// Create common automation curves
    pub fn create_fade_in(duration: u64) -> Vec<AutomationPoint> {
        vec![
            AutomationPoint::new(0, 0.0),
            AutomationPoint::new(duration, 1.0),
        ]
    }

    pub fn create_fade_out(start: u64, duration: u64) -> Vec<AutomationPoint> {
        vec![
            AutomationPoint::new(start, 1.0),
            AutomationPoint::new(start + duration, 0.0),
        ]
    }

    pub fn create_linear_ramp(start: u64, end: u64, from: f32, to: f32) -> Vec<AutomationPoint> {
        vec![
            AutomationPoint::new(start, from),
            AutomationPoint::new(end, to),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_automation_point_creation() {
        let point = AutomationPoint::new(1000, 0.75);
        assert_eq!(point.position, 1000);
        assert_eq!(point.value, 0.75);
        assert_eq!(point.interpolation, InterpolationType::Linear);
    }

    #[test]
    fn test_automation_lane_operations() {
        let mut lane = AutomationLane::new("volume".to_string());
        
        // Add points
        lane.add_point(AutomationPoint::new(0, 0.0)).unwrap();
        lane.add_point(AutomationPoint::new(1000, 1.0)).unwrap();
        
        assert_eq!(lane.len(), 2);
        
        // Evaluate
        assert_eq!(lane.evaluate_at(0), 0.0);
        assert_eq!(lane.evaluate_at(500), 0.5);
        assert_eq!(lane.evaluate_at(1000), 1.0);
        
        // Remove point
        assert!(lane.remove_point(0));
        assert_eq!(lane.len(), 1);
    }

    #[test]
    fn test_interpolation_types() {
        let mut lane = AutomationLane::new("test".to_string());
        
        // Linear interpolation
        lane.add_point(AutomationPoint::new(0, 0.0)).unwrap();
        lane.add_point(AutomationPoint::new(1000, 1.0)).unwrap();
        
        assert_eq!(lane.evaluate_at(250), 0.25);
        
        // Hold interpolation
        lane.points[0].interpolation = InterpolationType::Hold;
        assert_eq!(lane.evaluate_at(250), 0.0);
        
        // Before first point
        assert_eq!(lane.evaluate_at(u64::MAX), 1.0);
        
        // After last point
        assert_eq!(lane.evaluate_at(2000), 1.0);
    }

    #[test]
    fn test_bezier_interpolation() {
        let mut lane = AutomationLane::new("test".to_string());
        
        let point1 = AutomationPoint::bezier(0, 0.0, (0.0, 0.0), (0.3, 0.8));
        let point2 = AutomationPoint::bezier(1000, 1.0, (0.7, 0.2), (1.0, 1.0));
        
        lane.add_point(point1).unwrap();
        lane.add_point(point2).unwrap();
        
        // Test bezier interpolation
        let mid_value = lane.evaluate_at(500);
        assert!(mid_value > 0.4 && mid_value < 0.6); // Should be roughly in the middle
    }

    #[test]
    fn test_automation_utils() {
        assert_eq!(automation_utils::scale_value(0.5, 0.0, 100.0), 50.0);
        assert_eq!(automation_utils::normalize_value(50.0, 0.0, 100.0), 0.5);
        
        let fade_in = automation_utils::create_fade_in(1000);
        assert_eq!(fade_in.len(), 2);
        assert_eq!(fade_in[0].value, 0.0);
        assert_eq!(fade_in[1].value, 1.0);
    }

    #[test]
    fn test_binary_search_performance() {
        let mut lane = AutomationLane::new("test".to_string());
        
        // Add many points
        for i in 0..1000 {
            lane.add_point(AutomationPoint::new(i * 100, i as f32 / 1000.0)).unwrap();
        }
        
        // Test performance of evaluation
        let start = std::time::Instant::now();
        for _ in 0..1000 {
            let _ = lane.evaluate_at(50000);
        }
        let duration = start.elapsed();
        
        // Should be very fast due to binary search
        assert!(duration.as_micros() < 1000); // Less than 1ms for 1000 evaluations
    }
}