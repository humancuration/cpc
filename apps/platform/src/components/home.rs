use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::AppRoute;
use crate::cooperative::routing::CooperativeRoute;
use crate::supply_chain::routing::SupplyChainRoute;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <h1>{ "CPC Platform" }</h1>
            <nav>
                <ul>
                    <li>
                        <Link<AppRoute> to={AppRoute::Discovery}>
                            { "Discover Products" }
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::SupplyChain(SupplyChainRoute::List)}>
                            { "Supply Chain" }
                        </Link<AppRoute>>
                    </li>
                    <li>
                        <Link<AppRoute> to={AppRoute::Cooperative(CooperativeRoute::List)}>
                            { "Cooperatives" }
                        </Link<AppRoute>>
                    </li>
                </ul>
            </nav>
        </div>
    }
}