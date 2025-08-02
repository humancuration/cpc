use yew::prelude::*;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct AuthState {
    pub user_id: Option<String>,
    pub is_authenticated: bool,
}

#[derive(Debug, Clone)]
pub enum AuthAction {
    Login(String),
    Logout,
}

pub type AuthContext = UseReducerHandle<AuthState>;

#[function_component(AuthContextProvider)]
pub fn auth_context_provider(props: &ChildrenProperties) -> Html {
    let auth_state = use_reducer(|| AuthState {
        user_id: None,
        is_authenticated: false,
    });

    html! {
        <ContextProvider<UseReducerHandle<AuthState>> context={auth_state}>
            {props.children.clone()}
        </ContextProvider<UseReducerHandle<AuthState>>>
    }
}

pub fn use_auth() -> UseReducerHandle<AuthState> {
    use_context::<UseReducerHandle<AuthState>>()
        .expect("Auth context is missing")
}