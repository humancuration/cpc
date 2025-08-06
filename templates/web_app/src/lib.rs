use yew::prelude::*;
use yew_router::prelude::*;
use web_core::components::{Button, ButtonVariant};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <main>
                <nav>
                    <ul>
                        <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                        <li><Link<Route> to={Route::About}>{"About"}</Link<Route>></li>
                    </ul>
                </nav>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <HomePage /> },
        Route::About => html! { <AboutPage /> },
        Route::NotFound => html! { <h1>{"404 - Not Found"}</h1> },
    }
}

#[function_component(HomePage)]
fn home_page() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <div>
            <h1>{"Welcome to the CPC Web App Template"}</h1>
            <p>{"This is a minimal starter template for CPC web applications."}</p>
            <div>
                <p>{"Counter: "} {*counter}</p>
                <Button onclick={onclick}>{"Increment"}</Button>
            </div>
        </div>
    }
}

#[function_component(AboutPage)]
fn about_page() -> Html {
    html! {
        <div>
            <h1>{"About"}</h1>
            <p>{"This template includes:"}</p>
            <ul>
                <li>{"Preconfigured Trunk.toml"}</li>
                <li>{"Basic routing setup"}</li>
                <li>{"Example component using web_core"}</li>
                <li>{"Auth integration example"}</li>
            </ul>
        </div>
    }
}