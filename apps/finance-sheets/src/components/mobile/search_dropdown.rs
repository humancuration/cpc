//! Mobile-optimized searchable dropdown component
//!
//! This component provides a mobile-optimized version of the searchable dropdown
//! with full-screen implementation and touch-friendly controls.

use yew::prelude::*;
use stylist::yew::use_style;
use stylist::Style;
use gloo_timers::callback::Timeout;
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use crate::services::mobile::{DeviceSize, get_device_size};

/// Properties for the mobile search dropdown component
#[derive(Properties, PartialEq, Clone)]
pub struct MobileSearchDropdownProps<T: PartialEq + Clone + 'static> {
    /// Callback when an item is selected
    pub on_select: Callback<T>,
    
    /// List of items to display in the dropdown
    pub items: Vec<T>,
    
    /// Function to convert an item to a display string
    pub item_to_string: Callback<T, String>,
    
    /// Optional selected item
    #[prop_or(None)]
    pub selected: Option<T>,
    
    /// Placeholder text for the search input
    #[prop_or("Search...".to_string())]
    pub placeholder: String,
    
    /// ARIA label for accessibility
    #[prop_or("Searchable dropdown".to_string())]
    pub aria_label: String,
}

/// State for the mobile search dropdown component
#[derive(Debug, Clone, PartialEq)]
pub struct MobileSearchDropdownState<T: PartialEq + Clone> {
    /// Current search query
    search_query: String,
    
    /// Currently selected item
    selected_item: Option<T>,
    
    /// Whether the dropdown is open
    is_open: bool,
    
    /// Filtered items based on search query
    filtered_items: Vec<T>,
    
    /// Index of the currently highlighted item (for keyboard navigation)
    highlighted_index: Option<usize>,
    
    /// Device size for responsive behavior
    device_size: DeviceSize,
}

/// Messages for the mobile search dropdown component
#[derive(Debug, Clone)]
pub enum MobileSearchDropdownMsg<T: PartialEq + Clone> {
    /// Update the search query
    SearchQueryChanged(String),
    
    /// Toggle the dropdown open/closed
    ToggleDropdown,
    
    /// Close the dropdown
    CloseDropdown,
    
    /// Select an item
    SelectItem(T),
    
    /// Highlight next item (arrow down)
    HighlightNext,
    
    /// Highlight previous item (arrow up)
    HighlightPrevious,
    
    /// Select highlighted item (enter)
    SelectHighlighted,
    
    /// Update device size
    UpdateDeviceSize(DeviceSize),
}

/// Mobile-optimized searchable dropdown component
#[derive(Debug)]
pub struct MobileSearchDropdown<T: PartialEq + Clone + 'static> {
    /// Component state
    state: MobileSearchDropdownState<T>,
    
    /// Debounce timeout for search
    search_timeout: Option<Timeout>,
}

