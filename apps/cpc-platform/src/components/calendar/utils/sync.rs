//! Conflict resolution helpers for calendar events
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Import domain models
use packages::cpc_core::calendar::domain::event::CalendarEvent;

/// Options for conflict resolution
#[derive(Debug, Clone, PartialEq)]
pub enum ConflictResolutionOption {
    UseTheirs,
    UseOurs,
    MergeManually,
    SaveAsNew,
}

/// Resolve a conflict between two calendar events
pub fn resolve_conflict(
    original_event: &CalendarEvent,
    current_event: &CalendarEvent,
    server_event: &CalendarEvent,
    resolution: ConflictResolutionOption,
) -> CalendarEvent {
    match resolution {
        ConflictResolutionOption::UseTheirs => server_event.clone(),
        ConflictResolutionOption::UseOurs => current_event.clone(),
        ConflictResolutionOption::MergeManually => {
            // In a real implementation, this would open a UI for manual merging
            // For now, we'll default to using the server version
            server_event.clone()
        },
        ConflictResolutionOption::SaveAsNew => {
            let mut new_event = current_event.clone();
            // Generate a new ID for the event
            // Note: In a real implementation, this would use a proper ID generation mechanism
            new_event.id = Uuid::new_v4();
            new_event
        },
    }
}

/// Generate a visual diff between two calendar events
pub fn generate_visual_diff(
    original_event: &CalendarEvent,
    modified_event: &CalendarEvent,
) -> Vec<String> {
    let mut differences = Vec::new();
    
    if original_event.title != modified_event.title {
        differences.push(format!("Title: '{}' -> '{}'", original_event.title, modified_event.title));
    }
    
    if original_event.description != modified_event.description {
        let original_desc = original_event.description.as_deref().unwrap_or("");
        let modified_desc = modified_event.description.as_deref().unwrap_or("");
        differences.push(format!("Description: '{}' -> '{}'", original_desc, modified_desc));
    }
    
    if original_event.start != modified_event.start {
        differences.push(format!(
            "Start time: {} -> {}", 
            original_event.start.to_rfc3339(), 
            modified_event.start.to_rfc3339()
        ));
    }
    
    if original_event.end != modified_event.end {
        differences.push(format!(
            "End time: {} -> {}", 
            original_event.end.to_rfc3339(), 
            modified_event.end.to_rfc3339()
        ));
    }
    
    if original_event.event_type != modified_event.event_type {
        differences.push(format!(
            "Event type: {:?} -> {:?}", 
            original_event.event_type, 
            modified_event.event_type
        ));
    }
    
    if original_event.visibility != modified_event.visibility {
        differences.push(format!(
            "Visibility: {:?} -> {:?}", 
            original_event.visibility, 
            modified_event.visibility
        ));
    }
    
    // Check if recurrence changed
    match (&original_event.recurrence, &modified_event.recurrence) {
        (None, Some(_)) => differences.push("Recurrence: None -> Added".to_string()),
        (Some(_), None) => differences.push("Recurrence: Removed".to_string()),
        (Some(orig), Some(modified)) if orig != modified => {
            differences.push("Recurrence: Modified".to_string())
        },
        _ => (),
    }
    
    // Check if location changed
    match (&original_event.location, &modified_event.location) {
        (None, Some(_)) => differences.push("Location: None -> Added".to_string()),
        (Some(_), None) => differences.push("Location: Removed".to_string()),
        (Some(orig), Some(modified)) if orig != modified => {
            differences.push("Location: Modified".to_string())
        },
        _ => (),
    }
    
    differences
}