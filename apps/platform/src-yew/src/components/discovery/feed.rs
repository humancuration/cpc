use yew::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::components::discovery::video_player::VideoPlayer;
use crate::components::discovery::product_preview::ProductPreview;
use crate::components::discovery::engagement::Engagement;

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveryItem {
    pub id: String,
    pub product_id: String,
    pub title: String,
    pub description: String,
    pub price: f64,
    pub currency: String,
    pub vendor_id: String,
    pub vendor_name: String,
    pub vendor_reputation: f64,
    pub video_url: String,
    pub thumbnail_url: String,
    pub created_at: u64,
    pub likes: u32,
    pub saves: u32,
    pub comments: u32,
    pub shares: u32,
    pub view_count: u32,
    pub tags: Vec<String>,
    pub category: String,
    pub is_liked: bool,
    pub is_saved: bool,
}

#[derive(Properties, PartialEq)]
pub struct FeedProps {
    pub items: Vec<DiscoveryItem>,
    pub on_item_click: Callback<String>,
    pub on_engagement: Callback<(String, String)>,
}

#[function_component(Feed)]
pub fn feed(props: &FeedProps) -> Html {
    let current_item = use_state(|| 0usize);
    let is_intersecting = use_state(|| true);
    
    // Intersection observer for lazy loading
    {
        let is_intersecting = is_intersecting.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move |entries: Vec<JsValue>| {
                for entry in entries {
                    let intersection_entry: web_sys::IntersectionObserverEntry = 
                        entry.into();
                    is_intersecting.set(intersection_entry.is_intersecting());
                }
            }) as Box<dyn FnMut(Vec<JsValue>)>);
            
            let observer = web_sys::IntersectionObserver::new(closure.as_ref().unchecked_ref())
                .unwrap();
            
            // Clean up
            move || {
                drop(observer);
                drop(closure);
            }
        });
    }
    
    let handle_item_click = {
        let on_item_click = props.on_item_click.clone();
        Callback::from(move |item_id: String| {
            on_item_click.emit(item_id);
        })
    };
    
    let handle_engagement = {
        let on_engagement = props.on_engagement.clone();
        Callback::from(move |(item_id, action): (String, String)| {
            on_engagement.emit((item_id, action));
        })
    };
    
    let handle_next = {
        let current_item = current_item.clone();
        let items_len = props.items.len();
        Callback::from(move |_| {
            current_item.set((*current_item + 1) % items_len);
        })
    };
    
    let handle_prev = {
        let current_item = current_item.clone();
        let items_len = props.items.len();
        Callback::from(move |_| {
            current_item.set((*current_item + items_len - 1) % items_len);
        })
    };
    
    if props.items.is_empty() {
        return html! {
            <div class="feed-container">
                <p>{ "No items to display" }</p>
            </div>
        };
    }
    
    let item = &props.items[*current_item];
    
    html! {
        <div class="feed-container">
            <div class="feed-item">
                <div class="video-section" onclick={handle_item_click.reform(|_| item.id.clone())}>
                    <VideoPlayer 
                        video_url={item.video_url.clone()}
                        thumbnail_url={item.thumbnail_url.clone()}
                    />
                </div>
                
                <div class="product-section">
                    <ProductPreview 
                        title={item.title.clone()}
                        description={item.description.clone()}
                        price={item.price}
                        currency={item.currency.clone()}
                        vendor_name={item.vendor_name.clone()}
                        vendor_reputation={item.vendor_reputation}
                    />
                    
                    <Engagement 
                        item_id={item.id.clone()}
                        likes={item.likes}
                        saves={item.saves}
                        comments={item.comments}
                        shares={item.shares}
                        is_liked={item.is_liked}
                        is_saved={item.is_saved}
                        on_engagement={handle_engagement}
                    />
                </div>
                
                <div class="navigation">
                    <button class="nav-button prev" onclick={handle_prev}>
                        { "←" }
                    </button>
                    <span class="counter">
                        { format!("{}/{}", *current_item + 1, props.items.len()) }
                    </span>
                    <button class="nav-button next" onclick={handle_next}>
                        { "→" }
                    </button>
                </div>
            </div>
        </div>
    }
}