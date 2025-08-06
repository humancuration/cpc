# Integration Guide

This guide provides step-by-step instructions for integrating the web_core package into your CPC web application.

## Prerequisites

Before integrating web_core, ensure you have:

1. A Rust development environment set up
2. A CPC web application project using Yew
3. Access to the shared_packages directory

## Installation

1. Add web_core as a dependency in your `Cargo.toml`:

```toml
[dependencies]
web_core = { path = "../shared_packages/web_core" }
```

2. Run `cargo build` to fetch and compile the dependency.

## Authentication Integration

To integrate authentication:

```rust
use web_core::auth::AuthService;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let auth_service = AuthService::new();
    
    let on_login = {
        let auth_service = auth_service.clone();
        Callback::from(move |(username, password)| {
            // Handle login
            // This would typically be done in an async context
        })
    };
    
    html! {
        <div class="app">
            // Your app content
        </div>
    }
}
```

## Component Integration

To use web_core components:

```rust
use web_core::components::{Button, ButtonVariant, Modal, TextInput};
use yew::prelude::*;

#[function_component(MyForm)]
fn my_form() -> Html {
    let modal_open = use_state(|| false);
    
    let on_open_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(true))
    };
    
    let on_close_modal = {
        let modal_open = modal_open.clone();
        Callback::from(move |_| modal_open.set(false))
    };
    
    html! {
        <div class="my-form">
            <Button 
                variant={ButtonVariant::Primary} 
                onclick={on_open_modal}
            >
                {"Open Modal"}
            </Button>
            
            <Modal 
                open={*modal_open}
                onclose={on_close_modal}
                title="My Modal"
            >
                <TextInput 
                    placeholder="Enter your name"
                />
            </Modal>
        </div>
    }
}
```

## API Client Integration

To use the API client:

```rust
use web_core::api_client::ApiClient;
use web_core::utils::error_handling::WebError;
use yew::prelude::*;

#[function_component(DataFetcher)]
fn data_fetcher() -> Html {
    let api_client = ApiClient::new("https://api.example.com".to_string());
    let data = use_state(|| None);
    let loading = use_state(|| false);
    let error = use_state(|| None);
    
    let fetch_data = {
        let api_client = api_client.clone();
        let data = data.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        Callback::from(move |_| {
            let api_client = api_client.clone();
            let data = data.clone();
            let loading = loading.clone();
            let error = error.clone();
            
            wasm_bindgen_futures::spawn_local(async move {
                loading.set(true);
                error.set(None);
                
                let query = "query { users { id name email } }";
                match api_client.graphql_query::<serde_json::Value>(query, None).await {
                    Ok(response) => {
                        if let Some(response_data) = response.data {
                            data.set(Some(response_data));
                        }
                    }
                    Err(e) => {
                        error.set(Some(WebError::ApiError(e)));
                    }
                }
                
                loading.set(false);
            });
        })
    };
    
    html! {
        <div class="data-fetcher">
            <Button onclick={fetch_data}>
                {"Fetch Data"}
            </Button>
            
            if *loading {
                <p>{"Loading..."}</p>
            }
            
            if let Some(err) = &*error {
                <p class="error">{ format!("Error: {:?}", err) }</p>
            }
            
            if let Some(d) = &*data {
                <pre>{ format!("{:#?}", d) }</pre>
            }
        </div>
    }
}
```

## Theme Integration

To use the design system:

```rust
use web_core::theme::DesignSystem;
use stylist::{style, yew::styled_component};
use yew::prelude::*;

#[styled_component(ThemedComponent)]
fn themed_component() -> Html {
    let theme = DesignSystem::default();
    
    let component_style = style!(
        r#"
        background-color: ${primary_color};
        color: ${text_color};
        padding: ${padding};
        border-radius: ${border_radius};
        font-family: ${font_family};
        "#,
        primary_color = theme.colors.primary,
        text_color = theme.colors.white,
        padding = theme.spacing.md,
        border_radius = theme.border_radius.md,
        font_family = theme.typography.font_family,
    );
    
    html! {
        <div class={component_style.get_class_name()}>
            {"This component uses the design system"}
        </div>
    }
}
```

## Error Handling Integration

To implement comprehensive error handling:

```rust
use web_core::components::ErrorBoundary;
use web_core::utils::error_handling::WebError;
use web_core::utils::error_reporting::use_error_reporting;
use yew::prelude::*;

#[function_component(MyApp)]
fn my_app() -> Html {
    let error_reporting = use_error_reporting();
    
    let on_error = {
        let error_reporting = error_reporting.clone();
        Callback::from(move |error: WebError| {
            // Report the error
            // error_reporting.report_error(error, "MyApp").await;
            
            // Log to console
            web_sys::console::error_1(&format!("Error: {:?}", error).into());
        })
    };
    
    html! {
        <ErrorBoundary on_error={on_error}>
            <div class="my-app">
                // Your app content
            </div>
        </ErrorBoundary>
    }
}
```

## Testing Integration

To write tests using web_core utilities:

```rust
#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use web_core::tests::unit::utils::mock_storage::MockStorageAdapter;
    use web_core::auth::{AuthService, User};
    
    #[wasm_bindgen_test]
    fn test_auth_service_with_mock_storage() {
        let storage = MockStorageAdapter::new();
        let auth_service = AuthService::new();
        
        // Test user serialization
        let user = User {
            id: uuid::Uuid::nil(),
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
        };
        
        storage.set_value("current_user", &user).unwrap();
        let retrieved: Option<User> = storage.get_value("current_user").unwrap();
        
        assert_eq!(retrieved.unwrap().username, "testuser");
    }
}
```

## Best Practices

1. **Use the provided components** - They're designed for consistency and accessibility.

2. **Follow the design system** - Use the DesignSystem for consistent styling.

3. **Implement error boundaries** - Wrap components that might fail in ErrorBoundary components.

4. **Use the API client** - It provides offline support, batching, and rate limiting.

5. **Handle errors appropriately** - Use the error handling utilities for consistent error management.

6. **Write tests** - Use the mock utilities for testing components and services.

7. **Keep dependencies up to date** - Regularly update the web_core package to get the latest features and fixes.

8. **Contribute back** - If you add new components or utilities, consider contributing them back to web_core.