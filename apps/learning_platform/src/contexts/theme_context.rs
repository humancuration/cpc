use yew::prelude::*;
use stylist::{style, yew::styled_component};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub enum Theme {
    Light,
    Dark,
}

#[derive(Debug, Clone)]
pub enum ThemeAction {
    Toggle,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeState {
    pub theme: Theme,
}

impl Reducible for ThemeState {
    type Action = ThemeAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ThemeAction::Toggle => {
                let new_theme = match self.theme {
                    Theme::Light => Theme::Dark,
                    Theme::Dark => Theme::Light,
                };
                Rc::new(ThemeState {
                    theme: new_theme,
                })
            }
        }
    }
}

#[function_component(ThemeContextProvider)]
pub fn theme_context_provider(props: &ChildrenProperties) -> Html {
    let theme_state = use_reducer(|| ThemeState {
        theme: Theme::Light,
    });

    let theme_style = style!(
        r#"
        :root {
            --primary: #4361ee;
            --secondary: #3f37c9;
            --background: #ffffff;
            --surface: #f8f9fa;
            --text: #2b2d42;
            --text-secondary: #8d99ae;
            --success: #4caf50;
            --warning: #ff9800;
            --error: #f44336;
            --border: #e0e0e0;
        }

        [data-theme="dark"] {
            --primary: #4cc9f0;
            --secondary: #4895ef;
            --background: #121212;
            --surface: #1e1e1e;
            --text: #f8f9fa;
            --text-secondary: #b0b0b0;
            --success: #66bb6a;
            --warning: #ffa726;
            --error: #ef5350;
            --border: #444444;
        }

        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
            background-color: var(--background);
            color: var(--text);
            transition: background-color 0.3s, color 0.3s;
        }
    "#
    ).unwrap();

    html! {
        <ContextProvider<UseReducerHandle<ThemeState>> context={theme_state}>
            <div class={theme_style} data-theme={match theme_state.theme {
                Theme::Light => "light",
                Theme::Dark => "dark",
            }}>
                {props.children.clone()}
            </div>
        </ContextProvider<UseReducerHandle<ThemeState>>>
    }
}

pub fn use_theme() -> UseReducerHandle<ThemeState> {
    use_context::<UseReducerHandle<ThemeState>>()
        .expect("Theme context is missing")
}