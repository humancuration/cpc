use yew::prelude::*;

#[function_component(CommentTree)]
pub fn comment_tree() -> Html {
    html! {
        <div>
            <h1>{ "Comment Tree" }</h1>
            <p>{ "Here you will see a nested tree of comments." }</p>
        </div>
    }
}