//! Example application demonstrating the UI toolkit
//!
//! This module provides a complete example application that demonstrates
//! how to use the UI toolkit components with the theme system.

use yew::prelude::*;
use crate::components::theme_provider::ThemeProvider;
use crate::examples::theme_example::ThemeExample;

/// Main example application
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <ThemeProvider>
            <div style="min-height: 100vh; background-color: var(--cpc-background); color: var(--cpc-text);">
                <header style="background-color: var(--cpc-surface); padding: var(--cpc-spacing-lg); border-bottom: 1px solid var(--cpc-border);">
                    <div style="max-width: 1200px; margin: 0 auto; display: flex; justify-content: space-between; align-items: center;">
                        <h1 style="margin: 0; font-size: var(--cpc-font-size-xxl);">{"UI Toolkit Demo"}</h1>
                        <nav>
                            <a href="#" style="margin-right: var(--cpc-spacing-md); color: var(--cpc-primary); text-decoration: none;">{"Home"}</a>
                            <a href="#" style="margin-right: var(--cpc-spacing-md); color: var(--cpc-text); text-decoration: none;">{"About"}</a>
                            <a href="#" style="color: var(--cpc-text); text-decoration: none;">{"Contact"}</a>
                        </nav>
                    </div>
                </header>
                
                <main style="max-width: 1200px; margin: 0 auto; padding: var(--cpc-spacing-lg);">
                    <ThemeExample />
                </main>
                
                <footer style="background-color: var(--cpc-surface); padding: var(--cpc-spacing-lg); border-top: 1px solid var(--cpc-border); margin-top: var(--cpc-spacing-xl);">
                    <div style="max-width: 1200px; margin: 0 auto; text-align: center;">
                        <p>{"© 2025 CPC UI Toolkit. All rights reserved."}</p>
                    </div>
                </footer>
            </div>
        </ThemeProvider>
    }
}