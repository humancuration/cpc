use yew::prelude::*;
use stylist::{style, yew::styled_component};

#[derive(Properties, PartialEq)]
pub struct ErrorBoundaryProps {
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ErrorState {
    pub has_error: bool,
    pub error_message: String,
}

impl Reducible for ErrorState {
    type Action = ();

    fn reduce(self: std::rc::Rc<Self>, _action: Self::Action) -> std::rc::Rc<Self> {
        std::rc::Rc::new(ErrorState {
            has_error: true,
            error_message: "An unexpected error occurred".to_string(),
        })
    }
}

#[styled_component(ErrorBoundary)]
pub fn error_boundary(props: &ErrorBoundaryProps) -> Html {
    let error_state = use_state(|| ErrorState {
        has_error: false,
        error_message: String::new(),
    });

    let container_style = style!(
        r#"
        padding: 2rem;
        text-align: center;
        max-width: 600px;
        margin: 2rem auto;
        background: var(--surface);
        border-radius: 8px;
        box-shadow: 0 2px 10px rgba(0,0,0,0.1);
    "#
    ).unwrap();

    let heading_style = style!(
        r#"
        color: var(--error);
        margin-top: 0;
    "#
    ).unwrap();

    let button_style = style!(
        r#"
        background: var(--primary);
        color: white;
        border: none;
        padding: 0.75rem 1.5rem;
        border-radius: 4px;
        cursor: pointer;
        font-weight: bold;
        margin-top: 1rem;

        &:hover {
            background: var(--secondary);
        }
    "#
    ).unwrap();

    let on_retry = {
        let error_state = error_state.clone();
        Callback::from(move |_| {
            error_state.set(ErrorState {
                has_error: false,
                error_message: String::new(),
            });
        })
    };

    if error_state.has_error {
        html! {
            <div class={container_style}>
                <h2 class={heading_style}>{"Oops! Something went wrong."}</h2>
                <p>{&error_state.error_message}</p>
                <button class={button_style} onclick={on_retry}>
                    {"Try Again"}
                </button>
            </div>
        }
    } else {
        // In a real implementation, we would catch errors from children
        // For now, we just render the children
        html! {
            <>
                {for props.children.iter()}
            </>
        }
    }
}