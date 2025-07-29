use yew::prelude::*;

#[function_component(ThreadComposer)]
pub fn thread_composer() -> Html {
    html! {
        <div>
            <h1>{ "Thread Composer" }</h1>
            <p>{ "Here you will be able to compose a new thread." }</p>
        </div>
    }
}