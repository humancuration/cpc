use serde::{Deserialize, Serialize};
use rust_decimal::Decimal;
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    pub min: Option<Decimal>,
    pub max: Option<Decimal>,
}

impl SalaryRange {
    pub fn new(min: Option<Decimal>, max: Option<Decimal>) -> Result<Self, String> {
        if let (Some(min_val), Some(max_val)) = (min, max) {
            if min_val > max_val {
                return Err("Minimum salary cannot be greater than maximum salary".to_string());
            }
        }
        Ok(SalaryRange { min, max })
    }
    
    pub fn contains(&self, amount: Decimal) -> bool {
        match (self.min, self.max) {
            (Some(min), Some(max)) => amount >= min && amount <= max,
            (Some(min), None) => amount >= min,
            (None, Some(max)) => amount <= max,
            (None, None) => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmploymentType {
    #[serde(rename = "full_time")]
    FullTime,
    #[serde(rename = "part_time")]
    PartTime,
    #[serde(rename = "contract")]
    Contract,
    #[serde(rename = "internship")]
    Internship,
}

impl EmploymentType {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "full_time" => Ok(EmploymentType::FullTime),
            "part_time" => Ok(EmploymentType::PartTime),
            "contract" => Ok(EmploymentType::Contract),
            "internship" => Ok(EmploymentType::Internship),
            _ => Err(format!("Invalid employment type: {}", s)),
        }
    }
}

impl fmt::Display for EmploymentType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EmploymentType::FullTime => write!(f, "Full Time"),
            EmploymentType::PartTime => write!(f, "Part Time"),
            EmploymentType::Contract => write!(f, "Contract"),
            EmploymentType::Internship => write!(f, "Internship"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApplicationStatus {
    #[serde(rename = "applied")]
    Applied,
    #[serde(rename = "reviewed")]
    Reviewed,
    #[serde(rename = "interviewing")]
    Interviewing,
    #[serde(rename = "offered")]
    Offered,
    #[serde(rename = "hired")]
    Hired,
    #[serde(rename = "rejected")]
    Rejected,
}

impl ApplicationStatus {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "applied" => Ok(ApplicationStatus::Applied),
            "reviewed" => Ok(ApplicationStatus::Reviewed),
            "interviewing" => Ok(ApplicationStatus::Interviewing),
            "offered" => Ok(ApplicationStatus::Offered),
            "hired" => Ok(ApplicationStatus::Hired),
            "rejected" => Ok(ApplicationStatus::Rejected),
            _ => Err(format!("Invalid application status: {}", s)),
        }
    }
}

impl fmt::Display for ApplicationStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplicationStatus::Applied => write!(f, "Applied"),
            ApplicationStatus::Reviewed => write!(f, "Reviewed"),
            ApplicationStatus::Interviewing => write!(f, "Interviewing"),
            ApplicationStatus::Offered => write!(f, "Offered"),
            ApplicationStatus::Hired => write!(f, "Hired"),
            ApplicationStatus::Rejected => write!(f, "Rejected"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    
    #[test]
    fn test_salary_range_creation() {
        // Test valid range
        let range = SalaryRange::new(
            Some(Decimal::new(50000, 0)),
            Some(Decimal::new(80000, 0)),
        );
        assert!(range.is_ok());
        
        let range = range.unwrap();
        assert_eq!(range.min, Some(Decimal::new(50000, 0)));
        assert_eq!(range.max, Some(Decimal::new(80000, 0)));
    }
    
    #[test]
    fn test_salary_range_invalid() {
        // Test invalid range (min > max)
        let range = SalaryRange::new(
            Some(Decimal::new(80000, 0)),
            Some(Decimal::new(50000, 0)),
        );
        assert!(range.is_err());
    }
    
    #[test]
    fn test_salary_range_contains() {
        let range = SalaryRange::new(
            Some(Decimal::new(50000, 0)),
            Some(Decimal::new(80000, 0)),
        ).unwrap();
        
        // Test values within range
        assert!(range.contains(Decimal::new(60000, 0)));
        assert!(range.contains(Decimal::new(50000, 0)));
        assert!(range.contains(Decimal::new(80000, 0)));
        
        // Test values outside range
        assert!(!range.contains(Decimal::new(40000, 0)));
        assert!(!range.contains(Decimal::new(90000, 0)));
    }
    
    #[test]
    fn test_employment_type_from_str() {
        assert_eq!(EmploymentType::from_str("full_time").unwrap(), EmploymentType::FullTime);
        assert_eq!(EmploymentType::from_str("part_time").unwrap(), EmploymentType::PartTime);
        assert_eq!(EmploymentType::from_str("contract").unwrap(), EmploymentType::Contract);
        assert_eq!(EmploymentType::from_str("internship").unwrap(), EmploymentType::Internship);
        
        // Test invalid type
        assert!(EmploymentType::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_employment_type_display() {
        assert_eq!(format!("{}", EmploymentType::FullTime), "Full Time");
        assert_eq!(format!("{}", EmploymentType::PartTime), "Part Time");
        assert_eq!(format!("{}", EmploymentType::Contract), "Contract");
        assert_eq!(format!("{}", EmploymentType::Internship), "Internship");
    }
    
    #[test]
    fn test_application_status_from_str() {
        assert_eq!(ApplicationStatus::from_str("applied").unwrap(), ApplicationStatus::Applied);
        assert_eq!(ApplicationStatus::from_str("reviewed").unwrap(), ApplicationStatus::Reviewed);
        assert_eq!(ApplicationStatus::from_str("interviewing").unwrap(), ApplicationStatus::Interviewing);
        assert_eq!(ApplicationStatus::from_str("offered").unwrap(), ApplicationStatus::Offered);
        assert_eq!(ApplicationStatus::from_str("hired").unwrap(), ApplicationStatus::Hired);
        assert_eq!(ApplicationStatus::from_str("rejected").unwrap(), ApplicationStatus::Rejected);
        
        // Test invalid status
        assert!(ApplicationStatus::from_str("invalid").is_err());
    }
    
    #[test]
    fn test_application_status_display() {
        assert_eq!(format!("{}", ApplicationStatus::Applied), "Applied");
        assert_eq!(format!("{}", ApplicationStatus::Reviewed), "Reviewed");
        assert_eq!(format!("{}", ApplicationStatus::Interviewing), "Interviewing");
        assert_eq!(format!("{}", ApplicationStatus::Offered), "Offered");
        assert_eq!(format!("{}", ApplicationStatus::Hired), "Hired");
        assert_eq!(format!("{}", ApplicationStatus::Rejected), "Rejected");
    }
}