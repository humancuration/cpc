//! Mobile-specific components for Finance-Sheets
//!
//! This module contains UI components that are specifically designed for mobile devices,
//! including layout components, navigation elements, and touch-optimized controls.

pub mod search_dropdown;
pub mod sheet_grid;
pub mod gesture_detector;
pub mod haptics;
pub mod toolbar;
pub mod virtual_scroll;
#[cfg(test)]
pub mod test;

use yew::prelude::*;
use crate::services::mobile::{DeviceSize, get_device_size};

/// Properties for the MobileLayout component
#[derive(Properties, PartialEq)]
pub struct MobileLayoutProps {
    /// The main content to display
    pub children: Children,
}

/// Mobile layout component that provides a responsive structure for mobile devices
///
/// This component implements the mobile-specific layout structure as outlined in the
/// mobile optimization plan, including:
/// - Navigation rail for vertical navigation
/// - Bottom navigation for key app sections
/// - Floating action button for primary actions
#[function_component(MobileLayout)]
pub fn mobile_layout(props: &MobileLayoutProps) -> Html {
    let device_size = use_state(|| get_device_size());
    
    // Update device size when window is resized
    {
        let device_size = device_size.clone();
        use_effect_with((), move |_| {
            let device_size = device_size.clone();
            let closure = Closure::wrap(Box::new(move || {
                device_size.set(get_device_size());
            }) as Box<dyn Fn()>);
            
            if let Some(window) = web_sys::window() {
                let _ = window.add_event_listener_with_callback(
                    "resize",
                    closure.as_ref().unchecked_ref()
                );
            }
            
            move || {
                if let Some(window) = web_sys::window() {
                    let _ = window.remove_event_listener_with_callback(
                        "resize",
                        closure.as_ref().unchecked_ref()
                    );
                }
                drop(closure);
            }
        });
    }
    
    // Only render mobile layout on mobile devices
    if *device_size != DeviceSize::Mobile {
        return html! { <>{ for props.children.iter() }</> };
    }
    
    html! {
        <div class="mobile-layout">
            // Navigation rail on the left side
            <NavigationRail />
            
            // Main content area
            <div class="mobile-content">
                { for props.children.iter() }
            </div>
            
            // Floating action button
            < FloatingActionButton />
            
            // Bottom navigation
            <BottomNavigation />
        </div>
    }
}

/// Properties for the NavigationRail component
#[derive(Properties, PartialEq)]
pub struct NavigationRailProps {}

/// Navigation rail component for mobile vertical navigation
#[function_component(NavigationRail)]
pub fn navigation_rail(_props: &NavigationRailProps) -> Html {
    html! {
        <nav class="navigation-rail">
            <ul>
                <li><button class="nav-button">{"Home"}</button></li>
                <li><button class="nav-button">{"Sheets"}</button></li>
                <li><button class="nav-button">{"Settings"}</button></li>
            </ul>
        </nav>
    }
}

/// Properties for the BottomNavigation component
#[derive(Properties, PartialEq)]
pub struct BottomNavigationProps {}

/// Bottom navigation component for mobile
#[function_component(BottomNavigation)]
pub fn bottom_navigation(_props: &BottomNavigationProps) -> Html {
    html! {
        <nav class="bottom-navigation">
            <ul>
                <li><button class="bottom-nav-button">{"Sheets"}</button></li>
                <li><button class="bottom-nav-button">{"Currency"}</button></li>
                <li><button class="bottom-nav-button">{"Settings"}</button></li>
            </ul>
        </nav>
    }
}

/// Properties for the FloatingActionButton component
#[derive(Properties, PartialEq)]
pub struct FloatingActionButtonProps {
    #[prop_or_default]
    pub on_click: Callback<MouseEvent>,
}

/// Floating action button for primary actions on mobile
#[function_component(FloatingActionButton)]
pub fn floating_action_button(props: &FloatingActionButtonProps) -> Html {
    let on_click = props.on_click.clone();
    
    html! {
        <button class="fab" onclick={on_click}>
            <span class="fab-icon">{"+"}</span>
        </button>
    }
}

// Re-export key components for easier access
pub use mobile_layout::MobileLayout;
pub use floating_action_button::FloatingActionButton;
pub use navigation_rail::NavigationRail;
pub use bottom_navigation::BottomNavigation;