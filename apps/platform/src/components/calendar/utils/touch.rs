//! Touch gesture handlers for calendar components
use yew::prelude::*;
use web_sys::{TouchEvent, Touch};
use wasm_bindgen::JsCast;
use std::rc::Rc;

/// Touch gesture state
#[derive(Debug, Clone, PartialEq)]
pub struct TouchState {
    pub start_x: f64,
    pub start_y: f64,
    pub current_x: f64,
    pub current_y: f64,
    pub is_swiping: bool,
    pub is_long_pressing: bool,
}

impl Default for TouchState {
    fn default() -> Self {
        Self {
            start_x: 0.0,
            start_y: 0.0,
            current_x: 0.0,
            current_y: 0.0,
            is_swiping: false,
            is_long_pressing: false,
        }
    }
}

/// Handle touch start event
pub fn handle_touch_start(state: &mut TouchState, e: TouchEvent) {
    if let Some(touch) = e.touches().get(0) {
        state.start_x = touch.client_x() as f64;
        state.start_y = touch.client_y() as f64;
        state.current_x = state.start_x;
        state.current_y = state.start_y;
        state.is_swiping = false;
    }
}

/// Handle touch move event
pub fn handle_touch_move(state: &mut TouchState, e: TouchEvent) {
    if let Some(touch) = e.touches().get(0) {
        state.current_x = touch.client_x() as f64;
        state.current_y = touch.client_y() as f64;
        
        // Check if we've moved enough to be considered swiping
        let delta_x = (state.current_x - state.start_x).abs();
        let delta_y = (state.current_y - state.start_y).abs();
        
        if delta_x > 10.0 || delta_y > 10.0 {
            state.is_swiping = true;
        }
    }
}

/// Handle touch end event
pub fn handle_touch_end(state: &mut TouchState) -> Option<SwipeDirection> {
    if state.is_swiping {
        let delta_x = state.current_x - state.start_x;
        let delta_y = state.current_y - state.start_y;
        
        // Determine swipe direction based on greatest movement
        if delta_x.abs() > delta_y.abs() {
            if delta_x > 50.0 {
                return Some(SwipeDirection::Right);
            } else if delta_x < -50.0 {
                return Some(SwipeDirection::Left);
            }
        } else {
            if delta_y > 50.0 {
                return Some(SwipeDirection::Down);
            } else if delta_y < -50.0 {
                return Some(SwipeDirection::Up);
            }
        }
    }
    
    None
}

/// Swipe directions
#[derive(Debug, Clone, PartialEq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Long press detection
pub struct LongPressHandler {
    timeout_id: Option<i32>,
    on_long_press: Callback<()>,
}

impl LongPressHandler {
    pub fn new(on_long_press: Callback<()>) -> Self {
        Self {
            timeout_id: None,
            on_long_press,
        }
    }
    
    pub fn handle_touch_start(&mut self, e: TouchEvent) {
        // Clear any existing timeout
        if let Some(timeout_id) = self.timeout_id {
            web_sys::window()
                .unwrap()
                .clear_timeout_with_handle(timeout_id);
        }
        
        // Set a new timeout for long press (500ms)
        let on_long_press = self.on_long_press.clone();
        let closure = Closure::wrap(Box::new(move || {
            on_long_press.emit(());
        }) as Box<dyn Fn()>);
        
        let timeout_id = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.as_ref().unchecked_ref(),
                500,
            )
            .unwrap();
            
        self.timeout_id = Some(timeout_id);
        closure.forget(); // Prevent closure from being dropped
    }
    
    pub fn handle_touch_end(&mut self) {
        // Clear the timeout if touch ends before long press
        if let Some(timeout_id) = self.timeout_id {
            web_sys::window()
                .unwrap()
                .clear_timeout_with_handle(timeout_id);
            self.timeout_id = None;
        }
    }
}

/// Trigger haptic feedback for drag operations
pub fn trigger_haptic_feedback() {
    // In a real implementation, this would trigger actual haptic feedback
    // For now, we'll just log to the console
    web_sys::console::log_1(&"Haptic feedback triggered".into());
}

/// Check if the device supports touch events
pub fn is_touch_device() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(navigator) = window.navigator() {
            return navigator.max_touch_points() > 0;
        }
    }
    false
}