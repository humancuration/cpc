//! Calendar store implementation using Yew Context API
use yew::prelude::*;
use yewdux::prelude::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::rc::Rc;

// Import domain models
use packages::cpc_core::calendar::domain::event::CalendarEvent;
use packages::cpc_core::calendar::domain::shift::WorkShift;

// Import local modules
use crate::components::calendar::state::view::CalendarView;

/// Calendar state
#[derive(Clone, PartialEq, Default)]
pub struct CalendarState {
    pub current_view: CalendarView,
    pub selected_date: DateTime<Utc>,
    pub events: Vec<CalendarEvent>,
    pub work_shifts: Vec<WorkShift>,
    pub loading: bool,
    pub error: Option<String>,
}

/// Calendar actions
#[derive(Clone, PartialEq)]
pub enum CalendarAction {
    SetView(CalendarView),
    SetDate(DateTime<Utc>),
    EventsFetched(Vec<CalendarEvent>),
    ShiftsFetched(Vec<WorkShift>),
    EventCreated(CalendarEvent),
    EventUpdated(CalendarEvent),
    EventDeleted(Uuid),
    Loading(bool),
    Error(String),
}

impl Reducer<CalendarState> for CalendarAction {
    fn reduce(state: Rc<CalendarState>, action: CalendarAction) -> Rc<CalendarState> {
        match action {
            CalendarAction::SetView(view) => {
                state.with(|s| s.current_view = view)
            }
            CalendarAction::SetDate(date) => {
                state.with(|s| s.selected_date = date)
            }
            CalendarAction::EventsFetched(events) => {
                state.with(|s| {
                    s.events = events;
                    s.loading = false;
                    s.error = None;
                })
            }
            CalendarAction::ShiftsFetched(shifts) => {
                state.with(|s| {
                    s.work_shifts = shifts;
                    s.loading = false;
                    s.error = None;
                })
            }
            CalendarAction::EventCreated(event) => {
                let mut events = state.events.clone();
                events.push(event);
                state.with(|s| s.events = events)
            }
            CalendarAction::EventUpdated(event) => {
                let mut events = state.events.clone();
                if let Some(pos) = events.iter().position(|e| e.id == event.id) {
                    events[pos] = event;
                }
                state.with(|s| s.events = events)
            }
            CalendarAction::EventDeleted(id) => {
                let mut events = state.events.clone();
                events.retain(|e| e.id != id);
                state.with(|s| s.events = events)
            }
            CalendarAction::Loading(loading) => {
                state.with(|s| s.loading = loading)
            }
            CalendarAction::Error(error) => {
                state.with(|s| {
                    s.error = Some(error);
                    s.loading = false;
                })
            }
        }
    }
}

/// Calendar store context
#[derive(Clone, PartialEq, Store)]
#[store(storage = "none")]
pub struct CalendarStore {
    pub state: CalendarState,
}

impl Default for CalendarStore {
    fn default() -> Self {
        Self {
            state: CalendarState {
                current_view: CalendarView::Month,
                selected_date: Utc::now(),
                events: Vec::new(),
                work_shifts: Vec::new(),
                loading: false,
                error: None,
            }
        }
    }
}

impl CalendarStore {
    /// Get events for a specific date range
    pub fn get_events_for_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<CalendarEvent> {
        self.state.events.iter()
            .filter(|event| {
                // Check if event overlaps with the range
                event.start < end && event.end > start
            })
            .cloned()
            .collect()
    }
    
    /// Get work shifts for a specific date range
    pub fn get_shifts_for_range(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<WorkShift> {
        self.state.work_shifts.iter()
            .filter(|shift| {
                // For now, we'll return all shifts
                // In a more complete implementation, we would filter by date
                true
            })
            .cloned()
            .collect()
    }
    
    /// Expand recurring events for a specific date range
    pub fn expand_recurring_events(&self, start: DateTime<Utc>, end: DateTime<Utc>) -> Vec<CalendarEvent> {
        let mut expanded_events = Vec::new();
        
        for event in &self.state.events {
            if event.is_recurring() {
                // For recurring events, we would generate instances for the date range
                // This is a simplified implementation
                expanded_events.push(event.clone());
            } else {
                // For non-recurring events, just check if they fall in the range
                if event.start >= start && event.start < end {
                    expanded_events.push(event.clone());
                }
            }
        }
        
        expanded_events
    }
}