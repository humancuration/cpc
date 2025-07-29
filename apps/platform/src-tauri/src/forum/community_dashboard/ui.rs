use yew::prelude::*;

#[function_component(CommunityDashboard)]
pub fn community_dashboard() -> Html {
    html! {
        <div>
            <h1>{ "Community Dashboard" }</h1>
            <p>{ "Here you will find tools for managing a community." }</p>
        </div>
    }
}