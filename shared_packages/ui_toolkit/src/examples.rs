//! Example components demonstrating the UI toolkit
//!
//! This module provides example components that demonstrate how to
//! use the various components in the UI toolkit.

use yew::prelude::*;
use stylist::{style, yew::styled_component};
use crate::themes::{ThemeProvider, ColorScheme};
use crate::hooks::{use_platform, use_breakpoint, use_theme, Platform, Breakpoint};
use crate::components::*;

pub mod app;

/// Example component demonstrating the theme provider
#[styled_component(ThemeProviderExample)]
pub fn theme_provider_example() -> Html {
    let theme = use_state(|| ThemeProvider::default());
    let is_dark = use_state(|| false);
    
    let toggle_theme = {
        let theme = theme.clone();
        let is_dark = is_dark.clone();
        Callback::from(move |_| {
            let mut new_theme = (*theme).clone();
            if *is_dark {
                new_theme.set_color_scheme(ColorScheme::Light);
            } else {
                new_theme.set_color_scheme(ColorScheme::Dark);
            }
            theme.set(new_theme);
            is_dark.set(!*is_dark);
        })
    };
    
    let theme_css = theme.get_theme_css();
    let style = style!(r#"{}"#, theme_css).unwrap();
    
    html! {
        <div class={style}>
            <div data-theme={if *is_dark { "dark" } else { "light" }}>
                <h1>{"Theme Provider Example"}</h1>
                <Button onclick={toggle_theme}>
                    {if *is_dark { "Switch to Light Mode" } else { "Switch to Dark Mode" }}
                </Button>
                <Card 
                    header={html! { <h2>{"Sample Card"}</h2> }}
                    footer={html! { <p>{"Card Footer"}</p> }}
                >
                    <p>{"This is a sample card component."}</p>
                </Card>
            </div>
        </div>
    }
}

/// Example component demonstrating hooks
#[styled_component(HooksExample)]
pub fn hooks_example() -> Html {
    let platform = use_platform();
    let breakpoint = use_breakpoint();
    
    let platform_text = match platform {
        Platform::Web => "Web",
        Platform::Desktop => "Desktop",
    };
    
    let breakpoint_text = match breakpoint {
        Breakpoint::XS => "Extra Small",
        Breakpoint::SM => "Small",
        Breakpoint::MD => "Medium",
        Breakpoint::LG => "Large",
        Breakpoint::XL => "Extra Large",
        Breakpoint::XXL => "Extra Extra Large",
    };
    
    html! {
        <div>
            <h1>{"Hooks Example"}</h1>
            <p>{"Current Platform: "}{platform_text}</p>
            <p>{"Current Breakpoint: "}{breakpoint_text}</p>
            
            // Responsive container example
            <Container max_width={ContainerMaxWidth::Large}>
                <Card>
                    <h2>{"Responsive Container"}</h2>
                    <p>{"This content is inside a responsive container."}</p>
                </Card>
            </Container>
        </div>
    }
}

/// Example component demonstrating all button variants
#[styled_component(ButtonExample)]
pub fn button_example() -> Html {
    let on_click = Callback::from(|_| {
        web_sys::console::log_1(&"Button clicked!".into());
    });
    
    html! {
        <div>
            <h1>{"Button Examples"}</h1>
            
            <div style="margin: 1rem 0;">
                <h2>{"Contained Buttons"}</h2>
                <Button variant={ButtonVariant::Contained} size={ButtonSize::Small} onclick={on_click.clone()}>
                    {"Small Contained"}
                </Button>
                <Button variant={ButtonVariant::Contained} size={ButtonSize::Medium} onclick={on_click.clone()}>
                    {"Medium Contained"}
                </Button>
                <Button variant={ButtonVariant::Contained} size={ButtonSize::Large} onclick={on_click.clone()}>
                    {"Large Contained"}
                </Button>
            </div>
            
            <div style="margin: 1rem 0;">
                <h2>{"Outlined Buttons"}</h2>
                <Button variant={ButtonVariant::Outlined} size={ButtonSize::Small} onclick={on_click.clone()}>
                    {"Small Outlined"}
                </Button>
                <Button variant={ButtonVariant::Outlined} size={ButtonSize::Medium} onclick={on_click.clone()}>
                    {"Medium Outlined"}
                </Button>
                <Button variant={ButtonVariant::Outlined} size={ButtonSize::Large} onclick={on_click.clone()}>
                    {"Large Outlined"}
                </Button>
            </div>
            
            <div style="margin: 1rem 0;">
                <h2>{"Text Buttons"}</h2>
                <Button variant={ButtonVariant::Text} size={ButtonSize::Small} onclick={on_click.clone()}>
                    {"Small Text"}
                </Button>
                <Button variant={ButtonVariant::Text} size={ButtonSize::Medium} onclick={on_click.clone()}>
                    {"Medium Text"}
                </Button>
                <Button variant={ButtonVariant::Text} size={ButtonSize::Large} onclick={on_click.clone()}>
                    {"Large Text"}
                </Button>
            </div>
        </div>
    }
}

/// Example component demonstrating input components
#[styled_component(InputExample)]
pub fn input_example() -> Html {
    let text_value = use_state(|| String::from("Sample text"));
    let password_value = use_state(|| String::from(""));
    let email_value = use_state(|| String::from(""));
    
    let on_text_change = {
        let text_value = text_value.clone();
        Callback::from(move |value: String| {
            text_value.set(value);
        })
    };
    
    let on_password_change = {
        let password_value = password_value.clone();
        Callback::from(move |value: String| {
            password_value.set(value);
        })
    };
    
    let on_email_change = {
        let email_value = email_value.clone();
        Callback::from(move |value: String| {
            email_value.set(value);
        })
    };
    
    html! {
        <div>
            <h1>{"Input Examples"}</h1>
            
            <div style="margin: 1rem 0;">
                <h2>{"Text Input"}</h2>
                <Input
                    value={(*text_value).clone()}
                    onchange={on_text_change}
                    placeholder="Enter text"
                    input_type={InputType::Text}
                />
                <p>{"Current value: "}{&*text_value}</p>
            </div>
            
            <div style="margin: 1rem 0;">
                <h2>{"Password Input"}</h2>
                <Input
                    value={(*password_value).clone()}
                    onchange={on_password_change}
                    placeholder="Enter password"
                    input_type={InputType::Password}
                />
            </div>
            
            <div style="margin: 1rem 0;">
                <h2>{"Email Input"}</h2>
                <Input
                    value={(*email_value).clone()}
                    onchange={on_email_change}
                    placeholder="Enter email"
                    input_type={InputType::Email}
                />
            </div>
        </div>
    }
}

/// Example component demonstrating card components
#[styled_component(CardExample)]
pub fn card_example() -> Html {
    html! {
        <div>
            <h1>{"Card Examples"}</h1>
            
            <div style="display: flex; flex-wrap: wrap; gap: 1rem;">
                <Card
                    header={html! { <h3>{"Basic Card"}</h3> }}
                    footer={html! { <small>{"Footer content"}</small> }}
                    bordered=true
                    shadow=true
                >
                    <p>{"This is a basic card with header and footer."}</p>
                </Card>
                
                <Card
                    size={CardSize::Small}
                    bordered=true
                >
                    <h3>{"Small Card"}</h3>
                    <p>{"This is a small card with compact padding."}</p>
                </Card>
                
                <Card
                    size={CardSize::Large}
                    shadow=true
                >
                    <h3>{"Large Card"}</h3>
                    <p>{"This is a large card with more padding."}</p>
                    <Button>{"Call to Action"}</Button>
                </Card>
            </div>
        </div>
    }
}

/// Example component demonstrating the new theme provider
#[function_component(NewThemeProviderExample)]
pub fn new_theme_provider_example() -> Html {
    use crate::examples::theme_example::ThemeExample;
    
    html! {
        <ThemeExample />
    }
}

/// Main example component that demonstrates all features
#[styled_component(ExampleApp)]
pub fn example_app() -> Html {
    html! {
        <div>
            <h1>{"UI Toolkit Examples"}</h1>
            
            <section style="margin: 2rem 0;">
                <h2>{"Theme Provider"}</h2>
                <ThemeProviderExample />
            </section>
            
            <section style="margin: 2rem 0;">
                <h2>{"New Theme Provider"}</h2>
                <NewThemeProviderExample />
            </section>
            
            <section style="margin: 2rem 0;">
                <h2>{"Hooks"}</h2>
                <HooksExample />
            </section>
            
            <section style="margin: 2rem 0;">
                <h2>{"Buttons"}</h2>
                <ButtonExample />
            </section>
            
            <section style="margin: 2rem 0;">
                <h2>{"Inputs"}</h2>
                <InputExample />
            </section>
            
            <section style="margin: 2rem 0;">
                <h2>{"Cards"}</h2>
                <CardExample />
            </section>
        </div>
    }
}