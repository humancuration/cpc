//! Main application component for Finance-Sheets
//!
//! This module defines the root application component that ties together
//! all the UI components for the financial spreadsheet application.

use yew::prelude::*;
use crate::components::currency::{
    CurrencySelector, CurrencyConverter, FormattingPreview, ExchangeRateManager
};
use crate::components::mobile::{MobileLayout, FloatingActionButton};
use crate::services::mobile::{DeviceSize, get_device_size};
use crate::styles::{get_mobile_styles, get_responsive_styles};

/// Properties for the main application component
#[derive(Properties, PartialEq)]
pub struct AppProps {
    // Add any props needed for the app
}

/// Main application component
#[function_component(App)]
pub fn app(_props: &AppProps) -> Html {
    let device_size = use_state(|| get_device_size());
    let mobile_styles = get_mobile_styles();
    let responsive_styles = get_responsive_styles();
    
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
    
    let handle_fab_click = Callback::from(|_| {
        web_sys::console::log_1(&"FAB clicked".into());
    });
    
    match *device_size {
        DeviceSize::Mobile => {
            html! {
                <div class={classes!("finance-sheets-app", mobile_styles.clone(), responsive_styles.clone())}>
                    <MobileLayout>
                        <header>
                            <h1>{"Finance Sheets"}</h1>
                        </header>
                        
                        <main>
                            <div class="currency-tools">
                                <section class="tool-section">
                                    <h2>{"Currency Converter"}</h2>
                                    <CurrencyConverter />
                                </section>
                                
                                <section class="tool-section">
                                    <h2>{"Currency Selector"}</h2>
                                    <CurrencySelector />
                                </section>
                                
                                <section class="tool-section">
                                    <h2>{"Formatting Preview"}</h2>
                                    <FormattingPreview />
                                </section>
                                
                                <section class="tool-section">
                                    <h2>{"Exchange Rate Manager"}</h2>
                                    <ExchangeRateManager />
                                </section>
                            </div>
                        </main>
                        
                        <footer>
                            <p>{"Finance Sheets - Mobile"}</p>
                        </footer>
                    </MobileLayout>
                    < FloatingActionButton on_click={handle_fab_click} />
                </div>
            }
        },
        DeviceSize::Tablet | DeviceSize::Desktop => {
            html! {
                <div class={classes!("finance-sheets-app", mobile_styles.clone(), responsive_styles.clone())}>
                    <header>
                        <h1>{"Finance Sheets"}</h1>
                    </header>
                    
                    <main>
                        <div class="currency-tools">
                            <section class="tool-section">
                                <h2>{"Currency Converter"}</h2>
                                <CurrencyConverter />
                            </section>
                            
                            <section class="tool-section">
                                <h2>{"Currency Selector"}</h2>
                                <CurrencySelector />
                            </section>
                            
                            <section class="tool-section">
                                <h2>{"Formatting Preview"}</h2>
                                <FormattingPreview />
                            </section>
                            
                            <section class="tool-section">
                                <h2>{"Exchange Rate Manager"}</h2>
                                <ExchangeRateManager />
                            </section>
                        </div>
                    </main>
                    
                    <footer>
                        <p>{"Finance Sheets - Currency Internationalization"}</p>
                    </footer>
                </div>
            }
        }
    }
}