use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    html! {
        <nav class="main-menu">
            <ul>
                <li><Link<Route> to={Route::Home}>{"Home"}</Link<Route>></li>
                <li><Link<Route> to={Route::Discovery}>{"Discover"}</Link<Route>></li>
                <li><Link<Route> to={Route::ProductScan}>{"Scan Product"}</Link<Route>></li>
                <li>
                    <Link<Route> to={Route::ProductDetails { id: "PRODUCT_ID".to_string() }}>
                        {"Product Details (Example)"}
                    </Link<Route>>
                </li>
            </ul>
        </nav>
    }
}