impl<T: PartialEq + Clone + 'static> Component for MobileSearchDropdown<T> {
    type Message = MobileSearchDropdownMsg<T>;
    type Properties = MobileSearchDropdownProps<T>;

    fn create(ctx: &Context<Self>) -> Self {
        let props = ctx.props();
        let filtered_items = props.items.clone();
        let device_size = get_device_size();
        
        Self {
            state: MobileSearchDropdownState {
                search_query: String::new(),
                selected_item: props.selected.clone(),
                is_open: false,
                filtered_items,
                highlighted_index: None,
                device_size,
            },
            search_timeout: None,
        }
    }
    
    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MobileSearchDropdownMsg::SearchQueryChanged(query) => {
                // Cancel any existing timeout
                self.search_timeout = None;
                
                // Update search query immediately
                self.state.search_query = query.clone();
                
                // Set a new timeout to filter items after 300ms
                let link = ctx.link().clone();
                self.search_timeout = Some(Timeout::new(300, move || {
                    link.send_message(MobileSearchDropdownMsg::SearchQueryChanged(query.clone()));
                }));
                
                // Filter items immediately for better UX
                self.filter_items(ctx);
                
                true
            }
            
            MobileSearchDropdownMsg::ToggleDropdown => {
                self.state.is_open = !self.state.is_open;
                if !self.state.is_open {
                    self.state.highlighted_index = None;
                }
                true
            }
            
            MobileSearchDropdownMsg::CloseDropdown => {
                self.state.is_open = false;
                self.state.highlighted_index = None;
                true
            }
            
            MobileSearchDropdownMsg::SelectItem(item) => {
                self.state.selected_item = Some(item.clone());
                self.state.is_open = false;
                self.state.highlighted_index = None;
                ctx.props().on_select.emit(item);
                true
            }
            
            MobileSearchDropdownMsg::HighlightNext => {
                if self.state.is_open && !self.state.filtered_items.is_empty() {
                    let current_index = self.state.highlighted_index.unwrap_or(0);
                    let next_index = if current_index >= self.state.filtered_items.len() - 1 {
                        0
                    } else {
                        current_index + 1
                    };
                    self.state.highlighted_index = Some(next_index);
                    true
                } else {
                    false
                }
            }
            
            MobileSearchDropdownMsg::HighlightPrevious => {
                if self.state.is_open && !self.state.filtered_items.is_empty() {
                    let current_index = self.state.highlighted_index.unwrap_or(0);
                    let prev_index = if current_index == 0 {
                        self.state.filtered_items.len() - 1
                    } else {
                        current_index - 1
                    };
                    self.state.highlighted_index = Some(prev_index);
                    true
                } else {
                    false
                }
            }
            
            MobileSearchDropdownMsg::SelectHighlighted => {
                if let Some(index) = self.state.highlighted_index {
                    if index < self.state.filtered_items.len() {
                        let item = self.state.filtered_items[index].clone();
                        ctx.link().send_message(MobileSearchDropdownMsg::SelectItem(item));
                    }
                }
                true
            }
            
            MobileSearchDropdownMsg::UpdateDeviceSize(device_size) => {
                self.state.device_size = device_size;
                true
            }
        }
    }
    
    fn view(&self, ctx: &Context<Self>) -> Html {
        let style = get_mobile_styles();
        
        let link = ctx.link();
        let props = ctx.props();
        
        let on_input_change = link.callback(|e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            MobileSearchDropdownMsg::SearchQueryChanged(input.value())
        });
        
        let on_toggle = link.callback(|_| MobileSearchDropdownMsg::ToggleDropdown);
        let on_blur = link.callback(|_| MobileSearchDropdownMsg::CloseDropdown);
        
        let on_key_down = link.callback(|e: KeyboardEvent| {
            match e.key().as_str() {
                "ArrowDown" => {
                    e.prevent_default();
                    MobileSearchDropdownMsg::HighlightNext
                }
                "ArrowUp" => {
                    e.prevent_default();
                    MobileSearchDropdownMsg::HighlightPrevious
                }
                "Enter" => {
                    e.prevent_default();
                    MobileSearchDropdownMsg::SelectHighlighted
                }
                "Escape" => {
                    e.prevent_default();
                    MobileSearchDropdownMsg::CloseDropdown
                }
                _ => return MobileSearchDropdownMsg::CloseDropdown, // This won't actually close, just a placeholder
            }
        });
        
        let on_close = link.callback(|_| MobileSearchDropdownMsg::CloseDropdown);
        
        let selected_display = if let Some(item) = &self.state.selected_item {
            props.item_to_string.emit(item.clone())
        } else {
            "Select an option".to_string()
        };
        
        // For mobile devices, show full-screen dropdown when open
        if self.state.device_size == DeviceSize::Mobile && self.state.is_open {
            html! {
                <div class={style}>
                    <div class="mobile-search-dropdown-overlay">
                        <div class="mobile-search-dropdown-fullscreen">
                            <div class="mobile-search-header">
                                <button 
                                    class="mobile-search-close-button"
                                    onclick={on_close}
                                >
                                    {"Cancel"}
                                </button>
                                <h2>{"Select Option"}</h2>
                                <div class="mobile-search-header-spacer"></div>
                            </div>
                            <input
                                type="text"
                                class="mobile-search-input"
                                placeholder={props.placeholder.clone()}
                                value={self.state.search_query.clone()}
                                oninput={on_input_change}
                                onkeydown={on_key_down}
                                aria-autocomplete="list"
                                aria-controls="mobile-dropdown-list"
                                autofocus=true
                            />
                            <ul 
                                class="mobile-dropdown-list" 
                                id="mobile-dropdown-list"
                                role="listbox"
                            >
                                {self.render_dropdown_items(ctx)}
                            </ul>
                        </div>
                    </div>
                </div>
            }
        } else {
            // Desktop or closed state - use regular dropdown
            html! {
                <div class={style}>
                    <div 
                        class="mobile-search-dropdown"
                        aria-label={props.aria_label.clone()}
                    >
                        <div 
                            class="mobile-selected-display"
                            onclick={on_toggle}
                            tabindex="0"
                            role="button"
                            aria-haspopup="listbox"
                            aria-expanded={self.state.is_open.to_string()}
                        >
                            <span class="mobile-selected-text">{selected_display}</span>
                            <span class="mobile-dropdown-arrow">{"▼"}</span>
                        </div>
                        
                        if self.state.is_open {
                            <div class="mobile-dropdown-container">
                                <input
                                    type="text"
                                    class="mobile-search-input-compact"
                                    placeholder={props.placeholder.clone()}
                                    value={self.state.search_query.clone()}
                                    oninput={on_input_change}
                                    onkeydown={on_key_down}
                                    aria-autocomplete="list"
                                    aria-controls="mobile-dropdown-list"
                                />
                                <ul 
                                    class="mobile-dropdown-list-compact" 
                                    id="mobile-dropdown-list"
                                    role="listbox"
                                >
                                    {self.render_dropdown_items(ctx)}
                                </ul>
                            </div>
                        }
                    </div>
                </div>
            }
        }
    }
}

