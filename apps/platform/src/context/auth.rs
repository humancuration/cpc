use yew::prelude::*;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct AuthContext {
    pub user_id: Option<String>,
    pub organization_id: Option<Uuid>,
    pub is_authenticated: bool,
    pub loading: bool,
}

impl Default for AuthContext {
    fn default() -> Self {
        Self {
            user_id: None,
            organization_id: None,
            is_authenticated: false,
            loading: true,
        }
    }
}

#[derive(Properties, PartialEq)]
pub struct AuthProviderProps {
    pub children: Children,
}

#[function_component(AuthProvider)]
pub fn auth_provider(props: &AuthProviderProps) -> Html {
    let auth = use_state(|| AuthContext::default());
    
    // In a real app, this would fetch from auth service
    use_effect_with_deps(
        move |_| {
            // Simulate auth check
            let auth = auth.clone();
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate loading delay
                gloo_timers::future::TimeoutFuture::new(100).await;
                
                // For now, use a test user ID
                auth.set(AuthContext {
                    user_id: Some("test-user-123".to_string()),
                    organization_id: Some(Uuid::new_v4()),
                    is_authenticated: true,
                    loading: false,
                });
            });
            || ()
        },
        (),
    );

    html! {
        <ContextProvider<AuthContext> context={(*auth).clone()}>
            {props.children.clone()}
        </ContextProvider<AuthContext>>
    }
}

pub fn use_auth() -> AuthContext {
    use_context::<AuthContext>().unwrap_or_default()
}
