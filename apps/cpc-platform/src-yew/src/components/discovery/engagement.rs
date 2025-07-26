use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct EngagementProps {
    pub item_id: String,
    pub likes: u32,
    pub saves: u32,
    pub comments: u32,
    pub shares: u32,
    pub is_liked: bool,
    pub is_saved: bool,
    pub on_engagement: Callback<(String, String)>,
}

#[function_component(Engagement)]
pub fn engagement(props: &EngagementProps) -> Html {
    let handle_like = {
        let item_id = props.item_id.clone();
        let on_engagement = props.on_engagement.clone();
        let is_liked = props.is_liked;
        
        Callback::from(move |_| {
            let action = if is_liked { "unlike" } else { "like" };
            on_engagement.emit((item_id.clone(), action.to_string()));
        })
    };
    
    let handle_save = {
        let item_id = props.item_id.clone();
        let on_engagement = props.on_engagement.clone();
        let is_saved = props.is_saved;
        
        Callback::from(move |_| {
            let action = if is_saved { "unsave" } else { "save" };
            on_engagement.emit((item_id.clone(), action.to_string()));
        })
    };
    
    let handle_comment = {
        let item_id = props.item_id.clone();
        let on_engagement = props.on_engagement.clone();
        
        Callback::from(move |_| {
            on_engagement.emit((item_id.clone(), "comment".to_string()));
        })
    };
    
    let handle_share = {
        let item_id = props.item_id.clone();
        let on_engagement = props.on_engagement.clone();
        
        Callback::from(move |_| {
            on_engagement.emit((item_id.clone(), "share".to_string()));
        })
    };
    
    html! {
        <div class="engagement-actions">
            <div class="engagement-button-group">
                <button 
                    class={classes!("engagement-btn", props.is_liked.then(|| "active"))}
                    onclick={handle_like}
                    aria-label="Like"
                >
                    <span class="icon">{ if props.is_liked { "❤️" } else { "🤍" } }</span>
                    <span class="count">{ props.likes }</span>
                </button>
                
                <button 
                    class={classes!("engagement-btn", props.is_saved.then(|| "active"))}
                    onclick={handle_save}
                    aria-label="Save"
                >
                    <span class="icon">{ if props.is_saved { "🔖" } else { "📌" } }</span>
                    <span class="count">{ props.saves }</span>
                </button>
                
                <button 
                    class="engagement-btn"
                    onclick={handle_comment}
                    aria-label="Comment"
                >
                    <span class="icon">{ "💬" }</span>
                    <span class="count">{ props.comments }</span>
                </button>
                
                <button 
                    class="engagement-btn"
                    onclick={handle_share}
                    aria-label="Share"
                >
                    <span class="icon">{ "📤" }</span>
                    <span class="count">{ props.shares }</span>
                </button>
            </div>
        </div>
    }
}