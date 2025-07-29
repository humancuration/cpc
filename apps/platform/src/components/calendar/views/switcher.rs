//! Calendar view switcher component
use yew::prelude::*;
use chrono::{DateTime, Utc, Duration, Datelike, Weekday};
use web_sys::window;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;

// Import local modules
use crate::components::calendar::state::view::CalendarView;

/// Properties for the CalendarViewSwitcher component
#[derive(Properties, PartialEq)]
pub struct CalendarViewSwitcherProps {
    pub children: Children,
    pub current_view: CalendarView,
    pub on_view_change: Callback<CalendarView>,
    pub selected_date: DateTime<Utc>,
    pub on_date_change: Callback<DateTime<Utc>>,
}

/// Check if the current device is mobile
fn check_is_mobile() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(media) = window.match_media("(max-width: 600px)") {
            if let Some(media) = media {
                return media.matches();
            }
        }
    }
    false
}

/// Calendar view switcher component
#[function_component(CalendarViewSwitcher)]
pub fn calendar_view_switcher(props: &CalendarViewSwitcherProps) -> Html {
    let is_mobile = use_state(|| check_is_mobile());
    
    // Update mobile detection on resize
    {
        let is_mobile = is_mobile.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move || {
                is_mobile.set(check_is_mobile());
            }) as Box<dyn Fn()>);
            
            if let Some(window) = window() {
                let _ = window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
            }
            
            move || {
                if let Some(window) = window() {
                    let _ = window.remove_event_listener_with_callback("resize", closure.as_ref().unchecked_ref());
                }
                drop(closure);
            }
        });
    }
    
    let on_view_change = props.on_view_change.clone();
    let on_date_change = props.on_date_change.clone();
    let selected_date = props.selected_date;
    
    let on_month_view = Callback::from(move |_| on_view_change.emit(CalendarView::Month));
    let on_week_view = Callback::from(move |_| on_view_change.emit(CalendarView::Week));
    let on_day_view = Callback::from(move |_| on_view_change.emit(CalendarView::Day));
    let on_shift_view = Callback::from(move |_| on_view_change.emit(CalendarView::Shift));
    
    let on_prev = Callback::from(move |_| {
        let new_date = match props.current_view {
            CalendarView::Month => selected_date - Duration::days(30),
            CalendarView::Week => selected_date - Duration::days(7),
            CalendarView::Day => selected_date - Duration::days(1),
            CalendarView::Shift => selected_date - Duration::days(7),
        };
        on_date_change.emit(new_date);
    });
    
    let on_next = Callback::from(move |_| {
        let new_date = match props.current_view {
            CalendarView::Month => selected_date + Duration::days(30),
            CalendarView::Week => selected_date + Duration::days(7),
            CalendarView::Day => selected_date + Duration::days(1),
            CalendarView::Shift => selected_date + Duration::days(7),
        };
        on_date_change.emit(new_date);
    });
    
    let on_today = Callback::from(move |_| {
        on_date_change.emit(Utc::now());
    });
    
    // Format date for display
    let date_display = match props.current_view {
        CalendarView::Month => format!("{} {}", selected_date.format("%B"), selected_date.year()),
        CalendarView::Week => {
            let start_of_week = selected_date - Duration::days(selected_date.weekday().num_days_from_monday() as i64);
            let end_of_week = start_of_week + Duration::days(6);
            format!("{} - {}", start_of_week.format("%b %d"), end_of_week.format("%b %d, %Y"))
        },
        CalendarView::Day => format!("{}", selected_date.format("%A, %B %d, %Y")),
        CalendarView::Shift => format!("Shifts for {}", selected_date.format("%B %Y")),
    };
    
    html! {
        <div class="calendar-view-switcher">
            <div class="view-switcher-header">
                <div class="date-navigation">
                    <button class="nav-button" onclick={on_prev}>{"<"}</button>
                    <button class="today-button" onclick={on_today}>{"Today"}</button>
                    <button class="nav-button" onclick={on_next}>{">"}</button>
                    <span class="date-display">{date_display}</span>
                </div>
                
                <div class="view-switcher-buttons">
                    <button 
                        class={classes!("view-button", if props.current_view == CalendarView::Month { "active" } else { "" })}
                        onclick={on_month_view}
                    >
                        {"Month"}
                    </button>
                    <button 
                        class={classes!("view-button", if props.current_view == CalendarView::Week { "active" } else { "" })}
                        onclick={on_week_view}
                    >
                        {"Week"}
                    </button>
                    <button 
                        class={classes!("view-button", if props.current_view == CalendarView::Day { "active" } else { "" })}
                        onclick={on_day_view}
                    >
                        {"Day"}
                    </button>
                    <button 
                        class={classes!("view-button", if props.current_view == CalendarView::Shift { "active" } else { "" })}
                        onclick={on_shift_view}
                    >
                        {"Shift"}
                    </button>
                </div>
            </div>
            
            <div class="view-content">
                {for props.children.iter()}
            </div>
        </div>
    }
}
