//! Tests for the format_recurrence utility function
#[cfg(test)]
mod tests {
    use chrono::{Utc, Duration};
    
    // Import domain models
    use packages::cpc_core::calendar::domain::event::{RecurrenceRule, RecurrenceFrequency, Weekday};
    
    // Import local modules
    use crate::components::calendar::utils::format_recurrence::format_recurrence;
    
    #[test]
    fn test_format_daily_recurrence() {
        let rule = RecurrenceRule {
            frequency: RecurrenceFrequency::Daily,
            interval: 1,
            by_day: None,
            by_month_day: None,
            until: None,
            count: None,
        };
        
        let result = format_recurrence(&rule);
        assert_eq!(result, "Every day");
    }
    
    #[test]
    fn test_format_weekly_recurrence() {
        let rule = RecurrenceRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: 2,
            by_day: None,
            by_month_day: None,
            until: None,
            count: None,
        };
        
        let result = format_recurrence(&rule);
        assert_eq!(result, "Every 2 weeks");
    }
    
    #[test]
    fn test_format_weekly_recurrence_with_until() {
        let rule = RecurrenceRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            by_day: None,
            by_month_day: None,
            until: Some(Utc::now() + Duration::days(30)),
            count: None,
        };
        
        let result = format_recurrence(&rule);
        // The exact date format may vary, but it should contain "until"
        assert!(result.contains("until"));
    }
    
    #[test]
    fn test_format_weekly_recurrence_with_count() {
        let rule = RecurrenceRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            by_day: None,
            by_month_day: None,
            until: None,
            count: Some(10),
        };
        
        let result = format_recurrence(&rule);
        assert_eq!(result, "Every week for 10 occurrences");
    }
    
    #[test]
    fn test_format_weekly_recurrence_with_weekdays() {
        let rule = RecurrenceRule {
            frequency: RecurrenceFrequency::Weekly,
            interval: 1,
            by_day: Some(vec![Weekday::Monday, Weekday::Wednesday, Weekday::Friday]),
            by_month_day: None,
            until: None,
            count: None,
        };
        
        let result = format_recurrence(&rule);
        assert_eq!(result, "Every week on Monday, Wednesday, Friday");
    }
}