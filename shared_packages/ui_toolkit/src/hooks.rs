//! Custom hooks for the UI toolkit
//!
//! This module provides reusable hooks for common UI functionality
//! such as media queries, theme access, and platform detection.

use yew::prelude::*;
use gloo_events::EventListener;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
use web_sys::{window, MediaQueryList};
use std::rc::Rc;

/// Hook to detect platform (web/desktop)
pub fn use_platform() -> Platform {
    // In a web environment, we're always on web
    // In a desktop environment, we would detect differently
    #[cfg(target_arch = "wasm32")]
    {
        Platform::Web
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    {
        Platform::Desktop
    }
}

/// Platform types
#[derive(Debug, Clone, PartialEq)]
pub enum Platform {
    /// Web platform (WASM)
    Web,
    /// Desktop platform (Tauri)
    Desktop,
}

/// Hook to detect media queries
#[hook]
pub fn use_media_query(query: &str) -> bool {
    let query_clone = query.to_string();
    let matches = use_state(|| {
        window()
            .and_then(|w| w.match_media(&query_clone).ok().flatten())
            .map(|mq| mq.matches())
            .unwrap_or(false)
    });

    let matches_clone = matches.clone();
    use_effect_with(query.to_string(), move |_| {
        let query = query_clone.clone();
        let matches_handle = matches_clone.clone();
        
        let media_query = window()
            .and_then(|w| w.match_media(&query).ok().flatten());

        if let Some(mq) = media_query {
            let closure = Closure::wrap(Box::new(move |e: web_sys::Event| {
                let mq: MediaQueryList = e.target().unwrap().dyn_into().unwrap();
                matches_handle.set(mq.matches());
            }) as Box<dyn FnMut(web_sys::Event)>);

            let listener = EventListener::new(&mq, "change", closure);
            Rc::new(listener) // Keep the listener alive
        } else {
            Rc::new(()) // Return a dummy Rc
        }
    });

    *matches
}

/// Hook to detect screen size breakpoints
#[hook]
pub fn use_breakpoint() -> Breakpoint {
    let is_xxl = use_media_query("(min-width: 1400px)");
    let is_xl = use_media_query("(min-width: 1200px)");
    let is_lg = use_media_query("(min-width: 992px)");
    let is_md = use_media_query("(min-width: 768px)");
    let is_sm = use_media_query("(min-width: 576px)");
    
    if is_xxl {
        Breakpoint::XXL
    } else if is_xl {
        Breakpoint::XL
    } else if is_lg {
        Breakpoint::LG
    } else if is_md {
        Breakpoint::MD
    } else if is_sm {
        Breakpoint::SM
    } else {
        Breakpoint::XS
    }
}

/// Breakpoint sizes
#[derive(Debug, Clone, PartialEq)]
pub enum Breakpoint {
    /// Extra small devices (portrait phones, less than 576px)
    XS,
    /// Small devices (landscape phones, 576px and up)
    SM,
    /// Medium devices (tablets, 768px and up)
    MD,
    /// Large devices (desktops, 992px and up)
    LG,
    /// Extra large devices (large desktops, 1200px and up)
    XL,
    /// Extra extra large devices (larger desktops, 1400px and up)
    XXL,
}

impl Breakpoint {
    /// Get the minimum width for this breakpoint
    pub fn min_width(&self) -> Option<u32> {
        match self {
            Breakpoint::XS => None,
            Breakpoint::SM => Some(576),
            Breakpoint::MD => Some(768),
            Breakpoint::LG => Some(992),
            Breakpoint::XL => Some(1200),
            Breakpoint::XXL => Some(1400),
        }
    }
    
    /// Check if this is a mobile breakpoint
    pub fn is_mobile(&self) -> bool {
        matches!(self, Breakpoint::XS | Breakpoint::SM)
    }
    
    /// Check if this is a tablet breakpoint
    pub fn is_tablet(&self) -> bool {
        matches!(self, Breakpoint::MD)
    }
    
    /// Check if this is a desktop breakpoint
    pub fn is_desktop(&self) -> bool {
        matches!(self, Breakpoint::LG | Breakpoint::XL | Breakpoint::XXL)
    }
}