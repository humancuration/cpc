use yew::prelude::*;

#[function_component(VotingWidget)]
pub fn voting_widget() -> Html {
    html! {
        <div>
            <h1>{ "Voting Widget" }</h1>
            <p>{ "Upvote/downvote buttons will be here." }</p>
        </div>
    }
}