use yew::prelude::*;
use yew_hooks::use_state;
use wasm_bindgen_futures::spawn_local;

use crate::components::discovery::VideoPlayer;
use crate::components::product::display::ProductDisplay;

#[derive(Properties, PartialEq, Clone)]
pub struct ProductVideoPreviewProps {
    pub item: super::feed::DiscoveryItem,
    pub is_active: bool,
}

#[function_component(ProductVideoPreview)]
pub fn product_video_preview(props: &ProductVideoPreviewProps) -> Html {
    let item = props.item.clone();
    let is_active = props.is_active;
    
    let show_product_details = use_state(|| false);
    let video_loaded = use_state(|| false);
    
    // Auto-play when active
    {
        let is_active = is_active;
        let video_loaded = video_loaded.clone();
        
        use_effect_with_deps(move |_| {
            if is_active {
                video_loaded.set(true);
            }
            || ()
        }, is_active);
    }
    
    // Handle product details toggle
    let toggle_product_details = {
        let show_product_details = show_product_details.clone();
        Callback::from(move |_| {
            show_product_details.set(!*show_product_details);
        })
    };
    
    // Handle video events
    let handle_video_ended = {
        Callback::from(move |_| {
            // Auto-advance to next video or show product details
        })
    };
    
    let handle_video_loaded = {
        Callback::from(move |_| {
            // Video loaded successfully
        })
    };
    
    let handle_time_update = {
        Callback::from(move |time: f64| {
            // Track video progress for analytics
        })
    };
    
    html! {
        <div class="relative w-full h-full">
            // Video player
            <VideoPlayer
                src={item.video_url.clone()}
                poster={item.thumbnail_url.clone()}
                autoplay={is_active}
                muted={false}
                on_ended={handle_video_ended}
                on_loaded={handle_video_loaded}
                on_time_update={handle_time_update}
            />
            
            // Product info overlay
            <div class="absolute bottom-20 left-0 right-0 p-4">
                <div class="bg-gradient-to-t from-black/80 via-black/60 to-transparent p-4 rounded-lg">
                    <h3 class="text-white font-bold text-lg mb-1">{ &item.title }</h3>
                    <p class="text-white/90 text-sm mb-2 line-clamp-2">{ &item.description }</p>
                    
                    <div class="flex items-center justify-between">
                        <div class="flex items-center space-x-2">
                            <span class="text-white font-bold text-xl">
                                { format!("${:.2}", item.price) }
                            </span>
                            <span class="text-white/70 text-sm">
                                { format!("by {}", item.vendor_name) }
                            </span>
                        </div>
                        
                        <button
                            onclick={toggle_product_details}
                            class="bg-white/20 backdrop-blur-sm text-white px-4 py-2 rounded-full text-sm font-medium hover:bg-white/30 transition-colors"
                        >
                            { "View Details" }
                        </button>
                    </div>
                </div>
            </div>
            
            // Shop Now CTA overlay
            <div class="absolute bottom-32 right-4">
                <button class="bg-yellow-500 text-black px-6 py-3 rounded-full font-bold text-sm shadow-lg hover:bg-yellow-400 transition-colors transform hover:scale-105">
                    { "Shop Now" }
                </button>
            </div>
            
            // Product details modal (slide-up)
            if *show_product_details {
                <div 
                    class="absolute inset-0 bg-black/50 backdrop-blur-sm flex items-end z-50"
                    onclick={toggle_product_details.clone()}
                >
                    <div 
                        class="bg-white w-full max-h-3/4 rounded-t-2xl p-6 transform transition-transform duration-300"
                        onclick={Callback::from(|e| e.stop_propagation())}
                    >
                        <div class="flex justify-between items-center mb-4">
                            <h2 class="text-xl font-bold">{ "Product Details" }</h2>
                            <button
                                onclick={toggle_product_details}
                                class="text-gray-500 hover:text-gray-700"
                            >
                                <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12" />
                                </svg>
                            </button>
                        </div>
                        
                        // Product details would go here
                        // This is where we'd integrate with the existing ProductDisplay
                        <div class="space-y-4">
                            <div>
                                <h3 class="text-lg font-semibold">{ &item.title }</h3>
                                <p class="text-gray-600">{ &item.description }</p>
                            </div>
                            
                            <div>
                                <span class="text-2xl font-bold text-green-600">
                                    { format!("${:.2}", item.price) }
                                </span>
                            </div>
                            
                            <div>
                                <p class="text-sm text-gray-500">
                                    { format!("Sold by {}", item.vendor_name) }
                                </p>
                                <p class="text-sm text-gray-500">
                                    { format!("Vendor reputation: {:.1}/5.0", item.vendor_reputation) }
                                </p>
                            </div>
                            
                            <button class="w-full bg-green-600 text-white py-3 rounded-lg font-semibold hover:bg-green-700 transition-colors">
                                { "Add to Cart" }
                            </button>
                        </div>
                    </div>
                </div>
            }
            
            // Loading indicator
            if !*video_loaded && is_active {
                <div class="absolute inset-0 flex items-center justify-center bg-black">
                    <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-white"></div>
                </div>
            }
        </div>
    }
}