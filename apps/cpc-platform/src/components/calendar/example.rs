//! Example component demonstrating how to use the calendar components
use yew::prelude::*;
use yewdux::prelude::*;
use chrono::{Utc, Duration};
use uuid::Uuid;

// Import domain models
use packages::cpc_core::calendar::domain::event::{CalendarEvent, EventType, EventVisibility};

// Import local modules
use crate::components::calendar::state::store::{CalendarStore, CalendarAction};
use crate::components::calendar::state::view::CalendarView;
use crate::components::calendar::views::switcher::CalendarViewSwitcher;
use crate::components::calendar::cards::event_card::EventCard;

/// Example calendar app component
#[function_component(CalendarExample)]
pub fn calendar_example() -> Html {
    let (store, dispatch) = use_store::<CalendarStore>();
    
    // Initialize with some sample events
    use_effect_with((), {
        let dispatch = dispatch.clone();
        move |_| {
            // Create sample events
            let mut events = Vec::new();
            
            // Add a personal event
            let personal_event = CalendarEvent::new(
                Uuid::new_v4(),
                "Team Meeting".to_string(),
                Some("Weekly team sync".to_string()),
                Utc::now() + Duration::days(1),
                Utc::now() + Duration::days(1) + Duration::hours(1),
                EventType::Business,
                EventVisibility::Shared(vec![Uuid::new_v4()]),
                None,
                None,
            );
            events.push(personal_event);
            
            // Add a recurring event
            let recurring_event = CalendarEvent::new(
                Uuid::new_v4(),
                "Daily Standup".to_string(),
                Some("Daily standup meeting".to_string()),
                Utc::now() + Duration::hours(2),
                Utc::now() + Duration::hours(2) + Duration::minutes(30),
                EventType::Business,
                EventVisibility::CooperativeMembers("cooperative_id".to_string()),
                None, // In a real app, this would have a recurrence rule
                None,
            );
            events.push(recurring_event);
            
            // Dispatch events to store
            dispatch.apply(CalendarAction::EventsFetched(events));
        }
    });
    
    let on_view_change = {
        let dispatch = dispatch.clone();
        Callback::from(move |view: CalendarView| {
            dispatch.apply(CalendarAction::SetView(view));
        })
    };
    
    let on_date_change = {
        let dispatch = dispatch.clone();
        Callback::from(move |date: chrono::DateTime<Utc>| {
            dispatch.apply(CalendarAction::SetDate(date));
        })
    };
    
    let on_event_edit = Callback::from(|event_id: Uuid| {
        web_sys::console::log_1(&format!("Edit event: {}", event_id).into());
    });
    
    let on_event_delete = Callback::from(|event_id: Uuid| {
        web_sys::console::log_1(&format!("Delete event: {}", event_id).into());
    });
    
    html! {
        <div class="calendar-example">
            <h1>{"Calendar Example"}</h1>
            
            <CalendarViewSwitcher
                current_view={store.state.current_view.clone()}
                on_view_change={on_view_change}
                selected_date={store.state.selected_date}
                on_date_change={on_date_change}
            >
                <div class="events-list">
                    {for store.state.events.iter().map(|event| {
                        html! {
                            <EventCard
                                event={event.clone()}
                                on_edit={on_event_edit.clone()}
                                on_delete={on_event_delete.clone()}
                                is_touch_device={true}
                            />
                        }
                    })}
                </div>
            </CalendarViewSwitcher>
        </div>
    }
}