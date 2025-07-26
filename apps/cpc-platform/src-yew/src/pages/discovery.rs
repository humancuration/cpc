use yew::prelude::*;
use yew_hooks::use_async;
use wasm_bindgen_futures::spawn_local;
use serde::{Deserialize, Serialize};

use crate::components::discovery::feed::Feed;
use crate::components::discovery::video_player::VideoPlayer;
use crate::components::discovery::product_preview::ProductPreview;
use crate::components::discovery::engagement::Engagement;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum DiscoveryView {
    Feed,
    Item(String),
}

#[function_component(DiscoveryPage)]
pub fn discovery_page() -> Html {
    let items = use_state(|| vec![]);
    let current_item = use_state(|| None);
    let loading = use_state(|| false);
    let error = use_state(|| None::<String>);
    let view = use_state(|| DiscoveryView::Feed);
    
    // Fetch discovery feed
    let fetch_feed = {
        let items = items.clone();
        let loading = loading.clone();
        let error = error.clone();
        
        use_async(async move {
            loading.set(true);
            
            let response = fetch_discovery_feed().await;
            match response {
                Ok(data) => {
                    items.set(data);
                    loading.set(false);
                    Ok(())
                }
                Err(e) => {
                    error.set(Some(e.to_string()));
                    loading.set(false);
                    Err(e)
                }
            }
        })
    };
    
    // Fetch on component mount
    {
        let fetch_feed = fetch_feed.clone();
        use_effect_with((), move |_| {
            fetch_feed.run();
        });
    }
    
    let on_item_click = {
        let current_item = current_item.clone();
        let view = view.clone();
        
        Callback::from(move |item_id: String| {
            view.set(DiscoveryView::Item(item_id.clone()));
            spawn_local(async move {
                if let Ok(item) = fetch_discovery_item(&item_id).await {
                    current_item.set(Some(item));
                }
            });
        })
    };
    
    let on_back_to_feed = {
        let view = view.clone();
        Callback::from(move |_| {
            view.set(DiscoveryView::Feed);
        })
    };
    
    let on_engagement = {
        let items = items.clone();
        let fetch_feed = fetch_feed.clone();
        
        Callback::from(move |(item_id, action): (String, String)| {
            let items = items.clone();
            let fetch_feed = fetch_feed.clone();
            
            spawn_local(async move {
                match action.as_str() {
                    "like" => {
                        let _ = like_discovery_item(&item_id, "current_user").await;
                    }
                    "unlike" => {
                        let _ = unlike_discovery_item(&item_id, "current_user").await;
                    }
                    "save" => {
                        let _ = save_discovery_item(&item_id, "current_user").await;
                    }
                    "unsave" => {
                        let _ = unsave_discovery_item(&item_id, "current_user").await;
                    }
                    _ => {}
                }
                
                // Refresh feed
                fetch_feed.run();
            });
        })
    };
    
    html! {
        <div class="discovery-page">
            <div class="discovery-header">
                <h1>{ "Discover Amazing Products" }</h1>
                <p>{ "Explore unique items from cooperatives around the world" }</p>
            </div>
            
            if *loading {
                <div class="loading">
                    <div class="spinner"></div>
                    <p>{ "Loading amazing products..." }</p>
                </div>
            } else if let Some(err) = &*error {
                <div class="error">
                    <p>{ format!("Error: {}", err) }</p>
                    <button onclick={|_| fetch_feed.run()}>{ "Retry" }</button>
                </div>
            } else {
                match &*view {
                    DiscoveryView::Feed => {
                        <Feed 
                            items={(*items).clone()}
                            on_item_click={on_item_click}
                            on_engagement={on_engagement}
                        />
                    }
                    DiscoveryView::Item(item_id) => {
                        <div class="item-detail">
                            <button class="back-button" onclick={on_back_to_feed}>
                                { "← Back to Feed" }
                            </button>
                            if let Some(item) = &*current_item {
                                <div class="item-content">
                                    <VideoPlayer 
                                        video_url={item.video_url.clone()}
                                        thumbnail_url={item.thumbnail_url.clone()}
                                    />
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
                                        on_engagement={on_engagement}
                                    />
                                </div>
                            }
                        </div>
                    }
                }
            }
        </div>
    }
}

// API client functions
async fn fetch_discovery_feed() -> Result<Vec<DiscoveryItem>, anyhow::Error> {
    // TODO: Replace with actual GraphQL query
    Ok(vec![
        DiscoveryItem {
            id: "1".to_string(),
            product_id: "p1".to_string(),
            title: "Handcrafted Ceramic Mug".to_string(),
            description: "Beautiful handcrafted ceramic mug made by local artisans".to_string(),
            price: 25.0,
            currency: "USD".to_string(),
            vendor_id: "v1".to_string(),
            vendor_name: "Local Artisans Co-op".to_string(),
            vendor_reputation: 4.8,
            video_url: "https://videos.cpc.local/mug-demo.webm".to_string(),
            thumbnail_url: "https://images.cpc.local/mug-thumb.jpg".to_string(),
            created_at: 1609459200,
            likes: 142,
            saves: 89,
            comments: 23,
            shares: 45,
            view_count: 1247,
            tags: vec!["ceramic".to_string(), "handmade".to_string(), "mug".to_string()],
            category: "Home & Kitchen".to_string(),
            is_liked: false,
            is_saved: false,
        },
        DiscoveryItem {
            id: "2".to_string(),
            product_id: "p2".to_string(),
            title: "Organic Cotton T-Shirt".to_string(),
            description: "Sustainably sourced organic cotton t-shirt from worker-owned coop".to_string(),
            price: 35.0,
            currency: "USD".to_string(),
            vendor_id: "v2".to_string(),
            vendor_name: "Sustainable Threads Co-op".to_string(),
            vendor_reputation: 4.9,
            video_url: "https://videos.cpc.local/tshirt-demo.webm".to_string(),
            thumbnail_url: "https://images.cpc.local/tshirt-thumb.jpg".to_string(),
            created_at: 1609459200,
            likes: 267,
            saves: 134,
            comments: 42,
            shares: 89,
            view_count: 2341,
            tags: vec!["organic".to_string(), "cotton".to_string(), "tshirt".to_string()],
            category: "Clothing".to_string(),
            is_liked: false,
            is_saved: false,
        },
    ])
}

async fn fetch_discovery_item(item_id: &str) -> Result<DiscoveryItem, anyhow::Error> {
    let items = fetch_discovery_feed().await?;
    items.into_iter()
        .find(|item| item.id == item_id)
        .ok_or_else(|| anyhow::anyhow!("Item not found"))
}

async fn like_discovery_item(item_id: &str, user_id: &str) -> Result<(), anyhow::Error> {
    // TODO: Replace with actual GraphQL mutation
    Ok(())
}

async fn unlike_discovery_item(item_id: &str, user_id: &str) -> Result<(), anyhow::Error> {
    // TODO: Replace with actual GraphQL mutation
    Ok(())
}

async fn save_discovery_item(item_id: &str, user_id: &str) -> Result<(), anyhow::Error> {
    // TODO: Replace with actual GraphQL mutation
    Ok(())
}

async fn unsave_discovery_item(item_id: &str, user_id: &str) -> Result<(), anyhow::Error> {
    // TODO: Replace with actual GraphQL mutation
    Ok(())
}