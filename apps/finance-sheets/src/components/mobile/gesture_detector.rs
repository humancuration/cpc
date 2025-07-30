//! Gesture detector component for mobile touch interactions
//!
//! This component provides gesture detection capabilities for mobile devices,
//! including swipe detection, tap-and-hold, and other touch gestures.

use yew::prelude::*;
use web_sys::{Touch, TouchList};
use super::haptics;

/// Properties for the gesture detector component
#[derive(Properties, PartialEq)]
pub struct GestureDetectorProps {
    /// Child elements to wrap with gesture detection
    pub children: Children,
    
    /// Callback for swipe left gesture
    #[prop_or_default]
    pub on_swipe_left: Callback<()>,
    
    /// Callback for swipe right gesture
    #[prop_or_default]
    pub on_swipe_right: Callback<()>,
    
    /// Callback for swipe up gesture
    #[prop_or_default]
    pub on_swipe_up: Callback<()>,
    
    /// Callback for swipe down gesture
    #[prop_or_default]
    pub on_swipe_down: Callback<()>,
    
    /// Callback for tap gesture
    #[prop_or_default]
    pub on_tap: Callback<()>,
    
    /// Callback for double tap gesture
    #[prop_or_default]
    pub on_double_tap: Callback<()>,
    
    /// Callback for tap and hold gesture
    #[prop_or_default]
    pub on_tap_hold: Callback<()>,
    
    /// Callback for pinch gesture
    #[prop_or_default]
    pub on_pinch: Callback<f64>,
    
    /// Callback for zoom gesture
    #[prop_or_default]
    pub on_zoom: Callback<f64>,
}

/// State for the gesture detector component
#[derive(Debug, Clone, PartialEq)]
pub struct GestureDetectorState {
    /// Touch start position
    touch_start: Option<(i32, i32)>,
    
    /// Touch start time for tap hold detection
    touch_start_time: Option<f64>,
    
    /// Previous touch positions for multi-touch gestures
    previous_touches: Option<(i32, i32, i32, i32)>,
    
    /// Tap count for double tap detection
    tap_count: u32,
    
    /// Last tap time for double tap detection
    last_tap_time: Option<f64>,
}

/// Messages for the gesture detector component
#[derive(Debug, Clone)]
pub enum GestureDetectorMsg {
    /// Touch start event
    TouchStart(i32, i32),
    
    /// Touch move event
    TouchMove(i32, i32),
    
    /// Touch end event
    TouchEnd(i32, i32),
    
    /// Multi-touch start event
    MultiTouchStart(i32, i32, i32, i32),
    
    /// Multi-touch move event
    MultiTouchMove(i32, i32, i32, i32),
    
    /// Multi-touch end event
    MultiTouchEnd,
    
    /// Reset tap count
    ResetTapCount,
}

/// Gesture detector component
#[derive(Debug)]
pub struct GestureDetector {
    /// Component state
    state: GestureDetectorState,
}

impl GestureDetector {
    /// Minimum distance for swipe detection (in pixels)
    const MIN_SWIPE_DISTANCE: i32 = 50;
    
    /// Maximum time for tap detection (in milliseconds)
    const MAX_TAP_TIME: f64 = 200.0;
    
    /// Minimum time for tap hold detection (in milliseconds)
    const MIN_TAP_HOLD_TIME: f64 = 500.0;
    
    /// Maximum distance for tap detection (in pixels)
    const MAX_TAP_DISTANCE: i32 = 10;
}

impl Component for GestureDetector {
    type Message = GestureDetectorMsg;
    type Properties = GestureDetectorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: GestureDetectorState {
                touch_start: None,
                touch_start_time: None,
                previous_touches: None,
                tap_count: 0,
                last_tap_time: None,
            },
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            GestureDetectorMsg::TouchStart(x, y) => {
                self.state.touch_start = Some((x, y));
                self.state.touch_start_time = Some(js_sys::Date::now());
                true
            }
            
            GestureDetectorMsg::TouchMove(_x, _y) => {
                // For now, we don't need to do anything on touch move
                // In a more complex implementation, we might track movement
                false
            }
            
