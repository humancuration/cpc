//! Utility functions for formatting recurrence rules
use packages::cpc_core::calendar::domain::event::{RecurrenceRule, RecurrenceFrequency, Weekday};
use chrono::{DateTime, Utc};

/// Format a recurrence rule into a human-readable string
pub fn format_recurrence(rule: &RecurrenceRule) -> String {
    let frequency = format_frequency(&rule.frequency, rule.interval);
    
    let mut result = format!("Every {}", frequency);
    
    if let Some(until) = &rule.until {
        result.push_str(&format!(" until {}", format_date(until)));
    } else if let Some(count) = rule.count {
        result.push_str(&format!(" for {} occurrences", count));
    }
    
    if let Some(by_day) = &rule.by_day {
        if !by_day.is_empty() {
            result.push_str(" on ");
            result.push_str(&format_weekdays(by_day));
        }
    }
    
    result
}

/// Format the frequency with interval
fn format_frequency(frequency: &RecurrenceFrequency, interval: u32) -> String {
    let unit = match frequency {
        RecurrenceFrequency::Daily => "day",
        RecurrenceFrequency::Weekly => "week",
        RecurrenceFrequency::Monthly => "month",
        RecurrenceFrequency::Yearly => "year",
    };
    
    if interval == 1 {
        unit.to_string()
    } else {
        format!("{} {}s", interval, unit)
    }
}

/// Format weekdays into a readable string
fn format_weekdays(weekdays: &[Weekday]) -> String {
    weekdays.iter()
        .map(|day| format_weekday(day))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Format a single weekday
fn format_weekday(weekday: &Weekday) -> &'static str {
    match weekday {
        Weekday::Monday => "Monday",
        Weekday::Tuesday => "Tuesday",
        Weekday::Wednesday => "Wednesday",
        Weekday::Thursday => "Thursday",
        Weekday::Friday => "Friday",
        Weekday::Saturday => "Saturday",
        Weekday::Sunday => "Sunday",
    }
}

/// Format a date for display
fn format_date(date: &DateTime<Utc>) -> String {
    format!("{}/{}/{}", date.month(), date.day(), date.year())
}