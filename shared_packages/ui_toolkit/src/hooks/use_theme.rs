//! Custom hook for accessing the theme context
//!
//! This module provides a hook for easily accessing the theme context
//! from within functional components.

use yew::prelude::*;
use crate::components::theme_provider::ThemeContext;

/// Hook to access the current theme context
/// 
/// This hook provides access to the theme manager and functions for
/// updating the theme from within functional components.
/// 
/// # Example
/// 
/// ```rust
/// use yew::prelude::*;
/// use ui_toolkit::hooks::use_theme::use_theme;
/// use ui_toolkit::themes::ColorScheme;
/// 
/// #[function_component(MyComponent)]
/// fn my_component() -> Html {
///     let theme_context = use_theme();
///     
///     let onclick = {
///         let toggle_theme = theme_context.toggle_theme.clone();
///         Callback::from(move |_| toggle_theme.emit(()))
///     };
///     
///     html! {
///         <button onclick={onclick}>
///             { "Toggle Theme" }
///         </button>
///     }
/// }
/// ```
#[hook]
pub fn use_theme() -> ThemeContext {
    use_context::<ThemeContext>().expect("Theme context not found. Make sure you're using ThemeProvider at the root of your app.")
}