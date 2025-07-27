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
