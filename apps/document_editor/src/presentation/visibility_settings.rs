use yew::prelude::*;
use web_sys::HtmlSelectElement;
use shared_packages::collaborative_docs::core::Visibility;

#[derive(Properties, PartialEq)]
pub struct VisibilitySettingsProps {
    pub current_visibility: Visibility,
    pub on_visibility_change: Callback<Visibility>,
}

#[function_component(VisibilitySettings)]
pub fn visibility_settings(props: &VisibilitySettingsProps) -> Html {
    let on_change = {
        let on_visibility_change = props.on_visibility_change.clone();
        Callback::from(move |e: Event| {
            let select: HtmlSelectElement = e.target_unchecked_into();
            let value = select.value();
            
            let visibility = match value.as_str() {
                "public" => Visibility::Public,
                "friends_only" => Visibility::FriendsOnly,
                "private" => Visibility::Private,
                _ => Visibility::Private,
            };
            
            on_visibility_change.emit(visibility);
        })
    };
    
    let visibility_value = match &props.current_visibility {
        Visibility::Public => "public",
        Visibility::FriendsOnly => "friends_only",
        Visibility::Private => "private",
    };
    
    html! {
        <div class="visibility-settings">
            <label for="visibility-select">{"Visibility: "}</label>
            <select 
                id="visibility-select"
                onchange={on_change}
                value={visibility_value}
            >
                <option value="public">{"Public"}</option>
                <option value="friends_only">{"Friends Only"}</option>
                <option value="private">{"Private"}</option>
            </select>
        </div>
    }
}