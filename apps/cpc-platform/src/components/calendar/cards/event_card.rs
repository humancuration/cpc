//! Event card component for displaying calendar events
use yew::prelude::*;
use chrono::{DateTime, Utc, Datelike, Timelike};
use uuid::Uuid;
use crate::components::calendar::utils::format_recurrence;

// Import the domain models
use packages::cpc_core::calendar::domain::event::{CalendarEvent, EventType, EventVisibility};
use packages::cpc_core::calendar::domain::event::Location;

/// Properties for the EventCard component
#[derive(Properties, PartialEq)]
pub struct EventCardProps {
    pub event: CalendarEvent,
    #[prop_or_default]
    pub is_selected: bool,
    #[prop_or_default]
    pub on_click: Callback<Uuid>,
    #[prop_or_default]
    pub on_edit: Callback<Uuid>,
    #[prop_or_default]
    pub on_delete: Callback<Uuid>,
    #[prop_or(false)]
    pub is_touch_device: bool,
}

/// Event card component for displaying calendar events with all visualization elements
#[function_component(EventCard)]
pub fn event_card(props: &EventCardProps) -> Html {
    let event = &props.event;
    
    // Get privacy indicator
    let privacy_icon = get_privacy_icon(&event.visibility);
    
    // Get event type color
    let event_type_color = get_event_type_color(&event.event_type);
    
    // Check if event is recurring
    let is_recurring = event.is_recurring();
    
    // Format time range
    let time_range = format_time_range(&event.start, &event.end);
    
    // Format recurrence text if applicable
    let recurrence_text = if let Some(recurrence) = &event.recurrence {
        format_recurrence(recurrence)
    } else {
        String::new()
    };
    
    // Check if location has geofencing
    let has_geofence = if let Some(location) = &event.location {
        location.radius.is_some()
    } else {
        false
    };
    
    let on_card_click = {
        let on_click = props.on_click.clone();
        let event_id = event.id;
        Callback::from(move |_| on_click.emit(event_id))
    };
    
    let on_edit_click = {
        let on_edit = props.on_edit.clone();
        let event_id = event.id;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_edit.emit(event_id);
        })
    };
    
    let on_delete_click = {
        let on_delete = props.on_delete.clone();
        let event_id = event.id;
        Callback::from(move |e: MouseEvent| {
            e.stop_propagation();
            on_delete.emit(event_id);
        })
    };
    
    html! {
        <div 
            class={classes!(
                "event-card",
                "event-card-container",
                if props.is_selected { "selected" } else { "" }
            )}
            onclick={on_card_click}
        >
            <div class="event-header">
                <div class="event-title-section">
                    <span class="event-type-badge" style={format!("background-color: {}", event_type_color)}>
                        {get_event_type_label(&event.event_type)}
                    </span>
                    <h3 class="event-title">{&event.title}</h3>
                </div>
                <div class="event-indicators">
                    <span class="privacy-indicator">{privacy_icon}</span>
                    if is_recurring {
                        <span class="recurring-indicator">{"↻"}</span>
                    }
                    if has_geofence {
                        <span class="geofence-indicator">{"◎"}</span>
                    }
                </div>
            </div>
            
            <div class="event-time-location">
                <div class="event-time">{time_range}</div>
                if let Some(location) = &event.location {
                    <div class="event-location">
                        <span class="location-icon">{"📍"}</span>
                        <span class="location-name">{&location.name}</span>
                        if location.radius.is_some() {
                            <span class="geofence-badge">{"Geofenced"}</span>
                        }
                    </div>
                }
            </div>
            
            if !recurrence_text.is_empty() {
                <div class="recurrence-info">
                    <span class="recurrence-icon">{"↻"}</span>
                    <span class="recurrence-text">{recurrence_text}</span>
                </div>
            }
            
            if let Some(description) = &event.description {
                if !description.is_empty() {
                    <div class="event-description">
                        {description}
                    </div>
                }
            }
            
            <div class="event-actions">
                <button 
                    class="edit-button" 
                    onclick={on_edit_click}
                    style={if props.is_touch_device { "min-height: 48px; min-width: 48px;" } else { "" }}
                >
                    {"Edit"}
                </button>
                <button 
                    class="delete-button" 
                    onclick={on_delete_click}
                    style={if props.is_touch_device { "min-height: 48px; min-width: 48px;" } else { "" }}
                >
                    {"Delete"}
                </button>
            </div>
        </div>
    }
}

/// Get the privacy icon based on event visibility
fn get_privacy_icon(visibility: &EventVisibility) -> &'static str {
    match visibility {
        EventVisibility::Private => "🔒",
        EventVisibility::Shared(_) => "👥",
        EventVisibility::Public => "🌐",
        EventVisibility::CooperativeMembers(_) => "🤝",
    }
}

/// Get the color for event type badge
fn get_event_type_color(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::Personal => "#4285f4",      // Blue
        EventType::Business => "#34a853",       // Green
        EventType::Cooperative(_) => "#fbbc04", // Yellow
        EventType::TaskDeadline(_) => "#ea4335", // Red
        EventType::HealthAppointment => "#9c27b0", // Purple
    }
}

/// Get the label for event type
fn get_event_type_label(event_type: &EventType) -> &'static str {
    match event_type {
        EventType::Personal => "Personal",
        EventType::Business => "Business",
        EventType::Cooperative(_) => "Cooperative",
        EventType::TaskDeadline(_) => "Task",
        EventType::HealthAppointment => "Health",
    }
}

/// Format time range for display
fn format_time_range(start: &DateTime<Utc>, end: &DateTime<Utc>) -> String {
    if start.date_naive() == end.date_naive() {
        // Same day event
        format!(
            "{}:{:02} - {}:{:02}",
            start.hour(),
            start.minute(),
            end.hour(),
            end.minute()
        )
    } else {
        // Multi-day event
        format!(
            "{}/{}/{} {}:{:02} - {}/{}/{} {}:{:02}",
            start.month(),
            start.day(),
            start.year(),
            start.hour(),
            start.minute(),
            end.month(),
            end.day(),
            end.year(),
            end.hour(),
            end.minute()
        )
    }
}