impl<T: PartialEq + Clone + 'static> MobileSearchDropdown<T> {
    /// Filter items based on the search query
    fn filter_items(&mut self, ctx: &Context<Self>) {
        let props = ctx.props();
        let query = self.state.search_query.to_lowercase();
        
        if query.is_empty() {
            self.state.filtered_items = props.items.clone();
        } else {
            self.state.filtered_items = props.items
                .iter()
                .filter(|item| {
                    let item_str = props.item_to_string.emit((*item).clone());
                    item_str.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
        }
        
        // Reset highlighted index when filtering
        self.state.highlighted_index = if self.state.filtered_items.is_empty() {
            None
        } else {
            Some(0)
        };
    }
    
    /// Render the dropdown items
    fn render_dropdown_items(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        let props = ctx.props();
        
        if self.state.filtered_items.is_empty() {
            html! {
                <li class="mobile-no-results">{"No results found"}</li>
            }
        } else {
            html! {
                <>
                    {for self.state.filtered_items.iter().enumerate().map(|(index, item)| {
                        let is_highlighted = self.state.highlighted_index == Some(index);
                        let item_str = props.item_to_string.emit(item.clone());
                        let on_click = link.callback(move |_| MobileSearchDropdownMsg::SelectItem(item.clone()));
                        
                        html! {
                            <li
                                class={classes!("mobile-dropdown-item", is_highlighted.then(|| "highlighted"))}
                                onclick={on_click}
                                role="option"
                                aria-selected={if let Some(selected) = &self.state.selected_item {
                                    props.item_to_string.emit(selected.clone()) == item_str
                                } else {
                                    false
                                }}
                            >
                                {item_str}
                                if let Some(selected) = &self.state.selected_item {
                                    if props.item_to_string.emit(selected.clone()) == item_str {
                                        <span class="mobile-checkmark">{"✓"}</span>
                                    }
                                }
                            </li>
                        }
                    })}
                </>
            }
        }
    }
}

/// Get the CSS styles for the mobile component
fn get_mobile_styles() -> Style {
    use_style!(
        r#"
        .mobile-search-dropdown {
            position: relative;
            width: 100%;
            font-family: Arial, sans-serif;
        }
        
        .mobile-selected-display {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px;
            border: 1px solid #ccc;
            border-radius: 8px;
            background-color: white;
            cursor: pointer;
            min-height: 48px; /* Touch target optimization */
        }
        
        .mobile-selected-display:focus {
            outline: 2px solid #007bff;
            outline-offset: -2px;
        }
        
        .mobile-selected-text {
            flex-grow: 1;
            text-align: left;
            font-size: 16px; /* Prevents zoom on iOS */
        }
        
        .mobile-dropdown-arrow {
            margin-left: 8px;
            color: #666;
        }
        
        .mobile-dropdown-container {
            position: absolute;
            top: 100%;
            left: 0;
            right: 0;
            z-index: 1000;
            border: 1px solid #ccc;
            border-radius: 8px;
            background-color: white;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.15);
            margin-top: 4px;
        }
        
        .mobile-search-input-compact {
            width: 100%;
            padding: 16px;
            border: none;
            border-bottom: 1px solid #eee;
            border-radius: 8px 8px 0 0;
            box-sizing: border-box;
            font-size: 16px; /* Prevents zoom on iOS */
        }
        
        .mobile-search-input-compact:focus {
            outline: none;
        }
        
        .mobile-dropdown-list-compact {
            list-style: none;
            margin: 0;
            padding: 0;
            max-height: 200px;
            overflow-y: auto;
        }
        
        .mobile-dropdown-item {
            padding: 16px;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
            min-height: 48px; /* Touch target optimization */
            font-size: 16px;
        }
        
        .mobile-dropdown-item:hover,
        .mobile-dropdown-item.highlighted {
            background-color: #f5f5f5;
        }
        
        .mobile-checkmark {
            color: #007bff;
            font-weight: bold;
        }
        
        .mobile-no-results {
            padding: 16px;
            color: #666;
            font-style: italic;
        }
        
        /* Full-screen mobile styles */
        .mobile-search-dropdown-overlay {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: rgba(0, 0, 0, 0.5);
            z-index: 2000;
            display: flex;
            justify-content: center;
            align-items: center;
        }
        
        .mobile-search-dropdown-fullscreen {
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            background-color: white;
            z-index: 2001;
            display: flex;
            flex-direction: column;
        }
        
        .mobile-search-header {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 16px;
            border-bottom: 1px solid #eee;
        }
        
        .mobile-search-header h2 {
            margin: 0;
            font-size: 1.25rem;
        }
        
        .mobile-search-close-button {
            background: none;
            border: none;
            padding: 8px 16px;
            font-size: 16px;
            color: #007bff;
            cursor: pointer;
            min-height: 48px; /* Touch target optimization */
        }
        
        .mobile-search-header-spacer {
            width: 64px; /* Spacer to balance the header */
        }
        
        .mobile-search-input {
            padding: 16px;
            border: none;
            border-bottom: 1px solid #eee;
            font-size: 16px; /* Prevents zoom on iOS */
        }
        
        .mobile-search-input:focus {
            outline: none;
        }
        
        .mobile-dropdown-list {
            list-style: none;
            margin: 0;
            padding: 0;
            overflow-y: auto;
            flex: 1;
        }
        
        .mobile-dropdown-item {
            padding: 16px;
            border-bottom: 1px solid #f5f5f5;
            cursor: pointer;
            display: flex;
            justify-content: space-between;
            align-items: center;
            min-height: 48px; /* Touch target optimization */
            font-size: 16px;
        }
        
        .mobile-dropdown-item:hover,
        .mobile-dropdown-item.highlighted {
            background-color: #f5f5f5;
        }
        
        .mobile-checkmark {
            color: #007bff;
            font-weight: bold;
        }
        
        .mobile-no-results {
            padding: 16px;
            color: #666;
            font-style: italic;
        }
    "#
    )
}