            GestureDetectorMsg::TouchEnd(x, y) => {
                if let Some((start_x, start_y)) = self.state.touch_start {
                    let delta_x = x - start_x;
                    let delta_y = y - start_y;
                    
                    // Check if this is a tap (short duration, minimal movement)
                    if let Some(start_time) = self.state.touch_start_time {
                        let duration = js_sys::Date::now() - start_time;
                        
                        if duration <= Self::MAX_TAP_TIME && 
                           delta_x.abs() <= Self::MAX_TAP_DISTANCE && 
                           delta_y.abs() <= Self::MAX_TAP_DISTANCE {
                            
                            // Check for double tap
                            let is_double_tap = if let Some(last_tap_time) = self.state.last_tap_time {
                                let time_since_last_tap = js_sys::Date::now() - last_tap_time;
                                time_since_last_tap <= 300.0 // 300ms between taps for double tap
                            } else {
                                false
                            };
                            
                            if is_double_tap {
                                // Double tap - trigger cell edit start haptic
                                haptics::trigger_cell_edit_start();
                                ctx.props().on_double_tap.emit(());
                                self.state.tap_count = 0;
                                self.state.last_tap_time = None;
                            } else {
                                // Single tap - trigger cell selection haptic
                                haptics::trigger_cell_selection();
                                ctx.props().on_tap.emit(());
                                self.state.tap_count += 1;
                                self.state.last_tap_time = Some(js_sys::Date::now());
                                
                                // Reset tap count after a delay
                                let link = ctx.link().clone();
                                wasm_bindgen_futures::spawn_local(async move {
                                    gloo_timers::future::TimeoutFuture::new(300).await;
                                    link.send_message(GestureDetectorMsg::ResetTapCount);
                                });
                            }
                        } else if duration >= Self::MIN_TAP_HOLD_TIME {
                            // Tap and hold
                            ctx.props().on_tap_hold.emit(());
                        } else {
                            // Swipe gesture
                            if delta_x.abs() > delta_y.abs() {
                                // Horizontal swipe
                                if delta_x > Self::MIN_SWIPE_DISTANCE {
                                    // Swipe right - trigger sheet switch haptic
                                    haptics::trigger_sheet_switch();
                                    ctx.props().on_swipe_right.emit(());
                                } else if delta_x < -Self::MIN_SWIPE_DISTANCE {
                                    // Swipe left - trigger sheet switch haptic
                                    haptics::trigger_sheet_switch();
                                    ctx.props().on_swipe_left.emit(());
                                }
                            } else {
                                // Vertical swipe
                                if delta_y > Self::MIN_SWIPE_DISTANCE {
                                    // Swipe down - trigger sheet switch haptic
                                    haptics::trigger_sheet_switch();
                                    ctx.props().on_swipe_down.emit(());
                                } else if delta_y < -Self::MIN_SWIPE_DISTANCE {
                                    // Swipe up - trigger sheet switch haptic
                                    haptics::trigger_sheet_switch();
                                    ctx.props().on_swipe_up.emit(());
                                }
                            }
                        }
                    }
                }
                
                // Reset touch state
                self.state.touch_start = None;
                self.state.touch_start_time = None;
                true
            }
            
            GestureDetectorMsg::MultiTouchStart(x1, y1, x2, y2) => {
                self.state.previous_touches = Some((x1, y1, x2, y2));
                true
            }
            
            GestureDetectorMsg::MultiTouchMove(x1, y1, x2, y2) => {
                if let Some((prev_x1, prev_y1, prev_x2, prev_y2)) = self.state.previous_touches {
                    // Calculate distances between touches
                    let prev_distance = ((prev_x1 - prev_x2).pow(2) + (prev_y1 - prev_y2).pow(2)) as f64;
                    let current_distance = ((x1 - x2).pow(2) + (y1 - y2).pow(2)) as f64;
                    
                    // Calculate scale factor
                    let scale = current_distance / prev_distance;
                    
                    // Pinch (zoom out) or zoom (zoom in)
                    if scale < 0.9 {
                        ctx.props().on_pinch.emit(scale);
                    } else if scale > 1.1 {
                        ctx.props().on_zoom.emit(scale);
                    }
                }
                
                self.state.previous_touches = Some((x1, y1, x2, y2));
                true
            }
            
            GestureDetectorMsg::MultiTouchEnd => {
                self.state.previous_touches = None;
                true
            }
            
            GestureDetectorMsg::ResetTapCount => {
                self.state.tap_count = 0;
                self.state.last_tap_time = None;
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        
        // Touch event handlers
        let on_touch_start = link.callback(|e: TouchEvent| {
            let touches = e.touches();
            
            if touches.length() == 1 {
                let touch = touches.get(0).unwrap();
                GestureDetectorMsg::TouchStart(touch.client_x(), touch.client_y())
            } else if touches.length() == 2 {
                let touch1 = touches.get(0).unwrap();
                let touch2 = touches.get(1).unwrap();
                GestureDetectorMsg::MultiTouchStart(
                    touch1.client_x(),
                    touch1.client_y(),
                    touch2.client_x(),
                    touch2.client_y(),
                )
            } else {
                // More than 2 touches, ignore for now
                GestureDetectorMsg::TouchStart(0, 0)
            }
        });
        
        let on_touch_move = link.callback(|e: TouchEvent| {
            let touches = e.touches();
            
            if touches.length() == 1 {
                let touch = touches.get(0).unwrap();
                GestureDetectorMsg::TouchMove(touch.client_x(), touch.client_y())
            } else if touches.length() == 2 {
                let touch1 = touches.get(0).unwrap();
                let touch2 = touches.get(1).unwrap();
                GestureDetectorMsg::MultiTouchMove(
                    touch1.client_x(),
                    touch1.client_y(),
                    touch2.client_x(),
                    touch2.client_y(),
                )
            } else {
                // Ignore for now
                GestureDetectorMsg::TouchMove(0, 0)
            }
        });
        
        let on_touch_end = link.callback(|e: TouchEvent| {
            let touches = e.changed_touches();
            
            if touches.length() >= 1 {
                let touch = touches.get(0).unwrap();
                GestureDetectorMsg::TouchEnd(touch.client_x(), touch.client_y())
            } else {
                GestureDetectorMsg::TouchEnd(0, 0)
            }
        });
        
        let on_touch_cancel = link.callback(|_: TouchEvent| {
            GestureDetectorMsg::MultiTouchEnd
        });
        
        html! {
            <div
                ontouchstart={on_touch_start}
                ontouchmove={on_touch_move}
                ontouchend={on_touch_end}
                ontouchcancel={on_touch_cancel}
            >
                {for props.children.iter()}
            </div>
        }
    }
}