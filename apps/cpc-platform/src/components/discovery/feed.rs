use yew::prelude::*;
use yew_hooks::{use_infinite_scroll, use_state};
use wasm_bindgen_futures::spawn_local;
use web_sys::{HtmlElement, TouchEvent, MouseEvent};
use std::collections::HashMap;

use crate::components::discovery::{ProductVideoPreview, EngagementActions};
use crate::graphql::queries::discovery_feed_query;

#[derive(Properties, PartialEq, Clone)]
pub struct DiscoveryFeedProps {
    pub user_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DiscoveryItem {
    pub id: String,
    pub product_id: String,
    pub video_url: String,
    pub thumbnail_url: String,
    pub title: String,
    pub description: String,
    pub vendor_name: String,
    pub price: f64,
    pub likes: u32,
    pub saves: u32,
    pub comments: u32,
    pub is_liked: bool,
    pub is_saved: bool,
    pub created_at: String,
    pub vendor_reputation: f64,
    pub engagement_score: f64,
}

#[function_component(DiscoveryFeed)]
pub fn discovery_feed(props: &DiscoveryFeedProps) -> Html {
    let items = use_state(|| Vec::<DiscoveryItem>::new());
    let current_index = use_state(|| 0);
    let loading = use_state(|| false);
    let has_more = use_state(|| true);
    let touch_start = use_state(|| None::<i32>);
    let touch_end = use_state(|| None::<i32>);
    
    let container_ref = use_node_ref();
    let observer_ref = use_node_ref();

    // Fetch discovery feed items
    let fetch_items = {
        let items = items.clone();
        let loading = loading.clone();
        let has_more = has_more.clone();
        let user_id = props.user_id.clone();
        
        Callback::from(move |_| {
            if *loading {
                return;
            }
            
            loading.set(true);
            spawn_local(async move {
                match discovery_feed_query(&user_id, items.len(), 10).await {
                    Ok(new_items) => {
                        if new_items.is_empty() {
                            has_more.set(false);
                        } else {
                            items.update(|prev| {
                                prev.extend(new_items);
                            });
                        }
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to fetch discovery items: {:?}", err).into());
                    }
                }
                loading.set(false);
            });
        })
    };

    // Handle touch events for swipe gestures
    let handle_touch_start = {
        let touch_start = touch_start.clone();
        Callback::from(move |e: TouchEvent| {
            if let Some(touch) = e.touches().get(0) {
                touch_start.set(Some(touch.client_y()));
            }
        })
    };

    let handle_touch_end = {
        let touch_start = touch_start.clone();
        let touch_end = touch_end.clone();
        let current_index = current_index.clone();
        let items = items.clone();
        
        Callback::from(move |e: TouchEvent| {
            if let Some(touch) = e.changed_touches().get(0) {
                let end_y = touch.client_y();
                if let Some(start_y) = *touch_start {
                    let delta = start_y - end_y;
                    
                    // Swipe threshold
                    if delta.abs() > 50 {
                        if delta > 0 && *current_index < items.len().saturating_sub(1) {
                            // Swipe up - next item
                            current_index.update(|idx| *idx + 1);
                        } else if delta < 0 && *current_index > 0 {
                            // Swipe down - previous item
                            current_index.update(|idx| *idx - 1);
                        }
                    }
                }
            }
            touch_start.set(None);
        })
    };

    // Handle mouse wheel for desktop
    let handle_wheel = {
        let current_index = current_index.clone();
        let items = items.clone();
        
        Callback::from(move |e: WheelEvent| {
            e.prevent_default();
            
            if e.delta_y() > 0 && *current_index < items.len().saturating_sub(1) {
                // Scroll down - next item
                current_index.update(|idx| *idx + 1);
            } else if e.delta_y() < 0 && *current_index > 0 {
                // Scroll up - previous item
                current_index.update(|idx| *idx - 1);
            }
        })
    };

    // Handle keyboard navigation
    let handle_keydown = {
        let current_index = current_index.clone();
        let items = items.clone();
        
        Callback::from(move |e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    if *current_index < items.len().saturating_sub(1) {
                        current_index.update(|idx| *idx + 1);
                    }
                }
                "ArrowUp" => {
                    e.prevent_default();
                    if *current_index > 0 {
                        current_index.update(|idx| *idx - 1);
                    }
                }
                _ => {}
            }
        })
    };

    // Infinite scroll setup
    let observer = {
        let fetch_items = fetch_items.clone();
        use_effect_with_deps(move |_| {
            if let Some(container) = container_ref.cast::<HtmlElement>() {
                let observer = web_sys::IntersectionObserver::new(
                    Closure::wrap(Box::new(move |entries| {
                        if let Some(entry) = entries.get(0) {
                            let entry = entry.unchecked_into::<web_sys::IntersectionObserverEntry>();
                            if entry.is_intersecting() {
                                fetch_items.emit(());
                            }
                        }
                    }) as Box<dyn FnMut(_)>)
                        .into_js_value()
                        .unchecked_ref(),
                ).unwrap();
                
                if let Some(observer_target) = observer_ref.cast::<HtmlElement>() {
                    observer.observe(&observer_target);
                }
                
                || ()
            } else {
                || ()
            }
        }, ());
    };

    // Initial load
    {
        let fetch_items = fetch_items.clone();
        use_effect_with_deps(move |_| {
            fetch_items.emit(());
            || ()
        }, ());
    }

    let current_item = items.get(*current_index).cloned();
    
    html! {
        <div 
            class="h-screen w-full overflow-hidden relative bg-black"
            on_touch_start={handle_touch_start}
            on_touch_end={handle_touch_end}
            on_wheel={handle_wheel}
            on_key_down={handle_keydown}
            tabindex="0"
            ref={container_ref}
        >
            if let Some(item) = current_item {
                <div class="h-full w-full">
                    <ProductVideoPreview 
                        item={item.clone()}
                        is_active={true}
                    />
                    
                    <div class="absolute bottom-0 left-0 right-0 pb-16">
                        <EngagementActions 
                            item={item}
                            on_like={Callback::from(move |_| {
                                // Handle like action
                            })}
                            on_save={Callback::from(move |_| {
                                // Handle save action
                            })}
                            on_share={Callback::from(move |_| {
                                // Handle share action
                            })}
                        />
                    </div>
                </div>
            } else if *loading {
                <div class="flex justify-center items-center h-full">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-white"></div>
                </div>
            } else {
                <div class="flex justify-center items-center h-full text-white">
                    <p>{ "No items to display" }</p>
                </div>
            }
            
            // Loading indicator for infinite scroll
            <div 
                ref={observer_ref}
                class="absolute bottom-4 left-0 right-0 flex justify-center"
            >
                if *loading && *has_more {
                    <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-white"></div>
                }
            </div>
            
            // Progress indicator
            <div class="absolute top-4 left-0 right-0 flex justify-center space-x-1">
                { for (0..items.len()).map(|i| {
                    let is_active = i == *current_index;
                    html! {
                        <div 
                            class={format!(
                                "h-1 rounded-full transition-all duration-300 {}",
                                if is_active { "w-8 bg-white" } else { "w-2 bg-white/50" }
                            )}
                        />
                    }
                })}
            </div>
        </div>
    }
}