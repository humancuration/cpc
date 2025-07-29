use yew::prelude::*;
use yew_hooks::use_state;
use wasm_bindgen_futures::spawn_local;

use crate::graphql::mutations::{
    like_discovery_item,
    save_discovery_item,
    add_comment,
};

#[derive(Properties, PartialEq, Clone)]
pub struct EngagementActionsProps {
    pub item: super::feed::DiscoveryItem,
    pub on_like: Callback<()>,
    pub on_save: Callback<()>,
    pub on_share: Callback<()>,
}

#[function_component(EngagementActions)]
pub fn engagement_actions(props: &EngagementActionsProps) -> Html {
    let item = props.item.clone();
    let show_comments = use_state(|| false);
    let new_comment = use_state(|| String::new());
    let comments = use_state(|| Vec::<Comment>::new());
    let is_liking = use_state(|| false);
    let is_saving = use_state(|| false);
    
    // Handle like action
    let handle_like = {
        let item = item.clone();
        let is_liking = is_liking.clone();
        let on_like = props.on_like.clone();
        
        Callback::from(move |_| {
            if *is_liking {
                return;
            }
            
            is_liking.set(true);
            spawn_local(async move {
                match like_discovery_item(&item.id).await {
                    Ok(_) => {
                        on_like.emit(());
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to like item: {:?}", err).into());
                    }
                }
                is_liking.set(false);
            });
        })
    };
    
    // Handle save action
    let handle_save = {
        let item = item.clone();
        let is_saving = is_saving.clone();
        let on_save = props.on_save.clone();
        
        Callback::from(move |_| {
            if *is_saving {
                return;
            }
            
            is_saving.set(true);
            spawn_local(async move {
                match save_discovery_item(&item.id).await {
                    Ok(_) => {
                        on_save.emit(());
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to save item: {:?}", err).into());
                    }
                }
                is_saving.set(false);
            });
        })
    };
    
    // Handle share action
    let handle_share = {
        let item = item.clone();
        let on_share = props.on_share.clone();
        
        Callback::from(move |_| {
            if let Some(navigator) = web_sys::window().and_then(|w| w.navigator()) {
                let share_data = web_sys::ShareData::new();
                share_data.set_title(&item.title);
                share_data.set_text(&item.description);
                share_data.set_url(&format!("https://cpc.coop/discovery/{}", item.id));
                
                let _ = navigator.share_with_data(&share_data);
            }
            on_share.emit(());
        })
    };
    
    // Handle comment toggle
    let toggle_comments = {
        let show_comments = show_comments.clone();
        Callback::from(move |_| {
            show_comments.set(!*show_comments);
        })
    };
    
    // Handle add comment
    let handle_add_comment = {
        let item = item.clone();
        let new_comment = new_comment.clone();
        let comments = comments.clone();
        
        Callback::from(move |_| {
            let comment_text = (*new_comment).clone();
            if comment_text.trim().is_empty() {
                return;
            }
            
            spawn_local(async move {
                match add_comment(&item.id, &comment_text).await {
                    Ok(comment) => {
                        comments.update(|prev| {
                            prev.push(comment);
                        });
                        new_comment.set(String::new());
                    }
                    Err(err) => {
                        web_sys::console::error_1(&format!("Failed to add comment: {:?}", err).into());
                    }
                }
            });
        })
    };
    
    // Handle comment input
    let handle_comment_input = {
        let new_comment = new_comment.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                new_comment.set(input.value());
            }
        })
    };
    
    html! {
        <div class="flex flex-col items-center space-y-4">
            // Like button
            <div class="flex flex-col items-center">
                <button
                    onclick={handle_like}
                    disabled={*is_liking}
                    class={format!(
                        "p-3 rounded-full transition-colors {}",
                        if item.is_liked {
                            "bg-red-500 text-white"
                        } else {
                            "bg-white/20 text-white hover:bg-white/30"
                        }
                    )}
                >
                    <svg class="w-6 h-6" fill="currentColor" viewBox="0 0 20 20">
                        <path fill-rule="evenodd" d="M3.172 5.172a4 4 0 015.656 0L10 6.343l1.172-1.171a4 4 0 115.656 5.656L10 17.657l-6.828-6.829a4 4 0 010-5.656z" clip-rule="evenodd" />
                    </svg>
                </button>
                <span class="text-white text-xs mt-1">{ item.likes }</span>
            </div>
            
            // Comments button
            <div class="flex flex-col items-center">
                <button
                    onclick={toggle_comments}
                    class="p-3 rounded-full bg-white/20 text-white hover:bg-white/30 transition-colors"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
                    </svg>
                </button>
                <span class="text-white text-xs mt-1">{ item.comments }</span>
            </div>
            
            // Save button
            <div class="flex flex-col items-center">
                <button
                    onclick={handle_save}
                    disabled={*is_saving}
                    class={format!(
                        "p-3 rounded-full transition-colors {}",
                        if item.is_saved {
                            "bg-yellow-500 text-white"
                        } else {
                            "bg-white/20 text-white hover:bg-white/30"
                        }
                    )}
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5a2 2 0 012-2h10a2 2 0 012 2v16l-7-3.5L5 21V5z" />
                    </svg>
                </button>
                <span class="text-white text-xs mt-1">{ item.saves }</span>
            </div>
            
            // Share button
            <div class="flex flex-col items-center">
                <button
                    onclick={handle_share}
                    class="p-3 rounded-full bg-white/20 text-white hover:bg-white/30 transition-colors"
                >
                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m9.632 4.684C18.886 16.938 19 17.482 19 18c0 1.657-1.343 3-3 3s-3-1.343-3-3 1.343-3 3-3c.482 0 .938.114 1.342.316m0-5.684a3 3 0 110-2.684M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                    </svg>
                </button>
                <span class="text-white text-xs mt-1">{ "Share" }</span>
            </div>
            
            // Comments modal
            if *show_comments {
                <div class="fixed inset-0 bg-black/50 z-50 flex items-end">
                    <div class="bg-white w-full max-h-96 rounded-t-2xl">
                        <div class="p-4 border-b">
                            <div class="flex justify-between items-center">
                                <h3 class="font-semibold">{ "Comments" }</h3>
                                <button
                                    onclick={toggle_comments}
                                    class="text-gray-500 hover:text-gray-700"
                                >
                                    <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>
                            </div>
                        </div>
                        
                        <div class="p-4 space-y-3 overflow-y-auto max-h-64">
                            { for (*comments).iter().map(|comment| {
                                html! {
                                    <div class="flex space-x-3">
                                        <div class="w-8 h-8 bg-gray-300 rounded-full"></div>
                                        <div>
                                            <p class="text-sm font-medium">{ &comment.author }</p>
                                            <p class="text-sm text-gray-600">{ &comment.content }</p>
                                        </div>
                                    </div>
                                }
                            })}
                        </div>
                        
                        <div class="p-4 border-t">
                            <div class="flex space-x-2">
                                <input
                                    type="text"
                                    placeholder="Add a comment..."
                                    value={(*new_comment).clone()}
                                    oninput={handle_comment_input}
                                    onkeypress={Callback::from(move |e: KeyboardEvent| {
                                        if e.key() == "Enter" {
                                            handle_add_comment.emit(());
                                        }
                                    })}
                                    class="flex-1 px-3 py-2 border rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                                />
                                <button
                                    onclick={handle_add_comment}
                                    disabled={(*new_comment).trim().is_empty()}
                                    class="px-4 py-2 bg-blue-500 text-white rounded-lg hover:bg-blue-600 disabled:opacity-50"
                                >
                                    { "Post" }
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            }
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Comment {
    pub id: String,
    pub author: String,
    pub content: String,
    pub created_at: String,
}