//! Virtual scroll component for mobile performance optimization
//!
//! This component provides efficient rendering of large lists by only rendering
//! the visible items and dynamically updating as the user scrolls.

use yew::prelude::*;
use stylist::yew::use_style;
use stylist::Style;

/// Properties for the virtual scroll component
#[derive(Properties, PartialEq)]
pub struct VirtualScrollProps<T: PartialEq + Clone + 'static> {
    /// Total number of items
    pub total_items: usize,
    
    /// Height of each item in pixels
    #[prop_or(48.0)]
    pub item_height: f64,
    
    /// Number of items to render outside the visible area
    #[prop_or(5)]
    pub buffer_items: usize,
    
    /// Callback to render an item
    pub render_item: Callback<(usize, T), Html>,
    
    /// Data for items (passed as a callback to avoid storing large amounts of data)
    pub get_item_data: Callback<usize, T>,
    
    /// Callback when scroll position changes
    #[prop_or_default]
    pub on_scroll: Callback<f64>,
}

/// State for the virtual scroll component
#[derive(Debug, Clone, PartialEq)]
pub struct VirtualScrollState {
    /// Current scroll position
    scroll_top: f64,
    
    /// Container height
    container_height: f64,
    
    /// Container width
    container_width: f64,
}

/// Messages for the virtual scroll component
#[derive(Debug, Clone)]
pub enum VirtualScrollMsg {
    /// Update scroll position
    Scroll(f64),
    
    /// Update container dimensions
    Resize(f64, f64),
}

/// Virtual scroll component
#[derive(Debug)]
pub struct VirtualScroll<T: PartialEq + Clone + 'static> {
    /// Component state
    state: VirtualScrollState,
    
    /// Resize observer for container dimensions
    _resize_observer: Option<gloo_events::EventListener>,
}

impl<T: PartialEq + Clone + 'static> Component for VirtualScroll<T> {
    type Message = VirtualScrollMsg;
    type Properties = VirtualScrollProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        
        // Set up resize observer
        let resize_observer = {
            let link = link.clone();
            gloo_events::EventListener::new(&web_sys::window().unwrap(), "resize", move |_| {
                if let Some(window) = web_sys::window() {
                    let width = window.inner_width().unwrap().as_f64().unwrap_or(0.0);
                    let height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);
                    link.send_message(VirtualScrollMsg::Resize(width, height));
                }
            })
        };
        
        Self {
            state: VirtualScrollState {
                scroll_top: 0.0,
                container_height: 0.0,
                container_width: 0.0,
            },
            _resize_observer: Some(resize_observer),
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            VirtualScrollMsg::Scroll(scroll_top) => {
                self.state.scroll_top = scroll_top;
                ctx.props().on_scroll.emit(scroll_top);
                true
            }
            
            VirtualScrollMsg::Resize(width, height) => {
                self.state.container_width = width;
                self.state.container_height = height;
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_virtual_scroll_styles();
        let props = ctx.props();
        let link = ctx.link();
        
        // Calculate visible range
        let visible_height = self.state.container_height.max(400.0); // Minimum 400px
        let start_index = (self.state.scroll_top / props.item_height).floor() as usize;
        let visible_items_count = (visible_height / props.item_height).ceil() as usize;
        let end_index = std::cmp::min(
            start_index + visible_items_count + props.buffer_items * 2,
            props.total_items
        );
        let start_index = start_index.saturating_sub(props.buffer_items);
        
        // Calculate offset for the visible items
        let offset_top = start_index as f64 * props.item_height;
        
        // Calculate total height
        let total_height = props.total_items as f64 * props.item_height;
        
        // Scroll handler
        let on_scroll = link.callback(|e: Event| {
            let target: web_sys::HtmlElement = e.target_unchecked_into();
            VirtualScrollMsg::Scroll(target.scroll_top() as f64)
        });
        
        html! {
            <div class={style}>
                <div 
                    class="virtual-scroll-container"
                    onscroll={on_scroll}
                >
                    <div 
                        class="virtual-scroll-content"
                        style={format!("height: {}px", total_height)}
                    >
                        <div 
                            class="virtual-scroll-offset"
                            style={format!("transform: translateY({}px)", offset_top)}
                        >
                            {for (start_index..end_index).map(|index| {
                                let item_data = props.get_item_data.emit(index);
                                props.render_item.emit((index, item_data))
                            })}
                        </div>
                    </div>
                </div>
            </div>
        }
    }
    
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            // Initialize container dimensions
            if let Some(window) = web_sys::window() {
                let width = window.inner_width().unwrap().as_f64().unwrap_or(0.0);
                let height = window.inner_height().unwrap().as_f64().unwrap_or(0.0);
                ctx.link().send_message(VirtualScrollMsg::Resize(width, height));
            }
        }
    }
}

/// Get the CSS styles for the virtual scroll component
fn get_virtual_scroll_styles() -> Style {
    use_style!(
        r#"
        .virtual-scroll-container {
            width: 100%;
            height: 100%;
            overflow-y: auto;
            position: relative;
            -webkit-overflow-scrolling: touch; /* Momentum scrolling on iOS */
        }
        
        .virtual-scroll-content {
            position: relative;
        }
        
        .virtual-scroll-offset {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
        }
    "#
    )
}